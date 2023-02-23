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

mod base;
pub mod savefile;
pub mod window;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("A system I/O error occurred")]
    Io(#[from] std::io::Error),

    #[error("A system time error occurred")]
    Time(#[from] std::time::SystemTimeError),

    #[error("An error occurred while writing the save file")]
    FileWrite(#[from] serde_json::Error),

    #[error("A save file name was not specified")]
    NameEmpty,

    #[error("Another file with the same name already exists")]
    DuplicateName,
}

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type CmdResult<T> = Result<T, CmdError>;
