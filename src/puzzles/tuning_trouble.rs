use std::collections::{HashSet, VecDeque};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::ParseContentsError,
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

fn deque_marks_start(deque: &VecDeque<char>, marker_len: usize) -> bool {
    let mut hash_set: HashSet<char> = HashSet::new();
    deque.iter().for_each(|c| {
        hash_set.insert(*c);
    });
    deque.len() == marker_len && hash_set.len() == marker_len
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn tuning_trouble(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let marker_len = match input.puzzle_part {
        PuzzlePart::Part1 => 4,
        PuzzlePart::Part2 => 14,
    };
    let mut chars = input.file_contents.chars();
    let mut deque: VecDeque<char> = VecDeque::new();
    let mut index: usize = 0;
    while !deque_marks_start(&deque, marker_len) {
        if let Some(c) = chars.next() {
            index += 1;
            deque.push_back(c);
            if deque.len() > marker_len {
                deque.pop_front();
            }
        } else {
            break;
        }
    }
    Ok(index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_TEXT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_TEXT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_TEXT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_TEXT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn example_1_1() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_1.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("7", output);
        Ok(())
    }

    #[test]
    fn example_1_2() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_2.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("5", output);
        Ok(())
    }

    #[test]
    fn example_1_3() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_3.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("6", output);
        Ok(())
    }

    #[test]
    fn example_1_4() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_4.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("10", output);
        Ok(())
    }

    #[test]
    fn example_1_5() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_5.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("11", output);
        Ok(())
    }

    #[test]
    fn example_2_1() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_1.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("19", output);
        Ok(())
    }

    #[test]
    fn example_2_2() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_2.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("23", output);
        Ok(())
    }

    #[test]
    fn example_2_3() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_3.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("23", output);
        Ok(())
    }

    #[test]
    fn example_2_4() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_4.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("29", output);
        Ok(())
    }

    #[test]
    fn example_2_5() -> Result<(), Box<dyn Error>> {
        let output = tuning_trouble(PuzzleInput {
            file_contents: INPUT_TEXT_5.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("26", output);
        Ok(())
    }
}
