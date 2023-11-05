use crate::{Fetch, GetIcons};
use anyhow::Result;
use indexmap::IndexMap;
use rayon::iter::ParallelIterator;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::{fs::File, io::BufWriter, path::Path};
use ureq::Agent;

#[derive(Deserialize)]
pub struct BaseData {
    #[serde(rename(deserialize = "chars"))]
    pub ops: OperatorSkills,
    #[serde(rename(deserialize = "buffs"))]
    pub skills: SkillTable,
}

impl Fetch for BaseData {
    const PATH: &'static str = "gamedata/excel/building_data.json";
}

#[derive(Deserialize)]
pub struct OperatorSkills(pub(crate) IndexMap<Box<str>, Operator>);

impl GetIcons for OperatorSkills {
    const ICON_DIR: &'static str = "torappu/dynamicassets/arts/charavatars";

    fn get_icons<P>(&self, client: Agent, target_dir: P, min_size: u32, quality: u8) -> Result<()>
    where
        P: AsRef<Path> + Sync,
    {
        self.0
            .par_keys()
            .map(|id| Self::get_icon(&client, id, target_dir.as_ref(), min_size, quality))
            .collect::<Result<()>>()
    }
}

#[derive(Deserialize)]
pub(crate) struct Operator {
    #[serde(rename(deserialize = "buffChar"))]
    pub(crate) inner: Box<[OperatorSkill]>,
}

#[derive(Deserialize)]
pub(crate) struct OperatorSkill {
    #[serde(rename(deserialize = "buffData"))]
    pub(crate) inner: Box<[SkillPhase]>,
}

#[derive(Deserialize)]
pub(crate) struct SkillPhase {
    #[serde(rename(deserialize = "buffId"))]
    pub(crate) id: Box<str>,
    pub(crate) cond: Level,
}

#[derive(Deserialize)]
pub(crate) struct Level {
    #[serde(rename(deserialize = "phase"), deserialize_with = "deserialize_elite")]
    pub(crate) elite: u8,
    pub(crate) level: u8,
}

fn deserialize_elite<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "PHASE_0" => Ok(0),
        "PHASE_1" => Ok(1),
        "PHASE_2" => Ok(2),
        s => Err(Error::invalid_value(
            Unexpected::Str(s),
            &"Expected a string with pattern \"PHASE_n\", where n is a number from 0 to 2",
        )),
    }
}

#[derive(Deserialize)]
pub struct SkillTable(IndexMap<Box<str>, Skill>);

impl SkillTable {
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = BufWriter::new(File::create(path)?);

        serde_json::to_writer(file, &self.0)?;

        Ok(())
    }
}

impl GetIcons for SkillTable {
    const ICON_DIR: &'static str = "torappu/dynamicassets/arts/building/skills";

    fn get_icons<P>(&self, client: Agent, target_dir: P, min_size: u32, quality: u8) -> Result<()>
    where
        P: AsRef<Path> + Sync,
    {
        self.0
            .par_keys()
            .map(|id| Self::get_icon(&client, id, target_dir.as_ref(), min_size, quality))
            .collect::<Result<()>>()
    }
}

#[derive(Deserialize, Serialize)]
struct Skill {
    #[serde(rename(deserialize = "buffName"))]
    name: Box<str>,
    description: Box<str>,
    #[serde(rename(deserialize = "skillIcon", serialize = "iconId"))]
    icon_id: Box<str>,
}
