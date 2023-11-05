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
mod operator;

pub use base::BaseData;
pub use operator::OperatorTable;

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
