use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "wasm")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct ParseByCharError<TError> {
    pub column: usize,
    pub error: TError,
}

impl<TError> Display for ParseByCharError<TError>
where
    TError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "column {}: {}", self.column, self.error)
    }
}

impl<TError> Error for ParseByCharError<TError> where TError: Error {}

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct ParseByCharsError<TError> {
    pub char_errors: Vec<ParseByCharError<TError>>,
}

impl<TError> Display for ParseByCharsError<TError>
where
    TError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for error in self.char_errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl<TError> Error for ParseByCharsError<TError> where TError: Error {}

pub struct ByChars<T>(pub Vec<T>);

impl<T> FromStr for ByChars<T>
where
    T: TryFrom<char>,
{
    type Err = ParseByCharsError<T::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_errors: Vec<ParseByCharError<T::Error>> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let char_results = s.chars().map(|c| TryInto::<T>::try_into(c)).enumerate();
        for (index, char_result) in char_results {
            match char_result {
                Ok(result) => results.push(result),
                Err(error) => char_errors.push(ParseByCharError {
                    column: index,
                    error,
                }),
            }
        }
        if char_errors.len() > 0 {
            Err(ParseByCharsError { char_errors })
        } else {
            Ok(ByChars(results))
        }
    }
}
