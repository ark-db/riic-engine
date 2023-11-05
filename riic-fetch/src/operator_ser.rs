use crate::base::OperatorSkills;
use crate::operator_de::{Operator as Op, OperatorTable as OpTable};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct OperatorTable<'a>(IndexMap<Box<str>, Operator<'a>>);

pub(crate) fn create_operator_table(ops: OpTable, skills: &OperatorSkills) -> OperatorTable<'_> {
    let table = ops
        .0
        .into_iter()
        .filter(|(_, op)| op.is_operator())
        .map(|(id, op)| transform_operator(id, op, skills))
        .collect();

    OperatorTable(table)
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
