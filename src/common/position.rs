use std::{
    hash::Hash,
    ops::{Add, Sub},
};

use super::direction::Direction;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position<T>
where
    T: Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T>,
{
    pub x: T,
    pub y: T,
}

impl<T> Position<T>
where
    T: Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T>,
{
    pub fn manhattan_distance(&self, other: Position<T>) -> T {
        let zero = self.x - self.x;
        let mut diff_x = self.x - other.x;
        let mut diff_y = self.y - other.y;
        if diff_x < zero {
            diff_x = zero - diff_x;
        }
        if diff_y < zero {
            diff_y = zero - diff_y;
        }
        diff_x + diff_y
    }
}

impl Position<isize> {
    pub fn cone(&self, direction: Direction) -> Vec<Position<isize>> {
        match direction {
            Direction::Up => (-1..=1)
                .map(|x| Position {
                    x: self.x + x,
                    y: self.y - 1,
                })
                .collect::<Vec<Position<isize>>>(),
            Direction::Down => (-1..=1)
                .map(|x| Position {
                    x: self.x + x,
                    y: self.y + 1,
                })
                .collect::<Vec<Position<isize>>>(),
            Direction::Left => (-1..=1)
                .map(|y| Position {
                    x: self.x - 1,
                    y: self.y + y,
                })
                .collect::<Vec<Position<isize>>>(),
            Direction::Right => (-1..=1)
                .map(|y| Position {
                    x: self.x + 1,
                    y: self.y + y,
                })
                .collect::<Vec<Position<isize>>>(),
        }
    }
}

impl<T> Add<Position<T>> for Position<T>
where
    T: Copy + Hash + Ord + Add<T, Output = T> + Sub<T, Output = T>,
{
    type Output = Position<T>;

    fn add(self, rhs: Position<T>) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Direction> for Position<isize> {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}
