use phf::{phf_map, phf_set, Map, Set};

pub(crate) const IGNORED_FACILITIES: Set<&'static str> = phf_set! {
    "elevator", "corridor"
};

pub(crate) const FACILITY_COLORS: Map<&'static str, &'static str> = phf_map! {
    "CONTROL" => "#005752",
    "DORMITORY" => "#21cdcb",
    "HIRE" => "#565656",
    "MANUFACTURE" => "#ffd800",
    "MEETING" => "#dd653f",
    "POWER" => "#8fc31f",
    "TRADING" => "#0075a9",
    "TRAINING" => "#7d0022",
    "WORKSHOP" => "#e3eb00",
};

pub(crate) const NAME_OVERRIDES: Map<&'static str, &'static str> = phf_map! {
    "char_118_yuki" => "Shirayuki",
    "char_196_sunbr" => "Gummy",
    "char_115_headbr" => "Zima",
    "char_195_glassb" => "Istina",
    "char_197_poca" => "Rosa",
    "char_1001_amiya2" => "Amiya (Guard)",
    "char_4055_bgsnow" => "Pozyomka",
    "char_4064_mlynar" => "Mlynar",
};
