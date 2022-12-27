use std::collections::HashMap;

use crate::common::interval::{Interval, IntervalUnion};

use super::rock_range::{RockRange, RockRangeChain};

pub struct CaveIn {
    depth: usize,
    rock_ranges: HashMap<usize, IntervalUnion<usize>>,
}

impl From<Vec<RockRangeChain>> for CaveIn {
    fn from(chains: Vec<RockRangeChain>) -> Self {
        let mut rock_ranges: HashMap<usize, IntervalUnion<usize>> = HashMap::new();
        let mut depth = 0;

        for RockRangeChain(ranges) in chains {
            for range in ranges {
                match range {
                    RockRange::Horizontal { x, y } => {
                        if y > depth {
                            depth = y
                        };
                        if let Some(rock_range) = rock_ranges.get_mut(&y) {
                            rock_range.add(&Interval::build(x.start, x.end + 1).unwrap());
                        } else {
                            let mut rock_range = IntervalUnion::new();
                            rock_range.add(&Interval::build(x.start, x.end + 1).unwrap());
                            rock_ranges.insert(y, rock_range);
                        }
                    }
                    RockRange::Vertical { x, y } => {
                        if y.end > depth {
                            depth = y.end
                        };
                        for y in (y.start)..=(y.end) {
                            if let Some(rock_range) = rock_ranges.get_mut(&y) {
                                rock_range.add(&Interval::build(x, x + 1).unwrap());
                            } else {
                                let mut rock_range = IntervalUnion::new();
                                rock_range.add(&Interval::build(x, x + 1).unwrap());
                                rock_ranges.insert(y, rock_range);
                            }
                        }
                    }
                }
            }
        }

        CaveIn { depth, rock_ranges }
    }
}

pub struct IntoFloor<'a> {
    cave_in: &'a CaveIn,
    sand_ranges: Vec<IntervalUnion<usize>>,
}

impl<'a> IntoFloor<'a> {
    pub fn fill_with_sand(mut self) -> usize {
        for _ in 0..(self.cave_in.depth + 1) {
            self.step();
        }

        self.sand_ranges
            .into_iter()
            .map(|interval| interval.count())
            .sum::<usize>()
    }

    fn step(&mut self) {
        let current_depth = self.sand_ranges.len();
        let previous_sand_range = self.sand_ranges.last().unwrap();
        let mut next_sand_range = previous_sand_range.expand(1);
        if let Some(rock_range) = self.cave_in.rock_ranges.get(&current_depth) {
            next_sand_range.remove(rock_range);
        }
        self.sand_ranges.push(next_sand_range);
    }
}

pub trait AsIntoFloor {
    fn into_floor(&self) -> IntoFloor;
}

impl AsIntoFloor for CaveIn {
    fn into_floor(&self) -> IntoFloor {
        let initial_interval = Interval::build(500, 501).unwrap();
        let mut initial_range: IntervalUnion<usize> = IntervalUnion::new();
        initial_range.add(&initial_interval);
        IntoFloor {
            cave_in: self,
            sand_ranges: vec![initial_range],
        }
    }
}
