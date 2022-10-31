#![warn(clippy::all, clippy::pedantic)]

use std::{fs, path::PathBuf};
use serde::Serialize;



#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Time(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    FileWrite(#[from] serde_json::Error),

    #[error("No name specified")]
    NameEmpty,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct Save {
    name: String,
    data: Option<i32>
}



fn get_saves_dir(app: tauri::AppHandle) -> Result<PathBuf, Error> {
    let app_dir = app.path_resolver().app_dir().expect("App directory should be retrievable");
    let saves_dir = app_dir.join("saves");

    if !&saves_dir.is_dir() {
        fs::create_dir_all(&saves_dir)?;
    }

    Ok(saves_dir)
}

fn get_save_fp(app: tauri::AppHandle, name: &str) -> Result<PathBuf, Error> {
    if name.is_empty() {
        return Err(Error::NameEmpty);
    }
    let target_path = get_saves_dir(app)?.join(format!("{}.json", name));
    Ok(target_path)
}

#[tauri::command]
pub async fn fetch_saves(app: tauri::AppHandle) -> Result<Vec<FileData>, Error> {
    let mut saves: Vec<FileData> = Vec::new();

    for entry in fs::read_dir(get_saves_dir(app)?)? {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() && path.extension().contains(&"json") {
            saves.push(FileData::new(path, metadata)?);
        }
    }

    Ok(saves)
}

#[tauri::command]
pub async fn create_save(app: tauri::AppHandle, name: &str) -> Result<(), Error> {
    let target_path = get_save_fp(app, name)?;
    serde_json::to_writer(fs::File::create(target_path)?, &Save{name: name.to_string(), data: None})?;
    Ok(())
}

#[tauri::command]
pub async fn delete_save(app: tauri::AppHandle, name: &str) -> Result<(), Error> {
    let target_path = get_save_fp(app, name)?;
    fs::remove_file(target_path)?;
    Ok(())
}