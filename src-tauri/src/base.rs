use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Save {
    layout: Layout,
    chars: Vec<CharData>,
    drones: u32,
}

#[derive(Serialize, Deserialize)]
struct Layout {
    cc: Facility,
    tp: Vec<BoostFacility>,
    fac: Vec<BoostFacility>,
    pp: Vec<Facility>,
    workshop: Facility,
    rr: Facility,
    office: Facility,
    train: Facility,
    dorm: Vec<Facility>,
}

#[derive(Serialize, Deserialize)]
struct Facility {
    level: u8,
    shifts: Vec<Shift>,
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

#[derive(Serialize, Deserialize)]
struct BoostFacility {
    level: u8,
    shifts: Vec<Shift>,
    boosts: Vec<Boost>,
}

#[derive(Serialize, Deserialize)]
struct Boost {
    drones: u32,
    col: u64,
}

impl BoostFacility {
    fn new(level: u8) -> Self {
        Self {
            level,
            shifts: Vec::new(),
            boosts: Vec::new(),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            cc: Facility::new(1),
            tp: vec![BoostFacility::new(1)],
            fac: vec![BoostFacility::new(1)],
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
