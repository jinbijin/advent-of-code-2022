mod lib;

use std::fmt::Display;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use self::lib::CampAssignment;

use crate::input::puzzle_part::PuzzlePart;

pub fn main(input: PuzzleInput) -> Result<Box<dyn Display>, FileErrorCollection> {
    let camp_assignments = file::parse_lines::<CampAssignment>(input.file_contents)?;
    let answer = camp_assignments
        .iter()
        .filter(|camp_assignment| match input.puzzle_part {
            PuzzlePart::Part1 => camp_assignment.one_is_contained_in_other(),
            PuzzlePart::Part2 => camp_assignment.overlaps(),
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
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("2", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("4", output.to_string());
        Ok(())
    }
}
