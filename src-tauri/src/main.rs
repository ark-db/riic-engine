#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use riic_engine::{db, window};

fn main() {
    tauri::Builder::default()
        .manage(db::Database::init().expect("Failed to initialize app database"))
        .invoke_handler(tauri::generate_handler![
            window::show_window,
            window::rename_window,
            db::fetch_saves,
            db::create_save,
            db::get_save,
            db::rename_save,
            db::update_save,
            db::delete_save,
            db::export_save
        ])
        .run(tauri::generate_context!())
        .expect("An error occurred while running the application.");
}
