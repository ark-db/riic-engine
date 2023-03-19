use ahash::HashMap;
use serde::{Deserialize, Serialize};

const SKILL_URL: &str = "https://raw.githubusercontent.com/Kengxxiao/ArknightsGameData/master/zh_CN/gamedata/excel/building_data.json";

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
