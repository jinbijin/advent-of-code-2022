mod crate_cell;
mod crate_stacks;
mod move_instruction;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use self::{
    crate_cell::CrateCell,
    crate_stacks::{CrateStacks, MoveMode},
    move_instruction::MoveInstruction,
};
use crate::{
    contents::{
        convert::contents::{AsParseContents, ParseContentsError, SectionPair},
        grid::Grid,
    },
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn supply_stacks(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SectionPair(grid, instructions) = input
        .file_contents
        .as_str()
        .parse_contents::<SectionPair<Grid<3, 1, CrateCell>, Vec<MoveInstruction>>>()?;
    let mut crate_stacks: CrateStacks = grid.into();
    let move_mode = match input.puzzle_part {
        PuzzlePart::Part1 => MoveMode::OneByOne,
        PuzzlePart::Part2 => MoveMode::AllAtOnce,
    };

    crate_stacks.perform_instructions(&instructions, move_mode);

    Ok(crate_stacks.get_stack_tops())
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
