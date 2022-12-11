use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseIfTrueThrowToError {
    InvalidFormat(String),
}

impl Display for ParseIfTrueThrowToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
        }
    }
}

impl Debug for ParseIfTrueThrowToError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseIfTrueThrowToError {}

pub struct IfTrueThrowTo(pub String);

impl FromStr for IfTrueThrowTo {
    type Err = ParseIfTrueThrowToError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "    If true: throw to monkey ";
        if s.starts_with(prefix) {
            Ok(IfTrueThrowTo(s[(prefix.len())..].to_string()))
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
