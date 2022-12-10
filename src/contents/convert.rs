use std::{fmt::Display, str::FromStr};

use super::{
    errors::{ParseContentsError, ParseLineError, ParseSectionError},
    sections::AsSections,
};

// TODO maybe I don't need the plural traits;
// Can I replace those by "global validation traits"?

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

pub trait FromSections: Sized {
    fn from_sections(s: &str) -> Result<Self, Vec<ParseSectionError>>;
}

impl<T> FromSections for Vec<T>
where
    T: FromSection<Err = Vec<ParseLineError>>, // TODO generalize error type
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
                Err(err) => {
                    error_collection.push(ParseSectionError::new(index, section_start, err))
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
                Err(error) => error_collection.push(ParseLineError::new(index, error.to_string())),
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
