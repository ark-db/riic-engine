use crate::Fetch;
use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use std::{fs::File, io::BufWriter, path::Path};

#[derive(Deserialize)]
pub struct TermData {
    #[serde(rename(deserialize = "richTextStyles"))]
    pub styles: StyleTable,
    #[serde(rename(deserialize = "termDescriptionDict"))]
    pub terms: TermTable,
}

impl Fetch for TermData {
    const PATH: &'static str = "gamedata/excel/gamedata_const.json";
}

#[derive(Deserialize)]
pub struct StyleTable(IndexMap<Box<str>, Box<str>>);

impl StyleTable {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let table: IndexMap<_, _> = self
            .0
            .iter()
            .filter(|(id, _)| id.starts_with("cc"))
            .filter_map(|(id, style)| {
                style.split_once('#').map(|(_, s)| {
                    let hex: String = s.chars().take(6).collect();
                    (id, format!("#{hex}").into_boxed_str())
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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let table: IndexMap<_, _> = self
            .0
            .iter()
            .filter(|(id, _)| id.starts_with("cc"))
            .map(|(id, desc)| (id, &desc.description))
            .collect();

        let file = BufWriter::new(File::create(path)?);

        serde_json::to_writer(file, &table)?;

        Ok(())
    }
}

#[derive(Deserialize)]
struct Description {
    description: Box<str>,
}
