mod rope_motion;

use std::collections::HashSet;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
    puzzles::rope_bridge::rope_motion::{RopeMotion, RopePosition, RopePositionCollector},
};

use crate::input::puzzle_part::PuzzlePart;

use self::rope_motion::Direction;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn rope_bridge(input: PuzzleInput) -> Result<String, FileErrorCollection> {
    let motions = file::parse_lines::<RopeMotion>(input.file_contents)?;
    let directions = motions
        .into_iter()
        .flat_map(|motion| (0..motion.count).map(move |_| motion.direction))
        .collect::<Vec<Direction>>();
    let tail_positions = match input.puzzle_part {
        PuzzlePart::Part1 => {
            let collector: RopePositionCollector<std::vec::IntoIter<Direction>, 2> =
                RopePositionCollector::new(directions.into_iter());
            let tail_positions: HashSet<RopePosition> = collector.into();
            tail_positions
        }
        PuzzlePart::Part2 => {
            let collector: RopePositionCollector<std::vec::IntoIter<Direction>, 10> =
                RopePositionCollector::new(directions.into_iter());
            let tail_positions: HashSet<RopePosition> = collector.into();
            tail_positions
        }
    };
    let count = tail_positions.len();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = rope_bridge(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("13", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = rope_bridge(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("1", output.to_string());
        Ok(())
    }
}
