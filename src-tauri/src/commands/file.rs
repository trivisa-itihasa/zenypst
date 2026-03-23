use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileNode>>,
    pub extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Read a text file and return its contents.
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path, e))
}

/// Write content to a file, creating it if it doesn't exist.
#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }
    tokio::fs::write(&path, content)
        .await
        .map_err(|e| format!("Failed to write file '{}': {}", path, e))
}

/// List directory contents recursively, building a tree structure.
#[tauri::command]
pub async fn list_directory(path: String) -> Result<Vec<FileNode>, String> {
    list_dir_recursive(&path).await
}

fn get_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

#[async_recursion::async_recursion]
async fn list_dir_recursive(path: &str) -> Result<Vec<FileNode>, String> {
    let mut entries = tokio::fs::read_dir(path)
        .await
        .map_err(|e| format!("Failed to read directory '{}': {}", path, e))?;

    let mut nodes: Vec<FileNode> = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let entry_path = entry.path();
        let path_str = entry_path.to_string_lossy().to_string();
        let name = entry
            .file_name()
            .to_string_lossy()
            .to_string();

        // Skip hidden files/folders
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry
            .metadata()
            .await
            .map_err(|e| format!("Failed to get metadata: {}", e))?;

        let is_dir = metadata.is_dir();
        let extension = if is_dir {
            None
        } else {
            get_extension(&path_str)
        };

        let children = if is_dir {
            match list_dir_recursive(&path_str).await {
                Ok(c) => Some(c),
                Err(_) => Some(vec![]),
            }
        } else {
            None
        };

        nodes.push(FileNode {
            name,
            path: path_str,
            is_dir,
            children,
            extension,
        });
    }

    // Sort: directories first, then files, both alphabetically
    nodes.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(nodes)
}

/// Create a new file with the given content.
#[tauri::command]
pub async fn create_file(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create parent directory: {}", e))?;
    }
    tokio::fs::write(&path, content)
        .await
        .map_err(|e| format!("Failed to create file '{}': {}", path, e))
}

/// Create a new directory.
#[tauri::command]
pub async fn create_directory(path: String) -> Result<(), String> {
    tokio::fs::create_dir_all(&path)
        .await
        .map_err(|e| format!("Failed to create directory '{}': {}", path, e))
}

/// Delete a file or directory.
#[tauri::command]
pub async fn delete_path(path: String) -> Result<(), String> {
    let metadata = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("Failed to get metadata for '{}': {}", path, e))?;

    if metadata.is_dir() {
        tokio::fs::remove_dir_all(&path)
            .await
            .map_err(|e| format!("Failed to delete directory '{}': {}", path, e))
    } else {
        tokio::fs::remove_file(&path)
            .await
            .map_err(|e| format!("Failed to delete file '{}': {}", path, e))
    }
}

/// Rename or move a file/directory.
#[tauri::command]
pub async fn rename_path(old: String, new: String) -> Result<(), String> {
    tokio::fs::rename(&old, &new)
        .await
        .map_err(|e| format!("Failed to rename '{}' to '{}': {}", old, new, e))
}

/// Open a path in the OS file manager.
#[tauri::command]
pub async fn open_in_file_manager(path: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }
    Ok(())
}

/// Open a native folder picker dialog.
#[tauri::command]
pub async fn pick_folder(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let folder = app.dialog().file().blocking_pick_folder();
    Ok(folder.map(|p| p.to_string()))
}

/// Open a native file picker dialog.
#[tauri::command]
pub async fn pick_file(
    app: AppHandle,
    filters: Vec<FileFilter>,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let mut builder = app.dialog().file();
    for filter in &filters {
        let exts: Vec<&str> = filter.extensions.iter().map(|s| s.as_str()).collect();
        builder = builder.add_filter(&filter.name, &exts);
    }
    let file = builder.blocking_pick_file();
    Ok(file.map(|p| p.to_string()))
}

/// Open a native save file dialog.
#[tauri::command]
pub async fn save_file_dialog(
    app: AppHandle,
    default_name: String,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let file = app
        .dialog()
        .file()
        .set_file_name(&default_name)
        .blocking_save_file();
    Ok(file.map(|p| p.to_string()))
}
