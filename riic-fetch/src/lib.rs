mod base;
mod operator_de;
mod operator_ser;
mod terms;

pub use base::{BaseData, SkillTable};
pub use operator_de::OperatorTableDe;
pub use operator_ser::OperatorTableSer;
pub use terms::TermData;

use anyhow::Result;
use image::{
    codecs::avif::AvifEncoder,
    imageops::{resize, FilterType},
    load_from_memory_with_format, ColorType, EncodableLayout, ImageEncoder, ImageFormat,
};
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::{cmp::min, fs::File, path::Path};
use tokio::task::JoinSet;

pub enum Server {
    US,
    CN,
}

impl Server {
    const fn base_url(&self) -> &str {
        match self {
            Server::US => {
                "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData_YoStar/master/en_US"
            }
            Server::CN => {
                "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN"
            }
        }
    }
}

pub trait Fetch: Sized + DeserializeOwned {
    const PATH: &'static str;

    #[must_use]
    async fn fetch(client: Client, server: Server) -> Result<Self> {
        let url = format!("{}/{}", server.base_url(), Self::PATH);

        let data = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(data)
    }
}

pub trait GetIcons {
    const ICON_DIR: &'static str;

    fn get_icons<P>(
        &self,
        client: Client,
        target_dir: P,
        min_size: u32,
        quality: u8,
    ) -> JoinSet<Result<()>>
    where
        P: AsRef<Path>;

    #[must_use]
    async fn get_icon(
        client: Client,
        id: Box<str>,
        target_path: Box<Path>,
        min_size: u32,
        quality: u8,
    ) -> Result<()> {
        let url = format!(
            "https://raw.githubusercontent.com/astral4/arkdata/main/assets/{}/{id}.png",
            Self::ICON_DIR
        );

        let bytes = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let mut image = load_from_memory_with_format(&bytes, ImageFormat::Png)?.into_rgba8();

        let min_dim = min(image.width(), image.height());

        if min_dim < min_size {
            image = resize(
                &image,
                image.width() * min_size / min_dim,
                image.height() * min_size / min_dim,
                FilterType::Lanczos3,
            );
        }

        let file = File::create(target_path)?;

        AvifEncoder::new_with_speed_quality(file, 1, quality).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            ColorType::Rgba8,
        )?;

        Ok(())
    }
}
