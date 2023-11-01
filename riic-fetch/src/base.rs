use crate::consts::{FACILITY_COLORS, IGNORED_FACILITIES};
use crate::{Fetch, FetchImage, HashMap};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::borrow::Cow;

pub(crate) type CharSkills = HashMap<Box<str>, Box<[Box<[BaseSkill]>]>>;

#[derive(Deserialize)]
pub(crate) struct BaseData {
    #[serde(deserialize_with = "deserialize_skills")]
    pub(crate) chars: CharSkills,
    pub(crate) rooms: FacilityTable,
    pub(crate) buffs: SkillTable,
}

#[derive(Deserialize)]
struct UnprocessedCharEntry {
    #[serde(rename = "buffChar")]
    inner: Box<[UnprocessedSkillSet]>,
}

#[derive(Deserialize)]
struct UnprocessedSkillSet {
    #[serde(rename = "buffData")]
    inner: Box<[UnprocessedSkill]>,
}

#[derive(Deserialize)]
struct UnprocessedSkill {
    #[serde(rename = "buffId")]
    id: Box<str>,
    cond: UnprocessedSkillPhase,
}

#[derive(Deserialize)]
struct UnprocessedSkillPhase {
    #[serde(rename = "phase", deserialize_with = "deserialize_elite")]
    elite: u8,
    level: u8,
}

#[derive(Serialize)]
pub(crate) struct BaseSkill {
    id: Box<str>,
    elite: u8,
    level: u8,
}

fn deserialize_skills<'de, D>(deserializer: D) -> Result<CharSkills, D::Error>
where
    D: Deserializer<'de>,
{
    let data: HashMap<Box<str>, UnprocessedCharEntry> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .map(|(char_id, data)| (char_id, data.into()))
        .collect())
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

impl From<UnprocessedCharEntry> for Box<[Box<[BaseSkill]>]> {
    fn from(value: UnprocessedCharEntry) -> Self {
        value
            .inner
            .iter()
            .filter(|set| set.inner.len() != 0)
            .map(|set| {
                set.inner
                    .iter()
                    .map(|skill| BaseSkill {
                        id: skill.id.clone(),
                        elite: skill.cond.elite,
                        level: skill.cond.level,
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Serialize)]
pub(crate) struct FacilityTable(HashMap<Box<str>, Facility>);

#[derive(Deserialize)]
struct UnprocessedFacility<'a> {
    id: &'a str,
    name: &'a str,
    phases: Box<[UnprocessedFacilityPhase]>,
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
    name: Box<str>,
    color: Box<str>,
    power: Box<[i16]>,
    capacity: Box<[u8]>,
}

impl<'de> Deserialize<'de> for FacilityTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: HashMap<&'de str, UnprocessedFacility<'de>> =
            Deserialize::deserialize(deserializer)?;

        Ok(Self(
            data.into_iter()
                .filter(|(_, data)| !IGNORED_FACILITIES.contains(&data.id.to_lowercase()))
                .map(|(id, data)| (id.to_lowercase().into(), data.into()))
                .collect(),
        ))
    }
}

impl<'a> From<UnprocessedFacility<'a>> for Facility {
    fn from(value: UnprocessedFacility<'a>) -> Self {
        let (mut power, mut capacity) = (Vec::with_capacity(5), Vec::with_capacity(5));
        value.phases.iter().for_each(|phase| {
            power.push(phase.power);
            capacity.push(phase.capacity);
        });

        let color = *FACILITY_COLORS
            .get(&value.id.to_lowercase())
            .unwrap_or_else(|| panic!("Facility '{}' did not have an associated color", value.id));

        Self {
            name: value.name.into(),
            color: color.into(),
            power: power.into_boxed_slice(),
            capacity: capacity.into_boxed_slice(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SkillTable(HashMap<Box<str>, SkillInfo>);

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SkillInfo {
    #[serde(rename(deserialize = "buffName"))]
    name: Box<str>,
    #[serde(rename(deserialize = "description"))]
    desc: Box<str>,
    #[serde(rename(deserialize = "skillIcon"))]
    icon_id: Box<str>,
}

impl Fetch for BaseData {
    const FETCH_PATH: &'static str = "gamedata/excel/building_data.json";
}

impl FetchImage for FacilityTable {
    const FETCH_DIR: &'static str = "arts/building/buffs";

    fn image_ids(&self) -> Box<[Cow<'_, str>]> {
        self.0
            .keys()
            .map(|k| Cow::Owned(k.to_lowercase()))
            .collect()
    }
}

impl SkillTable {
    pub(crate) fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl FetchImage for SkillTable {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/building/skills";

    fn image_ids(&self) -> Box<[Cow<'_, str>]> {
        self.0
            .values()
            .map(|v| Cow::Borrowed(&*v.icon_id))
            .collect()
    }
}
