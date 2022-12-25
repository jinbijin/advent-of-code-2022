use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::{
    common::{direction::Direction, position::Position},
    parse::error::ParseContentsError,
};

use super::basin_tile::{BasinTile, ParseBasinTileError};

#[derive(Debug)]
pub enum ParseBasinStateError {
    InvalidTile,
}

impl From<ParseBasinTileError> for ParseBasinStateError {
    fn from(_: ParseBasinTileError) -> Self {
        Self::InvalidTile
    }
}

impl Display for ParseBasinStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTile => write!(f, "invalid tile"),
        }
    }
}

impl Error for ParseBasinStateError {}

impl From<ParseBasinStateError> for ParseContentsError {
    fn from(value: ParseBasinStateError) -> Self {
        ParseContentsError::new(value)
    }
}

pub struct BlizzardState {
    direction: Direction,
    position: Position<isize>,
}

pub struct BasinState {
    width: isize,
    height: isize,
    expedition: HashSet<Position<isize>>,
    wall: HashSet<Position<isize>>,
    blizzard: Vec<BlizzardState>,
    trip_count: usize,
}

impl FromStr for BasinState {
    type Err = ParseBasinStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width: isize = 0;
        let mut height: isize = 0;
        let mut expedition: HashSet<Position<isize>> = HashSet::new();
        let mut wall: HashSet<Position<isize>> = HashSet::new();
        let mut blizzard: Vec<BlizzardState> = Vec::new();
        let mut first_empty = true;

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;

                if x + 1 > width {
                    width = x + 1
                };
                if y + 1 > height {
                    height = y + 1
                };

                let tile: BasinTile = c.try_into()?;
                match tile {
                    BasinTile::Empty => {
                        if first_empty {
                            expedition.insert(Position { x, y });
                            first_empty = false;
                        }
                    }
                    BasinTile::Wall => {
                        wall.insert(Position { x, y });
                    }
                    BasinTile::Blizzard(direction) => {
                        blizzard.push(BlizzardState {
                            direction,
                            position: Position { x, y },
                        });
                    }
                }
            }
        }

        Ok(BasinState {
            width,
            height,
            expedition,
            wall,
            blizzard,
            trip_count: 0,
        })
    }
}

impl BasinState {
    pub fn step(&mut self) -> usize {
        let reached_end = self.spread_expedition();
        self.wreak_havoc();
        if reached_end {
            self.reset_expedition();
        }
        self.trip_count
    }

    /// Expand the expedition. If the exit is found, returns true, else returns false.
    fn spread_expedition(&mut self) -> bool {
        let elves = self
            .expedition
            .iter()
            .map(|x| *x)
            .collect::<Vec<Position<isize>>>();

        for elf in elves {
            for direction in Direction::all() {
                let position = elf + direction.into();
                if position.x >= 0
                    && position.x < self.width
                    && position.y >= 0
                    && position.y < self.height
                    && !self.wall.contains(&position)
                {
                    self.expedition.insert(position);
                    if (self.trip_count % 2 == 0 && position.y == self.height - 1)
                        || (self.trip_count % 2 == 1 && position.y == 0)
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Moves the blizzard and...
    fn wreak_havoc(&mut self) {
        for blizzard in self.blizzard.iter_mut() {
            let mut next_position = blizzard.position + blizzard.direction.into();
            if next_position.x < 1 {
                next_position.x = self.width - 2;
            } else if next_position.x > self.width - 2 {
                next_position.x = 1;
            } else if next_position.y < 1 {
                next_position.y = self.height - 2;
            } else if next_position.y > self.height - 2 {
                next_position.y = 1;
            }
            blizzard.position = next_position;

            self.expedition.remove(&next_position);
        }
    }

    fn reset_expedition(&mut self) {
        let elves_to_remove = if self.trip_count % 2 == 0 {
            self.expedition
                .iter()
                .filter_map(|elf| {
                    if elf.y < self.height - 1 {
                        Some(*elf)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Position<isize>>>()
        } else {
            self.expedition
                .iter()
                .filter_map(|elf| if elf.y > 0 { Some(*elf) } else { None })
                .collect::<Vec<Position<isize>>>()
        };
        for elf in elves_to_remove {
            self.expedition.remove(&elf);
        }
        self.trip_count += 1;
    }
}
