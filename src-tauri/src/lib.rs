#![warn(clippy::all, clippy::pedantic)]

mod base;
pub mod savefile;
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum CmdError {
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
}

impl serde::Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type CmdResult<T> = Result<T, CmdError>;
