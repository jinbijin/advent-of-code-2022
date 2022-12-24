mod blueprint;
mod collection_state;
mod collector;
mod resource_type;
mod resources;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::{blueprint::Blueprint, collector::AsCollector};

// PERF: Instead of this depth-first approach, I should probably do a breadth-first search;
// Storing results in a `HashMap<Resources, HashSet<Resources>>` (mapping from production levels to possible stored resource values leading to it)
// in combination with the bounds on useful production should make iteration *really fast*.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn not_enough_minerals(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(blueprints) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Vec<Blueprint>>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => blueprints
            .into_iter()
            .filter_map(|b| {
                dbg!(b.id);
                match b.collector(24).max() {
                    Some(max) => Some(b.id * max),
                    None => None,
                }
            })
            .sum::<usize>(),
        PuzzlePart::Part2 => blueprints
            .into_iter()
            .take(take_count())
            .filter_map(|b| {
                dbg!(b.id);
                b.collector(32).max()
            })
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
