use crate::Server;
use anyhow::Result;
use indexmap::IndexMap;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};
use ureq::Agent;

pub struct OperatorTable(pub(crate) IndexMap<Box<str>, Operator>);

impl OperatorTable {
    pub fn fetch(client: &Agent, server: Server) -> Result<Self> {
        let url = format!("{}/gamedata/excel/character_table.json", server.base_url());

        let data = client.get(&url).call()?.into_json()?;

        Ok(Self(data))
    }
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
    match Deserialize::deserialize(deserializer)? {
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
