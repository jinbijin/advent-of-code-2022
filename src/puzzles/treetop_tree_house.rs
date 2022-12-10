mod digit_grid;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use self::digit_grid::{Coordinates, DigitGrid};
use crate::{
    contents::errors::ParseContentsError,
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn treetop_tree_house(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let grid = input.file_contents.parse::<DigitGrid>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => (0..grid.height)
            .flat_map(move |row| (0..grid.width).map(move |col| Coordinates { row, col }))
            .filter(|coords| grid.visible(*coords))
            .count(),
        PuzzlePart::Part2 => (0..grid.height)
            .flat_map(move |row| (0..grid.width).map(move |col| Coordinates { row, col }))
            .map(|coords| grid.scenic_score(coords))
            .max()
            .map_or(0, |x| x),
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = treetop_tree_house(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("21", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = treetop_tree_house(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("8", output);
        Ok(())
    }
}
