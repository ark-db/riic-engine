use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Save {
    layout: Layout,
    chars: Vec<CharData>,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    cc: Facility,
    tp: Vec<Facility>,
    fac: Vec<Facility>,
    pp: Vec<Facility>,
    workshop: Facility,
    rr: Facility,
    office: Facility,
    train: Facility,
    dorm: Vec<Facility>,
}

#[derive(Serialize, Deserialize)]
struct Facility {
    shifts: Vec<Shift>,
    level: u8,
}

#[derive(Serialize, Deserialize)]
struct Shift {
    char: Operator,
    start: u64,
    end: u64,
}

#[derive(Serialize, Deserialize)]
struct CharData {
    char: Operator,
    tier: u8,
}

type Operator = String;

impl Facility {
    fn new(level: u8) -> Self {
        Self {
            shifts: Vec::new(),
            level,
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            cc: Facility::new(1),
            tp: vec![Facility::new(1)],
            fac: vec![Facility::new(1)],
            pp: vec![Facility::new(1)],
            workshop: Facility::new(1),
            rr: Facility::new(0),
            office: Facility::new(0),
            train: Facility::new(0),
            dorm: vec![
                Facility::new(1),
                Facility::new(0),
                Facility::new(0),
                Facility::new(0),
            ],
        }
    }
}
