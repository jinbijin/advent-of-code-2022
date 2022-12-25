use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[cfg(feature = "wasm")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct FormatError {
    pub format_description: String,
    pub actual: String,
}

impl Display for FormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}' is not in expected format '{}'",
            self.actual, self.format_description
        )
    }
}

impl Error for FormatError {}

pub trait FormatDescription {
    const FORMAT_DESCRIPTION: &'static str;
}

impl FormatError {
    pub fn from_actual<T>(actual: &str) -> FormatError
    where
        T: FormatDescription,
    {
        FormatError {
            format_description: T::FORMAT_DESCRIPTION.to_string(),
            actual: actual.to_string(),
        }
    }
}
