mod rps_match;
mod scorable;
mod strategy;

use self::{
    rps_match::{RpsMatch, RpsTargetMap},
    scorable::Scorable,
    strategy::RpsStrategy,
};
use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn rock_paper_scissors_validate(input: JsValue) -> JsValue {
    let input: String = serde_wasm_bindgen::from_value(input).unwrap();
    let result = match input.parse::<ByLines<RpsStrategy>>() {
        Ok(_) => None,
        Err(error) => Some(error),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn rock_paper_scissors(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(strategy) = input.file_contents.parse::<ByLines<RpsStrategy>>()?;
    let rps_target_map = RpsTargetMap::new();
    let interpretation: Box<dyn Fn(RpsStrategy) -> RpsMatch> = match input.puzzle_part {
        PuzzlePart::Part1 => Box::new(|strategy| rps_target_map.map_target_as_type(strategy)),
        PuzzlePart::Part2 => Box::new(|strategy| rps_target_map.map_target_as_result(strategy)),
    };
    let answer = strategy
        .into_iter()
        .map(interpretation)
        .map(|x| x.score())
        .sum::<i32>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT_TEXT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = rock_paper_scissors(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("15", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = rock_paper_scissors(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("12", output);
        Ok(())
    }
}
