use crate::Fetch;
use ahash::HashMap;
use serde::{de, Deserialize, Serialize};

type CharSkills<'a> = HashMap<&'a str, Vec<BaseSkill<'a>>>;

#[derive(Deserialize)]
struct BaseData<'a> {
    #[serde(deserialize_with = "deserialize_skills")]
    chars: CharSkills<'a>,
    rooms: u8, // TODO
    #[serde(borrow)]
    buffs: SkillTable<'a>,
}

#[derive(Deserialize)]
struct UnprocessedCharEntry<'a> {
    #[serde(borrow, rename = "buffChar")]
    inner: Vec<UnprocessedSkillSet<'a>>,
}

#[derive(Deserialize)]
struct UnprocessedSkillSet<'a> {
    #[serde(borrow, rename = "buffChar")]
    inner: Vec<UnprocessedSkill<'a>>,
}

#[derive(Deserialize)]
struct UnprocessedSkill<'a> {
    #[serde(rename = "buffId")]
    id: &'a str,
    cond: UnprocessedSkillPhase,
}

#[derive(Deserialize)]
struct UnprocessedSkillPhase {
    #[serde(rename = "phase")]
    elite: u8,
    level: u8,
}

struct BaseSkill<'a> {
    id: &'a str,
    elite: u8,
    level: u8,
}

fn deserialize_skills<'de, D>(deserializer: D) -> Result<CharSkills<'de>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedCharEntry<'de>> =
        Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .map(|(char_id, data)| (char_id, data.into()))
        .collect())
}

impl<'a> From<UnprocessedCharEntry<'a>> for Vec<BaseSkill<'a>> {
    fn from(value: UnprocessedCharEntry<'a>) -> Self {
        let mut skills = Vec::with_capacity(2);
        for set in value.inner {
            for skill in set.inner {
                skills.push(BaseSkill {
                    id: skill.id,
                    elite: skill.cond.elite,
                    level: skill.cond.level,
                });
            }
        }
        skills
    }
}

#[derive(Deserialize, Serialize)]
struct SkillTable<'a> {
    #[serde(flatten, borrow)]
    inner: HashMap<&'a str, SkillInfo<'a>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SkillInfo<'a> {
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
