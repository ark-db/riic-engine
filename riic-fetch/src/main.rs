use riic_fetch::Config;
use std::env::{current_dir, set_current_dir};

#[tokio::main]
async fn main() {
    if !current_dir()
        .expect("Failed to get current directory")
        .ends_with("riic-fetch")
    {
        set_current_dir("riic-fetch").expect("Failed to set current directory to /riic-fetch");
    }

    Config::from_toml("config.toml")
        .expect("Failed to read configuration file")
        .fetch()
        .await
        .expect("Failed to fetch and save data");
}
