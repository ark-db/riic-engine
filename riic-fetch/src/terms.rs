use crate::{Fetch, SaveJson};
use ahash::HashMap;
use serde::{de, Deserialize, Serialize};

#[derive(Deserialize)]
struct MiscGamedata<'a> {
    #[serde(borrow, rename = "richTextStyles")]
    styles: StyleTable<'a>,
    #[serde(rename = "termDescriptionDict")]
    terms: TermTable<'a>,
}

type StyleData<'a> = HashMap<&'a str, String>;

#[derive(Deserialize, Serialize)]
struct StyleTable<'a> {
    #[serde(flatten, borrow, deserialize_with = "deserialize_styles")]
    inner: StyleData<'a>,
}

fn deserialize_styles<'de, D>(deserializer: D) -> Result<StyleData<'de>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let data: StyleData<'de> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .filter(|(id, _)| id.starts_with("cc"))
        .filter_map(|(id, data)| {
            data.split_once('#')
                .map(|(_, s)| (id, format!("#{}", s.chars().take(6).collect::<String>())))
        })
        .collect())
}

type TermData<'a> = HashMap<&'a str, &'a str>;

#[derive(Deserialize, Serialize)]
struct TermTable<'a> {
    #[serde(flatten, borrow, deserialize_with = "deserialize_terms")]
    inner: TermData<'a>,
}

#[derive(Deserialize)]
struct UnprocessedTerm<'a> {
    description: &'a str,
}

fn deserialize_terms<'de, D>(deserializer: D) -> Result<TermData<'de>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let data: HashMap<&'de str, UnprocessedTerm<'de>> = Deserialize::deserialize(deserializer)?;

    Ok(data
        .into_iter()
        .map(|(id, entry)| (id, entry.description))
        .collect())
}

impl Fetch for MiscGamedata<'_> {
    const FETCH_PATH: &'static str = "gamedata/excel/gamedata_const.json";
}

impl SaveJson for StyleTable<'_> {}

impl SaveJson for TermTable<'_> {}
