use crate::{Fetch, SaveJson};
use ahash::HashMap;
use phf::{phf_map, phf_set, Map, Set};
use serde::{de, Deserialize, Serialize};

pub(crate) type CharSkills<'a> = HashMap<&'a str, Vec<BaseSkill<'a>>>;

#[derive(Deserialize)]
struct BaseData<'a> {
    #[serde(deserialize_with = "deserialize_skills")]
    chars: CharSkills<'a>,
    #[serde(borrow)]
    rooms: FacilityTable<'a>,
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

#[derive(Serialize)]
pub(crate) struct BaseSkill<'a> {
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

type FacilityData<'a> = HashMap<&'a str, Facility<'a>>;

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
struct FacilityTable<'a> {
    #[serde(flatten, borrow, deserialize_with = "deserialize_facilities")]
    inner: FacilityData<'a>,
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
struct Facility<'a> {
    name: &'a str,
    color: &'a str,
    power: Vec<i16>,
    capacity: Vec<u8>,
}

fn deserialize_facilities<'de, D>(deserializer: D) -> Result<FacilityData<'de>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedFacility<'de>> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(_, data)| !IGNORED_FACILITIES.contains(&data.id.to_lowercase()))
        .map(|(id, data)| (id, data.into()))
        .collect())
}

impl<'a> From<UnprocessedFacility<'a>> for Facility<'a> {
    fn from(value: UnprocessedFacility<'a>) -> Self {
        let (mut power, mut capacity) = (Vec::with_capacity(5), Vec::with_capacity(5));
        for phase in value.phases {
            power.push(phase.power);
            capacity.push(phase.capacity);
        }

        let color = FACILITY_COLORS
            .get_key(&value.id.to_lowercase())
            .unwrap_or_else(|| panic!("Facility '{}' did not have an associated color", &value.id));

        Self {
            name: value.name,
            color,
            power,
            capacity,
        }
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

impl SaveJson for FacilityTable<'_> {}

impl SaveJson for SkillTable<'_> {}
