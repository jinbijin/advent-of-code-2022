mod boulder_collection;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    common::three_d::position3::Position3,
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::boulder_collection::BoulderCollection;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn boiling_boulders(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(positions) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Vec<Position3<isize>>>>()?;
    let boulder_collection: BoulderCollection = positions.into();
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => boulder_collection.face_count(),
        PuzzlePart::Part2 => boulder_collection.external_face_count(),
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = boiling_boulders(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("64", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = boiling_boulders(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("58", output);
        Ok(())
    }
}
