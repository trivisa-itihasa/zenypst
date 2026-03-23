use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: String,
    #[serde(default)]
    pub built_in: bool,
}

fn templates_dir() -> Result<PathBuf, String> {
    let base = dirs::data_dir()
        .ok_or_else(|| "Failed to get data directory".to_string())?;
    Ok(base.join("zenypst").join("templates"))
}

fn built_in_templates() -> Vec<Template> {
    vec![
        Template {
            id: "built-in-blank".to_string(),
            name: "Blank".to_string(),
            description: "An empty document.".to_string(),
            content: String::new(),
            built_in: true,
        },
        Template {
            id: "built-in-article".to_string(),
            name: "Article".to_string(),
            description: "A basic article structure with title, abstract, and sections.".to_string(),
            content: "#set page(paper: \"a4\")\n#set text(font: \"New Computer Modern\", size: 11pt)\n#set heading(numbering: \"1.\")\n\n= Article Title\n\n*Author Name* \\\n_Institution_ \\\n#datetime.today().display()\n\n== Abstract\n\nWrite your abstract here.\n\n== Introduction\n\nIntroduce your topic.\n\n== Methods\n\nDescribe your methods.\n\n== Results\n\nPresent your results.\n\n== Conclusion\n\nConclude your work.\n\n== References\n\n".to_string(),
            built_in: true,
        },
        Template {
            id: "built-in-letter".to_string(),
            name: "Letter".to_string(),
            description: "A formal letter template.".to_string(),
            content: "#set page(paper: \"a4\", margin: (x: 2.5cm, y: 3cm))\n#set text(font: \"New Computer Modern\", size: 11pt)\n\n*Your Name* \\\nYour Address \\\nCity, State, ZIP \\\n#datetime.today().display()\n\n#v(1cm)\n\n*Recipient Name* \\\nRecipient Address \\\nCity, State, ZIP\n\n#v(1cm)\n\nDear [Recipient Name],\n\n#v(0.5cm)\n\nWrite the body of your letter here.\n\n#v(0.5cm)\n\nSincerely,\n\n#v(1cm)\n\nYour Name\n".to_string(),
            built_in: true,
        },
    ]
}

/// List all templates (built-in + custom) from the app data directory.
#[tauri::command]
pub async fn list_templates() -> Result<Vec<Template>, String> {
    let dir = templates_dir()?;

    // Seed built-in templates on first launch
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create templates directory: {}", e))?;

    let mut templates = built_in_templates();

    // Load custom templates from disk
    let mut entries = tokio::fs::read_dir(&dir)
        .await
        .map_err(|e| format!("Failed to read templates directory: {}", e))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            match tokio::fs::read_to_string(&path).await {
                Ok(content) => {
                    match serde_json::from_str::<Template>(&content) {
                        Ok(template) => {
                            if !template.built_in {
                                templates.push(template);
                            }
                        }
                        Err(e) => eprintln!("Failed to parse template {:?}: {}", path, e),
                    }
                }
                Err(e) => eprintln!("Failed to read template {:?}: {}", path, e),
            }
        }
    }

    Ok(templates)
}

/// Create or update a custom template.
#[tauri::command]
pub async fn save_template(mut template: Template) -> Result<(), String> {
    if template.built_in {
        return Err("Cannot modify built-in templates".to_string());
    }

    let dir = templates_dir()?;
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create templates directory: {}", e))?;

    if template.id.is_empty() {
        template.id = Uuid::new_v4().to_string();
    }

    let file_path = dir.join(format!("{}.json", template.id));
    let content = serde_json::to_string_pretty(&template)
        .map_err(|e| format!("Failed to serialize template: {}", e))?;

    tokio::fs::write(&file_path, content)
        .await
        .map_err(|e| format!("Failed to write template: {}", e))
}

/// Delete a custom template by ID.
#[tauri::command]
pub async fn delete_template(id: String) -> Result<(), String> {
    let dir = templates_dir()?;
    let file_path = dir.join(format!("{}.json", id));

    if !file_path.exists() {
        return Err(format!("Template '{}' not found", id));
    }

    tokio::fs::remove_file(&file_path)
        .await
        .map_err(|e| format!("Failed to delete template: {}", e))
}
