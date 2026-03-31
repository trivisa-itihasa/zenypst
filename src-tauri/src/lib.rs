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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(NativeCompilerState::new())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
