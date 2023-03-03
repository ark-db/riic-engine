#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]

use tauri::{Manager, Window};

/// # Panics
/// Panics if the main window cannot be fetched from the manager or shown.
#[tauri::command]
pub fn show_window(window: Window) {
    let w = window.get_window("main").expect("Failed to get app window");
    w.show().unwrap();
}

/// # Panics
/// Panics if the main window cannot be fetched from the manager or renamed.
#[tauri::command]
pub fn rename_window(window: Window, name: Option<&str>) {
    let w = window.get_window("main").expect("Failed to get app window");
    match name {
        Some(n) => w.set_title(&format!("RIIC Engine • {n}")),
        None => w.set_title("RIIC Engine"),
    }
    .expect("Failed to rename app window");
}
