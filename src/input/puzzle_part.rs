use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParsePuzzlePartError {
    InvalidValue(String),
}

impl Display for ParsePuzzlePartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(f, "invalid part '{}'", value)
    }
}

impl Debug for ParsePuzzlePartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParsePuzzlePartError {}

#[derive(Clone, Copy)]
pub enum PuzzlePart {
    Part1,
    Part2,
}

impl FromStr for PuzzlePart {
    type Err = ParsePuzzlePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Part1),
            "2" => Ok(Self::Part2),
            _ => Err(Self::Err::InvalidValue(String::from(s))),
        }
    }
}
