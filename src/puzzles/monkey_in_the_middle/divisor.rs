use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseDivisorError {
    InvalidFormat(String),
}

impl Display for ParseDivisorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
        }
    }
}

impl Debug for ParseDivisorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseDivisorError {}

pub struct Divisor(pub usize);

impl FromStr for Divisor {
    type Err = ParseDivisorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "  Test: divisible by ";
        if s.starts_with(prefix) {
            match s[(prefix.len())..].parse::<usize>() {
                Ok(value) => Ok(Divisor(value)),
                Err(_) => Err(Self::Err::InvalidFormat(s.to_string())),
            }
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
