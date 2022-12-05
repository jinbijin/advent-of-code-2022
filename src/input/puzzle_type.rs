use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::file::FileErrorCollection;

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
        }
    }

    pub fn solver(&self) -> impl FnOnce(PuzzleInput) -> Result<String, FileErrorCollection> {
        match self {
            Self::CalorieCounting => crate::puzzles::calorie_counting::calorie_counting,
            Self::RockPaperScissors => crate::puzzles::rock_paper_scissors::rock_paper_scissors,
            Self::RucksackReorganization => {
                crate::puzzles::rucksack_reorganization::rucksack_reorganization
            }
            Self::CampCleanup => crate::puzzles::camp_cleanup::camp_cleanup,
            Self::SupplyStacks => crate::puzzles::supply_stacks::supply_stacks,
        }
    }
}
