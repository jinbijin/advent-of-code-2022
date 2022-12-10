mod rope_motion;

use std::collections::HashSet;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use self::rope_motion::{Direction, RopeMotion, RopePosition, RopePositionCollector};
use crate::{
    contents::{convert::AsParseLines, errors::ParseContentsError},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn rope_bridge(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let motions = input
        .file_contents
        .as_str()
        .parse_lines::<Vec<RopeMotion>>()?;
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

    const INPUT_TEXT_1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    const INPUT_TEXT_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = rope_bridge(PuzzleInput {
            file_contents: INPUT_TEXT_1.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("13", output);
        Ok(())
    }

    #[test]
    fn example_2_1() -> Result<(), Box<dyn Error>> {
        let output = rope_bridge(PuzzleInput {
            file_contents: INPUT_TEXT_1.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("1", output);
        Ok(())
    }

    #[test]
    fn example_2_2() -> Result<(), Box<dyn Error>> {
        let output = rope_bridge(PuzzleInput {
            file_contents: INPUT_TEXT_2.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("36", output);
        Ok(())
    }
}
