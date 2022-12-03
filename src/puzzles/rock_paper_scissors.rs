mod common;
mod error;
mod lib;
mod mapping;

use crate::file::{self, FileErrorCollection};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use self::{
    error::RpsMatchParseError,
    lib::{RpsDesiredResult, RpsMatch},
};

pub enum ParseRockPaperScissorsArgsError {
    InvalidValue(String),
}

impl Display for ParseRockPaperScissorsArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(
            f,
            "Invalid option '{}' for puzzle 'rock_paper_scissors'",
            value
        )
    }
}

impl Debug for ParseRockPaperScissorsArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParseRockPaperScissorsArgsError {}

pub enum RockPaperScissorsArgs {
    Regular,
    Reverse,
}

impl FromStr for RockPaperScissorsArgs {
    type Err = ParseRockPaperScissorsArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "regular" => Ok(Self::Regular),
            "reverse" => Ok(Self::Reverse),
            _ => Err(Self::Err::InvalidValue(s.to_string())),
        }
    }
}

pub fn main(
    file_contents: String,
    args: &RockPaperScissorsArgs,
) -> Result<Box<dyn Display>, FileErrorCollection<RpsMatchParseError>> {
    let score = match args {
        RockPaperScissorsArgs::Regular => lib::get_tournament_score(
            &mut file::parse_lines::<RpsMatch>(file_contents)?.into_iter(),
        ),
        RockPaperScissorsArgs::Reverse => lib::get_tournament_score(
            &mut file::parse_lines::<RpsDesiredResult>(file_contents)?.into_iter(),
        ),
    };
    Ok(Box::new(score))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT_TEXT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &RockPaperScissorsArgs::Regular)?;

        assert_eq!("15", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &RockPaperScissorsArgs::Reverse)?;

        assert_eq!("12", output.to_string());
        Ok(())
    }
}
