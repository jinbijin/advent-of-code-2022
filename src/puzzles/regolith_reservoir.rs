mod rock_range;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::rock_range::{RockRangeChain, RockRangesWithAbyss, RockRangesWithFloor};

// PERF
// We currently run the entire simulation in the most naive way possible, which is kinda slow.
// (To note, I needed a release build for this to run in decent time for part 2, which still is basically a minute.)
// But it is also possible to compute in one pass the sand-covered range; that should be optimal.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn regolith_reservoir(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(rock_range_chains) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Vec<RockRangeChain>>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => {
            let rock_ranges_with_abyss: RockRangesWithAbyss = rock_range_chains.into();
            rock_ranges_with_abyss.count()
        }
        PuzzlePart::Part2 => {
            let rock_ranges_with_floor: RockRangesWithFloor = rock_range_chains.into();
            rock_ranges_with_floor.count()
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
