mod blueprint;
mod factory;
mod potential_production;
mod potential_resources;
mod resource_type;
mod resources;

use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines},
};

use self::{blueprint::Blueprint, factory::AsFactory};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn not_enough_minerals(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(blueprints) = input.file_contents.parse::<ByLines<Blueprint>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => blueprints
            .into_iter()
            .map(|b| b.id * b.factory().run(24))
            .sum::<usize>(),
        PuzzlePart::Part2 => blueprints
            .into_iter()
            .take(take_count())
            .map(|b| b.factory().run(32))
            .product::<usize>(),
    };
    Ok(answer.to_string())
}

#[cfg(test)]
fn take_count() -> usize {
    1
}

#[cfg(not(test))]
fn take_count() -> usize {
    3
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = not_enough_minerals(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("33", output);
        Ok(())
    }
}
