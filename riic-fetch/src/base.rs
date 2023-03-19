use crate::Fetch;
use ahash::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct BaseData<'a> {
    chars: u8, // TODO
    rooms: u8, // TODO
    #[serde(borrow)]
    buffs: SkillTable<'a>,
}

#[derive(Deserialize, Serialize)]
struct SkillTable<'a> {
    #[serde(flatten, borrow)]
    inner: HashMap<&'a str, SkillData<'a>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SkillData<'a> {
    #[serde(rename(deserialize = "buffName"))]
    name: &'a str,
    #[serde(rename(deserialize = "description"))]
    desc: &'a str,
    #[serde(rename(deserialize = "skillIcon"))]
    icon_id: &'a str,
}

impl Fetch for BaseData<'_> {
    const FETCH_PATH: &'static str = "gamedata/excel/building_data.json";
}
