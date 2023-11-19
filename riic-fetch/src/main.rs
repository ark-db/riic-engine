use anyhow::{Error, Result};
use reqwest::Client;
use riic_fetch::{
    BaseData, Fetch, GetIcons, OperatorTableDe, OperatorTableSer, Server, SkillTable, TermData,
};
use serde::{
    de::{Error as DeError, Unexpected},
    Deserialize, Deserializer,
};
use std::{fs::read_to_string, path::Path, sync::Arc, time::Duration};
use tokio::{
    task::{spawn, spawn_blocking, JoinHandle},
    try_join,
};

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
struct Config {
    operator: SaveConfig,
    skill: SaveConfig,
    terms_path: Box<Path>,
    styles_path: Box<Path>,
    min_image_size: u32,
}

impl Config {
    fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = read_to_string(path)?;
        let config = toml::from_str(&data)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
struct SaveConfig {
    data_path: Box<Path>,
    image_dir: Box<Path>,
    #[serde(deserialize_with = "deserialize_quality")]
    quality: u8,
}

fn deserialize_quality<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        n @ 1..=100 => Ok(n),
        other => Err(DeError::invalid_value(
            Unexpected::Unsigned(u64::from(other)),
            &"Expected a u8 from 1 to 100",
        )),
    }
}

#[allow(clippy::similar_names)]
#[tokio::main]
async fn main() -> Result<()> {
    // build HTTP client
    let client = Client::builder()
        .https_only(true)
        .timeout(Duration::from_secs(60))
        .use_rustls_tls()
        .build()
        .expect("failed to build HTTP client");

    // fetch operator data
    let ops_handle = spawn(OperatorTableDe::fetch(client.clone(), Server::CN));

    // read config file
    let config = Config::from_toml(CONFIG_PATH)?;

    let client1 = client.clone();
    let skills_handle = spawn(async move {
        let client2 = client1.clone();
        let cn_handle: JoinHandle<Result<_>> = spawn(async move {
            // fetch CN base data
            let data = BaseData::fetch(client2.clone(), Server::CN).await?;
            let (ops, skills) = (Arc::new(data.ops), Arc::new(data.skills));

            let o = ops.clone();
            let h0 = spawn(async move {
                // transform operator data
                let ser = OperatorTableSer::create(ops_handle.await??, &o);
                // save operator data
                ser.save(config.operator.data_path)
            });

            let client3 = client2.clone();
            let h1 = spawn(async move {
                // fetch + save operator icons
                let mut set = ops.get_icons(
                    client3.clone(),
                    config.operator.image_dir,
                    config.min_image_size,
                    config.operator.quality,
                );
                while let Some(res) = set.join_next().await {
                    res??;
                }
                Ok::<_, Error>(())
            });

            let s = skills.clone();
            let h2 = spawn(async move {
                // fetch + save skill icons
                let mut set = s.get_icons(
                    client2,
                    config.skill.image_dir,
                    config.min_image_size,
                    config.skill.quality,
                );
                while let Some(res) = set.join_next().await {
                    res??;
                }
                Ok::<_, Error>(())
            });

            // return base skills
            Ok((skills, (h0, h1, h2)))
        });

        let en_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch US base data
            let data = BaseData::fetch(client1, Server::US).await?;
            // return base skills
            Ok(data.skills)
        });

        let (cn_skills, en_skills) = try_join!(cn_handle, en_handle)?;
        let ((cn_skills, (h0, h1, h2)), en_skills) = (cn_skills?, en_skills?);

        // combine base skills
        let skills = SkillTable::combine(&cn_skills, &en_skills);

        // save base skills
        skills.save(config.skill.data_path)?;

        Ok::<_, Error>((h0, h1, h2))
    });

    let terms_handle = spawn(async move {
        let client1 = client.clone();
        let cn_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch CN term data
            let data = TermData::fetch(client1, Server::CN).await?;

            // save text style data
            let h = spawn_blocking(move || data.styles.save(config.styles_path));

            // return skill terms
            Ok((data.terms, h))
        });

        let en_handle: JoinHandle<Result<_>> = spawn(async {
            // fetch US term data
            let data = TermData::fetch(client, Server::US).await?;
            // return skill terms
            Ok(data.terms)
        });

        let (cn_terms, en_terms) = try_join!(cn_handle, en_handle)?;
        let ((mut cn_terms, h), en_terms) = (cn_terms?, en_terms?);

        // combine skill terms
        cn_terms.extend(en_terms);

        // save skill terms
        cn_terms.save(config.terms_path)?;

        Ok::<_, Error>(h)
    });

    // wait for all tasks to finish and catch errors
    let (skills_result, terms_result) = try_join!(skills_handle, terms_handle)?;
    let (h0, h1, h2) = skills_result?;
    let h3 = terms_result?;
    let (h0, h1, h2, h3) = try_join!(h0, h1, h2, h3)?;
    #[allow(clippy::unnecessary_operation)]
    (h0?, h1?, h2?, h3?);

    Ok(())
}
