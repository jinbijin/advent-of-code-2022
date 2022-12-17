use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseMoveInstructionError {
    InvalidFormat,
    InvalidCount { input: String },
    InvalidFrom { input: String },
    InvalidTo { input: String },
}

impl Display for ParseMoveInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidCount { input } => write!(f, "invalid value '{}' for count", input),
            Self::InvalidFrom { input } => write!(f, "invalid value '{}' for from", input),
            Self::InvalidTo { input } => write!(f, "invalid value '{}' for to", input),
        }
    }
}

impl Debug for ParseMoveInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseMoveInstructionError {}

pub struct MoveInstruction {
    pub count: usize,
    pub from: char,
    pub to: char,
}

impl FromStr for MoveInstruction {
    type Err = ParseMoveInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() != 6 || parts[0] != "move" || parts[2] != "from" || parts[4] != "to" {
            return Err(Self::Err::InvalidFormat);
        }

        let count = parts[1]
            .parse::<usize>()
            .map_err(|_| Self::Err::InvalidCount {
                input: parts[1].to_string(),
            })?;
        let from = parts[3]
            .parse::<char>()
            .map_err(|_| Self::Err::InvalidFrom {
                input: parts[3].to_string(),
            })?;
        let to = parts[5].parse::<char>().map_err(|_| Self::Err::InvalidTo {
            input: parts[5].to_string(),
        })?;
        Ok(MoveInstruction { count, from, to })
    }
}
