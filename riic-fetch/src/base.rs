use crate::consts::{FACILITY_COLORS, IGNORED_FACILITIES};
use crate::{Fetch, FetchImage, SaveJson};
use ahash::HashMap;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::borrow::Cow;

pub(crate) type CharSkills = HashMap<String, Vec<BaseSkill>>;

#[derive(Deserialize)]
pub(crate) struct BaseData {
    #[serde(deserialize_with = "deserialize_skills")]
    pub(crate) chars: CharSkills,
    pub(crate) rooms: FacilityTable,
    pub(crate) buffs: SkillTable,
}

#[derive(Deserialize)]
struct UnprocessedCharEntry<'a> {
    #[serde(borrow, rename = "buffChar")]
    inner: Vec<UnprocessedSkillSet<'a>>,
}

#[derive(Deserialize)]
struct UnprocessedSkillSet<'a> {
    #[serde(borrow, rename = "buffData")]
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
    #[serde(rename = "phase", deserialize_with = "deserialize_elite")]
    elite: u8,
    level: u8,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum EliteRepr<'a> {
    Number(u8),
    String(&'a str),
}

#[derive(Serialize)]
pub(crate) struct BaseSkill {
    id: String,
    elite: u8,
    level: u8,
}

fn deserialize_skills<'de, D>(deserializer: D) -> Result<CharSkills, D::Error>
where
    D: Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedCharEntry<'de>> =
        Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .map(|(char_id, data)| (char_id.to_string(), data.into()))
        .collect())
}

fn deserialize_elite<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let data = Deserialize::deserialize(deserializer)?;

    match data {
        EliteRepr::Number(num) => match num {
            n @ 0..=2 => Ok(n),
            _ => Err(Error::invalid_value(
                Unexpected::Unsigned(num.into()),
                &"Expected a u8 from 0 to 2",
            )),
        },
        EliteRepr::String(s) => match s {
            "PHASE_0" => Ok(0),
            "PHASE_1" => Ok(1),
            "PHASE_2" => Ok(2),
            _ => Err(Error::invalid_value(
                Unexpected::Str(s),
                &"Expected a string with pattern \"PHASE_{n}\", where n is a number from 0 to 2",
            )),
        },
    }
}

impl<'a> From<UnprocessedCharEntry<'a>> for Vec<BaseSkill> {
    fn from(value: UnprocessedCharEntry<'a>) -> Self {
        let mut skills = Vec::with_capacity(2);
        for set in value.inner {
            for skill in set.inner {
                skills.push(BaseSkill {
                    id: skill.id.to_string(),
                    elite: skill.cond.elite,
                    level: skill.cond.level,
                });
            }
        }
        skills
    }
}

type FacilityData = HashMap<String, Facility>;

#[derive(Deserialize, Serialize)]
pub(crate) struct FacilityTable {
    #[serde(flatten, deserialize_with = "deserialize_facilities")]
    inner: FacilityData,
}

#[derive(Deserialize)]
struct UnprocessedFacility<'a> {
    id: &'a str,
    name: &'a str,
    phases: Vec<UnprocessedFacilityPhase>,
}

#[derive(Deserialize)]
struct UnprocessedFacilityPhase {
    #[serde(rename = "electricity")]
    power: i16,
    #[serde(rename = "maxStationedNum")]
    capacity: u8,
}

#[derive(Deserialize, Serialize)]
struct Facility {
    name: String,
    color: String,
    power: Vec<i16>,
    capacity: Vec<u8>,
}

fn deserialize_facilities<'de, D>(deserializer: D) -> Result<FacilityData, D::Error>
where
    D: Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedFacility<'de>> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(_, data)| !IGNORED_FACILITIES.contains(&data.id.to_lowercase()))
        .map(|(id, data)| (id.to_lowercase(), data.into()))
        .collect())
}

impl<'a> From<UnprocessedFacility<'a>> for Facility {
    fn from(value: UnprocessedFacility<'a>) -> Self {
        let (mut power, mut capacity) = (Vec::with_capacity(5), Vec::with_capacity(5));
        for phase in value.phases {
            power.push(phase.power);
            capacity.push(phase.capacity);
        }

        let color = (*FACILITY_COLORS
            .get(&value.id.to_lowercase())
            .unwrap_or_else(|| {
                panic!("Facility '{}' did not have an associated color", &value.id)
            }))
        .to_string();

        Self {
            name: value.name.to_string(),
            color,
            power,
            capacity,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SkillTable {
    #[serde(flatten)]
    inner: HashMap<String, SkillInfo>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SkillInfo {
    #[serde(rename(deserialize = "buffName"))]
    name: String,
    #[serde(rename(deserialize = "description"))]
    desc: String,
    #[serde(rename(deserialize = "skillIcon"))]
    icon_id: String,
}

impl Fetch for BaseData {
    const FETCH_PATH: &'static str = "gamedata/excel/building_data.json";
}

impl SaveJson for FacilityTable {}

impl FetchImage for FacilityTable {
    const FETCH_DIR: &'static str = "arts/building/buffs";

    fn image_ids(&self) -> Vec<Cow<'_, str>> {
        self.inner
            .keys()
            .map(|k| Cow::Owned(k.to_lowercase()))
            .collect()
    }
}

impl SkillTable {
    pub(crate) fn extend(&mut self, other: Self) {
        self.inner.extend(other.inner);
    }
}

impl SaveJson for SkillTable {}

impl FetchImage for SkillTable {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/building/skills";

    fn image_ids(&self) -> Vec<Cow<'_, str>> {
        self.inner
            .values()
            .map(|v| Cow::Borrowed(v.icon_id.as_str()))
            .collect()
    }
}
