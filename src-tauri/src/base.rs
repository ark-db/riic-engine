use crate::MAX_SAVE_SIZE;
use bincode::{
    config::{standard, Configuration, Limit, LittleEndian, Varint},
    decode_from_slice, encode_to_vec, Decode, Encode,
};
use once_cell::sync::Lazy;
use rusqlite::{
    types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef},
    Error as SqlError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Save {
    layout: Layout,
    chars: Box<[CharData]>,
    drones: DroneCount, // Drone capacity; drones will regenerate up to this amount
    max_shift: ShiftCount, // Total number of shifts in rotation
    interval: u16,      // Duration of one shift (in minutes)
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Layout {
    cc: Facility,
    tp: Box<[TradingPost]>,
    fac: Box<[Factory]>,
    pp: Box<[Facility]>,
    workshop: NoShiftFacility,
    rr: Facility,
    office: Facility,
    train: NoShiftFacility,
    dorm: [Facility; 4],
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct NoShiftFacility {
    level: FacilityLevel,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Facility {
    level: FacilityLevel,
    shifts: Box<[Shift]>,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct TradingPost {
    level: FacilityLevel,
    shifts: Box<[Shift]>,
    boosts: Box<[Boost]>,
    products: Box<[Product<TradingProduct>]>,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Factory {
    level: FacilityLevel,
    shifts: Box<[Shift]>,
    boosts: Box<[Boost]>,
    products: Box<[Product<FactoryProduct>]>,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Shift {
    char: Operator,
    col: ShiftCount,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Boost {
    drones: DroneCount,
    col: ShiftCount,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(deny_unknown_fields)]
struct Product<T> {
    kind: T,
    col: ShiftCount,
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "lowercase")]
enum TradingProduct {
    Lmd,     // consumes Pure Gold, produces LMD
    Orundum, // consumes Originium Shard, produces Orundum
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "lowercase")]
enum FactoryProduct {
    Exp200,  // Drill Battle Record
    Exp400,  // Frontline Battle Record
    Exp1000, // Tactical Battle Record
    Gold,    // Pure Gold
    Shard,   // Originium Shard
}

#[derive(Serialize, Deserialize, Encode, Decode)]
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
            shifts: Box::default(),
            level,
        }
    }
}

impl TradingPost {
    fn new(level: u8) -> Self {
        Self {
            level,
            shifts: Box::default(),
            boosts: Box::default(),
            products: Box::default(),
        }
    }
}

impl Factory {
    fn new(level: u8) -> Self {
        Self {
            level,
            shifts: Box::default(),
            boosts: Box::default(),
            products: Box::default(),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            cc: Facility::new(1),
            tp: Box::new([TradingPost::new(1)]),
            fac: Box::new([Factory::new(1)]),
            pp: Box::new([Facility::new(1)]),
            workshop: NoShiftFacility::new(1),
            rr: Facility::new(0),
            office: Facility::new(0),
            train: NoShiftFacility::new(0),
            dorm: [
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
            chars: Box::default(),
            drones: 70,
            max_shift: 12,
            interval: 120,
        }
    }
}

type BincodeConfig = Configuration<LittleEndian, Varint, Limit<MAX_SAVE_SIZE>>;

static BINCODE_CONFIG: Lazy<BincodeConfig> = Lazy::new(|| standard().with_limit::<MAX_SAVE_SIZE>());

impl ToSql for Save {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, SqlError> {
        let data = encode_to_vec(self, *BINCODE_CONFIG)
            .map_err(|e| SqlError::ToSqlConversionFailure(e.into()))?;

        Ok(ToSqlOutput::Owned(Value::Blob(data)))
    }
}

impl FromSql for Save {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        decode_from_slice(value.as_blob()?, *BINCODE_CONFIG)
            .map_err(|e| FromSqlError::Other(e.into()))
            .map(|data| data.0)
    }
}
