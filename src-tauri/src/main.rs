#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use riic_engine::{db, open, window};
use tauri::{generate_context, generate_handler, Builder, Manager, RunEvent};

fn main() {
    Builder::new()
        .manage(db::Database::setup().expect("Failed to set up app database"))
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
        .build(generate_context!())
        .expect("An error occurred while building the application.")
        .run(|app, event| {
            if let RunEvent::Exit = event {
                app.state::<db::Database>()
                    .teardown()
                    .expect("Failed to tear down app database");
            }
        });
}
