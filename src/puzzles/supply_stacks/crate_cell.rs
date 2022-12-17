use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseCrateCellError {
    InvalidCell { input: String },
}

impl Display for ParseCrateCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCell { input } => write!(f, "invalid cell '{}'", input),
        }
    }
}

impl Debug for ParseCrateCellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseCrateCellError {}

pub enum CrateCell {
    Empty,
    Crate(char),
    Stack(char),
}

impl FromStr for CrateCell {
    type Err = ParseCrateCellError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() != 3 {
            return Err(Self::Err::InvalidCell {
                input: s.to_string(),
            });
        }

        if chars.iter().all(|c| c == &' ') {
            Ok(Self::Empty)
        } else if chars[0] == ' ' && chars[2] == ' ' {
            Ok(Self::Stack(chars[1]))
        } else if chars[0] == '[' && chars[2] == ']' {
            Ok(Self::Crate(chars[1]))
        } else {
            Err(Self::Err::InvalidCell {
                input: s.to_string(),
            })
        }
    }
}
