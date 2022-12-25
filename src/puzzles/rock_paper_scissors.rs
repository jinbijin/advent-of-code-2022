mod mapping;
mod rps_match;
mod scorable;
mod strategy;

use self::{scorable::Scorable, strategy::RpsStrategy};
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
    let interpretation = match input.puzzle_part {
        PuzzlePart::Part1 => rps_match::match_with_target_as_type,
        PuzzlePart::Part2 => rps_match::match_with_target_as_result,
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
