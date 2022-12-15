use super::sections::{AsParseSection, FromSection, ParseLineError, ParseSectionItemError};
use crate::contents::sections::AsSections;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

trait ContentsError {
    fn section_errors(&self) -> Vec<ParseSectionItemError>;
    fn error_description(&self) -> Option<String>;
}

pub trait CustomContentsError: Error {}

impl<T> ContentsError for T
where
    T: CustomContentsError,
{
    fn section_errors(&self) -> Vec<ParseSectionItemError> {
        Vec::new()
    }

    fn error_description(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl ContentsError for Vec<ParseSectionItemError> {
    fn section_errors(&self) -> Vec<ParseSectionItemError> {
        self.clone()
    }

    fn error_description(&self) -> Option<String> {
        None
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct ParseContentsError {
    error_description: Option<String>,
    section_errors: Vec<ParseSectionItemError>,
}

impl ContentsError for ParseContentsError {
    fn error_description(&self) -> Option<String> {
        self.error_description.clone()
    }

    fn section_errors(&self) -> Vec<ParseSectionItemError> {
        self.section_errors.clone()
    }
}

// TODO remove constructor, allow creation via local traits only
impl ParseContentsError {
    pub fn new(section_errors: Vec<ParseSectionItemError>) -> ParseContentsError {
        ParseContentsError {
            section_errors,
            error_description: None,
        }
    }
}

impl Display for ParseContentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(error_description) = &self.error_description {
            writeln!(f, "{}", error_description)?;
        }
        for section_error in self.section_errors.iter() {
            section_error.fmt(f)?;
        }
        Ok(())
    }
}

impl Debug for ParseContentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Error for ParseContentsError {}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ParseContentsError {
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.to_string()
    }
}

impl From<Vec<ParseSectionItemError>> for ParseContentsError {
    fn from(section_errors: Vec<ParseSectionItemError>) -> Self {
        ParseContentsError {
            section_errors,
            error_description: None,
        }
    }
}

impl From<Vec<ParseLineError>> for ParseContentsError {
    fn from(line_errors: Vec<ParseLineError>) -> Self {
        ParseContentsError {
            section_errors: vec![ParseSectionItemError::new(0, 0, line_errors.into())],
            error_description: None,
        }
    }
}

pub trait FromContents: Sized {
    fn from_contents(s: &str) -> Result<Self, ParseContentsError>;
}

impl<T> FromContents for T
where
    T: FromSections,
    T::Err: ContentsError,
{
    fn from_contents(s: &str) -> Result<Self, ParseContentsError> {
        s.parse_sections::<T>().map_err(|err| ParseContentsError {
            error_description: err.error_description(),
            section_errors: err.section_errors(),
        })
    }
}

pub trait AsParseContents {
    fn parse_contents<T>(&self) -> Result<T, ParseContentsError>
    where
        T: FromContents;
}

impl AsParseContents for &str {
    fn parse_contents<T>(&self) -> Result<T, ParseContentsError>
    where
        T: FromContents,
    {
        T::from_contents(self)
    }
}

pub trait FromSections: Sized {
    type Err;

    fn from_sections(s: &str) -> Result<Self, Self::Err>;
}

impl<T> FromSections for Vec<T>
where
    T: FromSection,
{
    type Err = Vec<ParseSectionItemError>;

    fn from_sections(s: &str) -> Result<Self, Self::Err> {
        let mut error_collection: Vec<ParseSectionItemError> = Vec::new();
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
                Err(err) => {
                    error_collection.push(ParseSectionItemError::new(index, section_start, err))
                }
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
    fn parse_sections<T>(&self) -> Result<T, T::Err>
    where
        T: FromSections;
}

impl AsParseSections for &str {
    fn parse_sections<T>(&self) -> Result<T, T::Err>
    where
        T: FromSections,
    {
        T::from_sections(self)
    }
}

pub struct SingleSection<T>(pub T);

impl<T> FromSections for SingleSection<T>
where
    T: FromSection,
{
    type Err = ParseContentsError;

    fn from_sections(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.sections();

        let result = match sections.next() {
            Some(section) => {
                section
                    .contents
                    .parse_section::<T>()
                    .map_err(|err| ParseContentsError {
                        error_description: None,
                        section_errors: vec![ParseSectionItemError::new(0, 0, err)],
                    })
            }
            None => Err(ParseContentsError {
                error_description: Some(format!("unexpected empty file")),
                section_errors: Vec::new(),
            }),
        }?;

        match sections.next() {
            Some(_) => Err(ParseContentsError {
                error_description: Some(format!("expected only 1 section, found more")),
                section_errors: Vec::new(),
            }),
            None => Ok(SingleSection(result)),
        }
    }
}

pub struct SectionPair<T, U>(pub T, pub U);

impl<T, U> FromSections for SectionPair<T, U>
where
    T: FromSection,
    U: FromSection,
{
    type Err = ParseContentsError;

    fn from_sections(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.sections();

        let first = match sections.next() {
            Some(section) => {
                section
                    .contents
                    .parse_section::<T>()
                    .map_err(|err| ParseContentsError {
                        error_description: None,
                        section_errors: vec![ParseSectionItemError::new(0, 0, err)],
                    })
            }
            None => Err(ParseContentsError {
                error_description: Some(format!("unexpected empty file")),
                section_errors: Vec::new(),
            }),
        }?;

        let second = match sections.next() {
            Some(section) => {
                section
                    .contents
                    .parse_section::<U>()
                    .map_err(|err| ParseContentsError {
                        error_description: None,
                        section_errors: vec![ParseSectionItemError::new(0, 0, err)],
                    })
            }
            None => Err(ParseContentsError {
                error_description: Some(format!(
                    "unexpected end of file while reading second section"
                )),
                section_errors: Vec::new(),
            }),
        }?;

        match sections.next() {
            Some(_) => Err(ParseContentsError {
                error_description: Some(format!("expected only 2 sections, found more")),
                section_errors: Vec::new(),
            }),
            None => Ok(SectionPair(first, second)),
        }
    }
}
