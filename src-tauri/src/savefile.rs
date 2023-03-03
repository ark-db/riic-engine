use crate::{base::Save, CmdError, CmdResult};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    fs::{copy, create_dir_all, metadata, read_dir, remove_file, rename, File, Metadata},
    io::BufReader,
    path::{Path, PathBuf},
};
use tauri::{api::path::download_dir, App};

static SAVE_DIR: OnceCell<PathBuf> = OnceCell::new();
static DOWNLOAD_DIR: OnceCell<PathBuf> = OnceCell::new();

pub(crate) fn load_savefile_dirs(app: &App) {
    let path_resolver = app.path_resolver();

    let saves_dir = path_resolver
        .app_data_dir()
        .expect("Failed to retrieve app data directory")
        .join("saves");
    if !saves_dir.is_dir() {
        create_dir_all(&saves_dir).expect("Failed to create save directory");
    }
    SAVE_DIR.set(saves_dir).unwrap();

    let download_dir = download_dir().expect("Failed to retrieve download directory");
    DOWNLOAD_DIR.set(download_dir).unwrap();
}

#[derive(Serialize)]
pub struct FileData {
    name: String,
    modified: f32,
    created: f32,
}

impl FileData {
    fn new(path: &Path, metadata: &Metadata) -> CmdResult<Self> {
        Ok(Self {
            name: path
                .file_stem()
                .ok_or(CmdError::NameEmpty)?
                .to_string_lossy()
                .to_string(),
            modified: metadata.modified()?.elapsed()?.as_secs_f32(),
            created: metadata.created()?.elapsed()?.as_secs_f32(),
        })
    }
}

#[derive(Deserialize)]
pub struct ImportedSave {
    name: String,
    data: Save,
}

fn get_save_fp(name: &str) -> CmdResult<PathBuf> {
    if name.is_empty() {
        Err(CmdError::NameEmpty)
    } else {
        Ok(SAVE_DIR.wait().join(name).with_extension("json"))
    }
}

fn get_available_fp(dir: &Path, name: &str) -> PathBuf {
    let mut path = dir.to_path_buf();
    path.push(name);
    path.set_extension("json");

    let mut index = 0u32;
    while path.is_file() {
        index += 1;
        path.set_file_name(format!("{}-{}.json", name, index));
    }
    path
}

/// # Errors
/// Returns error if:
/// - Save dir cannot be fetched or read
/// - Entry in save dir cannot be read
/// - FileData cannot be initialized
#[tauri::command]
pub fn fetch_saves() -> CmdResult<Vec<FileData>> {
    Ok(read_dir(SAVE_DIR.wait())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter_map(|path| {
            let data = metadata(&path).ok()?;
            let ext = path.extension()?;
            if ext == "json" && data.is_file() {
                FileData::new(&path, &data).ok()
            } else {
                None
            }
        })
        .collect())
}

/// # Errors
/// Returns error if:
/// - Save dir cannot be fetched
/// - New save file cannot be created
#[tauri::command]
pub fn create_save() -> CmdResult<()> {
    let target_path = get_available_fp(SAVE_DIR.wait(), "Untitled");
    serde_json::to_writer(File::create(target_path)?, &Save::default())?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be opened or serialized
#[tauri::command]
pub fn load_save(name: &str) -> CmdResult<Save> {
    let target_path = get_save_fp(name)?;
    let file = File::open(target_path)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

/// # Errors
/// Returns error if:
/// - Path of old or new save file cannot be fetched
/// - Save file cannot be renamed
#[tauri::command]
pub fn rename_save(old: &str, new: &str) -> CmdResult<()> {
    let new_path = get_save_fp(new)?;
    if new_path.is_file() {
        return Err(CmdError::DuplicateName);
    }

    let old_path = get_save_fp(old)?;
    rename(old_path, new_path)?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be created or deserialized
#[tauri::command]
pub fn update_save(save: Option<ImportedSave>) -> CmdResult<()> {
    if let Some(save) = save {
        let save_path = get_save_fp(&save.name)?;
        serde_json::to_writer(File::create(save_path)?, &save.data)?;
    }
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be removed
#[tauri::command]
pub fn delete_save(name: &str) -> CmdResult<()> {
    let target_path = get_save_fp(name)?;
    remove_file(target_path)?;
    Ok(())
}

/// # Errors
/// Returns error if:
/// - Path of save file cannot be fetched
/// - Save file cannot be copied
#[tauri::command]
pub fn export_save(name: &str) -> CmdResult<()> {
    let save_path = get_save_fp(name)?;
    let target_path = get_available_fp(DOWNLOAD_DIR.wait(), name);
    copy(save_path, target_path)?;
    Ok(())
}
