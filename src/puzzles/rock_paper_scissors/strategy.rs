use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::validation::{
    format::{FormatDescription, FormatError},
    true_lit::True,
};

#[cfg(feature = "wasm")]
use serde::Serialize;

#[derive(Debug)]
pub enum ParseRpsTypeError {
    Format { format: FormatError },
}

impl Display for ParseRpsTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format { format } => write!(f, "{}", format),
        }
    }
}

impl Error for ParseRpsTypeError {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RpsType {
    Rock,
    Paper,
    Scissors,
}

impl FormatDescription for RpsType {
    const FORMAT_DESCRIPTION: &'static str = "A|B|C";
}

impl FromStr for RpsType {
    type Err = ParseRpsTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(Self::Err::Format {
                format: FormatError::from_actual::<RpsType>(s),
            }),
        }
    }
}

#[derive(Debug)]
pub enum ParseRpsTargetError {
    Format { format: FormatError },
}

impl Display for ParseRpsTargetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format { format } => write!(f, "{}", format),
        }
    }
}

impl Error for ParseRpsTargetError {}

#[derive(PartialEq, Clone, Copy)]
pub enum RpsTarget {
    X,
    Y,
    Z,
}

impl FormatDescription for RpsTarget {
    const FORMAT_DESCRIPTION: &'static str = "X|Y|Z";
}

impl FromStr for RpsTarget {
    type Err = ParseRpsTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err(Self::Err::Format {
                format: FormatError::from_actual::<RpsTarget>(s),
            }),
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

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
#[cfg_attr(feature = "wasm", serde(untagged))]
pub enum ParseRpsStrategyError {
    Empty {
        required: True,
    },
    Format {
        format: FormatError,
    },
    OpponentChoice {
        opponent_choice: FormatError,
    },
    Target {
        target: FormatError,
    },
    Both {
        opponent_choice: FormatError,
        target: FormatError,
    },
}

impl Display for ParseRpsStrategyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty { required: _ } => write!(f, "unexpected empty strategy"),
            Self::Format { format } => write!(f, "{}", format),
            Self::OpponentChoice { opponent_choice } => {
                write!(f, "opponent choice invalid: {}", opponent_choice)
            }
            Self::Target { target } => write!(f, "target invalid: {}", target),
            Self::Both {
                opponent_choice,
                target,
            } => {
                writeln!(f, "opponent choice invalid: {}", opponent_choice)?;
                writeln!(f, "target invalid: {}", target)
            }
        }
    }
}

impl Error for ParseRpsStrategyError {}

impl From<ParseRpsTypeError> for ParseRpsStrategyError {
    fn from(value: ParseRpsTypeError) -> Self {
        match value {
            ParseRpsTypeError::Format {
                format: opponent_choice,
            } => Self::OpponentChoice { opponent_choice },
        }
    }
}

impl From<ParseRpsTargetError> for ParseRpsStrategyError {
    fn from(value: ParseRpsTargetError) -> Self {
        match value {
            ParseRpsTargetError::Format { format: target } => Self::Target { target },
        }
    }
}

impl From<(ParseRpsTypeError, ParseRpsTargetError)> for ParseRpsStrategyError {
    fn from(value: (ParseRpsTypeError, ParseRpsTargetError)) -> Self {
        match value {
            (
                ParseRpsTypeError::Format {
                    format: opponent_choice,
                },
                ParseRpsTargetError::Format { format: target },
            ) => Self::Both {
                opponent_choice,
                target,
            },
        }
    }
}

pub struct RpsStrategy {
    pub opponent_choice: RpsType,
    pub target: RpsTarget,
}

impl FormatDescription for RpsStrategy {
    const FORMAT_DESCRIPTION: &'static str = "<opponent choice> <target>";
}

impl FromStr for RpsStrategy {
    type Err = ParseRpsStrategyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let opponent_choice = match tokens.next() {
            Some(opponent_choice) => opponent_choice.parse::<RpsType>(),
            None => {
                return Err(ParseRpsStrategyError::Empty { required: True });
            }
        };

        let target = match tokens.next() {
            Some(target) => target.parse::<RpsTarget>(),
            None => {
                return Err(Self::Err::Format {
                    format: FormatError::from_actual::<RpsStrategy>(s),
                });
            }
        };

        match tokens.next() {
            Some(_) => Err(Self::Err::Format {
                format: FormatError::from_actual::<RpsStrategy>(s),
            }),
            None => match (opponent_choice, target) {
                (Ok(opponent_choice), Ok(target)) => Ok(RpsStrategy {
                    opponent_choice,
                    target,
                }),
                (Ok(_), Err(target)) => Err(target.into()),
                (Err(opponent_choice), Ok(_)) => Err(opponent_choice.into()),
                (Err(opponent_choice), Err(target)) => Err((opponent_choice, target).into()),
            },
        }
    }
}
