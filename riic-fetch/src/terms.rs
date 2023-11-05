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

#[derive(Deserialize)]
pub struct TermTable(IndexMap<Box<str>, Description>);

impl TermTable {
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0.into_iter())
    }
}

#[derive(Deserialize)]
struct Description {
    description: Box<str>,
}
