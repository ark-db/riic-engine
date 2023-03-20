use crate::{Fetch, FetchImage, SaveJson};
use ahash::HashMap;
use phf::{phf_map, phf_set, Map, Set};
use serde::{de, Deserialize, Serialize};

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

#[derive(Serialize)]
pub(crate) struct BaseSkill {
    id: String,
    elite: u8,
    level: u8,
}

fn deserialize_skills<'de, D>(deserializer: D) -> Result<CharSkills, D::Error>
where
    D: de::Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedCharEntry<'de>> =
        Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .map(|(char_id, data)| (char_id.to_string(), data.into()))
        .collect())
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

const IGNORED_FACILITIES: Set<&'static str> = phf_set! {
    "elevator", "corridor"
};

const FACILITY_COLORS: Map<&'static str, &'static str> = phf_map! {
    "control" => "#005752",
    "dormitory" => "#21cdcb",
    "hire" => "#565656",
    "manufacture" => "#ffd800",
    "meeting" => "#dd653f",
    "power" => "#8fc31f",
    "trading" => "#0075a9",
    "training" => "#7d0022",
    "workshop" => "#e3eb00",
};

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
    D: de::Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedFacility<'de>> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(_, data)| !IGNORED_FACILITIES.contains(&data.id.to_lowercase()))
        .map(|(id, data)| (id.to_string(), data.into()))
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
            .get_key(&value.id.to_lowercase())
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

    fn image_ids(&self) -> Vec<String> {
        self.inner.keys().map(|k| (*k).to_string()).collect()
    }
}

impl SaveJson for SkillTable {}

impl FetchImage for SkillTable {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/building/skills";

    fn image_ids(&self) -> Vec<String> {
        self.inner.keys().map(|k| (*k).to_string()).collect()
    }
}
