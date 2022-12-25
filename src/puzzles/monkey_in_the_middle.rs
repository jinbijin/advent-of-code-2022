mod divisor;
mod if_false_throw_to;
mod if_true_throw_to;
mod monkey;
mod monkey_name;
mod operation;
mod starting_items;

use self::monkey::MonkeyCollection;
use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::error::ParseContentsError,
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn monkey_in_the_middle(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let relieved_after_inspection = input.puzzle_part == PuzzlePart::Part1;
    let round_count = match input.puzzle_part {
        PuzzlePart::Part1 => 20,
        PuzzlePart::Part2 => 10000,
    };
    let mut monkey_collection = input.file_contents.parse::<MonkeyCollection>()?;
    for _ in 0..round_count {
        monkey_collection.round(relieved_after_inspection);
    }
    let throw_counts = monkey_collection.get_sorted_throw_counts();
    let answer = throw_counts[0] * throw_counts[1];
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = monkey_in_the_middle(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("10605", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = monkey_in_the_middle(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("2713310158", output);
        Ok(())
    }
}
