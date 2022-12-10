mod lib;

use crate::common::vector_chunks::AsVectorChunks;
use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use self::lib::Rucksack;

use crate::input::puzzle_part::PuzzlePart;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn rucksack_reorganization(input: PuzzleInput) -> Result<String, FileErrorCollection> {
    let rucksacks = file::parse_lines::<Rucksack>(input.file_contents)?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => rucksacks
            .into_iter()
            .map(|rucksack| lib::find_common_item(rucksack.compartments()).priority())
            .sum::<i32>(),
        PuzzlePart::Part2 => rucksacks
            .into_iter()
            .vector_chunks::<3>()
            .map(|group| lib::find_common_item(group).priority())
            .sum::<i32>(),
    };

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT_TEXT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = rucksack_reorganization(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("157", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = rucksack_reorganization(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("70", output.to_string());
        Ok(())
    }
}
