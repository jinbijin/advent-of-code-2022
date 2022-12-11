use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseIfFalseThrowToError {
    InvalidFormat(String),
}

impl Display for ParseIfFalseThrowToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
        }
    }
}

impl Debug for ParseIfFalseThrowToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseIfFalseThrowToError {}

pub struct IfFalseThrowTo(pub String);

impl FromStr for IfFalseThrowTo {
    type Err = ParseIfFalseThrowToError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "    If false: throw to monkey ";
        if s.starts_with(prefix) {
            Ok(IfFalseThrowTo(s[(prefix.len())..].to_string()))
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
