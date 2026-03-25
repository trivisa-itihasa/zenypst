use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::Emitter;
use typst::World;

use crate::world::ZenypstWorld;

pub struct NativeCompilerState {
    pub world: Arc<Mutex<ZenypstWorld>>,
}

impl NativeCompilerState {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(ZenypstWorld::new())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCompileError {
    pub severity: String,
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeRenderResult {
    pub success: bool,
    pub pages: Vec<String>, // base64-encoded PNG data
    pub errors: Vec<NativeCompileError>,
    pub warnings: Vec<NativeCompileError>,
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

    // Try to resolve location from span
    let (line, column) = diag
        .span
        .id()
        .and_then(|id| world.source(id).ok())
        .and_then(|src: typst::syntax::Source| {
            let range = src.range(diag.span)?;
            let lines = src.lines();
            let ln = lines.byte_to_line(range.start)? as u32 + 1;
            let col = lines.byte_to_column(range.start)? as u32 + 1;
            Some((ln, col))
        })
        .map(|(l, c)| (Some(l), Some(c)))
        .unwrap_or((None, None));

    NativeCompileError {
        severity,
        message,
        line,
        column,
    }
}

/// Compile typst source in-process and emit rendered PNG pages via event.
#[tauri::command]
pub async fn compile_native(
    content: String,
    root: Option<String>,
    state: tauri::State<'_, NativeCompilerState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let world = Arc::clone(&state.world);

    tokio::task::spawn_blocking(move || {
        use typst::diag::Warned;
        use typst::layout::PagedDocument;

        let root_path = root.map(std::path::PathBuf::from);

        let mut guard = world.lock().unwrap();
        guard.set_source(content, root_path);

        let Warned { output, warnings } = typst::compile::<PagedDocument>(&*guard);

        let warn_list: Vec<NativeCompileError> = warnings
            .iter()
            .map(|w| diag_to_error(w, &*guard))
            .collect();

        match output {
            Err(errors) => {
                let err_list: Vec<NativeCompileError> = errors
                    .iter()
                    .map(|e| diag_to_error(e, &*guard))
                    .collect();

                let _ = app_handle.emit(
                    "typst-native-result",
                    NativeRenderResult {
                        success: false,
                        pages: vec![],
                        errors: err_list,
                        warnings: warn_list,
                    },
                );
            }
            Ok(doc) => {
                // Render each page to PNG at 2x resolution (144 dpi)
                let pages: Vec<String> = doc
                    .pages
                    .iter()
                    .filter_map(|page| {
                        let pixmap = typst_render::render(page, 2.0);
                        pixmap
                            .encode_png()
                            .ok()
                            .map(|png| base64::engine::general_purpose::STANDARD.encode(&png))
                    })
                    .collect();

                let _ = app_handle.emit(
                    "typst-native-result",
                    NativeRenderResult {
                        success: true,
                        pages,
                        errors: vec![],
                        warnings: warn_list,
                    },
                );
            }
        }
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
