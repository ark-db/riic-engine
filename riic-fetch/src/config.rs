use ahash::HashMap;
use serde::{de, Deserialize};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Deserialize)]
pub(crate) struct Config {
    operator: SaveConfig,
    skill: SaveConfig,
    item: ImageConfig,
    elite: ImageConfig,
    facility: SaveConfig,
    name_overrides: HashMap<String, String>,
    item_whitelist: Vec<String>,
    min_image_size: u8,
}

#[derive(Deserialize)]
struct SaveConfig {
    data_path: PathBuf,
    image_dir: PathBuf,
    #[serde(deserialize_with = "deserialize_quality")]
    quality: u8,
}

#[derive(Deserialize)]
struct ImageConfig {
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
enum ConfigError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl Config {
    fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let data = read_to_string(path)?;
        toml::from_str(&data).map_err(ConfigError::Toml)
    }
}
