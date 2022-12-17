use std::collections::HashSet;

use crate::common::{direction::Direction, interval::Interval, position::Position};

#[derive(Clone, Copy)]
pub enum RockShapeKind {
    Horizontal,
    Cross,
    Angle,
    Vertical,
    Square,
}

impl RockShapeKind {
    pub fn create(self) -> RockShape {
        match self {
            Self::Horizontal => {
                let mut occupied_positions: HashSet<Position<usize>> = HashSet::new();
                occupied_positions.insert(Position { x: 0, y: 0 });
                occupied_positions.insert(Position { x: 1, y: 0 });
                occupied_positions.insert(Position { x: 2, y: 0 });
                occupied_positions.insert(Position { x: 3, y: 0 });

                RockShape {
                    occupied_positions,
                    width: 4,
                    height: 1,
                }
            }
            Self::Cross => {
                let mut occupied_positions: HashSet<Position<usize>> = HashSet::new();
                occupied_positions.insert(Position { x: 1, y: 0 });
                occupied_positions.insert(Position { x: 0, y: 1 });
                occupied_positions.insert(Position { x: 1, y: 1 });
                occupied_positions.insert(Position { x: 2, y: 1 });
                occupied_positions.insert(Position { x: 1, y: 2 });

                RockShape {
                    occupied_positions,
                    width: 3,
                    height: 3,
                }
            }
            Self::Angle => {
                let mut occupied_positions: HashSet<Position<usize>> = HashSet::new();
                occupied_positions.insert(Position { x: 2, y: 0 });
                occupied_positions.insert(Position { x: 2, y: 1 });
                occupied_positions.insert(Position { x: 0, y: 2 });
                occupied_positions.insert(Position { x: 1, y: 2 });
                occupied_positions.insert(Position { x: 2, y: 2 });

                RockShape {
                    occupied_positions,
                    width: 3,
                    height: 3,
                }
            }
            Self::Vertical => {
                let mut occupied_positions: HashSet<Position<usize>> = HashSet::new();
                occupied_positions.insert(Position { x: 0, y: 0 });
                occupied_positions.insert(Position { x: 0, y: 1 });
                occupied_positions.insert(Position { x: 0, y: 2 });
                occupied_positions.insert(Position { x: 0, y: 3 });

                RockShape {
                    occupied_positions,
                    width: 1,
                    height: 4,
                }
            }
            Self::Square => {
                let mut occupied_positions: HashSet<Position<usize>> = HashSet::new();
                occupied_positions.insert(Position { x: 0, y: 0 });
                occupied_positions.insert(Position { x: 0, y: 1 });
                occupied_positions.insert(Position { x: 1, y: 0 });
                occupied_positions.insert(Position { x: 1, y: 1 });

                RockShape {
                    occupied_positions,
                    width: 2,
                    height: 2,
                }
            }
        }
    }

    pub fn all() -> Vec<RockShapeKind> {
        vec![
            RockShapeKind::Horizontal,
            RockShapeKind::Cross,
            RockShapeKind::Angle,
            RockShapeKind::Vertical,
            RockShapeKind::Square,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct RockShape {
    occupied_positions: HashSet<Position<usize>>,
    width: usize,
    height: usize,
}

impl RockShape {
    pub fn spawn(&self, position: Position<usize>) -> Rock {
        Rock {
            shape: self.clone(),
            position,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Debug, Clone)]
pub struct Rock {
    shape: RockShape,
    position: Position<usize>,
}

impl Rock {
    pub fn collides_with(&self, other_rocks: &SettledRocks) -> bool {
        other_rocks
            .0
            .iter()
            .any(|other| self.collides_with_rock(other))
    }

    pub fn shift(&mut self, direction: Direction, other_rocks: &SettledRocks) -> bool {
        match direction {
            Direction::Down => {
                if self.position.y >= self.shape.height {
                    self.position.y -= 1;
                    if self.collides_with(other_rocks) {
                        self.position.y += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Direction::Up => {
                self.position.y += 1;
                if self.collides_with(other_rocks) {
                    self.position.y -= 1;
                    false
                } else {
                    true
                }
            }
            Direction::Left => {
                if self.position.x > 0 {
                    self.position.x -= 1;
                    if self.collides_with(other_rocks) {
                        self.position.x += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Direction::Right => {
                if self.position.x + self.shape.width < 7 {
                    self.position.x += 1;
                    if self.collides_with(other_rocks) {
                        self.position.x -= 1;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
        }
    }

    pub fn top(&self) -> usize {
        self.position.y
    }

    pub fn x(&self) -> usize {
        self.position.x
    }

    fn collides_with_rock(&self, other: &Rock) -> bool {
        self.horizontal_range()
            .overlap(other.horizontal_range())
            .is_some()
            && self
                .vertical_range()
                .overlap(other.vertical_range())
                .is_some()
            && self.collides_with_rock_positions(other)
    }

    fn collides_with_rock_positions(&self, other: &Rock) -> bool {
        self.positions()
            .intersection(&other.positions())
            .any(|_| true)
    }

    fn horizontal_range(&self) -> Interval<usize> {
        match Interval::build(self.position.x, self.position.x + self.shape.width) {
            Ok(interval) => interval,
            Err(_) => unreachable!("because self.shape.width >= 0"),
        }
    }

    fn vertical_range(&self) -> Interval<usize> {
        match Interval::build(self.position.y + 1 - self.shape.height, self.position.y + 1) {
            Ok(interval) => interval,
            Err(_) => unreachable!("because self.shape.height >= 0"),
        }
    }

    fn positions(&self) -> HashSet<Position<usize>> {
        self.shape
            .occupied_positions
            .iter()
            .map(|p| Position {
                x: self.position.x + p.x,
                y: self.position.y - p.y,
            })
            .collect::<HashSet<Position<usize>>>()
    }
}

#[derive(Debug)]
pub struct SettledRocks(Vec<Rock>);

impl SettledRocks {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, rock: Rock) {
        self.0.push(rock)
    }

    pub fn top(&self) -> usize {
        match self.0.iter().map(|rock| rock.top()).max() {
            Some(max) => max + 1,
            None => 0,
        }
    }
}
