mod lib;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};
use std::fmt::Display;

use crate::input::puzzle_part::PuzzlePart;

pub fn main(input: PuzzleInput) -> Result<Box<dyn Display>, FileErrorCollection> {
    let calories_lines = file::parse_optional_lines::<i32>(input.file_contents)?;
    let calories = lib::sum_of_top_group_sums(
        &mut calories_lines.into_iter(),
        match input.puzzle_part {
            PuzzlePart::Part1 => 1,
            PuzzlePart::Part2 => 3,
        },
    );
    Ok(Box::new(calories))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("24000", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("45000", output.to_string());
        Ok(())
    }
}
