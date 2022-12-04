mod lib;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use crate::input::puzzle_part::PuzzlePart;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn calorie_counting(input: PuzzleInput) -> Result<String, FileErrorCollection> {
    let calories_lines = file::parse_optional_lines::<i32>(input.file_contents)?;
    let calories = lib::sum_of_top_group_sums(
        &mut calories_lines.into_iter(),
        match input.puzzle_part {
            PuzzlePart::Part1 => 1,
            PuzzlePart::Part2 => 3,
        },
    );
    Ok(calories.to_string())
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
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("24000", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("45000", output.to_string());
        Ok(())
    }
}
