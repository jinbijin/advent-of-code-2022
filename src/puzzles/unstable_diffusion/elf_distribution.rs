use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{
    common::position::Position,
    contents::convert::sections::{CustomSectionError, FromLines},
};

#[derive(Debug, Clone, Copy)]
pub enum ParseElfDistributionError {
    InvalidTile,
}

impl Display for ParseElfDistributionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTile => write!(f, "invalid tile"),
        }
    }
}

impl Error for ParseElfDistributionError {}

impl CustomSectionError for ParseElfDistributionError {}

pub struct ElfDistribution(pub HashSet<Position<isize>>);

impl FromLines for ElfDistribution {
    type Err = ParseElfDistributionError;

    fn from_lines(s: &str) -> Result<Self, Self::Err> {
        let elves = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some(Ok(Position {
                        x: x as isize,
                        y: y as isize,
                    })),
                    _ => Some(Err(ParseElfDistributionError::InvalidTile)),
                })
            })
            .collect::<Result<HashSet<Position<isize>>, ParseElfDistributionError>>()?;
        Ok(ElfDistribution(elves))
    }
}
