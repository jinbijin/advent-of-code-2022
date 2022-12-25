use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use super::{error::ParseContentsError, sections::AsSections};

#[derive(Debug)]
pub enum ParseSectionPairError<TError, UError> {
    Empty,
    MissingSecondSection,
    MoreThanTwoSections,
    SectionsParseError {
        first: Option<TError>,
        second: Option<UError>,
    },
}

impl<TError, UError> Display for ParseSectionPairError<TError, UError>
where
    TError: Display,
    UError: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::MissingSecondSection => write!(f, "unexpected end of input"),
            Self::MoreThanTwoSections => write!(f, "unexpected extra input after two sections"),
            Self::SectionsParseError { first, second } => {
                if let Some(error) = first {
                    writeln!(f, "Error in first part: {}", error)?;
                }
                if let Some(error) = second {
                    writeln!(f, "Error in second part: {}", error)?;
                }
                Ok(())
            }
        }
    }
}

impl<TError, UError> Error for ParseSectionPairError<TError, UError>
where
    TError: Error,
    UError: Error,
{
}

impl<TError, UError> From<ParseSectionPairError<TError, UError>> for ParseContentsError
where
    TError: Error,
    UError: Error,
{
    fn from(value: ParseSectionPairError<TError, UError>) -> Self {
        ParseContentsError::new(value)
    }
}

pub struct SectionPair<T, U>(pub T, pub U);

impl<T, U> FromStr for SectionPair<T, U>
where
    T: FromStr,
    U: FromStr,
{
    type Err = ParseSectionPairError<T::Err, U::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.sections();

        let first = match sections.next() {
            Some(section) => section.contents.parse::<T>(),
            None => {
                return Err(Self::Err::Empty);
            }
        };

        let second = match sections.next() {
            Some(section) => section.contents.parse::<U>(),
            None => return Err(Self::Err::MissingSecondSection),
        };

        match sections.next() {
            Some(_) => Err(Self::Err::MoreThanTwoSections),
            None => match (first, second) {
                (Ok(first), Ok(second)) => Ok(Self(first, second)),
                (Ok(_), Err(second)) => Err(Self::Err::SectionsParseError {
                    first: None,
                    second: Some(second),
                }),
                (Err(first), Ok(_)) => Err(Self::Err::SectionsParseError {
                    first: Some(first),
                    second: None,
                }),
                (Err(first), Err(second)) => Err(Self::Err::SectionsParseError {
                    first: Some(first),
                    second: Some(second),
                }),
            },
        }
    }
}
