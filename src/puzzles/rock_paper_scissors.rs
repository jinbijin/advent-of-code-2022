mod common;
mod error;
mod lib;
mod mapping;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use self::lib::{RpsDesiredResult, RpsMatch};

use crate::input::puzzle_part::PuzzlePart;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn rock_paper_scissors(input: PuzzleInput) -> Result<String, FileErrorCollection> {
    let score = match input.puzzle_part {
        PuzzlePart::Part1 => lib::get_tournament_score(
            &mut file::parse_lines::<RpsMatch>(input.file_contents)?.into_iter(),
        ),
        PuzzlePart::Part2 => lib::get_tournament_score(
            &mut file::parse_lines::<RpsDesiredResult>(input.file_contents)?.into_iter(),
        ),
    };
    Ok(score.to_string())
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
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = rock_paper_scissors(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("15", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = rock_paper_scissors(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("12", output.to_string());
        Ok(())
    }
}
