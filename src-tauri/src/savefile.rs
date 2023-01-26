use crate::{base::Save, CmdError, CmdResult};
use serde::{Deserialize, Serialize};
use std::{
    fs::{copy, create_dir_all, metadata, read_dir, remove_file, rename, File, Metadata},
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
    fn new(path: &Path, metadata: &Metadata) -> CmdResult<Self> {
        let data = Self {
            name: path
                .file_stem()
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
        create_dir_all(&saves_dir)?;
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

/// # Errors
/// Returns error if:
/// - Save dir cannot be fetched or read
/// - Entry in save dir cannot be read
/// - FileData cannot be initialized
#[tauri::command]
pub fn fetch_saves(app: AppHandle) -> CmdResult<Vec<FileData>> {
    Ok(read_dir(get_saves_dir(&app)?)?
        .filter_map(|entry| entry.map(|e| e.path()).ok())
        .filter_map(|path| {
            if let Ok(metadata) = metadata(&path) {
                if let Some(ext) = path.extension() {
                    if ext == "json" && metadata.is_file() {
                        return FileData::new(&path, &metadata).ok();
                    }
                }
            }
            None
        })
        .collect())
}

/// # Errors
/// Returns error if:
/// - Save dir cannot be fetched
/// - New save file cannot be created
#[tauri::command]
pub fn create_save(app: AppHandle) -> CmdResult<()> {
    let save_dir = get_saves_dir(&app)?;
    let target_path = get_available_fp(save_dir, "Untitled");
    serde_json::to_writer(File::create(target_path)?, &Save::default())?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be opened or serialized
#[tauri::command]
pub fn load_save(app: AppHandle, name: &str) -> CmdResult<Save> {
    let target_path = get_save_fp(&app, name)?;
    let file = File::open(target_path)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

/// # Errors
/// Returns error if:
/// - Path of old or new save file cannot be fetched
/// - Save file cannot be renamed
#[tauri::command]
pub fn rename_save(app: AppHandle, old: &str, new: &str) -> CmdResult<()> {
    let new_path = get_save_fp(&app, new)?;
    if new_path.is_file() {
        return Err(CmdError::DuplicateName);
    }

    let old_path = get_save_fp(&app, old)?;
    rename(old_path, new_path)?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be created or deserialized
#[tauri::command]
pub fn update_save(app: AppHandle, save: Option<ImportedSave>) -> CmdResult<()> {
    if let Some(save) = save {
        let save_path = get_save_fp(&app, &save.name)?;
        serde_json::to_writer(File::create(save_path)?, &save.data)?;
    }
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be removed
#[tauri::command]
pub fn delete_save(app: AppHandle, name: &str) -> CmdResult<()> {
    let target_path = get_save_fp(&app, name)?;
    remove_file(target_path)?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be copied
#[tauri::command]
pub fn export_save(app: tauri::AppHandle, name: &str) -> CmdResult<()> {
    let save_path = get_save_fp(&app, name)?;
    let target_path = get_available_fp(
        tauri::api::path::download_dir().expect("Download directory should be retrievable"),
        name,
    );
    copy(save_path, target_path)?;
    Ok(())
}
