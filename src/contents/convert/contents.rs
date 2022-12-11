use super::{
    lines::ParseLineError,
    sections::{AsParseSections, FromSections, ParseSectionError},
};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct ParseContentsError {
    section_errors: Vec<ParseSectionError>,
}

// TODO remove constructor, allow creation via local traits only
impl ParseContentsError {
    pub fn new(section_errors: Vec<ParseSectionError>) -> ParseContentsError {
        ParseContentsError { section_errors }
    }
}

impl Display for ParseContentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl From<Vec<ParseSectionError>> for ParseContentsError {
    fn from(section_errors: Vec<ParseSectionError>) -> Self {
        ParseContentsError { section_errors }
    }
}

impl From<Vec<ParseLineError>> for ParseContentsError {
    fn from(line_errors: Vec<ParseLineError>) -> Self {
        ParseContentsError {
            section_errors: vec![ParseSectionError::new(0, 0, line_errors, None)],
        }
    }
}
pub trait FromContents: Sized {
    fn from_contents(s: &str) -> Result<Self, ParseContentsError>;
}

impl<T> FromContents for T
where
    T: FromSections,
{
    fn from_contents(s: &str) -> Result<Self, ParseContentsError> {
        s.parse_sections::<T>()
            .map_err(|err| ParseContentsError::new(err))
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
