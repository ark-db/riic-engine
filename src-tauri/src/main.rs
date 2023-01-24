#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(is_some_and)]
#![feature(path_file_prefix)]

use tauri::{Manager, Window};
mod base;
mod saves;

#[derive(Debug, thiserror::Error)]
pub enum CmdError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    FileWrite(#[from] serde_json::Error),

    #[error("No name specified")]
    NameEmpty,

    #[error("Another file with the same name already exists")]
    DuplicateName,

    #[error("Relative filepaths are forbidden")]
    RelativePath,
}

impl serde::Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CmdResult<T> = Result<T, CmdError>;

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
fn show_window(window: Window) {
    let w = window.get_window("main").unwrap();
    w.show().unwrap();
}

#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
fn rename_window(window: Window, name: Option<&str>) {
    let w = window.get_window("main").unwrap();

    if let Some(n) = name {
        w.set_title(&format!("RIIC Engine • {n}")).unwrap();
    } else {
        w.set_title("RIIC Engine").unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_window,
            rename_window,
            saves::fetch_saves,
            saves::create_save,
            saves::load_save,
            saves::rename_save,
            saves::update_save,
            saves::delete_save,
            saves::export_save
        ])
        .run(tauri::generate_context!())
        .expect("An error occurred while running the application.");
}
