use super::{
    super::sections::AsSections,
    lines::{AsParseLines, FromLines, ParseLineError},
};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

trait SectionError {
    fn line_errors(&self) -> Vec<ParseLineError>;
    fn section_error_description(&self) -> Option<String>;
}

pub trait CustomSectionError: Error {}

impl<T> SectionError for T
where
    T: CustomSectionError,
{
    fn line_errors(&self) -> Vec<ParseLineError> {
        Vec::new()
    }

    fn section_error_description(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl SectionError for Vec<ParseLineError> {
    fn line_errors(&self) -> Vec<ParseLineError> {
        self.clone()
    }

    fn section_error_description(&self) -> Option<String> {
        None
    }
}

pub struct ParseSectionError {
    section: usize,
    first_line: usize,
    line_errors: Vec<ParseLineError>,
    section_error_description: Option<String>,
}

// TODO remove constructor
impl ParseSectionError {
    pub fn new(
        section: usize,
        first_line: usize,
        line_errors: Vec<ParseLineError>,
        section_error_description: Option<String>,
    ) -> ParseSectionError {
        ParseSectionError {
            section,
            first_line,
            line_errors,
            section_error_description,
        }
    }
}

impl Display for ParseSectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "In section {} at line {}:",
            self.section + 1,
            self.first_line + 1
        )?;
        if let Some(error_description) = &self.section_error_description {
            writeln!(f, "{}", error_description)?;
        }
        for line_error in self.line_errors.iter() {
            line_error.fmt(f)?;
        }
        Ok(())
    }
}

pub trait FromSection: Sized {
    type Err;

    fn from_section(s: &str) -> Result<Self, Self::Err>;
}

impl<T> FromSection for T
where
    T: FromLines,
{
    type Err = Vec<ParseLineError>;

    fn from_section(s: &str) -> Result<Self, Self::Err> {
        s.parse_lines::<Self>()
    }
}

pub trait AsParseSection {
    fn parse_section<T>(&self) -> Result<T, T::Err>
    where
        T: FromSection;
}

impl AsParseSection for &str {
    fn parse_section<T>(&self) -> Result<T, T::Err>
    where
        T: FromSection,
    {
        T::from_section(self)
    }
}

pub trait FromSections: Sized {
    fn from_sections(s: &str) -> Result<Self, Vec<ParseSectionError>>;
}

impl<T> FromSections for Vec<T>
where
    T: FromSection,
    T::Err: SectionError,
{
    fn from_sections(s: &str) -> Result<Self, Vec<ParseSectionError>> {
        let mut error_collection: Vec<ParseSectionError> = Vec::new();
        let mut results: Vec<T> = Vec::new();

        let line_results = s
            .sections()
            .map(|section| {
                (
                    section.starts_at_line,
                    section.contents.parse_section::<T>(),
                )
            })
            .enumerate();
        for (index, (section_start, line_result)) in line_results {
            match line_result {
                Ok(result) => results.push(result),
                Err(err) => error_collection.push(ParseSectionError::new(
                    index,
                    section_start,
                    err.line_errors(),
                    err.section_error_description(),
                )),
            }
        }

        if error_collection.len() > 0 {
            Err(error_collection)
        } else {
            Ok(results)
        }
    }
}

pub trait AsParseSections {
    fn parse_sections<T>(&self) -> Result<T, Vec<ParseSectionError>>
    where
        T: FromSections;
}

impl AsParseSections for &str {
    fn parse_sections<T>(&self) -> Result<T, Vec<ParseSectionError>>
    where
        T: FromSections,
    {
        T::from_sections(self)
    }
}
