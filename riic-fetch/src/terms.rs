use crate::{Fetch, HashMap, SaveJson};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize)]
pub(crate) struct MiscGamedata {
    #[serde(rename = "richTextStyles")]
    pub(crate) styles: StyleTable,
    #[serde(rename = "termDescriptionDict")]
    pub(crate) terms: TermTable,
}

type StyleData = HashMap<String, String>;

#[derive(Serialize)]
pub(crate) struct StyleTable(StyleData);

impl<'de> Deserialize<'de> for StyleTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: StyleData = Deserialize::deserialize(deserializer)?;

        Ok(Self(
            data.into_iter()
                .filter(|(id, _)| id.starts_with("cc"))
                .filter_map(|(id, data)| {
                    data.split_once('#')
                        .map(|(_, s)| (id, format!("#{}", s.chars().take(6).collect::<String>())))
                })
                .collect(),
        ))
    }
}

type TermData = HashMap<String, String>;

#[derive(Serialize)]
pub(crate) struct TermTable(TermData);

#[derive(Deserialize)]
struct UnprocessedTerm {
    description: String,
}

impl<'de> Deserialize<'de> for TermTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: HashMap<String, UnprocessedTerm> = Deserialize::deserialize(deserializer)?;

        Ok(Self(
            data.into_iter()
                .filter(|(id, _)| id.starts_with("cc"))
                .map(|(id, entry)| (id, entry.description))
                .collect(),
        ))
    }
}

impl Fetch for MiscGamedata {
    const FETCH_PATH: &'static str = "gamedata/excel/gamedata_const.json";
}

impl SaveJson for StyleTable {}

impl TermTable {
    pub(crate) fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl SaveJson for TermTable {}
