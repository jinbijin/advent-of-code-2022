use serde::Serialize;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Serialize)]
pub struct ParseNativeError(String);

impl Display for ParseNativeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A wrapper for parsing primitive types, in order to wrap the resulting built-in parsing errors.
pub struct Native<T>(T);

impl<T> FromStr for Native<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = ParseNativeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<T>() {
            Ok(value) => Ok(Native(value)),
            Err(error) => Err(ParseNativeError(error.to_string())),
        }
    }
}
