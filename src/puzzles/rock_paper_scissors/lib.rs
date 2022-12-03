use std::iter::Sum;
use std::str::FromStr;

use super::common::{RpsMatchWithResult, RpsResult, RpsType, Scorable};
use super::error::RpsMatchParseError;
use super::mapping;

pub struct RpsMatch {
    pub opponent_choice: RpsType,
    pub own_choice: RpsType,
}

impl RpsMatchWithResult for RpsMatch {
    fn own_choice(&self) -> RpsType {
        self.own_choice
    }

    fn result(&self) -> RpsResult {
        if let Some(map) = mapping::RPS_MATCH_RESULT_MAPPING.iter().find(|map| {
            map.opponent_choice == self.opponent_choice && map.own_choice == self.own_choice
        }) {
            map.result
        } else {
            unreachable!("Cannot be reached due to mapping definition.")
        }
    }
}

impl RpsMatchWithResult for RpsDesiredResult {
    fn own_choice(&self) -> RpsType {
        if let Some(map) = mapping::RPS_MATCH_RESULT_MAPPING
            .iter()
            .find(|map| map.opponent_choice == self.opponent_choice && map.result == self.result)
        {
            map.own_choice
        } else {
            unreachable!("Cannot be reached due to mapping definition.")
        }
    }

    fn result(&self) -> RpsResult {
        self.result
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
            other => Err(RpsMatchParseError::OwnChoiceParseError(other)),
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
