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

pub struct ParseSectionError {
    section: usize,
    first_line: usize,
    line_errors: Vec<ParseLineError>,
}

// TODO remove constructor
impl ParseSectionError {
    pub fn new(
        section: usize,
        first_line: usize,
        line_errors: Vec<ParseLineError>,
    ) -> ParseSectionError {
        ParseSectionError {
            section,
            first_line,
            line_errors,
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
        for line_error in self.line_errors.iter() {
            line_error.fmt(f)?;
        }
        Ok(())
    }
}

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

impl From<Vec<ParseLineError>> for ParseContentsError {
    fn from(line_errors: Vec<ParseLineError>) -> Self {
        ParseContentsError {
            section_errors: vec![ParseSectionError {
                section: 0,
                first_line: 0,
                line_errors,
            }],
        }
    }
}
