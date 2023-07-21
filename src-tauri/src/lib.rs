#![warn(clippy::pedantic, future_incompatible, nonstandard_style, unused)]
#![deny(
    let_underscore_drop,
    macro_use_extern_crate,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_crate_dependencies,
    unused_import_braces,
    unused_qualifications
)]
#![allow(clippy::module_name_repetitions)]

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
