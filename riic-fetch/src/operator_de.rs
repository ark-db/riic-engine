use crate::Fetch;
use anyhow::Result;
use indexmap::IndexMap;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

#[derive(Deserialize)]
pub struct OperatorTableDe(pub(crate) IndexMap<Box<str>, Operator>);

impl Fetch for OperatorTableDe {
    const PATH: &'static str = "gamedata/excel/character_table.json";
}

#[derive(Deserialize)]
pub(crate) struct Operator {
    #[serde(rename(deserialize = "appellation"))]
    pub(crate) name: Box<str>,
    #[serde(deserialize_with = "deserialize_rarity")]
    pub(crate) rarity: u8,
    #[serde(rename(deserialize = "isNotObtainable"))]
    unobtainable: bool,
    profession: Profession,
}

impl Operator {
    pub(crate) fn is_operator(&self) -> bool {
        !self.unobtainable && !matches!(self.profession, Profession::Token | Profession::Trap)
    }
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
enum Profession {
    Pioneer, // Vanguard
    Warrior, // Guard
    Tank,    // Defender
    Sniper,
    Caster,
    Medic,
    Support, // Supporter
    Special, // Specialist
    Token,
    Trap,
}

fn deserialize_rarity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "TIER_1" => Ok(1),
        "TIER_2" => Ok(2),
        "TIER_3" => Ok(3),
        "TIER_4" => Ok(4),
        "TIER_5" => Ok(5),
        "TIER_6" => Ok(6),
        s => Err(Error::invalid_value(
            Unexpected::Str(s),
            &"Expected a string with pattern \"TIER_n\", where n is a number from 1 to 6",
        )),
    }
}
