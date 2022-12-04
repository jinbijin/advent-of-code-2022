mod common;
mod error;
mod lib;
mod mapping;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};
use std::fmt::Display;

use self::lib::{RpsDesiredResult, RpsMatch};

use crate::input::puzzle_part::PuzzlePart;

pub fn main(input: PuzzleInput) -> Result<Box<dyn Display>, FileErrorCollection> {
    let score = match input.puzzle_part {
        PuzzlePart::Part1 => lib::get_tournament_score(
            &mut file::parse_lines::<RpsMatch>(input.file_contents)?.into_iter(),
        ),
        PuzzlePart::Part2 => lib::get_tournament_score(
            &mut file::parse_lines::<RpsDesiredResult>(input.file_contents)?.into_iter(),
        ),
    };
    Ok(Box::new(score))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT_TEXT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("15", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("12", output.to_string());
        Ok(())
    }
}
