#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

use crate::base::Save;
use serde::Serialize;
use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
};

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

    #[error("Another file with the same name already exists")]
    DuplicateName,

    #[error("Relative filepaths are forbidden")]
    RelativePath,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize)]
pub struct FileData {
    name: String,
    modified: u64,
    created: u64,
}

impl FileData {
    fn new(path: &Path, metadata: &fs::Metadata) -> Result<Self, Error> {
        let data = Self {
            name: path
                .file_prefix()
                .map_or("Untitled", |f| f.to_str().unwrap())
                .to_owned(),
            modified: metadata.modified()?.elapsed()?.as_secs(),
            created: metadata.created()?.elapsed()?.as_secs(),
        };
        Ok(data)
    }
}

fn get_saves_dir(app: &tauri::AppHandle) -> Result<PathBuf, Error> {
    let saves_dir = app
        .path_resolver()
        .app_data_dir()
        .expect("App directory should be retrievable")
        .join("saves");

    if !saves_dir.is_dir() {
        fs::create_dir_all(&saves_dir)?;
    }

    Ok(saves_dir)
}

fn get_save_fp(app: &tauri::AppHandle, name: &str) -> Result<PathBuf, Error> {
    if name.is_empty() {
        return Err(Error::NameEmpty);
    }
    let target_path = get_saves_dir(app)?.join(name).with_extension("json");
    if target_path.is_relative() {
        return Err(Error::RelativePath);
    }
    Ok(target_path)
}

fn get_available_fp(dir: PathBuf, name: &str) -> PathBuf {
    let mut path = dir;
    path.push(name);
    path.set_extension("json");
    for i in 1.. {
        if !path.is_file() {
            return path;
        }
        path.set_file_name(format!("{}-{}.json", name, i));
    }
    unreachable!()
}

#[tauri::command]
pub fn fetch_saves(app: tauri::AppHandle) -> Result<Vec<FileData>, Error> {
    let mut saves = Vec::new();

    for entry in fs::read_dir(get_saves_dir(&app)?)? {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            saves.push(FileData::new(&path, &metadata)?);
        }
    }

    Ok(saves)
}

#[tauri::command]
pub fn create_save(app: tauri::AppHandle) -> Result<(), Error> {
    let save_dir = get_saves_dir(&app)?;
    let target_path = get_available_fp(save_dir, "Untitled");
    serde_json::to_writer(fs::File::create(target_path)?, &Save::new())?;
    Ok(())
}

#[tauri::command]
pub fn rename_save(app: tauri::AppHandle, old: &str, new: &str) -> Result<(), Error> {
    let new_path = get_save_fp(&app, new)?;
    if new_path.is_file() {
        return Err(Error::DuplicateName);
    }

    let old_path = get_save_fp(&app, old)?;
    fs::rename(old_path, new_path)?;
    Ok(())
}

#[tauri::command]
pub fn delete_save(app: tauri::AppHandle, name: &str) -> Result<(), Error> {
    let target_path = get_save_fp(&app, name)?;
    fs::remove_file(target_path)?;
    Ok(())
}

#[tauri::command]
pub fn load_save(app: tauri::AppHandle, name: &str) -> Result<Save, Error> {
    let target_path = get_save_fp(&app, name)?;
    let file = fs::File::open(target_path)?;
    let data: Save = serde_json::from_reader(BufReader::new(file))?;
    Ok(data)
}

#[tauri::command]
pub fn export_save(app: tauri::AppHandle, name: &str) -> Result<(), Error> {
    let save_path = get_save_fp(&app, name)?;
    let target_path = get_available_fp(
        tauri::api::path::download_dir().expect("Download directory should be retrievable"),
        name,
    );
    fs::copy(save_path, target_path)?;
    Ok(())
}
