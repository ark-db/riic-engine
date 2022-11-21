#![warn(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

const CURRENT_VERSION: SaveVersion = SaveVersion { major: 1, minor: 0 };

#[derive(Serialize, Deserialize)]
pub struct Save {
    version: SaveVersion,
    layout: Layout,
    chars: Vec<CharData>,
}

#[derive(Serialize, Deserialize)]
struct SaveVersion {
    major: usize,
    minor: usize,
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
    tier: u8
}

#[derive(Serialize, Deserialize)]
enum Operator {}

impl Save {
    #[must_use]
    pub fn new() -> Self {
        Self {
            version: CURRENT_VERSION,
            layout: Layout::new(),
            chars: Vec::new(),
        }
    }

    #[must_use]
    pub fn is_compatible(self) -> bool {
        self.version.major == CURRENT_VERSION.major
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
                level: 1,
            },
            office: Facility {
                shifts: Vec::new(),
                level: 1,
            },
            dorm: Vec::with_capacity(5),
        }
    }
}
