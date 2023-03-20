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
mod operator;
mod terms;

use async_trait::async_trait;
use futures::{stream::FuturesUnordered, StreamExt};
use image::{
    codecs::webp::{WebPEncoder, WebPQuality},
    imageops::{resize, FilterType},
    load_from_memory_with_format, ColorType, ImageFormat,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{cmp::min, fs::File, path::Path};
use thiserror::Error;

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
enum FetchError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[async_trait]
trait Fetch {
    const FETCH_PATH: &'static str;

    async fn fetch(client: &Client, server: Server) -> Result<Self, FetchError>
    where
        for<'de> Self: Sized + Deserialize<'de>,
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
enum SaveError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

trait SaveJson: Serialize {
    fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveError> {
        let file = File::create(path)?;
        serde_json::to_writer(file, self).map_err(SaveError::Serde)
    }
}

#[derive(Error, Debug)]
enum ImageSaveError {
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

    fn image_ids(&self) -> Vec<String>;

    async fn save_image(
        client: &Client,
        id: String,
        target_dir: &Path,
        quality: u8,
        min_size: u32,
    ) -> Result<(), ImageSaveError> {
        let target_path = target_dir.join(&id).with_extension(".webp");

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

        WebPEncoder::new_with_quality(file, WebPQuality::lossy(quality))
            .encode(&image, image.width(), image.height(), ColorType::Rgba8)
            .map_err(ImageSaveError::Image)
    }

    async fn save_images<'a, I: Iterator<Item = &'a str>, P: AsRef<Path> + Send>(
        &self,
        client: &Client,
        target_dir: P,
        quality: u8,
        min_size: u32,
    ) -> Result<(), ImageSaveError> {
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
