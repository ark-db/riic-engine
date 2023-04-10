use crate::{AppError, AppResult};
use once_cell::sync::OnceCell;
use std::{
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};
use tauri::{api::path::download_dir, App};

static SAVE_DIR: OnceCell<PathBuf> = OnceCell::new();
static DOWNLOAD_DIR: OnceCell<PathBuf> = OnceCell::new();

/// # Panics
/// Panics if:
/// - The app data directory cannot be retrieved
/// - The savefile directory cannot be created
/// - The download directory cannot be retrieved
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

fn validate_save_name(name: &str) -> AppResult<&str> {
    if name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        Ok(name)
    } else {
        Err(AppError::InvalidName)
    }
}

fn get_save_fp(name: &str) -> AppResult<PathBuf> {
    if name.is_empty() {
        Err(AppError::NameEmpty)
    } else {
        let path = SAVE_DIR
            .wait()
            .join(validate_save_name(name)?)
            .with_extension("json");
        Ok(path)
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
/// - Path of save file cannot be fetched
/// - Save file cannot be copied
#[tauri::command]
pub fn export_save(name: &str) -> AppResult<()> {
    let save_path = get_save_fp(name)?;
    let target_path = get_available_fp(DOWNLOAD_DIR.wait(), name);
    copy(save_path, target_path)?;
    Ok(())
}
