use crate::base::{BaseSkill, CharSkills};
use crate::consts::NAME_OVERRIDES;
use crate::{Fetch, FetchImage, HashMap, SaveJson};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::borrow::Cow;

type OpTable = HashMap<String, OperatorData>;

pub(crate) struct OperatorTable(OpTable);

impl<'de> Deserialize<'de> for OperatorTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut table: OpTable = Deserialize::deserialize(deserializer)?;
        table.retain(|_, data| data.is_operator());

        for (id, new_name) in NAME_OVERRIDES.entries() {
            if let Some(entry) = table.get_mut(*id) {
                entry.name = (*new_name).to_string();
            }
        }

        Ok(Self(table))
    }
}

#[derive(Deserialize)]
struct OperatorData {
    #[serde(rename = "appellation")]
    name: String,
    #[serde(deserialize_with = "deserialize_rarity")]
    rarity: u8,
    #[serde(rename = "isNotObtainable")]
    unobtainable: bool,
    profession: Profession,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RarityRepr<'a> {
    Number(u8),
    String(&'a str),
}

fn deserialize_rarity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let data = Deserialize::deserialize(deserializer)?;

    match data {
        RarityRepr::Number(num) => match num {
            n @ 0..=5 => Ok(n + 1),
            _ => Err(Error::invalid_value(
                Unexpected::Unsigned(num.into()),
                &"Expected a u8 from 0 to 5",
            )),
        },
        RarityRepr::String(s) => match s {
            "TIER_1" => Ok(1),
            "TIER_2" => Ok(2),
            "TIER_3" => Ok(3),
            "TIER_4" => Ok(4),
            "TIER_5" => Ok(5),
            "TIER_6" => Ok(6),
            _ => Err(Error::invalid_value(
                Unexpected::Str(s),
                &"Expected a string with pattern \"TIER_{n}\", where n is a number from 1 to 6",
            )),
        },
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
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

impl OperatorData {
    fn is_operator(&self) -> bool {
        !self.unobtainable && !matches!(self.profession, Profession::Token | Profession::Trap)
    }
}

impl Fetch for OperatorTable {
    const FETCH_PATH: &'static str = "gamedata/excel/character_table.json";
}

#[derive(Serialize)]
pub(crate) struct UpdatedOperatorTable<'a>(HashMap<String, UpdatedOperatorData<'a>>);

#[derive(Serialize)]
struct UpdatedOperatorData<'a> {
    name: String,
    rarity: u8,
    skills: &'a Vec<BaseSkill>,
}

impl OperatorTable {
    pub(crate) fn into_updated(self, skill_table: &CharSkills) -> UpdatedOperatorTable<'_> {
        UpdatedOperatorTable(
            self.0
                .into_iter()
                .map(|(id, data)| {
                    (
                        id.clone(),
                        UpdatedOperatorData {
                            name: data.name,
                            rarity: data.rarity,
                            skills: skill_table
                                .get(&id)
                                .unwrap_or_else(|| panic!("Operator '{id}' had no base skills")),
                        },
                    )
                })
                .collect(),
        )
    }
}

impl SaveJson for UpdatedOperatorTable<'_> {}

impl FetchImage for UpdatedOperatorTable<'_> {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/charavatars";

    fn image_ids(&self) -> Vec<Cow<'_, str>> {
        self.0.keys().map(|k| Cow::Borrowed(k.as_str())).collect()
    }
}
