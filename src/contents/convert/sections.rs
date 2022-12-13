use std::{
    error::Error,
    fmt::{self, Display, Formatter},
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

impl From<Vec<ParseLineError>> for ParseSectionError {
    fn from(line_errors: Vec<ParseLineError>) -> Self {
        ParseSectionError {
            error_description: None,
            line_errors,
        }
    }
}

#[derive(Clone)]
pub struct ParseSectionItemError {
    section: usize,
    first_line: usize,
    section_error: ParseSectionError,
}

// TODO remove constructor
impl ParseSectionItemError {
    pub fn new(
        section: usize,
        first_line: usize,
        section_error: ParseSectionError,
    ) -> ParseSectionItemError {
        ParseSectionItemError {
            section,
            first_line,
            section_error,
        }
    }
}

impl Display for ParseSectionItemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "In section {} at line {}:",
            self.section + 1,
            self.first_line + 1
        )?;
        self.section_error.fmt(f)
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
