use indexmap::IndexMap;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Deserialize)]
pub struct BaseData {
    pub ops: OperatorSkills,
    pub skills: SkillTable,
}

type OperatorSkills = IndexMap<Box<str>, Operator>;

#[derive(Deserialize)]
pub struct Operator {
    #[serde(rename(deserialize = "buffChar"))]
    inner: Box<[Skill]>,
}

#[derive(Deserialize)]
struct Skill {
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

#[derive(Deserialize)]
pub struct SkillTable;
