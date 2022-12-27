use std::{
    cmp,
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

pub enum ParseRockRangeChainError {
    InvalidPoint { input: String },
    NotEnoughPoints,
    NotHorizontal { from: Position, to: Position },
}

impl Display for ParseRockRangeChainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPoint { input } => write!(f, "'{}' is not a point", input),
            Self::NotEnoughPoints => write!(f, "rock range chain is too short"),
            Self::NotHorizontal { from, to } => write!(
                f,
                "({}, {}) to ({}, {}) is not a horizontal or vertical rock range",
                from.x, from.y, to.x, to.y
            ),
        }
    }
}

impl Debug for ParseRockRangeChainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseRockRangeChainError {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RangeIncl {
    pub start: usize,
    pub end: usize,
}

impl RangeIncl {
    fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RockRange {
    Horizontal { x: RangeIncl, y: usize },
    Vertical { x: usize, y: RangeIncl },
}

impl RockRange {
    pub fn bottom(&self) -> usize {
        match self {
            Self::Horizontal { x: _, y } => *y,
            Self::Vertical {
                x: _,
                y: RangeIncl { start: _, end },
            } => *end,
        }
    }

    fn occupies(&self, position: Position) -> bool {
        match self {
            Self::Horizontal { x, y } => x.contains(position.x) && *y == position.y,
            Self::Vertical { x, y } => *x == position.x && y.contains(position.y),
        }
    }
}

pub struct RockRangeChain(pub Vec<RockRange>);

impl FromStr for RockRangeChain {
    type Err = ParseRockRangeChainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(" -> ")
            .map(|part| match part.split_once(',') {
                None => Err(Self::Err::InvalidPoint {
                    input: part.to_string(),
                }),
                Some((x, y)) => match (x.parse::<usize>(), y.parse::<usize>()) {
                    (Ok(x), Ok(y)) => Ok(Position { x, y }),
                    _ => Err(Self::Err::InvalidPoint {
                        input: part.to_string(),
                    }),
                },
            })
            .collect::<Result<Vec<Position>, Self::Err>>()?;

        let mut coords = coords.into_iter();
        let mut rock_ranges: Vec<RockRange> = Vec::new();
        let mut previous = match coords.next() {
            Some(position) => Ok(position),
            None => Err(Self::Err::NotEnoughPoints),
        }?;
        let mut maybe_current = coords.next();
        while let Some(current) = maybe_current {
            if previous.x == current.x && previous.y != current.y {
                rock_ranges.push(RockRange::Vertical {
                    x: current.x,
                    y: RangeIncl {
                        start: cmp::min(previous.y, current.y),
                        end: cmp::max(previous.y, current.y),
                    },
                });
            } else if previous.x != current.x && previous.y == current.y {
                rock_ranges.push(RockRange::Horizontal {
                    x: RangeIncl {
                        start: cmp::min(previous.x, current.x),
                        end: cmp::max(previous.x, current.x),
                    },
                    y: current.y,
                });
            } else {
                return Err(Self::Err::NotHorizontal {
                    from: previous,
                    to: current,
                });
            }

            previous = current;
            maybe_current = coords.next();
        }

        Ok(RockRangeChain(rock_ranges))
    }
}

pub struct RockRangesWithAbyss {
    depth: usize,
    rock_ranges: HashSet<RockRange>,
    grains_of_sand: Vec<Position>,
}

impl RockRangesWithAbyss {
    fn drop(&self, position: Position) -> Option<Position> {
        let Position { x, y } = position;

        if y > self.depth {
            return None;
        }

        let direct_down = Position { x, y: y + 1 };
        if !self.occupies(direct_down) {
            return Some(direct_down);
        }

        let down_left = Position { x: x - 1, y: y + 1 };
        if !self.occupies(down_left) {
            return Some(down_left);
        }

        let down_right = Position { x: x + 1, y: y + 1 };
        if !self.occupies(down_right) {
            return Some(down_right);
        }

        None
    }

    fn occupies(&self, position: Position) -> bool {
        self.rock_ranges
            .iter()
            .any(|rock_range| rock_range.occupies(position))
            || self
                .grains_of_sand
                .iter()
                .any(|grain_of_sand| *grain_of_sand == position)
    }
}

impl From<Vec<RockRangeChain>> for RockRangesWithAbyss {
    fn from(chains: Vec<RockRangeChain>) -> Self {
        let rock_ranges = chains
            .into_iter()
            .flat_map(|RockRangeChain(rock_ranges)| rock_ranges)
            .collect::<HashSet<RockRange>>();
        let depth = rock_ranges
            .iter()
            .map(|rock_range| rock_range.bottom())
            .max()
            .map_or(0, |x| x);
        RockRangesWithAbyss {
            depth,
            rock_ranges,
            grains_of_sand: Vec::new(),
        }
    }
}

impl Iterator for RockRangesWithAbyss {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let mut previous = Position { x: 500, y: 0 };
        let mut maybe_current = self.drop(previous);
        while let Some(current) = maybe_current {
            previous = current;
            maybe_current = self.drop(previous);
        }

        if previous.y < self.depth {
            self.grains_of_sand.push(previous);
            Some(previous)
        } else {
            None
        }
    }
}
