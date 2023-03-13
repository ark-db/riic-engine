use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Save {
    layout: Layout,
    chars: Vec<CharData>,
    drones: u32,    // Maximum number of drones available for use
    max_shift: u16, // Total number of shifts in rotation
    interval: u16,  // Length of one shift in minutes
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Layout {
    cc: Facility,
    tp: Vec<BoostFacility>,
    fac: Vec<BoostFacility>,
    pp: Vec<Facility>,
    workshop: NoShiftFacility,
    rr: Facility,
    office: Facility,
    train: NoShiftFacility,
    dorm: Vec<Facility>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Facility {
    level: u8,
    shifts: Vec<Shift>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Shift {
    char: Operator,
    start: u64,
    end: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
struct BoostFacility {
    level: u8,
    shifts: Vec<Shift>,
    boosts: Vec<Boost>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NoShiftFacility {
    level: u8,
}

impl NoShiftFacility {
    fn new(level: u8) -> Self {
        Self { level }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            cc: Facility::new(1),
            tp: vec![BoostFacility::new(1)],
            fac: vec![BoostFacility::new(1)],
            pp: vec![Facility::new(1)],
            workshop: NoShiftFacility::new(1),
            rr: Facility::new(0),
            office: Facility::new(0),
            train: NoShiftFacility::new(0),
            dorm: vec![
                Facility::new(1),
                Facility::new(0),
                Facility::new(0),
                Facility::new(0),
            ],
        }
    }
}

impl Default for Save {
    fn default() -> Self {
        Self {
            layout: Layout::default(),
            chars: Vec::new(),
            drones: 70,
            max_shift: 12,
            interval: 120,
        }
    }
}
