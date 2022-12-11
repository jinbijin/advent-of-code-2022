use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseMonkeyNameError {
    InvalidFormat(String),
}

impl Display for ParseMonkeyNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
        }
    }
}

impl Debug for ParseMonkeyNameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseMonkeyNameError {}

pub struct MonkeyName(pub String);

impl FromStr for MonkeyName {
    type Err = ParseMonkeyNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "Monkey ";
        let suffix = ":";
        if s.starts_with(prefix) && s.ends_with(suffix) && s.len() > prefix.len() + suffix.len() {
            Ok(MonkeyName(
                s[(prefix.len())..(s.len() - suffix.len())].to_string(),
            ))
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
