use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

trait SectionError {
    fn line_errors(&self) -> Vec<ParseLineError>;
    fn error_description(&self) -> Option<String>;
}

pub trait CustomSectionError: Error {}

impl<T> SectionError for T
where
    T: CustomSectionError,
{
    fn line_errors(&self) -> Vec<ParseLineError> {
        Vec::new()
    }

    fn error_description(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl SectionError for Vec<ParseLineError> {
    fn line_errors(&self) -> Vec<ParseLineError> {
        self.clone()
    }

    fn error_description(&self) -> Option<String> {
        None
    }
}

#[derive(Clone)]
pub struct ParseSectionError {
    error_description: Option<String>,
    line_errors: Vec<ParseLineError>,
}

impl Display for ParseSectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(error_description) = &self.error_description {
            writeln!(f, "{}", error_description)?;
        }
        for line_error in self.line_errors.iter() {
            line_error.fmt(f)?;
        }
        Ok(())
    }
}

impl Debug for ParseSectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseSectionError {}

impl CustomSectionError for ParseSectionError {}

impl From<Vec<ParseLineError>> for ParseSectionError {
    fn from(line_errors: Vec<ParseLineError>) -> Self {
        ParseSectionError {
            error_description: None,
            line_errors,
        }
    }
}

pub trait FromSection: Sized {
    fn from_section(s: &str) -> Result<Self, ParseSectionError>;
}

impl<T> FromSection for T
where
    T: FromLines,
    T::Err: SectionError,
{
    fn from_section(s: &str) -> Result<Self, ParseSectionError> {
        s.parse_lines::<Self>().map_err(|err| ParseSectionError {
            error_description: err.error_description(),
            line_errors: err.line_errors(),
        })
    }
}

pub trait AsParseSection {
    fn parse_section<T>(&self) -> Result<T, ParseSectionError>
    where
        T: FromSection;
}

impl AsParseSection for &str {
    fn parse_section<T>(&self) -> Result<T, ParseSectionError>
    where
        T: FromSection,
    {
        T::from_section(self)
    }
}

#[derive(Clone)]
pub struct ParseLineError {
    line: usize,
    error_description: String,
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Line {}: {}", self.line + 1, self.error_description)
    }
}

pub trait FromLines: Sized {
    type Err;

    fn from_lines(s: &str) -> Result<Self, Self::Err>;
}

impl<T> FromLines for Vec<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = Vec<ParseLineError>;

    fn from_lines(s: &str) -> Result<Self, Self::Err> {
        let mut error_collection: Vec<ParseLineError> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let line_results = s.lines().map(|line| line.parse::<T>()).enumerate();
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

trait AsParseLines {
    fn parse_lines<T>(&self) -> Result<T, T::Err>
    where
        T: FromLines;
}

impl AsParseLines for &str {
    fn parse_lines<T>(&self) -> Result<T, T::Err>
    where
        T: FromLines,
    {
        T::from_lines(self)
    }
}

pub struct SingleLine<T>(pub T);

impl<T> FromLines for SingleLine<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = ParseSectionError;

    fn from_lines(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let result = match lines.next() {
            Some(line) => line.parse::<T>().map_err(|err| ParseSectionError {
                error_description: None,
                line_errors: vec![ParseLineError {
                    line: 0,
                    error_description: err.to_string(),
                }],
            }),
            None => Err(ParseSectionError {
                error_description: Some(format!("unexpected empty file")),
                line_errors: Vec::new(),
            }),
        }?;

        match lines.next() {
            Some(_) => Err(ParseSectionError {
                error_description: Some(format!("expected only 1 section, found more")),
                line_errors: Vec::new(),
            }),
            None => Ok(SingleLine(result)),
        }
    }
}
