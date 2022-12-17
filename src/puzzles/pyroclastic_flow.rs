mod rock;
mod rock_shift;
mod rock_simulator;

use std::collections::HashMap;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::{
        contents::{AsParseContents, ParseContentsError, SingleSection},
        sections::SingleLine,
    },
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::{rock_shift::RockShiftCollection, rock_simulator::AsRockSimulator};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn pyroclastic_flow(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(SingleLine(collection)) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<SingleLine<RockShiftCollection>>>()?;
    let simulator = collection.as_rock_simulator();
    let cycles: usize = match input.puzzle_part {
        PuzzlePart::Part1 => 2022,
        PuzzlePart::Part2 => 1000000000000,
    };
    let cycle_info = determine_cycle(&collection);
    let cycle_length = cycle_info.cycle_end - cycle_info.cycle_start;
    let periodic_iterations = cycles - cycle_info.cycle_start;
    let cycle_count = periodic_iterations / cycle_length;
    let cycle_weight = cycle_info.height_after - cycle_info.height_before;
    let height_periodic_part = cycle_count * cycle_weight;
    let tail_cycles = periodic_iterations % cycle_length;
    let non_periodic_cycles = cycle_info.cycle_start + tail_cycles;

    dbg!(&cycle_info);

    let answer = height_periodic_part
        + match simulator
            .take(non_periodic_cycles)
            .map(|x| x.height_to)
            .max()
        {
            Some(max) => max,
            None => 0,
        };
    Ok(answer.to_string())
}

#[derive(Hash, PartialEq, Eq)]
struct CyclePosition {
    shape_cycle_position: usize,
    shift_cycle_position: usize,
    x: usize,
}

struct CycleData {
    iteration_count: usize,
    height_from: usize,
}

#[derive(Debug)]
struct CycleInfo {
    cycle_start: usize,
    cycle_end: usize, // non-inclusive
    height_before: usize,
    height_after: usize,
}

// I don't think this is 100% correct, but it's quick, and does its job for this set of inputs.
fn determine_cycle(collection: &RockShiftCollection) -> CycleInfo {
    let mut results: HashMap<CyclePosition, CycleData> = HashMap::new();

    for result in collection.as_rock_simulator() {
        let position = CyclePosition {
            shape_cycle_position: result.shape_cycle_position,
            shift_cycle_position: result.shift_cycle_position,
            x: result.x,
        };
        match results.get(&position) {
            Some(data) => {
                return CycleInfo {
                    cycle_start: data.iteration_count,
                    cycle_end: result.iteration_count,
                    height_before: data.height_from,
                    height_after: result.height_from,
                };
            }
            None => {
                results.insert(
                    position,
                    CycleData {
                        iteration_count: result.iteration_count,
                        height_from: result.height_from,
                    },
                );
            }
        }
    }

    unreachable!("infinite iterator");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = pyroclastic_flow(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("3068", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = pyroclastic_flow(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("1514285714288", output);
        Ok(())
    }
}
