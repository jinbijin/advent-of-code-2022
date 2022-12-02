pub mod common;
mod error;
mod mapping;

use crate::parse;
use common::{RpsMatchWithResult, RpsResult, RpsType, Scorable};
use error::RpsMatchParseError;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    iter::Sum,
    str::FromStr,
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

pub struct RpsMatch {
    pub opponent_choice: RpsType,
    pub own_choice: RpsType,
}

impl RpsMatchWithResult for RpsMatch {
    fn own_choice(&self) -> Option<RpsType> {
        Some(self.own_choice)
    }

    fn result(&self) -> Option<RpsResult> {
        let map = mapping::RPS_MATCH_RESULT_MAPPING.iter().find(|map| {
            map.opponent_choice == self.opponent_choice && map.own_choice == self.own_choice
        })?;
        Some(map.result)
    }
}

impl RpsMatchWithResult for RpsDesiredResult {
    fn own_choice(&self) -> Option<RpsType> {
        let map = mapping::RPS_MATCH_RESULT_MAPPING
            .iter()
            .find(|map| map.opponent_choice == self.opponent_choice && map.result == self.result)?;
        Some(map.own_choice)
    }

    fn result(&self) -> Option<RpsResult> {
        Some(self.result)
    }
}

impl FromStr for RpsMatch {
    type Err = RpsMatchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let opponent_choice = match chars.next() {
            Some('A') => Ok(RpsType::Rock),
            Some('B') => Ok(RpsType::Paper),
            Some('C') => Ok(RpsType::Scissors),
            other => Err(RpsMatchParseError::OpponentChoiceParseError(other)),
        }?;
        match chars.next() {
            Some(' ') => Ok(()),
            other => Err(RpsMatchParseError::SeparatorParseError(other)),
        }?;
        let own_choice = match chars.next() {
            Some('X') => Ok(RpsType::Rock),
            Some('Y') => Ok(RpsType::Paper),
            Some('Z') => Ok(RpsType::Scissors),
            other => Err(RpsMatchParseError::OpponentChoiceParseError(other)),
        }?;
        match chars.next() {
            None => Ok(()),
            _ => Err(RpsMatchParseError::EndOfLineParseError),
        }?;
        Ok(RpsMatch {
            opponent_choice,
            own_choice,
        })
    }
}

pub struct RpsDesiredResult {
    pub opponent_choice: RpsType,
    pub result: RpsResult,
}

impl FromStr for RpsDesiredResult {
    type Err = RpsMatchParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let opponent_choice = match chars.next() {
            Some('A') => Ok(RpsType::Rock),
            Some('B') => Ok(RpsType::Paper),
            Some('C') => Ok(RpsType::Scissors),
            other => Err(RpsMatchParseError::OpponentChoiceParseError(other)),
        }?;
        match chars.next() {
            Some(' ') => Ok(()),
            other => Err(RpsMatchParseError::SeparatorParseError(other)),
        }?;
        let result = match chars.next() {
            Some('X') => Ok(RpsResult::Loss),
            Some('Y') => Ok(RpsResult::Draw),
            Some('Z') => Ok(RpsResult::Win),
            other => Err(RpsMatchParseError::ResultParseError(other)),
        }?;
        match chars.next() {
            None => Ok(()),
            _ => Err(RpsMatchParseError::EndOfLineParseError),
        }?;
        Ok(RpsDesiredResult {
            opponent_choice,
            result,
        })
    }
}

pub fn get_tournament_score<T, U>(rps_matches: &mut impl Iterator<Item = U>) -> T
where
    T: Sum,
    U: Scorable<T>,
{
    rps_matches
        .into_iter()
        .map(|rps_match| rps_match.score())
        .sum::<T>()
}

pub fn main(
    file_contents: String,
    args: &RockPaperScissorsArgs,
) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let score = match args {
        RockPaperScissorsArgs::Regular => get_tournament_score(
            &mut parse::parse_as_newline_separated::<RpsMatch>(file_contents)?.into_iter(),
        ),
        RockPaperScissorsArgs::Reverse => get_tournament_score(
            &mut parse::parse_as_newline_separated::<RpsDesiredResult>(file_contents)?.into_iter(),
        ),
    };
    Ok(Box::new(score))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let rps_matches = vec![
            RpsMatch {
                opponent_choice: RpsType::Rock,
                own_choice: RpsType::Paper,
            },
            RpsMatch {
                opponent_choice: RpsType::Paper,
                own_choice: RpsType::Rock,
            },
            RpsMatch {
                opponent_choice: RpsType::Scissors,
                own_choice: RpsType::Scissors,
            },
        ];

        let score = get_tournament_score(&mut rps_matches.into_iter());

        assert_eq!(15, score);
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let rps_matches = vec![
            RpsDesiredResult {
                opponent_choice: RpsType::Rock,
                result: RpsResult::Draw,
            },
            RpsDesiredResult {
                opponent_choice: RpsType::Paper,
                result: RpsResult::Loss,
            },
            RpsDesiredResult {
                opponent_choice: RpsType::Scissors,
                result: RpsResult::Win,
            },
        ];

        let score = get_tournament_score(&mut rps_matches.into_iter());

        assert_eq!(12, score);
        Ok(())
    }
}
