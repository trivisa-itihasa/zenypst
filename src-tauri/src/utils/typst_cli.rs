use std::path::PathBuf;

/// Detect the path of the `typst` CLI binary.
#[allow(dead_code)]
pub fn find_typst_binary() -> Option<PathBuf> {
    // First try to find in PATH
    if let Ok(output) = std::process::Command::new("which")
        .arg("typst")
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                return Some(PathBuf::from(path_str));
            }
        }
    }

    // Try common installation paths
    let common_paths = [
        "/usr/local/bin/typst",
        "/usr/bin/typst",
        "/opt/homebrew/bin/typst",
        "~/.cargo/bin/typst",
    ];

    for path in &common_paths {
        let expanded = if path.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                home.join(&path[2..])
            } else {
                continue;
            }
        } else {
            PathBuf::from(path)
        };

        if expanded.exists() {
            return Some(expanded);
        }
    }

    None
}

/// Get a temp directory for Typst compilation outputs.
#[allow(dead_code)]
pub fn get_temp_dir() -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    base.join("zenypst").join("temp")
}
