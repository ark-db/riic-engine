#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use riic_engine::{savefile, window};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window::show_window,
            window::rename_window,
            savefile::fetch_saves,
            savefile::create_save,
            savefile::load_save,
            savefile::rename_save,
            savefile::update_save,
            savefile::delete_save,
            savefile::export_save
        ])
        .run(tauri::generate_context!())
        .expect("An error occurred while running the application.");
}
