use ahash::HashMap;
use serde::{de, Deserialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) operator: SaveConfig,
    pub(crate) skill: SaveConfig,
    pub(crate) item: ImageConfig,
    pub(crate) elite: ImageConfig,
    pub(crate) facility: SaveConfig,
    pub(crate) terms_path: PathBuf,
    pub(crate) styles_path: PathBuf,
    pub(crate) name_overrides: HashMap<String, String>,
    pub(crate) item_whitelist: Vec<String>,
    pub(crate) min_image_size: u32,
}

#[derive(Deserialize)]
pub(crate) struct SaveConfig {
    data_path: PathBuf,
    image_dir: PathBuf,
    #[serde(deserialize_with = "deserialize_quality")]
    quality: u8,
}

#[derive(Deserialize)]
pub(crate) struct ImageConfig {
    image_dir: PathBuf,
    #[serde(deserialize_with = "deserialize_quality")]
    quality: u8,
}

fn deserialize_quality<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: de::Deserializer<'de>,
{
    let num: u8 = Deserialize::deserialize(deserializer)?;

    match num {
        n @ 1..=100 => Ok(n),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(u64::from(other)),
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
