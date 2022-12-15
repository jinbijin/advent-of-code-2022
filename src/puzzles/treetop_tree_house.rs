mod digit_grid;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    common::position::Position,
    contents::{
        convert::contents::{AsParseContents, ParseContentsError, SingleSection},
        grid::Grid,
    },
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::digit_grid::TreetopGrid;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn treetop_tree_house(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(grid) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Grid<1, 0, usize>>>()?;
    let grid_positions = grid.positions().collect::<Vec<Position<usize>>>();
    let treetop_grid = TreetopGrid(grid);
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => grid_positions
            .into_iter()
            .filter(|position| treetop_grid.visible(*position))
            .count(),
        PuzzlePart::Part2 => grid_positions
            .into_iter()
            .map(|position| treetop_grid.scenic_score(position))
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
