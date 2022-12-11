use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone)]
pub struct ParseLineError {
    line: usize,
    error_description: String,
}

// TODO remove constructor
impl ParseLineError {
    pub fn new(line: usize, error_description: String) -> ParseLineError {
        ParseLineError {
            line,
            error_description,
        }
    }
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Line {}: {}", self.line + 1, self.error_description)
    }
}

pub trait FromLine: Sized {
    type Err;

    fn from_line(s: &str) -> Result<Self, Self::Err>;
}

impl<T> FromLine for T
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_line(s: &str) -> Result<Self, Self::Err> {
        s.parse::<T>()
    }
}

pub trait AsParseLine {
    fn parse_line<T>(&self) -> Result<T, <T as FromLine>::Err>
    where
        T: FromLine;
}

impl AsParseLine for &str {
    fn parse_line<T>(&self) -> Result<T, <T as FromLine>::Err>
    where
        T: FromLine,
    {
        T::from_line(self)
    }
}

pub trait FromLines: Sized {
    fn from_lines(s: &str) -> Result<Self, Vec<ParseLineError>>;
}

impl<T> FromLines for Vec<T>
where
    T: FromLine,
    T::Err: Display,
{
    fn from_lines(s: &str) -> Result<Self, Vec<ParseLineError>> {
        let mut error_collection: Vec<ParseLineError> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let line_results = s.lines().map(|line| line.parse_line::<T>()).enumerate();
        for (index, line_result) in line_results {
            match line_result {
                Ok(result) => results.push(result),
                Err(error) => error_collection.push(ParseLineError {
                    line: index,
                    error_description: error.to_string(),
                }),
            }
        }
        if error_collection.len() > 0 {
            Err(error_collection)
        } else {
            Ok(results)
        }
    }
}

pub trait AsParseLines {
    fn parse_lines<T>(&self) -> Result<T, Vec<ParseLineError>>
    where
        T: FromLines;
}

impl AsParseLines for &str {
    fn parse_lines<T>(&self) -> Result<T, Vec<ParseLineError>>
    where
        T: FromLines,
    {
        T::from_lines(self)
    }
}
