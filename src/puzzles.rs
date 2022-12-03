pub mod calorie_counting;
pub mod rock_paper_scissors;
pub mod rucksack_reorganization;

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs,
};

use crate::match_args::{MatchArgs, MatchArgsError, MatchArgsIterator};

use self::{
    calorie_counting::CalorieCountingArgs, rock_paper_scissors::RockPaperScissorsArgs,
    rucksack_reorganization::RucksackReorganizationArgs,
};

pub enum ParsePuzzleTypeError {
    InvalidValue(String),
}

impl Display for ParsePuzzleTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(f, "Invalid option '{}' for puzzle type", value)
    }
}

impl Debug for ParsePuzzleTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParsePuzzleTypeError {}

pub enum PuzzleInput {
    CalorieCounting(CalorieCountingArgs),
    RockPaperScissors(RockPaperScissorsArgs),
    RucksackReorganization(RucksackReorganizationArgs),
}

impl MatchArgs for PuzzleInput {
    type Err = MatchArgsError<Box<dyn Error>>;

    fn match_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Self::Err> {
        let puzzle = args.next_match::<String>().map_err(|err| match err {
            MatchArgsError::ParseError(err) => {
                MatchArgsError::ParseError(Box::new(err) as Box<dyn Error>)
            }
            MatchArgsError::EndOfArgsError => MatchArgsError::EndOfArgsError,
        })?;

        match puzzle.as_str() {
            "calorie_counting" => {
                let args = args.next_match().map_err(|err| match err {
                    MatchArgsError::ParseError(err) => {
                        MatchArgsError::ParseError(Box::new(err) as Box<dyn Error>)
                    }
                    MatchArgsError::EndOfArgsError => MatchArgsError::EndOfArgsError,
                })?;
                Ok(Self::CalorieCounting(args))
            }
            "rock_paper_scissors" => {
                let args = args.next_match().map_err(|err| match err {
                    MatchArgsError::ParseError(err) => {
                        MatchArgsError::ParseError(Box::new(err) as Box<dyn Error>)
                    }
                    MatchArgsError::EndOfArgsError => MatchArgsError::EndOfArgsError,
                })?;
                Ok(Self::RockPaperScissors(args))
            }
            "rucksack_reorganization" => {
                let args = args.next_match().map_err(|err| match err {
                    MatchArgsError::ParseError(err) => {
                        MatchArgsError::ParseError(Box::new(err) as Box<dyn Error>)
                    }
                    MatchArgsError::EndOfArgsError => MatchArgsError::EndOfArgsError,
                })?;
                Ok(Self::RucksackReorganization(args))
            }
            _ => Err(MatchArgsError::ParseError(Box::new(
                ParsePuzzleTypeError::InvalidValue(puzzle.clone()),
            ))),
        }
    }
}

impl PuzzleInput {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<PuzzleInput, Box<dyn Error>> {
        args.next();
        args.next_match()
            .map_err(|err| format!("Error while parsing: {}", err).into())
    }

    pub fn run_solution(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("input/{}.txt", self.file_name());
        let file_contents = fs::read_to_string(file_name)?;
        let output = match self {
            Self::CalorieCounting(args) => calorie_counting::main(file_contents, args)?,
            Self::RockPaperScissors(args) => rock_paper_scissors::main(file_contents, args)?,
            Self::RucksackReorganization(args) => {
                rucksack_reorganization::main(file_contents, args)?
            }
        };
        println!("The answer is: {}", output);
        Ok(())
    }

    pub fn file_name(&self) -> &str {
        match self {
            Self::CalorieCounting(_) => "calorie_counting",
            Self::RockPaperScissors(_) => "rock_paper_scissors",
            Self::RucksackReorganization(_) => "rucksack_reorganization",
        }
    }
}
