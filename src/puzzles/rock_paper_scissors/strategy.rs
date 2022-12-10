use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::contents::tokens::AsTokens;

pub enum ParseRpsTypeError {
    InvalidValue(String),
}

impl Display for ParseRpsTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(value) => write!(f, "invalid RPS option '{}'", value),
        }
    }
}

impl Debug for ParseRpsTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseRpsTypeError {}

#[derive(PartialEq, Clone, Copy)]
pub enum RpsType {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RpsType {
    type Err = ParseRpsTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            other => Err(Self::Err::InvalidValue(other.to_string())),
        }
    }
}

pub enum ParseRpsTargetError {
    InvalidValue(String),
}

impl Display for ParseRpsTargetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(value) => write!(f, "invalid RPS target '{}'", value),
        }
    }
}

impl Debug for ParseRpsTargetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseRpsTargetError {}

#[derive(PartialEq, Clone, Copy)]
pub enum RpsTarget {
    X,
    Y,
    Z,
}

impl FromStr for RpsTarget {
    type Err = ParseRpsTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            other => Err(Self::Err::InvalidValue(other.to_string())),
        }
    }
}

impl From<RpsTarget> for RpsType {
    fn from(value: RpsTarget) -> Self {
        match value {
            RpsTarget::X => RpsType::Rock,
            RpsTarget::Y => RpsType::Paper,
            RpsTarget::Z => RpsType::Scissors,
        }
    }
}

pub enum ParseRpsStrategyError {
    Empty,
    MissingTarget,
    TooManyParts,
    InvalidOpponentChoice(String),
    InvalidTarget(String),
}

impl Display for ParseRpsStrategyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "unexpected empty strategy"),
            Self::MissingTarget => write!(f, "missing target"),
            Self::TooManyParts => write!(f, "unexpected data at end of line"),
            Self::InvalidOpponentChoice(value) => write!(f, "invalid opponent choice {}", value),
            Self::InvalidTarget(value) => write!(f, "invalid target '{}'", value),
        }
    }
}

impl Debug for ParseRpsStrategyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseRpsStrategyError {}

pub struct RpsStrategy {
    pub opponent_choice: RpsType,
    pub target: RpsTarget,
}

impl FromStr for RpsStrategy {
    type Err = ParseRpsStrategyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.tokens();

        let opponent_choice = match tokens.next() {
            Some(opponent_choice) => opponent_choice.parse::<RpsType>().map_err(
                |ParseRpsTypeError::InvalidValue(err)| {
                    ParseRpsStrategyError::InvalidOpponentChoice(err)
                },
            ),
            None => Err(ParseRpsStrategyError::Empty),
        }?;

        let target = match tokens.next() {
            Some(target) => {
                target
                    .parse::<RpsTarget>()
                    .map_err(|ParseRpsTargetError::InvalidValue(err)| {
                        ParseRpsStrategyError::InvalidTarget(err)
                    })
            }
            None => Err(ParseRpsStrategyError::MissingTarget),
        }?;

        match tokens.next() {
            Some(_) => Err(ParseRpsStrategyError::TooManyParts),
            None => Ok(RpsStrategy {
                opponent_choice,
                target,
            }),
        }
    }
}
