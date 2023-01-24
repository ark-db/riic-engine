#![allow(clippy::module_name_repetitions)]

use tauri::{Manager, Window};

/// # Panics
/// Panics if the main window cannot be fetched from the manager or shown.
#[tauri::command]
pub fn show_window(window: Window) {
    let w = window.get_window("main").unwrap();
    w.show().unwrap();
}

/// # Panics
/// Panics if the main window cannot be fetched from the manager or renamed.
#[tauri::command]
pub fn rename_window(window: Window, name: Option<&str>) {
    let w = window.get_window("main").unwrap();

    if let Some(n) = name {
        w.set_title(&format!("RIIC Engine • {n}")).unwrap();
    } else {
        w.set_title("RIIC Engine").unwrap();
    }
}
