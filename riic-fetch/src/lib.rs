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
    unused_import_braces,
    unused_qualifications
)]

mod base;
mod operator_de;
mod operator_ser;
mod terms;

pub use base::BaseData;
pub use operator_de::OperatorTableDe;
pub use operator_ser::OperatorTableSer;
pub use terms::TermData;

use anyhow::Result;
use image::{
    codecs::webp::{WebPEncoder, WebPQuality},
    imageops::{resize, FilterType},
    load_from_memory_with_format, ColorType, ImageFormat,
};
use serde::de::DeserializeOwned;
use std::{cmp::min, fs::File, path::Path};
use ureq::Agent;

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

    fn fetch(client: Agent, server: Server) -> Result<Self> {
        let url = format!("{}/{}", server.base_url(), Self::PATH);

        let data = client.get(&url).call()?.into_json()?;

        Ok(data)
    }
}

pub trait GetIcons {
    const ICON_DIR: &'static str;

    fn get_icons<P>(&self, client: Agent, target_dir: P, min_size: u32, quality: u8) -> Result<()>
    where
        P: AsRef<Path> + Sync;

    fn get_icon(
        client: &Agent,
        id: &str,
        target_dir: &Path,
        min_size: u32,
        quality: u8,
    ) -> Result<()> {
        let target_path = target_dir.join(id).with_extension("webp");

        if target_path.is_file() {
            return Ok(());
        }

        let url = format!(
            "https://raw.githubusercontent.com/astral4/arkdata/main/assets/{}/{id}.png",
            Self::ICON_DIR
        );

        let res = client.get(&url).call()?;

        let mut bytes = match res.header("Content-Length") {
            Some(len) => Vec::with_capacity(len.parse()?),
            None => Vec::new(),
        };

        res.into_reader().read_to_end(&mut bytes)?;

        let mut image = load_from_memory_with_format(&bytes, ImageFormat::Png)?.to_rgb8();

        let (width, height) = image.dimensions();
        let min_dim = min(width, height);

        if min_dim < min_size {
            let scale_factor = f64::from(min_size) / f64::from(min_dim);
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
}
