mod base;
pub mod db;
pub mod window;

const MAX_SAVE_SIZE: usize = 1_000_000;

pub mod open {
    /// # Errors
    /// Returns error if the URL cannot be opened.
    #[tauri::command]
    pub fn open(url: &str) -> Result<(), &str> {
        open::that_detached(url).map_err(|_| "An error occurred while opening the URL")
    }
}
