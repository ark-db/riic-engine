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
    cc: ControlCenter,
    tp: Vec<Facility>,
    fac: Vec<Facility>,
    pp: Vec<Facility>,
    dorm: Vec<Facility>,
}

#[derive(Serialize, Deserialize)]
struct ControlCenter {
    shifts: Vec<Shift>,
    level: ControlCenterLevel,
}

#[derive(Serialize, Deserialize)]
struct Facility {
    shifts: Vec<Shift>,
    level: FacilityLevel,
}

#[derive(Serialize, Deserialize)]
enum ControlCenterLevel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Serialize, Deserialize)]
enum FacilityLevel {
    One = 1,
    Two = 2,
    Three = 3,
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
    elite: Elite,
}

#[derive(Serialize, Deserialize)]
enum Elite {
    Zero = 0,
    One = 1,
    Two = 2,
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
            cc: ControlCenter {
                shifts: Vec::new(),
                level: ControlCenterLevel::One,
            },
            tp: Vec::with_capacity(3),
            fac: Vec::with_capacity(5),
            pp: Vec::with_capacity(3),
            dorm: Vec::with_capacity(5),
        }
    }
}
