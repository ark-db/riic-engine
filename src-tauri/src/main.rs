#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use riic_engine::{savefile, window};
use std::fs::create_dir_all;

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
        .setup(|app| {
            let saves_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("Failed to retrieve app directory")
                .join("saves");

            if !saves_dir.is_dir() {
                create_dir_all(&saves_dir).expect("Failed to create save directory");
            }

            savefile::SAVE_DIR.set(saves_dir).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("An error occurred while running the application.");
}
