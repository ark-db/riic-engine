#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(is_some_and)]
#![feature(path_file_prefix)]

pub mod base;
pub mod saves;

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
