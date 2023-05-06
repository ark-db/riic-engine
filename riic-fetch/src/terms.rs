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

#[derive(Deserialize, Serialize)]
pub(crate) struct StyleTable {
    #[serde(flatten, deserialize_with = "deserialize_styles")]
    inner: StyleData,
}

fn deserialize_styles<'de, D>(deserializer: D) -> Result<StyleData, D::Error>
where
    D: Deserializer<'de>,
{
    let data: StyleData = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(id, _)| id.starts_with("cc"))
        .filter_map(|(id, data)| {
            data.split_once('#')
                .map(|(_, s)| (id, format!("#{}", s.chars().take(6).collect::<String>())))
        })
        .collect())
}

type TermData = HashMap<String, String>;

#[derive(Deserialize, Serialize)]
pub(crate) struct TermTable {
    #[serde(flatten, deserialize_with = "deserialize_terms")]
    inner: TermData,
}

#[derive(Deserialize)]
struct UnprocessedTerm {
    description: String,
}

fn deserialize_terms<'de, D>(deserializer: D) -> Result<TermData, D::Error>
where
    D: Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedTerm> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(id, _)| id.starts_with("cc"))
        .map(|(id, entry)| (id.to_string(), entry.description))
        .collect())
}

impl Fetch for MiscGamedata {
    const FETCH_PATH: &'static str = "gamedata/excel/gamedata_const.json";
}

impl SaveJson for StyleTable {}

impl TermTable {
    pub(crate) fn extend(&mut self, other: Self) {
        self.inner.extend(other.inner);
    }
}

impl SaveJson for TermTable {}
