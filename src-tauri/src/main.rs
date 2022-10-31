#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![feature(option_result_contains)]
#![feature(path_file_prefix)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![saves::fetch_saves, saves::create_save, saves::delete_save])
    .run(tauri::generate_context!())
    .expect("An error occurred while running the application.");
}

pub mod saves;