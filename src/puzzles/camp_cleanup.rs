mod lib;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use self::lib::CampAssignment;

use crate::input::puzzle_part::PuzzlePart;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn camp_cleanup(input: PuzzleInput) -> Result<String, FileErrorCollection> {
    let camp_assignments = file::parse_lines::<CampAssignment>(input.file_contents)?;
    let answer = camp_assignments
        .iter()
        .filter(|camp_assignment| match input.puzzle_part {
            PuzzlePart::Part1 => camp_assignment.one_is_contained_in_other(),
            PuzzlePart::Part2 => camp_assignment.overlaps(),
        })
        .count();

    Ok(answer.to_string())
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
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = camp_cleanup(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("2", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = camp_cleanup(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("4", output.to_string());
        Ok(())
    }
}
