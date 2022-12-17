use crate::common::{direction::Direction, position::Position};

use super::{
    rock::{RockShape, RockShapeKind, SettledRocks},
    rock_shift::{RockShift, RockShiftCollection},
};

pub struct SimulationResult {
    pub shape_cycle_position: usize,
    pub shift_cycle_position: usize,
    pub x: usize,
    pub iteration_count: usize,
    pub height_from: usize,
    pub height_to: usize,
}

pub struct RockSimulator {
    shape_cycle_position: usize,
    shift_cycle_position: usize,
    shift_cycle_length: usize,
    iteration_count: usize,
    current_height: usize,
    shapes: Box<dyn Iterator<Item = RockShape>>,
    shifts: Box<dyn Iterator<Item = RockShift>>,
    settled_rocks: SettledRocks,
}

impl Iterator for RockSimulator {
    type Item = SimulationResult; // current top

    fn next(&mut self) -> Option<Self::Item> {
        let shape_before = self.shape_cycle_position;
        let shift_before = self.shift_cycle_position;
        let iteration_before = self.iteration_count;
        let height_before = self.current_height;

        let top = self.settled_rocks.top();
        let next_shape = match self.next_shape() {
            Some(next_shape) => next_shape,
            None => unreachable!("as these iterators are infinite"),
        };

        let mut rock = next_shape.spawn(Position {
            x: 2,
            y: 3 + top + next_shape.height() - 1,
        });
        let mut shifted = true;

        while shifted {
            let next_shift = match self.next_shift() {
                Some(next_shift) => next_shift,
                None => unreachable!("as these iterators are infinite"),
            };

            rock.shift(next_shift.into(), &self.settled_rocks);
            shifted = rock.shift(Direction::Down, &self.settled_rocks);
        }

        let new_height = rock.top() + 1;
        let new_x = rock.x();
        self.current_height = new_height;
        self.iteration_count += 1;
        self.settled_rocks.push(rock);

        Some(SimulationResult {
            shape_cycle_position: shape_before,
            shift_cycle_position: shift_before,
            x: new_x,
            iteration_count: iteration_before,
            height_from: height_before,
            height_to: new_height,
        })
    }
}

impl RockSimulator {
    fn next_shape(&mut self) -> Option<RockShape> {
        self.shape_cycle_position = (self.shape_cycle_position + 1) % 5;
        self.shapes.next()
    }

    fn next_shift(&mut self) -> Option<RockShift> {
        self.shift_cycle_position = (self.shift_cycle_position + 1) % self.shift_cycle_length;
        self.shifts.next()
    }
}

pub trait AsRockSimulator {
    fn as_rock_simulator(&self) -> RockSimulator;
}

impl AsRockSimulator for RockShiftCollection {
    fn as_rock_simulator(&self) -> RockSimulator {
        let shapes =
            (0..).flat_map(move |_| RockShapeKind::all().into_iter().map(|kind| kind.create()));
        let shapes = Box::new(shapes);

        let shift_cycle_length = self.0.len();
        let shifts = self.0.clone();
        let shifts = (0..).flat_map(move |_| shifts.clone().into_iter());
        let shifts = Box::new(shifts);

        let settled_rocks = SettledRocks::new();

        RockSimulator {
            shape_cycle_position: 0,
            shift_cycle_position: 0,
            shift_cycle_length,
            current_height: 0,
            iteration_count: 0,
            shapes,
            shifts,
            settled_rocks,
        }
    }
}
