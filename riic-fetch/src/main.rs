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

use anyhow as _;
use rayon as _;
use riic_fetch as _;
use std::time::Duration;
use ureq::AgentBuilder;

fn main() {
    let client = AgentBuilder::new()
        .https_only(true)
        .timeout(Duration::from_secs(10))
        .user_agent("")
        .build();

    todo!()
}
