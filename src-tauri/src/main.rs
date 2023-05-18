#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use riic_engine::{db, open, window};
use tauri::{generate_context, generate_handler, Builder};

fn main() {
    Builder::new()
        .manage(db::Database::setup().expect("Failed to initialize app database"))
        .invoke_handler(generate_handler![
            db::fetch_saves,
            db::create_save,
            db::get_save,
            db::rename_save,
            db::update_save,
            db::delete_save,
            db::export_save,
            open::open,
            window::show_window,
            window::rename_window
        ])
        .run(generate_context!())
        .expect("An error occurred while running the application.");
}
