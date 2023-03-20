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
mod config;
mod consts;
mod operator;
mod terms;
pub use config::{Config, ConfigError};

use async_trait::async_trait;
use futures::{stream::FuturesUnordered, StreamExt};
use image::{
    codecs::webp::{WebPEncoder, WebPQuality},
    imageops::{resize, FilterType},
    load_from_memory_with_format, ColorType, ImageFormat,
};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    cmp::min,
    fs::{create_dir_all, File},
    path::Path,
    time::Duration,
};
use thiserror::Error;
use tokio as _;

enum Server {
    US,
    CN,
}

impl ToString for Server {
    fn to_string(&self) -> String {
        match self {
            Self::US => String::from("en_US"),
            Self::CN => String::from("zh_CN"),
        }
    }
}

#[derive(Error, Debug)]
pub enum FetchError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[async_trait]
trait Fetch {
    const FETCH_PATH: &'static str;

    async fn fetch(client: &Client, server: Server) -> Result<Self, FetchError>
    where
        Self: Sized + DeserializeOwned,
    {
        let url = format!(
            "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/{}/{}",
            server.to_string(),
            Self::FETCH_PATH
        );

        client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(FetchError::Reqwest)
    }
}

#[derive(Error, Debug)]
pub enum SaveError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

trait SaveJson {
    fn save_json<P>(&self, path: P) -> Result<(), SaveError>
    where
        Self: Serialize,
        P: AsRef<Path>,
    {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ImageSaveError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[async_trait(?Send)]
trait FetchImage {
    const FETCH_DIR: &'static str;

    fn image_ids(&self) -> Vec<Cow<'_, str>>;

    async fn save_image(
        client: &Client,
        id: Cow<'_, str>,
        target_dir: &Path,
        quality: u8,
        min_size: u32,
    ) -> Result<(), ImageSaveError> {
        let target_path = target_dir.join(id.as_ref()).with_extension("webp");

        if target_path.is_file() {
            return Ok(());
        }

        let url = format!(
            "https://raw.githubusercontent.com/astral4/arkdata/main/assets/{}/{}.png",
            Self::FETCH_DIR,
            id
        );

        let data = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let mut image = load_from_memory_with_format(&data, ImageFormat::Png)?.to_rgba8();

        let (width, height) = image.dimensions();
        let min_dimension = min(width, height);

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        if min_dimension < min_size {
            let scale_factor = f64::from(min_size) / f64::from(min_dimension);
            image = resize(
                &image,
                (scale_factor * f64::from(width)) as u32,
                (scale_factor * f64::from(height)) as u32,
                FilterType::Lanczos3,
            );
        }

        let file = File::create(target_path)?;

        WebPEncoder::new_with_quality(file, WebPQuality::lossy(quality)).encode(
            &image,
            image.width(),
            image.height(),
            ColorType::Rgba8,
        )?;

        Ok(())
    }

    async fn save_images<P>(
        &self,
        client: &Client,
        target_dir: P,
        quality: u8,
        min_size: u32,
    ) -> Result<(), ImageSaveError>
    where
        P: AsRef<Path>,
    {
        if !target_dir.as_ref().is_dir() {
            create_dir_all(&target_dir)?;
        }

        self.image_ids()
            .into_iter()
            .map(|id| Self::save_image(client, id, target_dir.as_ref(), quality, min_size))
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<Result<(), ImageSaveError>>>()
            .await
            .into_iter()
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Fetch(#[from] FetchError),
    #[error(transparent)]
    SaveJson(#[from] SaveError),
    #[error(transparent)]
    SaveImage(#[from] ImageSaveError),
}

impl Config {
    /// # Errors
    /// Returns an error if:
    /// - The `reqwest::Client` could not be constructed
    /// - An error occurred while fetching game data
    /// - An error occurred while saving JSON data
    /// - An error occurred while saving image data
    pub async fn fetch(&self) -> Result<(), AppError> {
        let client = Client::builder()
            .https_only(true)
            .timeout(Duration::from_secs(10))
            .use_rustls_tls()
            .build()?;

        self.item
            .save_images(
                &client,
                &self.item.image_dir,
                self.item.quality,
                self.min_image_size,
            )
            .await?;

        let mut cn_misc = terms::MiscGamedata::fetch(&client, Server::CN).await?;
        cn_misc.styles.save_json(&self.styles_path)?;
        let en_terms = terms::MiscGamedata::fetch(&client, Server::US).await?.terms;
        cn_misc.terms.extend(en_terms);
        cn_misc.terms.save_json(&self.terms_path)?;

        let mut cn_base_data = base::BaseData::fetch(&client, Server::CN).await?;
        let us_base_data = base::BaseData::fetch(&client, Server::US).await?;

        let facility_data = us_base_data.rooms;
        facility_data.save_json(&self.facility.data_path)?;
        facility_data
            .save_images(
                &client,
                &self.facility.image_dir,
                self.facility.quality,
                self.min_image_size,
            )
            .await?;

        cn_base_data.buffs.extend(us_base_data.buffs);
        cn_base_data.buffs.save_json(&self.skill.data_path)?;
        cn_base_data
            .buffs
            .save_images(
                &client,
                &self.skill.image_dir,
                self.skill.quality,
                self.min_image_size,
            )
            .await?;

        cn_base_data.chars.extend(us_base_data.chars);
        let chars = operator::OperatorTable::fetch(&client, Server::CN)
            .await?
            .into_updated(&cn_base_data.chars);
        chars.save_json(&self.operator.data_path)?;
        chars
            .save_images(
                &client,
                &self.operator.image_dir,
                self.operator.quality,
                self.min_image_size,
            )
            .await?;

        Ok(())
    }
}
