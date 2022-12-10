mod supply_input_line;

use std::{collections::HashMap, str::Lines};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::errors::ParseContentsError,
    file::{FileErrorCollection, FileParseResult},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::supply_input_line::{ParseSupplyInputIterator, SupplyInputLine};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn supply_stacks(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let supply_input_iterator: ParseSupplyInputIterator<Lines> = input.file_contents.lines().into();
    let supply_input: Result<Vec<SupplyInputLine>, FileErrorCollection> = supply_input_iterator
        .collect::<FileParseResult<SupplyInputLine>>()
        .into();
    let supply_input = supply_input?;

    // Help, ugly!
    let mut crate_layer_lines: Vec<Vec<Option<char>>> = Vec::new();
    let mut instructions: Vec<(usize, char, char)> = Vec::new();
    let mut hash_map: HashMap<char, Vec<char>> = HashMap::new();
    let mut stack_mapping: Vec<char> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for supply_line in supply_input.into_iter() {
        match supply_line {
            SupplyInputLine::CrateLayerLine(value) => crate_layer_lines.push(value),
            SupplyInputLine::EmptyLine => {}
            SupplyInputLine::MoveInstructionLine { count, from, to } => {
                instructions.push((count, from, to))
            }
            SupplyInputLine::StackMappingLine(value) => {
                stack_mapping = value.clone();
                stacks = value.iter().map(|_| Vec::new()).collect::<Vec<Vec<char>>>();
            }
        }
    }

    // Initialize stacks
    for crate_layer_line in crate_layer_lines.into_iter().rev() {
        for (index, crate_value) in crate_layer_line.into_iter().enumerate() {
            if let Some(value) = crate_value {
                stacks[index].push(value);
            }
        }
    }

    // Initialize hash map
    for (index, c) in stack_mapping.clone().into_iter().enumerate() {
        hash_map.insert(c, stacks[index].clone());
    }

    // Perform rearrangement
    for (count, from, to) in instructions.into_iter() {
        let mut inter_stack: Vec<char> = Vec::new();
        if let Some(from_stack) = hash_map.get_mut(&from) {
            for _ in 0..count {
                if let Some(c) = from_stack.pop() {
                    inter_stack.push(c);
                }
            }
        }
        if let Some(to_stack) = hash_map.get_mut(&to) {
            match input.puzzle_part {
                PuzzlePart::Part1 => {
                    for c in inter_stack.into_iter() {
                        to_stack.push(c);
                    }
                }
                PuzzlePart::Part2 => {
                    for c in inter_stack.into_iter().rev() {
                        to_stack.push(c);
                    }
                }
            }
        }
    }

    // Collect result
    Ok(stack_mapping
        .into_iter()
        .filter_map(|c| hash_map.get(&c))
        .filter_map(|v| v.last())
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = supply_stacks(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("CMZ", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = supply_stacks(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("MCD", output);
        Ok(())
    }
}
