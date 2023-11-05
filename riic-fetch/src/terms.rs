use crate::Server;
use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};
use std::{fs::File, io::BufWriter, path::Path};
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

#[derive(Deserialize)]
pub struct StyleTable(IndexMap<Box<str>, Box<str>>);

impl StyleTable {
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let table: IndexMap<_, Box<str>> = self
            .0
            .iter()
            .filter(|(id, _)| id.starts_with("cc"))
            .filter_map(|(id, style)| {
                style.split_once('#').map(|(_, s)| {
                    (
                        id,
                        format!("#{}", s.chars().take(6).collect::<String>()).into(),
                    )
                })
            })
            .collect();

        let file = BufWriter::new(File::create(path)?);

        serde_json::to_writer(file, &table)?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct TermTable(IndexMap<Box<str>, Description>);

impl TermTable {
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

#[derive(Deserialize)]
struct Description {
    description: Box<str>,
}
