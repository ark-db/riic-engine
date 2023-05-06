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
mod traits;
pub use config::{Config, ConfigError};

use reqwest::Client;
use std::time::Duration;
use thiserror::Error;
use tokio as _;
use traits::{Fetch, FetchError, FetchImage, ImageSaveError, SaveError, SaveJson};

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
