mod elevation_grid;

use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::error::ParseContentsError,
};

use self::elevation_grid::{ElevationGrid, ElevationGridTransversalResult, TransversalMode};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// TODO
// wasm version behaves in a weird way:
// runtime increases linearly with consecutive run count?
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn hill_climbing_algorithm(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let transversal_mode = match input.puzzle_part {
        PuzzlePart::Part1 => TransversalMode::FromStart,
        PuzzlePart::Part2 => TransversalMode::FromLowest,
    };
    let elevation_grid = input.file_contents.parse::<ElevationGrid>()?;
    let mut transverser = elevation_grid.start_transversal(transversal_mode);
    let mut result = transverser.step();
    while ElevationGridTransversalResult::Continue == result {
        result = transverser.step();
    }
    match result {
        ElevationGridTransversalResult::DistanceFound(distance) => Ok(distance.to_string()),
        ElevationGridTransversalResult::NoPath => Ok(format!("none")),
        ElevationGridTransversalResult::Continue => unreachable!("due to loop condition"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = hill_climbing_algorithm(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("31", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = hill_climbing_algorithm(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("29", output);
        Ok(())
    }
}
