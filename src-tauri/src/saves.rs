#![warn(clippy::all, clippy::pedantic)]

use std::{fs, path::PathBuf};
use serde::ser::SerializeStruct;



#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Time(#[from] std::time::SystemTimeError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub struct FileData {
    name: String,
    modified: u64,
    created: u64
}

impl FileData {
    fn new(path: PathBuf, metadata: fs::Metadata) -> Result<Self, Error> {
        let data = Self {
            name: path.file_prefix().map_or("Untitled", |f| f.to_str().unwrap()).to_owned(),
            modified: metadata.modified()?.elapsed()?.as_secs(),
            created: metadata.created()?.elapsed()?.as_secs()
        };
        Ok(data)
    }
}

impl serde::Serialize for FileData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut data = serializer.serialize_struct("FileData", 3)?;
        data.serialize_field("name", &self.name)?;
        data.serialize_field("modified", &self.modified)?;
        data.serialize_field("created", &self.created)?;
        data.end()
    }
}



#[tauri::command]
pub async fn fetch_saves(app: tauri::AppHandle) -> Result<Vec<FileData>, Error> {
    let app_dir = app.path_resolver().app_dir().expect("App directory should be retrievable");
    let saves_dir = app_dir.join("saves");

    if !&saves_dir.is_dir() {
        fs::create_dir_all(&saves_dir)?;
    }

    let mut saves: Vec<FileData> = Vec::new();

    for entry in fs::read_dir(saves_dir)? {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() && path.extension().contains(&"json") {
            saves.push(FileData::new(path, metadata)?);
        }
    }

    Ok(saves)
}