use super::Server;
use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use ureq::Agent;

#[derive(Deserialize)]
pub struct TermData {
    #[serde(rename(deserialize = "richTextStyles"))]
    pub styles: StyleTable,
    #[serde(rename(deserialize = "termDescriptionDict"))]
    pub terms: TermTable,
}

impl TermData {
    pub fn fetch(client: &Agent, server: Server) -> Result<Self> {
        let url = format!("{}/gamedata/excel/gamedata_const.json", server.base_url());

        let data = client.get(&url).call()?.into_json()?;

        Ok(data)
    }
}

type StyleTable = IndexMap<Box<str>, Box<str>>;

type TermTable = IndexMap<Box<str>, Description>;

#[derive(Deserialize)]
pub struct Description {
    description: Box<str>,
}
