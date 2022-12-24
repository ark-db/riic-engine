#![warn(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
enum Operator {}

impl Save {
    #[must_use]
    pub fn new() -> Self {
        Self {
            layout: Layout::new(),
            chars: Vec::new(),
        }
    }
}

impl Default for Save {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout {
    fn new() -> Self {
        Self {
            cc: Facility {
                shifts: Vec::new(),
                level: 1,
            },
            tp: Vec::with_capacity(3),
            fac: Vec::with_capacity(5),
            pp: Vec::with_capacity(3),
            rr: Facility {
                shifts: Vec::new(),
                level: 0,
            },
            office: Facility {
                shifts: Vec::new(),
                level: 0,
            },
            dorm: Vec::with_capacity(5),
        }
    }
}
