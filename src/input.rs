mod match_args;
pub mod puzzle_input;
pub mod puzzle_part;
mod puzzle_type;

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs, io,
};

use crate::{file::FileErrorCollection, input::puzzle_input::PuzzleInput};

use self::{
    match_args::{MatchArgs, MatchArgsError, MatchArgsIterator},
    puzzle_part::{ParsePuzzlePartError, PuzzlePart},
    puzzle_type::{ParsePuzzleTypeError, PuzzleType},
};

pub enum ParsePuzzleArgsError {
    MissingPuzzleType,
    InvalidPuzzleType(String),
    MissingPuzzlePart,
    InvalidPuzzlePart(String),
}

impl Display for ParsePuzzleArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPuzzleType => write!(f, "missing puzzle type"),
            Self::MissingPuzzlePart => write!(f, "missing puzzle part"),
            Self::InvalidPuzzleType(error) => write!(f, "invalid puzzle type '{}'", error),
            Self::InvalidPuzzlePart(error) => write!(f, "invalid puzzle part '{}'", error),
        }
    }
}

impl Debug for ParsePuzzleArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParsePuzzleArgsError {}

pub enum RunSolutionError {
    FileReadError {
        file_name: String,
        error: io::Error,
    },
    FileParseError {
        file_name: String,
        error: FileErrorCollection,
    },
}

impl Display for RunSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileReadError {
                file_name,
                error: _,
            } => write!(f, "error while reading '{}'", file_name),
            Self::FileParseError {
                file_name,
                error: _,
            } => write!(f, "error parsing contents of '{}'", file_name),
        }
    }
}

impl Debug for RunSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for RunSolutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileReadError {
                file_name: _,
                error,
            } => Some(error),
            Self::FileParseError {
                file_name: _,
                error,
            } => Some(error),
        }
    }
}

pub struct PuzzleArgs {
    puzzle_type: PuzzleType,
    puzzle_part: PuzzlePart,
}

impl MatchArgs for PuzzleArgs {
    type Err = ParsePuzzleArgsError;

    fn match_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Self::Err> {
        let puzzle_type = args.next_match::<PuzzleType>().map_err(|err| match err {
            MatchArgsError::ParseError(ParsePuzzleTypeError::InvalidValue(value)) => {
                Self::Err::InvalidPuzzleType(value)
            }
            MatchArgsError::EndOfArgsError => Self::Err::MissingPuzzleType,
        })?;
        let puzzle_part = args.next_match::<PuzzlePart>().map_err(|err| match err {
            MatchArgsError::ParseError(ParsePuzzlePartError::InvalidValue(value)) => {
                Self::Err::InvalidPuzzlePart(value)
            }
            MatchArgsError::EndOfArgsError => Self::Err::MissingPuzzlePart,
        })?;

        Ok(PuzzleArgs {
            puzzle_type,
            puzzle_part,
        })
    }
}

impl PuzzleArgs {
    pub fn build(
        args: &mut impl Iterator<Item = String>,
    ) -> Result<PuzzleArgs, ParsePuzzleArgsError> {
        args.next();
        args.next_match()
    }

    pub fn run_solution(&self) -> Result<(), RunSolutionError> {
        let file_name = format!("input/{}.txt", self.puzzle_type.file_name());
        let file_contents =
            fs::read_to_string(&file_name).map_err(|error| RunSolutionError::FileReadError {
                file_name: file_name.clone(),
                error,
            })?;
        let output = self.puzzle_type.solver()(PuzzleInput {
            file_contents,
            puzzle_part: self.puzzle_part,
        })
        .map_err(|error| RunSolutionError::FileParseError { file_name, error })?;
        println!("The answer is: {}", output);
        Ok(())
    }
}
