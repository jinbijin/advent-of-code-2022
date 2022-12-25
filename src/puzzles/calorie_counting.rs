use crate::{
    common::collection::max_items::AsMaxItems,
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines, sections::BySections},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::parse::native::Native;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn calorie_counting_validate(input: JsValue) -> JsValue {
    let input: String = serde_wasm_bindgen::from_value(input).unwrap();
    let result = match input.parse::<BySections<ByLines<Native<u64>>>>() {
        Ok(_) => None,
        Err(error) => Some(error),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn calorie_counting(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let BySections(calorie_counts) = input.file_contents.parse::<BySections<ByLines<u64>>>()?;
    let count = match input.puzzle_part {
        PuzzlePart::Part1 => 1,
        PuzzlePart::Part2 => 3,
    };

    let answer = calorie_counts
        .into_iter()
        .map(|ByLines(group)| group.into_iter().sum::<u64>())
        .max_items(count)
        .into_iter()
        .sum::<u64>();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("24000", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("45000", output);
        Ok(())
    }
}
