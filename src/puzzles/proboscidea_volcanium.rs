mod pair_crawler;
mod pair_decision;
mod solo_crawler;
mod solo_decision;
mod target_valve;
mod valve;
mod valve_system;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::{
    pair_crawler::AsPairCrawler, solo_crawler::AsSoloCrawler, valve::Valve,
    valve_system::ValveSystem,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn proboscidea_volcanium(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(valves) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<Vec<Valve>>>()?;
    let valve_system: ValveSystem = valves.into();
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => valve_system.solo_crawler().max().map_or(0, |x| x),
        PuzzlePart::Part2 => valve_system.pair_crawler().max().map_or(0, |x| x),
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = proboscidea_volcanium(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("1651", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = proboscidea_volcanium(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("1707", output);
        Ok(())
    }
}
