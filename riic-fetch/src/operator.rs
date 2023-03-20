use crate::base::{BaseSkill, CharSkills};
use crate::{Fetch, FetchImage, SaveJson};
use ahash::HashMap;
use phf::{phf_map, Map};
use serde::{de, Deserialize, Serialize};

type OpTable = HashMap<String, OperatorData>;

const NAME_OVERRIDES: Map<&'static str, &'static str> = phf_map! {
    "char_118_yuki" => "Shirayuki",
    "char_196_sunbr" => "Gummy",
    "char_115_headbr" => "Zima",
    "char_195_glassb" => "Istina",
    "char_197_poca" => "Rosa",
    "char_1001_amiya2" => "Amiya (Guard)",
    "char_4055_bgsnow" => "Pozyomka",
    "char_4064_mlynar" => "Mlynar",
};

#[derive(Deserialize)]
pub(crate) struct OperatorTable {
    #[serde(flatten, deserialize_with = "deserialize_ops")]
    inner: OpTable,
}

fn deserialize_ops<'de, D>(deserializer: D) -> Result<OpTable, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut table: OpTable = Deserialize::deserialize(deserializer)?;
    table.retain(|_, data| data.is_operator());

    for (id, new_name) in NAME_OVERRIDES.entries() {
        if let Some(entry) = table.get_mut(*id) {
            entry.name = (*new_name).to_string();
        }
    }

    Ok(table)
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

fn deserialize_rarity<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: de::Deserializer<'de>,
{
    let num: u8 = Deserialize::deserialize(deserializer)?;

    match num {
        n @ 0..=5 => Ok(n + 1),
        other => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(u64::from(other)),
            &"Expected a u8 from 0 to 5",
        )),
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
pub(crate) struct UpdatedOperatorTable<'a> {
    #[serde(flatten)]
    inner: HashMap<String, UpdatedOperatorData<'a>>,
}

#[derive(Serialize)]
struct UpdatedOperatorData<'a> {
    name: String,
    rarity: u8,
    skills: &'a Vec<BaseSkill>,
}

impl OperatorTable {
    pub(crate) fn into_updated(self, skill_table: &CharSkills) -> UpdatedOperatorTable<'_> {
        let updated = self
            .inner
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
            .collect();

        UpdatedOperatorTable { inner: updated }
    }
}

impl SaveJson for UpdatedOperatorTable<'_> {}

impl FetchImage for UpdatedOperatorTable<'_> {
    const FETCH_DIR: &'static str = "torappu/dynamicassets/arts/charavatars";

    fn image_ids(&self) -> Vec<String> {
        self.inner.keys().map(|k| (*k).to_string()).collect()
    }
}
