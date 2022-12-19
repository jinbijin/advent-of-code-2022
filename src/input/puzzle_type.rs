use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::contents::convert::contents::ParseContentsError;

use super::puzzle_input::PuzzleInput;

pub enum ParsePuzzleTypeError {
    InvalidValue(String),
}

impl Display for ParsePuzzleTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(f, "invalid puzzle type '{}'", value)
    }
}

impl Debug for ParsePuzzleTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParsePuzzleTypeError {}

#[derive(Clone, Copy)]
pub enum PuzzleType {
    CalorieCounting,
    RockPaperScissors,
    RucksackReorganization,
    CampCleanup,
    SupplyStacks,
    TuningTrouble,
    NoSpaceLeftOnDevice,
    TreetopTreeHouse,
    RopeBridge,
    CathodeRayTube,
    MonkeyInTheMiddle,
    HillClimbingAlgorithm,
    DistressSignal,
    RegolithReservoir,
    BeaconExclusionZone,
    ProboscideaVolcanium,
    PyroclasticFlow,
    BoilingBoulders,
    NotEnoughMinerals,
}

impl FromStr for PuzzleType {
    type Err = ParsePuzzleTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "calorie_counting" => Ok(Self::CalorieCounting),
            "rock_paper_scissors" => Ok(Self::RockPaperScissors),
            "rucksack_reorganization" => Ok(Self::RucksackReorganization),
            "camp_cleanup" => Ok(Self::CampCleanup),
            "supply_stacks" => Ok(Self::SupplyStacks),
            "tuning_trouble" => Ok(Self::TuningTrouble),
            "no_space_left_on_device" => Ok(Self::NoSpaceLeftOnDevice),
            "treetop_tree_house" => Ok(Self::TreetopTreeHouse),
            "rope_bridge" => Ok(Self::RopeBridge),
            "cathode_ray_tube" => Ok(Self::CathodeRayTube),
            "monkey_in_the_middle" => Ok(Self::MonkeyInTheMiddle),
            "hill_climbing_algorithm" => Ok(Self::HillClimbingAlgorithm),
            "distress_signal" => Ok(Self::DistressSignal),
            "regolith_reservoir" => Ok(Self::RegolithReservoir),
            "beacon_exclusion_zone" => Ok(Self::BeaconExclusionZone),
            "proboscidea_volcanium" => Ok(Self::ProboscideaVolcanium),
            "pyroclastic_flow" => Ok(Self::PyroclasticFlow),
            "boiling_boulders" => Ok(Self::BoilingBoulders),
            "not_enough_minerals" => Ok(Self::NotEnoughMinerals),
            _ => Err(Self::Err::InvalidValue(String::from(s))),
        }
    }
}

impl PuzzleType {
    pub fn file_name(&self) -> &str {
        match self {
            Self::CalorieCounting => "calorie_counting",
            Self::RockPaperScissors => "rock_paper_scissors",
            Self::RucksackReorganization => "rucksack_reorganization",
            Self::CampCleanup => "camp_cleanup",
            Self::SupplyStacks => "supply_stacks",
            Self::TuningTrouble => "tuning_trouble",
            Self::NoSpaceLeftOnDevice => "no_space_left_on_device",
            Self::TreetopTreeHouse => "treetop_tree_house",
            Self::RopeBridge => "rope_bridge",
            Self::CathodeRayTube => "cathode_ray_tube",
            Self::MonkeyInTheMiddle => "monkey_in_the_middle",
            Self::HillClimbingAlgorithm => "hill_climbing_algorithm",
            Self::DistressSignal => "distress_signal",
            Self::RegolithReservoir => "regolith_reservoir",
            Self::BeaconExclusionZone => "beacon_exclusion_zone",
            Self::ProboscideaVolcanium => "proboscidea_volcanium",
            Self::PyroclasticFlow => "pyroclastic_flow",
            Self::BoilingBoulders => "boiling_boulders",
            Self::NotEnoughMinerals => "not_enough_minerals",
        }
    }

    pub fn solver(&self) -> impl FnOnce(PuzzleInput) -> Result<String, ParseContentsError> {
        match self {
            Self::CalorieCounting => crate::puzzles::calorie_counting::calorie_counting,
            Self::RockPaperScissors => crate::puzzles::rock_paper_scissors::rock_paper_scissors,
            Self::RucksackReorganization => {
                crate::puzzles::rucksack_reorganization::rucksack_reorganization
            }
            Self::CampCleanup => crate::puzzles::camp_cleanup::camp_cleanup,
            Self::SupplyStacks => crate::puzzles::supply_stacks::supply_stacks,
            Self::TuningTrouble => crate::puzzles::tuning_trouble::tuning_trouble,
            Self::NoSpaceLeftOnDevice => {
                crate::puzzles::no_space_left_on_device::no_space_left_on_device
            }
            Self::TreetopTreeHouse => crate::puzzles::treetop_tree_house::treetop_tree_house,
            Self::RopeBridge => crate::puzzles::rope_bridge::rope_bridge,
            Self::CathodeRayTube => crate::puzzles::cathode_ray_tube::cathode_ray_tube,
            Self::MonkeyInTheMiddle => crate::puzzles::monkey_in_the_middle::monkey_in_the_middle,
            Self::HillClimbingAlgorithm => {
                crate::puzzles::hill_climbing_algorithm::hill_climbing_algorithm
            }
            Self::DistressSignal => crate::puzzles::distress_signal::distress_signal,
            Self::RegolithReservoir => crate::puzzles::regolith_reservoir::regolith_reservoir,
            Self::BeaconExclusionZone => {
                crate::puzzles::beacon_exclusion_zone::beacon_exclusion_zone
            }
            Self::ProboscideaVolcanium => {
                crate::puzzles::proboscidea_volcanium::proboscidea_volcanium
            }
            Self::PyroclasticFlow => crate::puzzles::pyroclastic_flow::pyroclastic_flow,
            Self::BoilingBoulders => crate::puzzles::boiling_boulders::boiling_boulders,
            Self::NotEnoughMinerals => crate::puzzles::not_enough_minerals::not_enough_minerals,
        }
    }
}
