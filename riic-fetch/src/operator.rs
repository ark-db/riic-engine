use crate::base::{BaseSkill, CharSkills};
use crate::{Fetch, FetchImage, SaveJson};
use ahash::HashMap;
use serde::{de, Deserialize, Serialize};

type OpTable<'a> = HashMap<&'a str, OperatorData<'a>>;

#[derive(Deserialize)]
struct OperatorTable<'a> {
    #[serde(flatten, borrow, deserialize_with = "deserialize_ops")]
    inner: OpTable<'a>,
}

fn deserialize_ops<'de, D>(deserializer: D) -> Result<OpTable<'de>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut table: OpTable<'de> = Deserialize::deserialize(deserializer)?;
    table.retain(|_, data| data.is_operator());
    Ok(table)
}

#[derive(Deserialize)]
struct OperatorData<'a> {
    #[serde(rename = "appellation")]
    name: &'a str,
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
    Vanguard,
    Guard,
    Defender,
    Sniper,
    Caster,
    Medic,
    Supporter,
    Specialist,
    Token,
    Trap,
}

impl OperatorData<'_> {
    fn is_operator(&self) -> bool {
        !self.unobtainable && !matches!(self.profession, Profession::Token | Profession::Trap)
    }
}

impl Fetch for OperatorTable<'_> {
    const FETCH_PATH: &'static str = "gamedata/excel/character_table.json";
}

#[derive(Serialize)]
struct UpdatedOperatorTable<'a> {
    #[serde(flatten)]
    inner: HashMap<&'a str, UpdatedOperatorData<'a>>,
}

#[derive(Serialize)]
struct UpdatedOperatorData<'a> {
    name: &'a str,
    rarity: u8,
    skills: &'a Vec<BaseSkill<'a>>,
}

impl<'a> OperatorTable<'a> {
    fn to_updated(self, skill_table: &'a CharSkills<'a>) -> UpdatedOperatorTable<'a> {
        let updated = self
            .inner
            .into_iter()
            .map(|(id, data)| {
                (
                    id,
                    UpdatedOperatorData {
                        name: data.name,
                        rarity: data.rarity,
                        skills: skill_table.get(id).unwrap_or_else(|| {
                            panic!("Operator '{}' had no base skills", data.name)
                        }),
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
