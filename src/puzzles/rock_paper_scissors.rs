pub mod common;
mod error;
mod mapping;

use common::{RpsMatchWithResult, RpsResult, RpsType, Scorable};
use error::RpsMatchParseError;
use std::{iter::Sum, str::FromStr};

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
