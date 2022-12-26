#![warn(clippy::all, clippy::pedantic)]

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

impl Layout {
    fn new() -> Self {
        Self {
            cc: Facility {
                shifts: Vec::new(),
                level: 1,
            },
            tp: vec![Facility {
                shifts: Vec::new(),
                level: 1,
            }],
            fac: vec![Facility {
                shifts: Vec::new(),
                level: 1,
            }],
            pp: vec![Facility {
                shifts: Vec::new(),
                level: 1,
            }],
            workshop: Facility {
                shifts: Vec::new(),
                level: 1,
            },
            rr: Facility {
                shifts: Vec::new(),
                level: 0,
            },
            office: Facility {
                shifts: Vec::new(),
                level: 0,
            },
            dorm: vec![Facility {
                shifts: Vec::new(),
                level: 1,
            }],
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}
