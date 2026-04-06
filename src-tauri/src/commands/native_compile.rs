use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use typst::World;

const TYPST_VERSION: &str = "0.14.2";

use crate::world::ZenypstWorld;

/// Lightweight glyph position record built from the compiled document.
struct TextPos {
    page: usize,
    x_pt: f64, // glyph center x in typst coordinates (pt, top-left origin)
    y_pt: f64, // baseline y
    line: u32, // 1-indexed source line
    col: u32,  // 1-indexed source column
}

pub struct NativeCompilerState {
    pub world: Arc<Mutex<ZenypstWorld>>,
    /// Glyph positions extracted after each successful compilation.
    positions: Arc<Mutex<Vec<TextPos>>>,
}

impl NativeCompilerState {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(ZenypstWorld::new())),
            positions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCompileError {
    pub severity: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
    pub source_line: Option<String>,
    pub hints: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeRenderResult {
    pub success: bool,
    pub pdf: Option<String>, // base64-encoded PDF
    pub errors: Vec<NativeCompileError>,
    pub warnings: Vec<NativeCompileError>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PdfExportResult {
    pub success: bool,
    pub errors: Vec<NativeCompileError>,
    pub warnings: Vec<NativeCompileError>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub line: u32,
    pub col: u32,
}

fn diag_to_error(
    diag: &typst::diag::SourceDiagnostic,
    world: &ZenypstWorld,
) -> NativeCompileError {
    let severity = match diag.severity {
        typst::diag::Severity::Error => "error",
        typst::diag::Severity::Warning => "warning",
    }
    .to_string();

    let message = diag.message.to_string();
    let hints: Vec<String> = diag.hints.iter().map(|h| h.to_string()).collect();

    let mut file: Option<String> = None;
    let mut line: Option<u32> = None;
    let mut column: Option<u32> = None;
    let mut end_line: Option<u32> = None;
    let mut end_column: Option<u32> = None;
    let mut source_line: Option<String> = None;

    if let Some(id) = diag.span.id() {
        file = Some(id.vpath().as_rootless_path().display().to_string());
        if let Ok(src) = world.source(id) {
            if let Some(range) = src.range(diag.span) {
                let lines = src.lines();
                if let Some(ln) = lines.byte_to_line(range.start) {
                    line = Some(ln as u32 + 1);
                    source_line = src.text().lines().nth(ln).map(|l| l.to_string());
                }
                if let Some(col) = lines.byte_to_column(range.start) {
                    column = Some(col as u32 + 1);
                }
                if let Some(ln) = lines.byte_to_line(range.end) {
                    end_line = Some(ln as u32 + 1);
                }
                if let Some(col) = lines.byte_to_column(range.end) {
                    end_column = Some(col as u32 + 1);
                }
            }
        }
    }

    NativeCompileError { severity, message, file, line, column, end_line, end_column, source_line, hints }
}

fn make_pdf_options() -> typst_pdf::PdfOptions<'static> {
    use typst::foundations::Smart;
    typst_pdf::PdfOptions {
        ident: Smart::Auto,
        timestamp: None,
        page_ranges: None,
        standards: typst_pdf::PdfStandards::default(),
        tagged: false,
    }
}

/// Convert a UTF-16 code-unit offset within `text` to a UTF-8 byte offset.
fn utf16_offset_to_byte(text: &str, utf16_offset: usize) -> usize {
    if utf16_offset == 0 {
        return 0;
    }
    let mut utf16_count = 0usize;
    for (byte_idx, ch) in text.char_indices() {
        if utf16_count >= utf16_offset {
            return byte_idx;
        }
        utf16_count += ch.len_utf16();
    }
    text.len()
}

/// Resolve a glyph's (span, utf16_offset_in_span) to exact (line, col) in source (1-indexed).
fn resolve_glyph(span: typst::syntax::Span, utf16_offset: u16, world: &ZenypstWorld) -> Option<(u32, u32)> {
    let id = span.id()?;
    let source = world.source(id).ok()?;
    let range = source.range(span)?;
    // Advance by the glyph's UTF-16 offset within the span to get the exact character
    let span_text = source.text().get(range.start..range.end)?;
    let byte_in_span = utf16_offset_to_byte(span_text, utf16_offset as usize);
    let abs_byte = range.start + byte_in_span;
    let lines = source.lines();
    let line = lines.byte_to_line(abs_byte)? as u32 + 1;
    let col = lines.byte_to_column(abs_byte)? as u32 + 1;
    Some((line, col))
}

/// Recursively collect glyph positions from a frame.
fn collect_positions(
    frame: &typst::layout::Frame,
    page: usize,
    dx: f64,
    dy: f64,
    world: &ZenypstWorld,
    out: &mut Vec<TextPos>,
) {
    use typst::layout::FrameItem;
    for (pos, item) in frame.items() {
        let ax = dx + pos.x.to_pt();
        let ay = dy + pos.y.to_pt();
        match item {
            FrameItem::Text(text) => {
                let size_pt = text.size.to_pt();
                let mut cursor_x = ax;
                for glyph in &text.glyphs {
                    let advance_pt = glyph.x_advance.get() * size_pt;
                    let x_offset_pt = glyph.x_offset.get() * size_pt;
                    let cx = cursor_x + x_offset_pt + advance_pt * 0.5;
                    if let Some((line, col)) = resolve_glyph(glyph.span.0, glyph.span.1, world) {
                        out.push(TextPos { page, x_pt: cx, y_pt: ay, line, col });
                    }
                    cursor_x += advance_pt;
                }
            }
            FrameItem::Group(group) => {
                collect_positions(&group.frame, page, ax, ay, world, out);
            }
            _ => {}
        }
    }
}

/// Build the glyph position map from a compiled document.
fn build_positions(
    doc: &typst::layout::PagedDocument,
    world: &ZenypstWorld,
) -> Vec<TextPos> {
    let mut positions = Vec::new();
    for (page_idx, page) in doc.pages.iter().enumerate() {
        collect_positions(&page.frame, page_idx, 0.0, 0.0, world, &mut positions);
    }
    positions
}

/// Compile content, returning (pdf_bytes_or_errors, warnings, optional_doc).
fn compile_to_pdf(
    world: &mut ZenypstWorld,
    content: String,
    root: Option<std::path::PathBuf>,
) -> (
    Result<(Vec<u8>, typst::layout::PagedDocument), Vec<NativeCompileError>>,
    Vec<NativeCompileError>,
) {
    use typst::diag::Warned;
    use typst::layout::PagedDocument;

    world.set_source(content, root);

    let Warned { output, warnings } = typst::compile::<PagedDocument>(world);

    let warn_list: Vec<NativeCompileError> = warnings
        .iter()
        .map(|w| diag_to_error(w, world))
        .collect();

    match output {
        Err(errors) => {
            let err_list = errors.iter().map(|e| diag_to_error(e, world)).collect();
            (Err(err_list), warn_list)
        }
        Ok(doc) => {
            let options = make_pdf_options();
            match typst_pdf::pdf(&doc, &options) {
                Ok(bytes) => (Ok((bytes, doc)), warn_list),
                Err(errors) => {
                    let msgs: Vec<String> =
                        errors.iter().map(|e| e.message.to_string()).collect();
                    let err = NativeCompileError {
                        severity: "error".into(),
                        message: format!("PDF generation failed: {}", msgs.join("; ")),
                        file: None,
                        line: None,
                        column: None,
                        end_line: None,
                        end_column: None,
                        source_line: None,
                        hints: vec![],
                    };
                    (Err(vec![err]), warn_list)
                }
            }
        }
    }
}

/// Return the built-in typst version string.
#[tauri::command]
pub fn get_typst_version() -> String {
    format!("typst {} (built-in)", TYPST_VERSION)
}

/// Compile typst source in-process and emit the resulting PDF via event.
#[tauri::command]
pub async fn compile_native(
    content: String,
    root: Option<String>,
    state: tauri::State<'_, NativeCompilerState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let world = Arc::clone(&state.world);
    let positions_store = Arc::clone(&state.positions);

    tokio::task::spawn_blocking(move || {
        let root_path = root.map(std::path::PathBuf::from);
        let mut guard = world.lock().unwrap();

        let (result, warnings) = compile_to_pdf(&mut guard, content, root_path);

        match result {
            Err(errors) => {
                let _ = app_handle.emit(
                    "typst-native-result",
                    NativeRenderResult { success: false, pdf: None, errors, warnings },
                );
            }
            Ok((bytes, doc)) => {
                // Build and store glyph position map for locate_source
                let new_positions = build_positions(&doc, &*guard);
                *positions_store.lock().unwrap() = new_positions;

                let pdf_b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                let _ = app_handle.emit(
                    "typst-native-result",
                    NativeRenderResult {
                        success: true,
                        pdf: Some(pdf_b64),
                        errors: vec![],
                        warnings,
                    },
                );
            }
        }
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Compile typst source in-process and write the result to a PDF file.
#[tauri::command]
pub async fn export_pdf(
    content: String,
    root: Option<String>,
    output_path: String,
    state: tauri::State<'_, NativeCompilerState>,
) -> Result<PdfExportResult, String> {
    let world = Arc::clone(&state.world);

    tokio::task::spawn_blocking(move || {
        let root_path = root.map(std::path::PathBuf::from);
        let mut guard = world.lock().unwrap();

        let (result, warnings) = compile_to_pdf(&mut guard, content, root_path);

        match result {
            Err(errors) => Ok(PdfExportResult { success: false, errors, warnings }),
            Ok((bytes, _doc)) => {
                std::fs::write(&output_path, &bytes)
                    .map_err(|e| format!("Failed to write PDF: {}", e))?;
                Ok(PdfExportResult { success: true, errors: vec![], warnings })
            }
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Find the source location (line, col) nearest to the given point in the PDF.
/// Coordinates are in typst pt units with top-left origin.
#[tauri::command]
pub async fn locate_source(
    page_index: usize,
    x_pt: f64,
    y_pt: f64,
    state: tauri::State<'_, NativeCompilerState>,
) -> Result<Option<SourceLocation>, String> {
    let positions = state.positions.lock().unwrap();
    let best = positions
        .iter()
        .filter(|p| p.page == page_index)
        .min_by(|a, b| {
            let da = (a.x_pt - x_pt).powi(2) + (a.y_pt - y_pt).powi(2);
            let db = (b.x_pt - x_pt).powi(2) + (b.y_pt - y_pt).powi(2);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });
    Ok(best.map(|p| SourceLocation { line: p.line, col: p.col }))
}
