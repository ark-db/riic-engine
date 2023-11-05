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
use serde::de::DeserializeOwned;
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
