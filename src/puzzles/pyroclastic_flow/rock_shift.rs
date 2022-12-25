use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::{common::direction::Direction, parse::error::ParseContentsError};

#[derive(Debug)]
pub enum ParseRockShiftError {
    InvalidInput { input: char },
}

impl Display for ParseRockShiftError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { input } => write!(f, "invalid rock shift '{}'", input),
        }
    }
}

impl Error for ParseRockShiftError {}

impl From<ParseRockShiftError> for ParseContentsError {
    fn from(value: ParseRockShiftError) -> Self {
        ParseContentsError::new(value)
    }
}

#[derive(Clone, Copy)]
pub enum RockShift {
    Left,
    Right,
}

impl TryFrom<char> for RockShift {
    type Error = ParseRockShiftError;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(Self::Error::InvalidInput { input }),
        }
    }
}

impl From<RockShift> for Direction {
    fn from(shift: RockShift) -> Self {
        match shift {
            RockShift::Left => Direction::Left,
            RockShift::Right => Direction::Right,
        }
    }
}

pub struct RockShiftCollection(pub Vec<RockShift>);

impl FromStr for RockShiftCollection {
    type Err = ParseRockShiftError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collection = s
            .chars()
            .map(|x| -> Result<RockShift, ParseRockShiftError> { x.try_into() })
            .collect::<Result<Vec<RockShift>, ParseRockShiftError>>()?;
        Ok(RockShiftCollection(collection))
    }
}
