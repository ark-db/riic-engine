use crate::Server;
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
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[async_trait]
pub(crate) trait Fetch {
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

pub(crate) trait SaveJson {
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
pub(crate) trait FetchImage {
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