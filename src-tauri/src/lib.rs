mod commands;
mod utils;

use commands::file::{
    read_file, write_file, list_directory, create_file, create_directory,
    delete_path, rename_path, open_in_file_manager, pick_folder, pick_file, save_file_dialog,
};
use commands::settings::{
    load_settings, save_settings, list_themes, save_theme, delete_theme,
};
use commands::template::{list_templates, save_template, delete_template};
use commands::typst::{
    compile_typst, check_typst_installed, get_typst_version,
    get_temp_input_path, get_temp_output_path,
    start_typst_watch, stop_typst_watch, WatchState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(WatchState::new())
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
            // Typst commands
            compile_typst,
            check_typst_installed,
            get_typst_version,
            get_temp_input_path,
            get_temp_output_path,
            start_typst_watch,
            stop_typst_watch,
            // Template commands
            list_templates,
            save_template,
            delete_template,
            // Settings commands
            load_settings,
            save_settings,
            // Theme commands
            list_themes,
            save_theme,
            delete_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
