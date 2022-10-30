#![warn(clippy::all, clippy::pedantic)]

use std::fs;



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
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}



#[tauri::command]
pub async fn fetch_saves(app: tauri::AppHandle) -> Result<Vec<[String; 3]>, Error> {
    let app_dir = app.path_resolver().app_dir().expect("App directory should be retrievable");
    let saves_dir = app_dir.join("saves");

    if !&saves_dir.is_dir() {
        fs::create_dir_all(&saves_dir)?;
    }

    let mut saves: Vec<[String; 3]> = Vec::new();

    for entry in fs::read_dir(saves_dir)? {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;
        
        if metadata.is_file() && path.extension().contains(&"json") {
            let file_data = [
                path.file_prefix().map_or("Untitled", |f| f.to_str().unwrap()).to_owned(),
                metadata.modified()?.elapsed()?.as_secs().to_string(),
                metadata.created()?.elapsed()?.as_secs().to_string()
            ];
            saves.push(file_data);
        }
    }

    Ok(saves)

}