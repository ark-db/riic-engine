#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]

use tauri::{Manager, Result as TauriResult, Window};

/// # Errors
/// Returns error if the main window cannot be shown.
#[tauri::command]
pub fn show_window(window: Window) -> TauriResult<()> {
    let w = window.get_window("main").expect("Failed to get app window");
    w.show()
}

/// # Errors
/// Returns error if the main window title cannot be renamed.
#[tauri::command]
pub fn rename_window(window: Window, name: Option<&str>) -> TauriResult<()> {
    let w = window.get_window("main").expect("Failed to get app window");
    match name {
        Some(n) => w.set_title(&format!("RIIC Engine • {n}")),
        None => w.set_title("RIIC Engine"),
    }
}
