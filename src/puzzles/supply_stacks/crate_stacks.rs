use std::collections::HashMap;

use crate::{common::position::Position, parse::grid::Grid};

use super::{crate_cell::CrateCell, move_instruction::MoveInstruction};

#[derive(Clone, Copy)]
pub enum MoveMode {
    OneByOne,
    AllAtOnce,
}

pub struct CrateStacks {
    stack_map: Vec<char>,
    stacks: HashMap<char, Vec<char>>,
}

impl From<Grid<3, 1, CrateCell>> for CrateStacks {
    fn from(grid: Grid<3, 1, CrateCell>) -> Self {
        let mut stack_map: Vec<char> = Vec::new();
        let mut stacks: HashMap<char, Vec<char>> = HashMap::new();

        for x in 0..(grid.width()) {
            let key = if let CrateCell::Stack(key) = grid.get_value(Position {
                x,
                y: grid.height() - 1,
            }) {
                *key
            } else {
                unreachable!()
            };

            stack_map.push(key);

            let crates: Vec<char> = (0..(grid.height() - 1))
                .rev()
                .map(|y| grid.get_value(Position { x, y }))
                .filter_map(|cell| match cell {
                    CrateCell::Empty => None,
                    CrateCell::Crate(x) => Some(*x),
                    CrateCell::Stack(_) => unreachable!(),
                })
                .collect::<Vec<char>>();

            stacks.insert(key, crates);
        }

        CrateStacks { stack_map, stacks }
    }
}

impl CrateStacks {
    pub fn perform_instructions(
        &mut self,
        instructions: &Vec<MoveInstruction>,
        move_mode: MoveMode,
    ) {
        for instruction in instructions {
            self.perform_instruction(instruction, move_mode);
        }
    }

    pub fn get_stack_tops(&self) -> String {
        self.stack_map
            .iter()
            .filter_map(|c| self.stacks.get(c))
            .filter_map(|stack| stack.last())
            .collect::<String>()
    }

    fn perform_instruction(&mut self, instruction: &MoveInstruction, move_mode: MoveMode) {
        let mut buffer: Vec<char> = Vec::new();

        if let Some(source) = self.stacks.get_mut(&instruction.from) {
            for _ in 0..(instruction.count) {
                if let Some(x) = source.pop() {
                    buffer.push(x);
                }
            }
        }

        if let Some(target) = self.stacks.get_mut(&instruction.to) {
            match move_mode {
                MoveMode::OneByOne => {
                    for item in buffer.into_iter() {
                        target.push(item);
                    }
                }
                MoveMode::AllAtOnce => {
                    for item in buffer.into_iter().rev() {
                        target.push(item);
                    }
                }
            }
        }
    }
}
