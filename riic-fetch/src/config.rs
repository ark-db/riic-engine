use crate::FetchImage;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use std::{
    borrow::Cow,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) operator: SaveConfig,
    pub(crate) skill: SaveConfig,
    pub(crate) facility: SaveConfig,
    pub(crate) item: ImageConfig,
    pub(crate) terms_path: PathBuf,
    pub(crate) styles_path: PathBuf,
    pub(crate) min_image_size: u32,
}

#[derive(Deserialize)]
pub(crate) struct SaveConfig {
    pub(crate) data_path: PathBuf,
    pub(crate) image_dir: PathBuf,
    #[serde(deserialize_with = "deserialize_quality")]
    pub(crate) quality: u8,
}

#[derive(Deserialize)]
pub(crate) struct ImageConfig {
    pub(crate) whitelist: Vec<String>,
    pub(crate) image_dir: PathBuf,
    #[serde(deserialize_with = "deserialize_quality")]
    pub(crate) quality: u8,
}

fn deserialize_quality<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let num = Deserialize::deserialize(deserializer)?;

    match num {
        n @ 1..=100 => Ok(n),
        other => Err(Error::invalid_value(
            Unexpected::Unsigned(u64::from(other)),
            &"Expected a u8 from 1 to 100",
        )),
    }
}

#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl Config {
    /// # Errors
    /// Returns an error if an I/O error occurs or the file at the input path cannot be parsed successfully.
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let data = read_to_string(path)?;
        toml::from_str(&data).map_err(ConfigError::Toml)
    }
}

impl FetchImage for ImageConfig {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/items/icons";

    fn image_ids(&self) -> Vec<Cow<'_, str>> {
        self.whitelist
            .iter()
            .map(|s| Cow::Borrowed(s.as_str()))
            .collect()
    }
}
