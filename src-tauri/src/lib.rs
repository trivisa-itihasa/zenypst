mod commands;
mod utils;
mod world;

use commands::file::{
    read_file, write_file, list_directory, create_file, create_directory,
    delete_path, rename_path, open_in_file_manager, pick_folder, pick_file, save_file_dialog,
};
use commands::settings::{
    load_settings, save_settings, list_themes, save_theme, delete_theme, list_system_fonts,
};
use commands::template::{list_templates, save_template, delete_template};
use commands::native_compile::{compile_native, export_pdf, get_typst_version, locate_source, NativeCompilerState};

#[cfg(target_os = "macos")]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
#[cfg(target_os = "macos")]
use tauri::Emitter;

#[cfg(target_os = "macos")]
fn build_menu(app: &tauri::App) -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let sep = PredefinedMenuItem::separator(app)?;

    // --- App menu ---
    let app_menu = {
        let about = PredefinedMenuItem::about(app, Some("About Zenypst"), None)?;
        let hide = PredefinedMenuItem::hide(app, Some("Hide Zenypst"))?;
        let hide_others = PredefinedMenuItem::hide_others(app, Some("Hide Others"))?;
        let show_all = PredefinedMenuItem::show_all(app, Some("Show All"))?;
        let quit = PredefinedMenuItem::quit(app, Some("Quit Zenypst"))?;
        Submenu::with_items(
            app,
            "Zenypst",
            true,
            &[&about, &sep, &hide, &hide_others, &show_all, &PredefinedMenuItem::separator(app)?, &quit],
        )?
    };

    // --- File menu ---
    let file_menu = {
        let new_file = MenuItem::with_id(app, "new-file", "New File", true, Some("CmdOrCtrl+N"))?;
        let open_file = MenuItem::with_id(app, "open-file", "Open File", true, Some("CmdOrCtrl+O"))?;
        let open_folder = MenuItem::with_id(app, "open-folder", "Open Folder", true, Some("CmdOrCtrl+Shift+O"))?;
        let save = MenuItem::with_id(app, "save", "Save", true, Some("CmdOrCtrl+S"))?;
        let save_as = MenuItem::with_id(app, "save-as", "Save As…", true, Some("CmdOrCtrl+Shift+S"))?;
        let export_pdf_item = MenuItem::with_id(app, "export-pdf", "Export PDF", true, None::<&str>)?;
        let manage_templates = MenuItem::with_id(app, "manage-templates", "Manage Templates", true, None::<&str>)?;
        Submenu::with_items(
            app,
            "File",
            true,
            &[
                &new_file,
                &PredefinedMenuItem::separator(app)?,
                &open_file,
                &open_folder,
                &PredefinedMenuItem::separator(app)?,
                &save,
                &save_as,
                &PredefinedMenuItem::separator(app)?,
                &export_pdf_item,
                &PredefinedMenuItem::separator(app)?,
                &manage_templates,
            ],
        )?
    };

    // --- Edit menu ---
    let edit_menu = {
        let undo = PredefinedMenuItem::undo(app, Some("Undo"))?;
        let redo = PredefinedMenuItem::redo(app, Some("Redo"))?;
        let cut = PredefinedMenuItem::cut(app, Some("Cut"))?;
        let copy = PredefinedMenuItem::copy(app, Some("Copy"))?;
        let paste = PredefinedMenuItem::paste(app, Some("Paste"))?;
        let select_all = PredefinedMenuItem::select_all(app, Some("Select All"))?;
        Submenu::with_items(
            app,
            "Edit",
            true,
            &[&undo, &redo, &PredefinedMenuItem::separator(app)?, &cut, &copy, &paste, &PredefinedMenuItem::separator(app)?, &select_all],
        )?
    };

    // --- Window menu ---
    let window_menu = {
        let minimize = PredefinedMenuItem::minimize(app, Some("Minimize"))?;
        let fullscreen = PredefinedMenuItem::fullscreen(app, Some("Toggle Full Screen"))?;
        let close = PredefinedMenuItem::close_window(app, Some("Close"))?;
        Submenu::with_items(
            app,
            "Window",
            true,
            &[&minimize, &fullscreen, &PredefinedMenuItem::separator(app)?, &close],
        )?
    };

    Ok(Menu::with_items(app, &[&app_menu, &file_menu, &edit_menu, &window_menu])?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(NativeCompilerState::new())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                // Enable native window decorations (traffic lights) on macOS
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(true);
                }
                // Set up native menu bar
                let menu = build_menu(app)?;
                app.set_menu(menu)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // File commands
            read_file,
            write_file,
            list_directory,
            create_file,
            create_directory,
            delete_path,
            rename_path,
            open_in_file_manager,
            pick_folder,
            pick_file,
            save_file_dialog,
            // Typst (native, in-process)
            compile_native,
            export_pdf,
            get_typst_version,
            locate_source,
            // Template commands
            list_templates,
            save_template,
            delete_template,
            // Settings commands
            load_settings,
            save_settings,
            list_system_fonts,
            // Theme commands
            list_themes,
            save_theme,
            delete_theme,
        ]);

    #[cfg(target_os = "macos")]
    let builder = builder.on_menu_event(|app, event| {
        let _ = app.emit(event.id().as_ref(), ());
    });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
