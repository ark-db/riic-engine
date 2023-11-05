use crate::base::OperatorSkills;
use crate::operator_de::{Operator as Op, OperatorTableDe};
use anyhow::Result;
use indexmap::IndexMap;
use serde::Serialize;
use std::{fs::File, io::BufWriter, path::Path};

pub struct OperatorTableSer<'a>(IndexMap<Box<str>, Operator<'a>>);

impl<'a> OperatorTableSer<'a> {
    pub fn create(ops: OperatorTableDe, skills: &'a OperatorSkills) -> Self {
        let table = ops
            .0
            .into_iter()
            .filter(|(_, op)| op.is_operator())
            .map(|(id, op)| transform_operator(id, op, skills))
            .collect();

        Self(table)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = BufWriter::new(File::create(path)?);

        serde_json::to_writer(file, &self.0)?;

        Ok(())
    }
}

fn transform_operator(id: Box<str>, op: Op, skills: &OperatorSkills) -> (Box<str>, Operator<'_>) {
    let op_skills = skills
        .get(&id)
        .unwrap_or_else(|| panic!("Operator '{}' had no base skills", &id));

    let operator = Operator {
        name: op.name,
        rarity: op.rarity,
        skills: op_skills
            .inner
            .into_iter()
            .map(|x| {
                x.inner
                    .into_iter()
                    .map(|phase| Skill {
                        id: &phase.id,
                        elite: phase.cond.elite,
                        level: phase.cond.level,
                    })
                    .collect()
            })
            .collect(),
    };

    (id, operator)
}

#[derive(Serialize)]
struct Operator<'a> {
    name: Box<str>,
    rarity: u8,
    skills: Box<[Box<[Skill<'a>]>]>,
}

#[derive(Serialize)]
struct Skill<'a> {
    id: &'a str,
    elite: u8,
    level: u8,
}
