use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Save {
    layout: Layout,
    chars: Vec<CharData>,
    drones: DroneCount, // Drone capacity; drones will regenerate up to this amount
    max_shift: ShiftCount, // Total number of shifts in rotation
    interval: u16,      // Duration of one shift (in minutes)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Layout {
    cc: Facility,
    tp: Vec<BoostFacility<TradingProduct>>,
    fac: Vec<BoostFacility<FactoryProduct>>,
    pp: Vec<Facility>,
    workshop: NoShiftFacility,
    rr: Facility,
    office: Facility,
    train: NoShiftFacility,
    dorm: Vec<Facility>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NoShiftFacility {
    level: FacilityLevel,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Facility {
    level: FacilityLevel,
    shifts: Vec<Shift>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BoostFacility<P> {
    level: FacilityLevel,
    shifts: Vec<Shift>,
    boosts: Vec<Boost>,
    products: Vec<Product<P>>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Shift {
    char: Operator,
    start: ShiftCount,
    end: ShiftCount,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Boost {
    drones: DroneCount,
    col: ShiftCount,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Product<T> {
    kind: T,
    end: ShiftCount,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TradingProduct {
    Lmd,     // consumes Pure Gold, produces LMD
    Orundum, // consumes Originium Shard, produces Orundum
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum FactoryProduct {
    Exp200,  // Drill Battle Record
    Exp400,  // Frontline Battle Record
    Exp1000, // Tactical Battle Record
    Gold,    // Pure Gold
    Shard,   // Originium Shard
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CharData {
    char: Operator,
    tier: u8,
}

type DroneCount = u32;

type ShiftCount = u16;

type FacilityLevel = u8;

type Operator = String;

impl NoShiftFacility {
    fn new(level: u8) -> Self {
        Self { level }
    }
}

impl Facility {
    fn new(level: u8) -> Self {
        Self {
            shifts: Vec::new(),
            level,
        }
    }
}

impl<P> BoostFacility<P> {
    fn new(level: u8) -> Self {
        Self {
            level,
            shifts: Vec::new(),
            boosts: Vec::new(),
            products: Vec::new(),
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
