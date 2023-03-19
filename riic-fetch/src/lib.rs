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
use reqwest::Client;
use serde::Deserialize;
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
