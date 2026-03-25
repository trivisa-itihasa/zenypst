use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Tauri はシェルを経由しないため ~/.cargo/bin 等が PATH に含まれない。
/// よく使われるインストール先を含めた拡張 PATH で typst を探す。
fn find_typst() -> String {
    // 追加で検索するディレクトリ
    let extra_dirs = [
        ".cargo/bin",
        ".local/bin",
        "bin",
    ];

    if let Some(home) = dirs::home_dir() {
        for rel in &extra_dirs {
            let candidate = home.join(rel).join("typst");
            if candidate.is_file() {
                return candidate.to_string_lossy().to_string();
            }
        }
    }

    // 絶対パスで見つからなければ名前だけ返す（PATH が通っていれば動く）
    "typst".to_string()
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileError {
    pub severity: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    pub success: bool,
    pub pdf_path: Option<String>,
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileError>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WatchEvent {
    pub success: bool,
    pub pdf_path: Option<String>,
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileError>,
}

/// typst watch プロセスのハンドルを保持する状態
pub struct WatchState {
    pub child: std::sync::Mutex<Option<tokio::process::Child>>,
}

impl WatchState {
    pub fn new() -> Self {
        Self {
            child: std::sync::Mutex::new(None),
        }
    }
}

fn get_zenypst_temp_dir() -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or_else(|| std::env::temp_dir());
    base.join("zenypst").join("temp")
}

/// Get the temporary input file path used for compilation.
#[tauri::command]
pub async fn get_temp_input_path() -> Result<String, String> {
    let dir = get_zenypst_temp_dir();
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;
    Ok(dir.join("input.typ").to_string_lossy().to_string())
}

/// Get the temporary output PDF path used for compilation.
#[tauri::command]
pub async fn get_temp_output_path() -> Result<String, String> {
    let dir = get_zenypst_temp_dir();
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;
    Ok(dir.join("output.pdf").to_string_lossy().to_string())
}

/// Build a typst Command, using the user-specified path if non-empty.
fn typst_command_with_path(typst_path: &str) -> Command {
    let bin = if !typst_path.is_empty() {
        typst_path.to_string()
    } else {
        find_typst()
    };
    let mut cmd = Command::new(&bin);
    let current_path = std::env::var("PATH").unwrap_or_default();
    let extra = if let Some(home) = dirs::home_dir() {
        format!(
            "{}/.cargo/bin:{}/.local/bin:/usr/local/bin:{}",
            home.display(), home.display(), current_path
        )
    } else {
        format!("/usr/local/bin:{}", current_path)
    };
    cmd.env("PATH", extra);
    cmd
}

/// Check if the `typst` CLI is installed and available in PATH.
#[tauri::command]
pub async fn check_typst_installed(typst_path: Option<String>) -> Result<bool, String> {
    let path = typst_path.unwrap_or_default();
    let result = typst_command_with_path(&path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;
    Ok(result.is_ok() && result.unwrap().status.success())
}

/// Get the Typst CLI version string.
#[tauri::command]
pub async fn get_typst_version(typst_path: Option<String>) -> Result<String, String> {
    let path = typst_path.unwrap_or_default();
    let output = typst_command_with_path(&path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run typst: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Compile a Typst file to PDF (manual mode).
#[tauri::command]
pub async fn compile_typst(input: String, output: String, typst_path: Option<String>) -> Result<CompileResult, String> {
    // Ensure output directory exists
    if let Some(parent) = PathBuf::from(&output).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let path = typst_path.unwrap_or_default();
    let result = typst_command_with_path(&path)
        .arg("compile")
        .arg(&input)
        .arg(&output)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run typst compile: {}", e))?;

    let stderr = String::from_utf8_lossy(&result.stderr).to_string();
    let success = result.status.success();

    let mut errors: Vec<CompileError> = Vec::new();
    let mut warnings: Vec<CompileError> = Vec::new();

    if !stderr.is_empty() {
        parse_typst_output(&stderr, &mut errors, &mut warnings);
    }

    Ok(CompileResult {
        success,
        pdf_path: if success { Some(output) } else { None },
        errors,
        warnings,
    })
}

/// Start a `typst watch` process for real-time or on-save preview modes.
/// Compilation results are emitted as `typst-watch-result` Tauri events.
#[tauri::command]
pub async fn start_typst_watch(
    input: String,
    output: String,
    typst_path: Option<String>,
    state: tauri::State<'_, WatchState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Stop any existing watch process
    let existing = {
        let mut guard = state.child.lock().unwrap();
        guard.take()
    };
    if let Some(mut child) = existing {
        let _ = child.kill().await;
    }

    // Ensure output directory exists
    if let Some(parent) = PathBuf::from(&output).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Ensure input file exists (typst watch fails if the input file is absent)
    if !tokio::fs::try_exists(&input).await.unwrap_or(false) {
        tokio::fs::write(&input, b"")
            .await
            .map_err(|e| format!("Failed to create temp input file: {}", e))?;
    }

    let path = typst_path.unwrap_or_default();
    let mut child = typst_command_with_path(&path)
        .arg("watch")
        .arg(&input)
        .arg(&output)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start typst watch: {}", e))?;

    let stderr = child.stderr.take().unwrap();
    let output_path = output.clone();

    // Store child handle for later cleanup
    {
        let mut guard = state.child.lock().unwrap();
        *guard = Some(child);
    }

    // Spawn background task to parse stderr and emit events
    tokio::spawn(async move {
        use tauri::Emitter;

        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();

        let mut errors: Vec<CompileError> = Vec::new();
        let mut warnings: Vec<CompileError> = Vec::new();
        let mut current_severity: Option<String> = None;
        let mut current_message: Option<String> = None;
        let mut current_file: Option<String> = None;
        let mut current_line_num: Option<u32> = None;
        let mut current_column: Option<u32> = None;
        let mut watcher_ready = false;

        while let Ok(Some(line)) = lines.next_line().await {
            let trimmed = line.trim().to_string();

            // Detect that the watcher started successfully
            if !watcher_ready && (trimmed.contains("watching") || trimmed.contains("Watching")) {
                watcher_ready = true;
                let _ = app_handle.emit("typst-watch-ready", ());
            }

            if trimmed.starts_with("error:") || trimmed.starts_with("warning:") {
                // Flush previous entry
                if let Some(sev) = current_severity.take() {
                    flush_entry(
                        sev, &mut current_message, &mut current_file,
                        &mut current_line_num, &mut current_column,
                        &mut errors, &mut warnings,
                    );
                }
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                current_severity = Some(parts[0].to_string());
                current_message = parts.get(1).map(|s| s.trim().to_string());
            } else if trimmed.starts_with("┌─") || trimmed.contains(".typ:") {
                if let Some(loc_start) = trimmed.find(".typ:") {
                    let loc_part = &trimmed[loc_start..];
                    let parts: Vec<&str> = loc_part.splitn(3, ':').collect();
                    if !parts.is_empty() {
                        let file_end = loc_start + parts[0].len();
                        let file_start = trimmed[..loc_start]
                            .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' && c != '/')
                            .map(|i| i + 1)
                            .unwrap_or(0);
                        current_file = Some(trimmed[file_start..file_end].to_string());
                        if parts.len() >= 2 {
                            current_line_num = parts[1].trim().parse().ok();
                        }
                        if parts.len() >= 3 {
                            current_column = parts[2].trim()
                                .split_whitespace().next()
                                .and_then(|s| s.parse().ok());
                        }
                    }
                }
            } else if trimmed.contains("compiled in") || trimmed.contains("compiled successfully") {
                // Flush any pending entry
                if let Some(sev) = current_severity.take() {
                    flush_entry(
                        sev, &mut current_message, &mut current_file,
                        &mut current_line_num, &mut current_column,
                        &mut errors, &mut warnings,
                    );
                }

                // "compiled in Xs with N error(s)" → failure; "compiled in Xs" → success
                let has_errors = !errors.is_empty() || trimmed.contains("error");

                let event = WatchEvent {
                    success: !has_errors,
                    pdf_path: if !has_errors { Some(output_path.clone()) } else { None },
                    errors: std::mem::take(&mut errors),
                    warnings: std::mem::take(&mut warnings),
                };
                let _ = app_handle.emit("typst-watch-result", event);
            }
        }
        // Process exited; notify TypeScript so it can fall back to compile mode
        let _ = app_handle.emit("typst-watch-died", ());
    });

    Ok(())
}

/// Stop the running `typst watch` process.
#[tauri::command]
pub async fn stop_typst_watch(
    state: tauri::State<'_, WatchState>,
) -> Result<(), String> {
    let existing = {
        let mut guard = state.child.lock().unwrap();
        guard.take()
    };
    if let Some(mut child) = existing {
        let _ = child.kill().await;
    }
    Ok(())
}

fn flush_entry(
    severity: String,
    message: &mut Option<String>,
    file: &mut Option<String>,
    line: &mut Option<u32>,
    column: &mut Option<u32>,
    errors: &mut Vec<CompileError>,
    warnings: &mut Vec<CompileError>,
) {
    let entry = CompileError {
        severity: severity.clone(),
        message: message.take().unwrap_or_default(),
        file: file.take(),
        line: line.take(),
        column: column.take(),
    };
    if severity == "error" {
        errors.push(entry);
    } else {
        warnings.push(entry);
    }
}

/// Parse Typst CLI stderr output to extract error information.
fn parse_typst_output(
    stderr: &str,
    errors: &mut Vec<CompileError>,
    warnings: &mut Vec<CompileError>,
) {
    let mut current_severity: Option<String> = None;
    let mut current_message: Option<String> = None;
    let mut current_file: Option<String> = None;
    let mut current_line: Option<u32> = None;
    let mut current_column: Option<u32> = None;

    for line in stderr.lines() {
        let trimmed = line.trim();

        // Match "error: ..." or "warning: ..."
        if trimmed.starts_with("error:") || trimmed.starts_with("warning:") {
            // Save any previous error/warning
            if let Some(severity) = current_severity.take() {
                let entry = CompileError {
                    severity: severity.clone(),
                    message: current_message.take().unwrap_or_default(),
                    file: current_file.take(),
                    line: current_line.take(),
                    column: current_column.take(),
                };
                if severity == "error" {
                    errors.push(entry);
                } else {
                    warnings.push(entry);
                }
            }

            let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
            current_severity = Some(parts[0].to_string());
            current_message = parts.get(1).map(|s| s.trim().to_string());
        } else if trimmed.starts_with("┌─") || trimmed.contains(".typ:") {
            // Parse location line like "┌─ input.typ:3:5" or similar
            if let Some(loc_start) = trimmed.find(".typ:") {
                let loc_part = &trimmed[loc_start..];
                let parts: Vec<&str> = loc_part.splitn(3, ':').collect();
                if parts.len() >= 1 {
                    // Find the file name part
                    let file_end = loc_start + parts[0].len();
                    let file_start = trimmed[..loc_start].rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '-' && c != '.' && c != '/')
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    current_file = Some(trimmed[file_start..file_end].to_string());

                    if parts.len() >= 2 {
                        current_line = parts[1].trim().parse().ok();
                    }
                    if parts.len() >= 3 {
                        current_column = parts[2].trim().split_whitespace().next()
                            .and_then(|s| s.parse().ok());
                    }
                }
            }
        }
    }

    // Save the last entry
    if let Some(severity) = current_severity.take() {
        let entry = CompileError {
            severity: severity.clone(),
            message: current_message.take().unwrap_or_default(),
            file: current_file.take(),
            line: current_line.take(),
            column: current_column.take(),
        };
        if severity == "error" {
            errors.push(entry);
        } else {
            warnings.push(entry);
        }
    }
}
