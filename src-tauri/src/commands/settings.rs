use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    // Editor
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default = "default_true")]
    pub show_line_numbers: bool,
    #[serde(default)]
    pub word_wrap: bool,

    // Theme
    #[serde(default = "default_theme_id")]
    pub active_theme_id: String,

    // Preview
    #[serde(default = "default_preview_mode")]
    pub preview_mode: String,
    #[serde(default = "default_debounce_ms")]
    pub realtime_debounce_ms: u32,

    // Layout
    #[serde(default = "default_file_tree_width")]
    pub file_tree_width: u32,
    #[serde(default = "default_preview_width")]
    pub preview_width: u32,
    #[serde(default = "default_true")]
    pub file_tree_visible: bool,
    #[serde(default = "default_true")]
    pub preview_visible: bool,

    // App
    #[serde(default = "default_color_scheme")]
    pub color_scheme: String,
    #[serde(default)]
    pub last_opened_path: Option<String>,
    #[serde(default)]
    pub recent_paths: Vec<String>,

    // Typst
    #[serde(default)]
    pub typst_path: String,
}

fn default_font_family() -> String { "Fira Code".to_string() }
fn default_font_size() -> u32 { 14 }
fn default_true() -> bool { true }
fn default_theme_id() -> String { "zenypst-dark".to_string() }
fn default_preview_mode() -> String { "realtime".to_string() }
fn default_debounce_ms() -> u32 { 500 }
fn default_file_tree_width() -> u32 { 250 }
fn default_preview_width() -> u32 { 400 }
fn default_color_scheme() -> String { "dark".to_string() }

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            font_family: default_font_family(),
            font_size: default_font_size(),
            show_line_numbers: true,
            word_wrap: false,
            active_theme_id: default_theme_id(),
            preview_mode: default_preview_mode(),
            realtime_debounce_ms: default_debounce_ms(),
            file_tree_width: default_file_tree_width(),
            preview_width: default_preview_width(),
            file_tree_visible: true,
            preview_visible: true,
            color_scheme: default_color_scheme(),
            last_opened_path: None,
            recent_paths: vec![],
            typst_path: String::new(),
        }
    }
}

fn settings_path() -> Result<PathBuf, String> {
    let base = dirs::data_dir()
        .ok_or_else(|| "Failed to get data directory".to_string())?;
    Ok(base.join("zenypst").join("settings.json"))
}

fn themes_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir()
        .ok_or_else(|| "Failed to get data directory".to_string())?;
    Ok(base.join("zenypst").join("themes"))
}

/// Load application settings from disk.
#[tauri::command]
pub async fn load_settings() -> Result<AppSettings, String> {
    let path = settings_path()?;

    if !path.exists() {
        return Ok(AppSettings::default());
    }

    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read settings: {}", e))?;

    serde_json::from_str::<AppSettings>(&content)
        .map_err(|e| format!("Failed to parse settings: {}", e))
}

/// Save application settings to disk.
#[tauri::command]
pub async fn save_settings(settings: AppSettings) -> Result<(), String> {
    let path = settings_path()?;

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    tokio::fs::write(&path, content)
        .await
        .map_err(|e| format!("Failed to write settings: {}", e))
}

// ─── Theme management ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub caret: String,
    pub selection: String,
    pub line_highlight: String,
    pub gutter_background: String,
    pub gutter_foreground: String,
    pub heading: String,
    pub emphasis: String,
    pub strong: String,
    pub keyword: String,
    pub function: String,
    pub string: String,
    pub number: String,
    pub comment: String,
    pub math: String,
    pub label: String,
    pub raw_block: String,
    pub operator: String,
    pub bracket: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub built_in: bool,
    pub colors: ThemeColors,
}

fn built_in_themes() -> Vec<Theme> {
    vec![
        Theme {
            id: "zenypst-dark".to_string(),
            name: "Zenypst Dark".to_string(),
            built_in: true,
            colors: ThemeColors {
                background: "#1e1e2e".to_string(),
                foreground: "#cdd6f4".to_string(),
                caret: "#f5e0dc".to_string(),
                selection: "#45475a".to_string(),
                line_highlight: "#313244".to_string(),
                gutter_background: "#1e1e2e".to_string(),
                gutter_foreground: "#6c7086".to_string(),
                heading: "#f38ba8".to_string(),
                emphasis: "#f5c2e7".to_string(),
                strong: "#fab387".to_string(),
                keyword: "#cba6f7".to_string(),
                function: "#89b4fa".to_string(),
                string: "#a6e3a1".to_string(),
                number: "#fab387".to_string(),
                comment: "#6c7086".to_string(),
                math: "#f9e2af".to_string(),
                label: "#74c7ec".to_string(),
                raw_block: "#a6adc8".to_string(),
                operator: "#89dceb".to_string(),
                bracket: "#9399b2".to_string(),
            },
        },
        Theme {
            id: "zenypst-light".to_string(),
            name: "Zenypst Light".to_string(),
            built_in: true,
            colors: ThemeColors {
                background: "#eff1f5".to_string(),
                foreground: "#4c4f69".to_string(),
                caret: "#dc8a78".to_string(),
                selection: "#acb0be".to_string(),
                line_highlight: "#e6e9f0".to_string(),
                gutter_background: "#eff1f5".to_string(),
                gutter_foreground: "#9ca0b0".to_string(),
                heading: "#d20f39".to_string(),
                emphasis: "#ea76cb".to_string(),
                strong: "#fe640b".to_string(),
                keyword: "#8839ef".to_string(),
                function: "#1e66f5".to_string(),
                string: "#40a02b".to_string(),
                number: "#fe640b".to_string(),
                comment: "#9ca0b0".to_string(),
                math: "#df8e1d".to_string(),
                label: "#209fb5".to_string(),
                raw_block: "#6c6f85".to_string(),
                operator: "#04a5e5".to_string(),
                bracket: "#7287fd".to_string(),
            },
        },
        Theme {
            id: "solarized-dark".to_string(),
            name: "Solarized Dark".to_string(),
            built_in: true,
            colors: ThemeColors {
                background: "#002b36".to_string(),
                foreground: "#839496".to_string(),
                caret: "#839496".to_string(),
                selection: "#073642".to_string(),
                line_highlight: "#073642".to_string(),
                gutter_background: "#002b36".to_string(),
                gutter_foreground: "#586e75".to_string(),
                heading: "#268bd2".to_string(),
                emphasis: "#6c71c4".to_string(),
                strong: "#cb4b16".to_string(),
                keyword: "#859900".to_string(),
                function: "#268bd2".to_string(),
                string: "#2aa198".to_string(),
                number: "#d33682".to_string(),
                comment: "#586e75".to_string(),
                math: "#b58900".to_string(),
                label: "#2aa198".to_string(),
                raw_block: "#657b83".to_string(),
                operator: "#2aa198".to_string(),
                bracket: "#657b83".to_string(),
            },
        },
    ]
}

/// List all themes (built-in + custom).
#[tauri::command]
pub async fn list_themes() -> Result<Vec<Theme>, String> {
    let dir = themes_dir()?;
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create themes directory: {}", e))?;

    let mut themes = built_in_themes();

    let mut entries = tokio::fs::read_dir(&dir)
        .await
        .map_err(|e| format!("Failed to read themes directory: {}", e))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match tokio::fs::read_to_string(&path).await {
                Ok(content) => {
                    match serde_json::from_str::<Theme>(&content) {
                        Ok(theme) => {
                            if !theme.built_in {
                                themes.push(theme);
                            }
                        }
                        Err(e) => eprintln!("Failed to parse theme {:?}: {}", path, e),
                    }
                }
                Err(e) => eprintln!("Failed to read theme {:?}: {}", path, e),
            }
        }
    }

    Ok(themes)
}

/// Create or update a custom theme.
#[tauri::command]
pub async fn save_theme(mut theme: Theme) -> Result<(), String> {
    if theme.built_in {
        return Err("Cannot modify built-in themes".to_string());
    }

    let dir = themes_dir()?;
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create themes directory: {}", e))?;

    if theme.id.is_empty() {
        theme.id = Uuid::new_v4().to_string();
    }

    let file_path = dir.join(format!("{}.json", theme.id));
    let content = serde_json::to_string_pretty(&theme)
        .map_err(|e| format!("Failed to serialize theme: {}", e))?;

    tokio::fs::write(&file_path, content)
        .await
        .map_err(|e| format!("Failed to write theme: {}", e))
}

/// Delete a custom theme by ID.
#[tauri::command]
pub async fn delete_theme(id: String) -> Result<(), String> {
    let dir = themes_dir()?;
    let file_path = dir.join(format!("{}.json", id));

    if !file_path.exists() {
        return Err(format!("Theme '{}' not found", id));
    }

    tokio::fs::remove_file(&file_path)
        .await
        .map_err(|e| format!("Failed to delete theme: {}", e))
}
