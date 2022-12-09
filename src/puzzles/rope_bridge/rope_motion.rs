use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseRopeMotionError {
    InvalidFormat,
    InvalidDirection(String),
    InvalidCount(String),
}

impl Display for ParseRopeMotionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidDirection(value) => write!(f, "invalid direction '{}'", value),
            Self::InvalidCount(value) => write!(f, "invalid count '{}'", value),
        }
    }
}

impl Debug for ParseRopeMotionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Error for ParseRopeMotionError {}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn relative_position(&self) -> RopePosition {
        match self {
            Self::Up => RopePosition { x: 0, y: 1 },
            Self::Right => RopePosition { x: 1, y: 0 },
            Self::Down => RopePosition { x: 0, y: -1 },
            Self::Left => RopePosition { x: -1, y: 0 },
        }
    }
}

#[derive(Clone, Copy)]
pub struct RopeMotion {
    pub direction: Direction,
    pub count: usize,
}

impl FromStr for RopeMotion {
    type Err = ParseRopeMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        if tokens.len() != 2 {
            return Err(ParseRopeMotionError::InvalidFormat);
        }

        let direction_token = tokens[0];
        let direction = match direction_token {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(ParseRopeMotionError::InvalidDirection(
                direction_token.to_string(),
            )),
        }?;

        let count_token = tokens[1];
        let count = count_token
            .parse::<usize>()
            .map_err(|_| ParseRopeMotionError::InvalidCount(count_token.to_string()))?;

        Ok(RopeMotion { direction, count })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RopePosition {
    pub x: i32,
    pub y: i32,
}

impl RopePosition {
    fn move_to(&mut self, other: RopePosition) -> () {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;
        if (diff_x == 2 || diff_x == -2) && (diff_y == 2 || diff_y == -2) {
            self.x += diff_x / 2;
            self.y += diff_y / 2;
        } else if diff_x == 2 || diff_x == -2 {
            self.x += diff_x / 2;
            self.y += diff_y;
        } else if diff_y == 2 || diff_y == -2 {
            self.y += diff_y / 2;
            self.x += diff_x;
        };
    }
}

pub struct RopePositionCollector<T, const N: usize>
where
    T: Iterator<Item = Direction>,
{
    source: T,
    knots: [RopePosition; N],
    tail_positions: HashSet<RopePosition>,
}

impl<T, const N: usize> RopePositionCollector<T, N>
where
    T: Iterator<Item = Direction>,
{
    fn process_direction(&mut self, direction: Direction) -> () {
        self.process_head_motion(direction);
        self.process_tail_motion();
    }

    fn process_head_motion(&mut self, direction: Direction) -> () {
        let relative = direction.relative_position();
        self.knots[0].x += relative.x;
        self.knots[0].y += relative.y;
    }

    fn process_tail_motion(&mut self) -> () {
        for i in 1..N {
            self.knots[i].move_to(self.knots[i - 1]);
        }
        self.tail_positions.insert(self.knots[N - 1]);
    }
}

impl<T, const N: usize> RopePositionCollector<T, N>
where
    T: Iterator<Item = Direction>,
{
    pub fn new(iter: T) -> RopePositionCollector<T, N> {
        RopePositionCollector {
            source: iter,
            knots: [RopePosition { x: 0, y: 0 }; N],
            tail_positions: HashSet::new(),
        }
    }
}

impl<T, const N: usize> Iterator for RopePositionCollector<T, N>
where
    T: Iterator<Item = Direction>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(direction) = self.source.next() {
            Some(self.process_direction(direction))
        } else {
            None
        }
    }
}

impl<T, const N: usize> From<RopePositionCollector<T, N>> for HashSet<RopePosition>
where
    T: Iterator<Item = Direction>,
{
    fn from(mut collector: RopePositionCollector<T, N>) -> Self {
        while Some(()) == collector.next() {}
        collector.tail_positions
    }
}
