use riic_fetch::Config;

#[tokio::main]
async fn main() {
    Config::from_toml("config.toml")
        .expect("Failed to read configuration file")
        .fetch()
        .await
        .expect("Failed to fetch and save data");
}
