use ecow::EcoString;
use serde::{Deserialize, Serialize};
use std::path::Path;
use typst_kit::download::{Downloader, ProgressSink};
use typst_kit::package::PackageStorage;
use typst_syntax::package::{PackageManifest, PackageSpec, PackageVersion, VersionBound, VersionlessPackageSpec};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniverseTemplate {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub categories: Vec<String>,
    pub entrypoint: String,
}

/// Fetch template packages from Typst Universe (packages.typst.org).
/// For packages with multiple versions, only the latest version compatible
/// with the built-in typst compiler is returned.
#[tauri::command]
pub async fn fetch_universe_templates() -> Result<Vec<UniverseTemplate>, String> {
    let body = ureq::get("https://packages.typst.org/preview/index.json")
        .call()
        .map_err(|e| format!("Failed to fetch package index: {}", e))?
        .into_string()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let packages: Vec<serde_json::Value> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse package index: {}", e))?;

    let template_categories: std::collections::HashSet<&str> = [
        "paper", "thesis", "report", "cv", "presentation", "book", "poster", "flyer",
    ]
    .into_iter()
    .collect();

    let current_version = PackageVersion::compiler();

    // Group packages by name
    let mut by_name: std::collections::HashMap<String, Vec<serde_json::Value>> =
        std::collections::HashMap::new();
    for pkg in packages {
        let name = pkg
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if !name.is_empty() {
            by_name.entry(name).or_default().push(pkg);
        }
    }

    let mut templates = Vec::new();
    for (_name, versions) in by_name {
        let mut candidates: Vec<(PackageVersion, serde_json::Value)> = Vec::new();

        for pkg in versions {
            let version_str = pkg.get("version").and_then(|v| v.as_str()).unwrap_or("");
            let Ok(version) = version_str.parse::<PackageVersion>() else {
                continue;
            };

            // Check compiler compatibility
            let compatible = if let Some(req_str) = pkg.get("compiler").and_then(|v| v.as_str()) {
                if let Ok(bound) = req_str.parse::<VersionBound>() {
                    current_version.matches_ge(&bound)
                } else {
                    true // malformed bound: be permissive
                }
            } else {
                true // no compiler requirement
            };

            if compatible {
                candidates.push((version, pkg));
            }
        }

        // Pick the latest compatible version
        candidates.sort_by(|a, b| b.0.cmp(&a.0));
        let Some((_, pkg)) = candidates.into_iter().next() else {
            continue;
        };

        let categories = pkg
            .get("categories")
            .and_then(|c| c.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let is_template = categories.iter().any(|c| template_categories.contains(c.as_str()));
        if !is_template {
            continue;
        }

        templates.push(UniverseTemplate {
            name: pkg
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            version: pkg
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            description: pkg
                .get("description")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            authors: pkg
                .get("authors")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            categories,
            entrypoint: pkg
                .get("entrypoint")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        });
    }

    templates.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(templates)
}

fn parse_manifest(package_path: &Path) -> Result<PackageManifest, String> {
    let toml_path = package_path.join("typst.toml");
    let string = std::fs::read_to_string(&toml_path)
        .map_err(|err| format!("failed to read package manifest ({})", err))?;
    toml::from_str(&string)
        .map_err(|err| format!("package manifest is malformed ({})", err.message()))
}

/// Initialize a new project from a Typst Universe template.
#[tauri::command]
pub async fn init_universe_template(
    template_name: String,
    version: Option<String>,
    target_dir: String,
) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let downloader = Downloader::new("zenypst/0.2.3");
        let package_storage = PackageStorage::new(None, None, downloader);

        // Determine package spec
        let spec = if let Some(v) = &version {
            let spec_str = format!("@preview/{}:{}", template_name, v);
            spec_str
                .parse::<PackageSpec>()
                .map_err(|e: EcoString| e.to_string())?
        } else {
            let versionless = VersionlessPackageSpec {
                namespace: "preview".into(),
                name: template_name.into(),
            };
            let latest = package_storage
                .determine_latest_version(&versionless)
                .map_err(|e: EcoString| e.to_string())?;
            versionless.at(latest)
        };

        // Download / prepare package
        let package_path = package_storage
            .prepare_package(&spec, &mut ProgressSink)
            .map_err(|e| e.to_string())?;

        // Parse manifest
        let manifest = parse_manifest(&package_path)?;
        manifest
            .validate(&spec)
            .map_err(|e: EcoString| e.to_string())?;

        let template = manifest
            .template
            .ok_or_else(|| format!("package {} is not a template", spec))?;

        let project_dir = Path::new(&target_dir);
        if project_dir.exists() {
            return Err(format!(
                "project directory already exists (at {})",
                project_dir.display()
            ));
        }

        let template_dir = package_path.join(template.path.as_str());
        if !template_dir.exists() {
            return Err(format!(
                "template directory does not exist (at {})",
                template_dir.display()
            ));
        }

        fs_extra::dir::copy(
            &template_dir,
            project_dir,
            &fs_extra::dir::CopyOptions::new().content_only(true),
        )
        .map_err(|err| format!("failed to create project directory ({err})"))?;

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
