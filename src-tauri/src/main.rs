#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(is_some_and)]
#![feature(path_file_prefix)]

pub mod base;
pub mod saves;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            saves::fetch_saves,
            saves::create_save,
            saves::rename_save,
            saves::delete_save,
            saves::load_save,
            saves::export_save
        ])
        .run(tauri::generate_context!())
        .expect("An error occurred while running the application.");
}
