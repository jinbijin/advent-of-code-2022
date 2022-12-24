use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Formatter},
};

use crate::common::{direction::Direction, position::Position};

use super::elf_distribution::ElfDistribution;

pub struct ElfDiffuser {
    elves: HashSet<Position<isize>>,
    direction_order: VecDeque<Direction>,
}

impl Debug for ElfDiffuser {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "State:")?;
        for y in -2..=9 {
            for x in -3..=10 {
                if self.elves.contains(&Position { x, y }) {
                    write!(f, "#")
                } else {
                    write!(f, ".")
                }?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl ElfDiffuser {
    pub fn covered_ground(&self) -> isize {
        let count = self.elves.len() as isize;
        let min_x = match self.elves.iter().map(|p| p.x).min() {
            Some(x) => x,
            None => unreachable!(),
        };
        let max_x = match self.elves.iter().map(|p| p.x).max() {
            Some(x) => x,
            None => unreachable!(),
        };
        let min_y = match self.elves.iter().map(|p| p.y).min() {
            Some(y) => y,
            None => unreachable!(),
        };
        let max_y = match self.elves.iter().map(|p| p.y).max() {
            Some(y) => y,
            None => unreachable!(),
        };

        (max_x - min_x + 1) * (max_y - min_y + 1) - count
    }

    pub fn diffuse(&mut self) -> Option<()> {
        let proposed_positions = self.propose_moves();
        let moved_count = self.perform_moves(proposed_positions);
        self.alter_directions();

        if moved_count > 0 {
            Some(())
        } else {
            None
        }
    }

    /// The propose move phase; returns a map mapping a position to the positions of elves who propose to move there.
    fn propose_moves(&self) -> HashMap<Position<isize>, Vec<Position<isize>>> {
        let mut reverse_index: HashMap<Position<isize>, Vec<Position<isize>>> = HashMap::new();

        for elf in self.elves.iter() {
            if self.is_near_other_elf(elf) {
                if let Some(proposed_position) = self
                    .direction_order
                    .iter()
                    .filter_map(|direction| self.try_propose_move(elf, *direction))
                    .find(|_| true)
                {
                    if let Some(proposers) = reverse_index.get_mut(&proposed_position) {
                        proposers.push(*elf);
                    } else {
                        reverse_index.insert(proposed_position, vec![*elf]);
                    }
                }
            }
        }

        reverse_index
    }

    fn is_near_other_elf(&self, elf: &Position<isize>) -> bool {
        (-1..=1)
            .flat_map(|x: isize| {
                (-1..=1).filter_map(move |y: isize| {
                    if x != 0 || y != 0 {
                        Some(Position { x, y })
                    } else {
                        None
                    }
                })
            })
            .any(|p| self.elves.contains(&(*elf + p)))
    }

    fn try_propose_move(
        &self,
        elf: &Position<isize>,
        direction: Direction,
    ) -> Option<Position<isize>> {
        if elf
            .cone(direction)
            .into_iter()
            .any(|p| self.elves.contains(&p))
        {
            None
        } else {
            Some(*elf + direction.into())
        }
    }

    /// Perform the proposed moves; returns the number of elves that moved.
    fn perform_moves(
        &mut self,
        reverse_index: HashMap<Position<isize>, Vec<Position<isize>>>,
    ) -> usize {
        let moves = reverse_index
            .into_iter()
            .filter(|(_, from)| from.len() == 1)
            .filter_map(|(to, from)| match from.first() {
                Some(from) => Some((to, *from)),
                None => None,
            })
            .collect::<Vec<(Position<isize>, Position<isize>)>>();

        for (to, from) in moves.iter() {
            self.elves.remove(from);
            self.elves.insert(*to);
        }

        moves.len()
    }

    /// Move the first considered direction to the back of the queue.
    fn alter_directions(&mut self) {
        let direction = match self.direction_order.pop_front() {
            Some(direction) => direction,
            None => unreachable!(),
        };
        self.direction_order.push_back(direction);
    }
}

pub trait AsDiffuser {
    fn diffuser(self) -> ElfDiffuser;
}

impl AsDiffuser for ElfDistribution {
    fn diffuser(self) -> ElfDiffuser {
        let mut direction_order: VecDeque<Direction> = VecDeque::new();
        direction_order.push_back(Direction::Up);
        direction_order.push_back(Direction::Down);
        direction_order.push_back(Direction::Left);
        direction_order.push_back(Direction::Right);
        ElfDiffuser {
            elves: self.0,
            direction_order,
        }
    }
}
