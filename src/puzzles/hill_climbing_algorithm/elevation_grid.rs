use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

use crate::parse::error::ParseContentsError;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
enum ParseGridCellError {
    InvalidValue(char),
}

#[derive(Debug, Clone, Copy)]
enum GridCell {
    Start,
    End,
    Empty { elevation: usize },
}

impl TryFrom<char> for GridCell {
    type Error = ParseGridCellError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'S' {
            Ok(Self::Start)
        } else if value == 'E' {
            Ok(Self::End)
        } else if value >= 'a' && value <= 'z' {
            let elevation = value as usize - 'a' as usize;
            Ok(Self::Empty { elevation })
        } else {
            Err(Self::Error::InvalidValue(value))
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum ParseElevationGridError {
    EmptyGrid,
    UnequalWidths,
    InvalidCell {
        x: usize,
        y: usize,
        value: char,
    },
    MissingStartCell,
    MissingEndCell,
    NonUniqueStartCell {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
    NonUniqueEndCell {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    },
}

impl Display for ParseElevationGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGrid => write!(f, "input is empty"),
            Self::UnequalWidths => write!(f, "input is not a grid"),
            Self::InvalidCell { x, y, value } => {
                write!(f, "cell at ({}, {}) has invalid value '{}'", x, y, value)
            }
            Self::MissingStartCell => write!(f, "input does not contain a starting cell"),
            Self::MissingEndCell => write!(f, "input does not contain an ending cell"),
            Self::NonUniqueStartCell { x1, y1, x2, y2 } => write!(
                f,
                "multiple starting cells encountered, at ({}, {}) and ({}, {})",
                x1, y1, x2, y2
            ),
            Self::NonUniqueEndCell { x1, y1, x2, y2 } => write!(
                f,
                "multiple ending cells encountered, at ({}, {}) and ({}, {})",
                x1, y1, x2, y2
            ),
        }
    }
}

impl Debug for ParseElevationGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseElevationGridError {}

impl From<ParseElevationGridError> for ParseContentsError {
    fn from(value: ParseElevationGridError) -> Self {
        ParseContentsError::new(value)
    }
}

pub enum TransversalMode {
    FromStart,
    FromLowest,
}

pub struct ElevationGrid {
    pub width: usize,
    pub height: usize,
    contents: Vec<usize>,
    pub start: Position,
    pub end: Position,
}

impl ElevationGrid {
    pub fn start_transversal<'a>(
        &'a self,
        transversal_mode: TransversalMode,
    ) -> ElevationGridTransverser<'a> {
        let mut distances: Vec<Option<usize>> = vec![None; self.width * self.height];
        match transversal_mode {
            TransversalMode::FromStart => {
                distances[self.index(self.start)] = Some(0);
            }
            TransversalMode::FromLowest => {
                for position in self
                    .positions()
                    .into_iter()
                    .filter(|p| self.elevation(*p) == 0)
                {
                    distances[self.index(position)] = Some(0);
                }
            }
        }
        ElevationGridTransverser {
            source: self,
            last_distance: 0,
            distances,
        }
    }

    fn positions(&self) -> Vec<Position> {
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| Position { x, y }))
            .collect::<Vec<Position>>()
    }

    fn position_adjacent(&self, position: Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                if position.y >= 1 {
                    Some(Position {
                        x: position.x,
                        y: position.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if position.x < self.width - 1 {
                    Some(Position {
                        x: position.x + 1,
                        y: position.y,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if position.y < self.height - 1 {
                    Some(Position {
                        x: position.x,
                        y: position.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if position.x >= 1 {
                    Some(Position {
                        x: position.x - 1,
                        y: position.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn elevation(&self, position: Position) -> usize {
        self.contents[self.index(position)]
    }

    fn index(&self, position: Position) -> usize {
        self.width * position.y + position.x
    }
}

impl FromStr for ElevationGrid {
    type Err = ParseElevationGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let height = lines.len();
        if height == 0 {
            return Err(Self::Err::EmptyGrid);
        }

        let width = lines[0].chars().count();
        if lines.iter().any(|line| line.chars().count() != width) {
            return Err(Self::Err::UnequalWidths);
        }

        let mut contents: Vec<usize> = Vec::new();
        let mut start: Option<Position> = None;
        let mut end: Option<Position> = None;

        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let result: Result<GridCell, ParseGridCellError> = c.try_into();
                let result = match result {
                    Ok(GridCell::Start) => match start {
                        Some(Position { x: x1, y: y1 }) => Err(Self::Err::NonUniqueStartCell {
                            x1,
                            y1,
                            x2: x,
                            y2: y,
                        }),
                        None => {
                            start = Some(Position { x, y });
                            Ok(0)
                        }
                    },
                    Ok(GridCell::End) => match end {
                        Some(Position { x: x1, y: y1 }) => Err(Self::Err::NonUniqueEndCell {
                            x1,
                            y1,
                            x2: x,
                            y2: y,
                        }),
                        None => {
                            end = Some(Position { x, y });
                            Ok(25)
                        }
                    },
                    Ok(GridCell::Empty { elevation }) => Ok(elevation),
                    Err(ParseGridCellError::InvalidValue(value)) => {
                        Err(Self::Err::InvalidCell { x, y, value })
                    }
                }?;
                contents.push(result);
            }
        }

        let start = match start {
            Some(start) => Ok(start),
            None => Err(Self::Err::MissingStartCell),
        }?;
        let end = match end {
            Some(end) => Ok(end),
            None => Err(Self::Err::MissingEndCell),
        }?;

        Ok(ElevationGrid {
            width,
            height,
            contents,
            start,
            end,
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ElevationGridTransversalResult {
    Continue,
    DistanceFound(usize),
    NoPath,
}

pub struct ElevationGridTransverser<'a> {
    source: &'a ElevationGrid,
    last_distance: usize,
    distances: Vec<Option<usize>>,
}

impl<'a> ElevationGridTransverser<'a> {
    pub fn step(&mut self) -> ElevationGridTransversalResult {
        let target_positions = self
            .source
            .positions()
            .into_iter()
            .filter(|p| self.distance(*p).is_none() && self.is_adjacent_to_known_distance(*p))
            .collect::<Vec<Position>>();
        let new_cell_count = target_positions.len();
        for position in target_positions {
            self.set_distance(position);
            if position == self.source.end {
                return ElevationGridTransversalResult::DistanceFound(self.last_distance + 1);
            }
        }
        self.last_distance += 1;
        if new_cell_count > 0 {
            ElevationGridTransversalResult::Continue
        } else {
            ElevationGridTransversalResult::NoPath
        }
    }

    fn is_adjacent_to_known_distance(&self, position: Position) -> bool {
        for direction in Direction::all() {
            if let Some(adjacent_position) = self.source.position_adjacent(position, direction) {
                if self.distance(adjacent_position) == Some(self.last_distance)
                    && self.source.elevation(position)
                        <= self.source.elevation(adjacent_position) + 1
                {
                    return true;
                }
            }
        }
        false
    }

    fn distance(&self, position: Position) -> Option<usize> {
        self.distances[self.source.index(position)]
    }

    fn set_distance(&mut self, position: Position) {
        self.distances[self.source.index(position)] = Some(self.last_distance + 1);
    }
}
