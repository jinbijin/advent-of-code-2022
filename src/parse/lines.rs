use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug)]
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
pub struct ParseByLinesError<TError>(pub Vec<ParseByLineError<TError>>);

impl<TError> Display for ParseByLinesError<TError>
where
    TError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for error in self.0.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl<TError> Error for ParseByLinesError<TError> where TError: Error {}

pub struct ByLines<T>(pub Vec<T>);

impl<T> FromStr for ByLines<T>
where
    T: FromStr,
{
    type Err = ParseByLinesError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut error_collection: Vec<ParseByLineError<T::Err>> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let line_results = s.lines().map(|line| line.parse::<T>()).enumerate();
        for (index, line_result) in line_results {
            match line_result {
                Ok(result) => results.push(result),
                Err(error) => error_collection.push(ParseByLineError { line: index, error }),
            }
        }
        if error_collection.len() > 0 {
            Err(ParseByLinesError(error_collection))
        } else {
            Ok(ByLines(results))
        }
    }
}
