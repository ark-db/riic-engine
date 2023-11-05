use super::Server;
use anyhow::Result;
use indexmap::IndexMap;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use ureq::Agent;

#[derive(Deserialize)]
pub struct BaseData {
    #[serde(rename(deserialize = "chars"))]
    pub ops: OperatorSkills,
    #[serde(rename(deserialize = "buffs"))]
    pub skills: SkillTable,
}

impl BaseData {
    pub fn fetch(client: &Agent, server: Server) -> Result<Self> {
        let url = format!("{}/gamedata/excel/building_data.json", server.base_url());

        let data = client.get(&url).call()?.into_json()?;

        Ok(data)
    }
}

type OperatorSkills = IndexMap<Box<str>, Operator>;

#[derive(Deserialize)]
pub struct Operator {
    #[serde(rename(deserialize = "buffChar"))]
    inner: Box<[OperatorSkill]>,
}

#[derive(Deserialize)]
struct OperatorSkill {
    #[serde(rename(deserialize = "buffData"))]
    inner: Box<[SkillPhase]>,
}

#[derive(Deserialize)]
struct SkillPhase {
    #[serde(rename(deserialize = "buffId"))]
    id: Box<str>,
    cond: Level,
}

#[derive(Deserialize)]
struct Level {
    #[serde(rename(deserialize = "phase"), deserialize_with = "deserialize_elite")]
    elite: u8,
    level: u8,
}

fn deserialize_elite<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        "PHASE_0" => Ok(0),
        "PHASE_1" => Ok(1),
        "PHASE_2" => Ok(2),
        s => Err(Error::invalid_value(
            Unexpected::Str(s),
            &"Expected a string with pattern \"PHASE_n\", where n is a number from 0 to 2",
        )),
    }
}

type SkillTable = IndexMap<Box<str>, Skill>;

#[derive(Deserialize)]
pub struct Skill {
    #[serde(rename(deserialize = "buffName"))]
    name: Box<str>,
    description: Box<str>,
    #[serde(rename(deserialize = "skillIcon"))]
    icon_id: Box<str>,
}
