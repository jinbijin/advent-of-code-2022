mod lib;

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::file::{self, FileErrorCollection};

use self::lib::{CampAssignment, ParseCampAssignmentError};

pub enum ParseCampCleanupArgsError {
    InvalidValue(String),
}

impl Display for ParseCampCleanupArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(f, "Invalid option '{}' for puzzle 'camp_cleanup'", value)
    }
}

impl Debug for ParseCampCleanupArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParseCampCleanupArgsError {}

pub enum CampCleanupArgs {
    Contain,
    Overlap,
}

impl FromStr for CampCleanupArgs {
    type Err = ParseCampCleanupArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contain" => Ok(Self::Contain),
            "overlap" => Ok(Self::Overlap),
            _ => Err(Self::Err::InvalidValue(s.to_string())),
        }
    }
}

pub fn main(
    file_contents: String,
    args: &CampCleanupArgs,
) -> Result<Box<dyn Display>, FileErrorCollection<ParseCampAssignmentError>> {
    let camp_assignments = file::parse_lines::<CampAssignment>(file_contents)?;
    let answer = camp_assignments
        .iter()
        .filter(|camp_assignment| match args {
            CampCleanupArgs::Contain => camp_assignment.one_is_contained_in_other(),
            CampCleanupArgs::Overlap => camp_assignment.overlaps(),
        })
        .count();

    Ok(Box::new(answer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &CampCleanupArgs::Contain)?;

        assert_eq!("2", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &CampCleanupArgs::Overlap)?;

        assert_eq!("4", output.to_string());
        Ok(())
    }
}
