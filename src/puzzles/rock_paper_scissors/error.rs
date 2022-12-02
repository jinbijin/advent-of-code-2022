use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RpsMatchParseError {
    OpponentChoiceParseError(Option<char>),
    SeparatorParseError(Option<char>),
    OwnChoiceParseError(Option<char>),
    ResultParseError(Option<char>),
    EndOfLineParseError,
}

impl Display for RpsMatchParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpponentChoiceParseError(Some(other)) => {
                writeln!(f, "Invalid choice '{}' for opponent.", other)
            }
            Self::OpponentChoiceParseError(None) => {
                writeln!(f, "Line ended while reading opponent's choice.")
            }
            Self::SeparatorParseError(Some(other)) => writeln!(
                f,
                "Invalid character '{}' found while expecting a space.",
                other
            ),
            Self::SeparatorParseError(None) => writeln!(f, "Line ended after opponent's choice."),
            Self::OwnChoiceParseError(Some(other)) => {
                writeln!(f, "Invalid choice '{}' for self.", other)
            }
            Self::OwnChoiceParseError(None) => writeln!(f, "Line ended while reading own choice."),
            Self::ResultParseError(Some(other)) => {
                writeln!(f, "Invalid choice '{}' for result.", other)
            }
            Self::ResultParseError(None) => writeln!(f, "Line ended while reading result."),
            Self::EndOfLineParseError => writeln!(f, "Line did not end after reading match data."),
        }
    }
}

impl Error for RpsMatchParseError {}
