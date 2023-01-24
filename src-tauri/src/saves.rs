#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

use crate::base::Save;
use crate::{CmdError, CmdResult};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
};
use tauri::AppHandle;

#[derive(Serialize)]
pub struct FileData {
    name: String,
    modified: u64,
    created: u64,
}

impl FileData {
    fn new(path: &Path, metadata: &fs::Metadata) -> CmdResult<Self> {
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

#[derive(Deserialize)]
pub struct ImportedSave {
    name: String,
    data: Save,
}

fn get_saves_dir(app: &AppHandle) -> CmdResult<PathBuf> {
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

fn get_save_fp(app: &AppHandle, name: &str) -> CmdResult<PathBuf> {
    if name.is_empty() {
        return Err(CmdError::NameEmpty);
    }
    let target_path = get_saves_dir(app)?.join(name).with_extension("json");
    if target_path.is_relative() {
        return Err(CmdError::RelativePath);
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
pub fn fetch_saves(app: AppHandle) -> CmdResult<Vec<FileData>> {
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
pub fn create_save(app: AppHandle) -> CmdResult<()> {
    let save_dir = get_saves_dir(&app)?;
    let target_path = get_available_fp(save_dir, "Untitled");
    serde_json::to_writer(fs::File::create(target_path)?, &Save::default())?;
    Ok(())
}

#[tauri::command]
pub fn load_save(app: AppHandle, name: &str) -> CmdResult<Save> {
    let target_path = get_save_fp(&app, name)?;
    let file = fs::File::open(target_path)?;
    let data: Save = serde_json::from_reader(BufReader::new(file))?;
    Ok(data)
}

#[tauri::command]
pub fn rename_save(app: AppHandle, old: &str, new: &str) -> CmdResult<()> {
    let new_path = get_save_fp(&app, new)?;
    if new_path.is_file() {
        return Err(CmdError::DuplicateName);
    }

    let old_path = get_save_fp(&app, old)?;
    fs::rename(old_path, new_path)?;
    Ok(())
}

#[tauri::command]
pub fn update_save(app: AppHandle, save: Option<ImportedSave>) -> CmdResult<()> {
    if let Some(save) = save {
        let save_path = get_save_fp(&app, &save.name)?;
        serde_json::to_writer(fs::File::create(save_path)?, &save.data)?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_save(app: AppHandle, name: &str) -> CmdResult<()> {
    let target_path = get_save_fp(&app, name)?;
    fs::remove_file(target_path)?;
    Ok(())
}

#[tauri::command]
pub fn export_save(app: tauri::AppHandle, name: &str) -> CmdResult<()> {
    let save_path = get_save_fp(&app, name)?;
    let target_path = get_available_fp(
        tauri::api::path::download_dir().expect("Download directory should be retrievable"),
        name,
    );
    fs::copy(save_path, target_path)?;
    Ok(())
}
