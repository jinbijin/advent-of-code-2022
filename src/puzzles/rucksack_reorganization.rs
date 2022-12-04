mod lib;

use std::{fmt::Display, vec::IntoIter};

use crate::{
    file::{self, FileErrorCollection},
    input::puzzle_input::PuzzleInput,
};

use self::lib::{Rucksack, VectorChunkIterator};

use crate::input::puzzle_part::PuzzlePart;

pub fn main(input: PuzzleInput) -> Result<Box<dyn Display>, FileErrorCollection> {
    let rucksacks = file::parse_lines::<Rucksack>(input.file_contents)?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => rucksacks
            .into_iter()
            .map(|rucksack| lib::find_common_item(rucksack.compartments()).priority())
            .sum::<i32>(),
        PuzzlePart::Part2 => VectorChunkIterator::<3, Rucksack, IntoIter<Rucksack>> {
            iterator: &mut rucksacks.into_iter(),
        }
        .map(|group| lib::find_common_item(group).priority())
        .sum::<i32>(),
    };

    Ok(Box::new(answer))
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
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("157", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("70", output.to_string());
        Ok(())
    }
}
