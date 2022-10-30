#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![feature(option_result_contains)]
#![feature(path_file_prefix)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![saves::fetch_saves])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

pub mod saves;