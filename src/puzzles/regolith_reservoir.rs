mod cave_in;
mod rock_range;

use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines},
};

use self::{
    cave_in::{AsIntoFloor, CaveIn},
    rock_range::{RockRangeChain, RockRangesWithAbyss},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn regolith_reservoir(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(rock_range_chains) = input.file_contents.parse::<ByLines<RockRangeChain>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => {
            let rock_ranges_with_abyss: RockRangesWithAbyss = rock_range_chains.into();
            rock_ranges_with_abyss.count()
        }
        PuzzlePart::Part2 => {
            let cave_in: CaveIn = rock_range_chains.into();
            cave_in.into_floor().fill_with_sand()
        }
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = regolith_reservoir(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("24", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = regolith_reservoir(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("93", output);
        Ok(())
    }
}
