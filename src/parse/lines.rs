use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "wasm")]
use serde::Serialize;

use super::error::ParseContentsError;

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct ParseByLineError<TError> {
    line: usize,
    error: TError,
}

impl<TError> Display for ParseByLineError<TError>
where
    TError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.error)
    }
}

impl<TError> Error for ParseByLineError<TError> where TError: Error {}

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct ParseByLinesError<TError> {
    pub line_errors: Vec<ParseByLineError<TError>>,
}

impl<TError> Display for ParseByLinesError<TError>
where
    TError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for error in self.line_errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl<TError> Error for ParseByLinesError<TError> where TError: Error {}

impl<TError> From<ParseByLinesError<TError>> for ParseContentsError
where
    TError: Error,
{
    fn from(value: ParseByLinesError<TError>) -> Self {
        ParseContentsError::new(value)
    }
}

pub struct ByLines<T>(pub Vec<T>);

impl<T> FromStr for ByLines<T>
where
    T: FromStr,
{
    type Err = ParseByLinesError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_errors: Vec<ParseByLineError<T::Err>> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let line_results = s.lines().map(|line| line.parse::<T>()).enumerate();
        for (index, line_result) in line_results {
            match line_result {
                Ok(result) => results.push(result),
                Err(error) => line_errors.push(ParseByLineError { line: index, error }),
            }
        }
        if line_errors.len() > 0 {
            Err(ParseByLinesError { line_errors })
        } else {
            Ok(ByLines(results))
        }
    }
}
