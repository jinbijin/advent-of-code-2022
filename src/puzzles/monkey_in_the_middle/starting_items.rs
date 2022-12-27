use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

pub enum ParseStartingItemsError {
    InvalidFormat(String),
    InvalidItem(String),
}

impl Display for ParseStartingItemsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
            Self::InvalidItem(value) => write!(f, "invalid item '{}'", value),
        }
    }
}

impl Debug for ParseStartingItemsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseStartingItemsError {}

pub struct StartingItems(pub Vec<u64>);

impl FromStr for StartingItems {
    type Err = ParseStartingItemsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "  Starting items: ";
        if s.starts_with(prefix) {
            let items = s[(prefix.len())..]
                .split(", ")
                .map(|item| item.parse::<u64>())
                .collect::<Result<Vec<u64>, ParseIntError>>()
                .map_err(|_| Self::Err::InvalidItem(s.to_string()))?;
            Ok(StartingItems(items))
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
