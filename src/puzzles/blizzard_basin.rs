mod basin_state;
mod basin_tile;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    puzzles::blizzard_basin::basin_state::BasinState,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn blizzard_basin(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(mut basin_state) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<BasinState>>()?;
    let target_trip_count = match input.puzzle_part {
        PuzzlePart::Part1 => 1,
        PuzzlePart::Part2 => 3,
    };

    let mut answer: usize = 1;
    while basin_state.step() < target_trip_count {
        answer += 1;
    }
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = blizzard_basin(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("18", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = blizzard_basin(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("54", output);
        Ok(())
    }
}
