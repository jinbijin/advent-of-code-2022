mod mill;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    puzzles::grove_positioning_system::mill::Mill,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn grove_positioning_system(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let key: i64 = match input.puzzle_part {
        PuzzlePart::Part1 => 1,
        PuzzlePart::Part2 => 811589153,
    };
    let mix_count: i64 = match input.puzzle_part {
        PuzzlePart::Part1 => 1,
        PuzzlePart::Part2 => 10,
    };

    let SingleSection(numbers) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Vec<i64>>>()?;
    let decrypted = numbers.into_iter().map(|x| x * key).collect::<Vec<i64>>();
    let mill: Mill<i64> = decrypted.into();
    for _ in 0..mix_count {
        for index in 0..(mill.len()) {
            let value = mill.get_value(index);
            let sign = value.signum();
            if sign < 0 {
                mill.move_value_previous(index, value.abs_diff(0));
            } else if sign > 0 {
                mill.move_value_next(index, value.abs_diff(0));
            }
        }
    }

    let index = if let Some(index) = mill.find_value(0) {
        index
    } else {
        unreachable!();
    };
    let answer = (1..=3)
        .map(|x| mill.get_value_next_from(index, x * 1000))
        .sum::<i64>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
1
2
-3
3
-2
0
4
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = grove_positioning_system(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("3", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = grove_positioning_system(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("1623178306", output);
        Ok(())
    }
}
