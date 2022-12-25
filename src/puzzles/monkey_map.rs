mod crazy_map;
mod edge;
mod map_data;
mod map_tile;
mod travel_instruction;

use std::collections::HashMap;

use crate::{
    common::direction::Direction,
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, section_pair::SectionPair},
    puzzles::monkey_map::{
        crazy_map::{CrazyMap, TransverseCrazyMap},
        map_data::MapData,
        travel_instruction::TravelInstructionSequence,
    },
};

use self::edge::{Edge, GlueOrientation, Glueing};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn monkey_map(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SectionPair(map_data, TravelInstructionSequence(travel_instructions)) = input
        .file_contents
        .parse::<SectionPair<MapData, TravelInstructionSequence>>()?;
    let glueing = match input.puzzle_part {
        PuzzlePart::Part1 => create_opposite_glueing(),
        PuzzlePart::Part2 => create_cube_glueing(),
    };
    let crazy_map = CrazyMap::from(map_data, glueing);
    let mut transverser = crazy_map.transverse();
    for instruction in travel_instructions {
        transverser.follow(instruction);
    }
    let answer = transverser.password();
    Ok(answer.to_string())
}

#[cfg(not(test))]
fn create_opposite_glueing() -> Glueing {
    let mut map: HashMap<Edge, (Edge, GlueOrientation)> = HashMap::new();

    map.insert(
        Edge::new(1, 0, Direction::Left),
        (Edge::new(2, 0, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 0, Direction::Right),
        (Edge::new(1, 0, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(1, 1, Direction::Left),
        (Edge::new(1, 1, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 1, Direction::Right),
        (Edge::new(1, 1, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 2, Direction::Left),
        (Edge::new(1, 2, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 2, Direction::Right),
        (Edge::new(0, 2, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 3, Direction::Left),
        (Edge::new(0, 3, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 3, Direction::Right),
        (Edge::new(0, 3, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 2, Direction::Up),
        (Edge::new(0, 3, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 3, Direction::Down),
        (Edge::new(0, 2, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(1, 0, Direction::Up),
        (Edge::new(1, 2, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 2, Direction::Down),
        (Edge::new(1, 0, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(2, 0, Direction::Up),
        (Edge::new(2, 0, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 0, Direction::Down),
        (Edge::new(2, 0, Direction::Up), GlueOrientation::Aligned),
    );

    Glueing {
        map,
        resolution: 50,
    }
}

#[cfg(not(test))]
fn create_cube_glueing() -> Glueing {
    let mut map: HashMap<Edge, (Edge, GlueOrientation)> = HashMap::new();

    map.insert(
        Edge::new(1, 0, Direction::Up),
        (Edge::new(0, 3, Direction::Left), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 3, Direction::Left),
        (Edge::new(1, 0, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(2, 0, Direction::Up),
        (Edge::new(0, 3, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 3, Direction::Down),
        (Edge::new(2, 0, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(2, 0, Direction::Right),
        (Edge::new(1, 2, Direction::Right), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(1, 2, Direction::Right),
        (Edge::new(2, 0, Direction::Right), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(2, 0, Direction::Down),
        (Edge::new(1, 1, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 1, Direction::Right),
        (Edge::new(2, 0, Direction::Down), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(1, 2, Direction::Down),
        (Edge::new(0, 3, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 3, Direction::Right),
        (Edge::new(1, 2, Direction::Down), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 2, Direction::Left),
        (Edge::new(1, 0, Direction::Left), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(1, 0, Direction::Left),
        (Edge::new(0, 2, Direction::Left), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(0, 2, Direction::Up),
        (Edge::new(1, 1, Direction::Left), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 1, Direction::Left),
        (Edge::new(0, 2, Direction::Up), GlueOrientation::Aligned),
    );

    Glueing {
        map,
        resolution: 50,
    }
}

#[cfg(test)]
fn create_opposite_glueing() -> Glueing {
    let mut map: HashMap<Edge, (Edge, GlueOrientation)> = HashMap::new();

    map.insert(
        Edge::new(2, 0, Direction::Left),
        (Edge::new(2, 0, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 0, Direction::Right),
        (Edge::new(2, 0, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 1, Direction::Left),
        (Edge::new(2, 1, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 1, Direction::Right),
        (Edge::new(0, 1, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(2, 2, Direction::Left),
        (Edge::new(3, 2, Direction::Right), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(3, 2, Direction::Right),
        (Edge::new(2, 2, Direction::Left), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(0, 1, Direction::Up),
        (Edge::new(0, 1, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(0, 1, Direction::Down),
        (Edge::new(0, 1, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(1, 1, Direction::Up),
        (Edge::new(1, 1, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(1, 1, Direction::Down),
        (Edge::new(1, 1, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(2, 0, Direction::Up),
        (Edge::new(2, 2, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 2, Direction::Down),
        (Edge::new(2, 0, Direction::Up), GlueOrientation::Aligned),
    );

    map.insert(
        Edge::new(3, 2, Direction::Up),
        (Edge::new(3, 2, Direction::Down), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(3, 2, Direction::Down),
        (Edge::new(3, 2, Direction::Up), GlueOrientation::Aligned),
    );

    Glueing { map, resolution: 4 }
}

#[cfg(test)]
fn create_cube_glueing() -> Glueing {
    let mut map: HashMap<Edge, (Edge, GlueOrientation)> = HashMap::new();

    map.insert(
        Edge::new(2, 0, Direction::Up),
        (Edge::new(0, 1, Direction::Up), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(0, 1, Direction::Up),
        (Edge::new(2, 0, Direction::Up), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(2, 0, Direction::Right),
        (Edge::new(3, 2, Direction::Right), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(3, 2, Direction::Right),
        (Edge::new(2, 0, Direction::Right), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(2, 1, Direction::Right),
        (Edge::new(3, 2, Direction::Up), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(3, 2, Direction::Up),
        (Edge::new(2, 1, Direction::Right), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(3, 2, Direction::Down),
        (Edge::new(0, 1, Direction::Left), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(0, 1, Direction::Left),
        (Edge::new(3, 2, Direction::Down), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(2, 2, Direction::Down),
        (Edge::new(0, 1, Direction::Down), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(0, 1, Direction::Down),
        (Edge::new(2, 2, Direction::Down), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(2, 2, Direction::Left),
        (Edge::new(1, 1, Direction::Down), GlueOrientation::Opposite),
    );
    map.insert(
        Edge::new(1, 1, Direction::Down),
        (Edge::new(2, 2, Direction::Left), GlueOrientation::Opposite),
    );

    map.insert(
        Edge::new(1, 1, Direction::Up),
        (Edge::new(2, 0, Direction::Left), GlueOrientation::Aligned),
    );
    map.insert(
        Edge::new(2, 0, Direction::Left),
        (Edge::new(1, 1, Direction::Up), GlueOrientation::Aligned),
    );

    Glueing { map, resolution: 4 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = monkey_map(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("6032", output);
        Ok(())
    }
    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = monkey_map(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("5031", output);
        Ok(())
    }
}
