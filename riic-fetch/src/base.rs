use crate::Server;
use anyhow::Result;
use image::{
    codecs::webp::{WebPEncoder, WebPQuality},
    imageops::{resize, FilterType},
    load_from_memory_with_format, ColorType, ImageFormat,
};
use indexmap::IndexMap;
use rayon::iter::ParallelIterator;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use std::{cmp::min, fs::File, io::BufWriter, path::Path};
use ureq::Agent;

#[derive(Deserialize)]
pub struct BaseData {
    #[serde(rename(deserialize = "chars"))]
    pub ops: OperatorSkills,
    #[serde(rename(deserialize = "buffs"))]
    pub skills: SkillTable,
}

impl BaseData {
    pub fn fetch(client: &Agent, server: Server) -> Result<Self> {
        let url = format!("{}/gamedata/excel/building_data.json", server.base_url());

        let data = client.get(&url).call()?.into_json()?;

        Ok(data)
    }
}

#[derive(Deserialize)]
pub struct OperatorSkills(pub(crate) IndexMap<Box<str>, Operator>);

impl OperatorSkills {
    pub fn get_operator_icons<P: AsRef<Path> + Sync>(
        &self,
        client: &Agent,
        target_dir: P,
        min_size: u32,
        quality: u8,
    ) -> Result<()> {
        self.0
            .par_keys()
            .map(|id| get_operator_icon(client, id, target_dir.as_ref(), min_size, quality))
            .collect::<Result<()>>()
    }
}

fn get_operator_icon(
    client: &Agent,
    id: &str,
    target_dir: &Path,
    min_size: u32,
    quality: u8,
) -> Result<()> {
    let target_path = target_dir.join(id).with_extension("webp");

    if target_path.is_file() {
        return Ok(());
    }

    let url = format!(
        "https://raw.githubusercontent.com/astral4/arkdata/main/assets/torappu/dynamicassets/arts/charavatars/{id}.png"
    );

    let res = client.get(&url).call()?;

    let mut bytes = match res.header("Content-Length") {
        Some(len) => Vec::with_capacity(len.parse()?),
        None => Vec::new(),
    };

    res.into_reader().read_to_end(&mut bytes)?;

    let mut image = load_from_memory_with_format(&bytes, ImageFormat::Png)?.to_rgb8();

    let (width, height) = image.dimensions();
    let min_dim = min(width, height);

    if min_dim < min_size {
        let scale_factor = f64::from(min_size) / f64::from(min_dim);
        image = resize(
            &image,
            (scale_factor * f64::from(width)) as u32,
            (scale_factor * f64::from(height)) as u32,
            FilterType::Lanczos3,
        );
    }

    let file = File::create(target_path)?;

    WebPEncoder::new_with_quality(file, WebPQuality::lossy(quality)).encode(
        &image,
        image.width(),
        image.height(),
        ColorType::Rgba8,
    )?;

    Ok(())
}

#[derive(Deserialize)]
pub(crate) struct Operator {
    #[serde(rename(deserialize = "buffChar"))]
    pub(crate) inner: Box<[OperatorSkill]>,
}

#[derive(Deserialize)]
pub(crate) struct OperatorSkill {
    #[serde(rename(deserialize = "buffData"))]
    pub(crate) inner: Box<[SkillPhase]>,
}

#[derive(Deserialize)]
pub(crate) struct SkillPhase {
    #[serde(rename(deserialize = "buffId"))]
    pub(crate) id: Box<str>,
    pub(crate) cond: Level,
}

#[derive(Deserialize)]
pub(crate) struct Level {
    #[serde(rename(deserialize = "phase"), deserialize_with = "deserialize_elite")]
    pub(crate) elite: u8,
    pub(crate) level: u8,
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

#[derive(Deserialize)]
pub struct SkillTable(IndexMap<Box<str>, Skill>);

impl SkillTable {
    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn get_skill_icons<P: AsRef<Path> + Sync>(
        &self,
        client: &Agent,
        target_dir: P,
        min_size: u32,
        quality: u8,
    ) -> Result<()> {
        self.0
            .par_keys()
            .map(|id| get_skill_icon(client, id, target_dir.as_ref(), min_size, quality))
            .collect::<Result<()>>()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = BufWriter::new(File::create(path)?);

        serde_json::to_writer(file, &self.0)?;

        Ok(())
    }
}

fn get_skill_icon(
    client: &Agent,
    id: &str,
    target_dir: &Path,
    min_size: u32,
    quality: u8,
) -> Result<()> {
    let target_path = target_dir.join(id).with_extension("webp");

    if target_path.is_file() {
        return Ok(());
    }

    let url = format!(
        "https://raw.githubusercontent.com/astral4/arkdata/main/assets/torappu/dynamicassets/arts/building/skills/{id}.png"
    );

    let res = client.get(&url).call()?;

    let mut bytes = match res.header("Content-Length") {
        Some(len) => Vec::with_capacity(len.parse()?),
        None => Vec::new(),
    };

    res.into_reader().read_to_end(&mut bytes)?;

    let mut image = load_from_memory_with_format(&bytes, ImageFormat::Png)?.to_rgb8();

    let (width, height) = image.dimensions();
    let min_dim = min(width, height);

    if min_dim < min_size {
        let scale_factor = f64::from(min_size) / f64::from(min_dim);
        image = resize(
            &image,
            (scale_factor * f64::from(width)) as u32,
            (scale_factor * f64::from(height)) as u32,
            FilterType::Lanczos3,
        );
    }

    let file = File::create(target_path)?;

    WebPEncoder::new_with_quality(file, WebPQuality::lossy(quality)).encode(
        &image,
        image.width(),
        image.height(),
        ColorType::Rgba8,
    )?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct Skill {
    #[serde(rename(deserialize = "buffName"))]
    name: Box<str>,
    description: Box<str>,
    #[serde(rename(deserialize = "skillIcon", serialize = "iconId"))]
    icon_id: Box<str>,
}
