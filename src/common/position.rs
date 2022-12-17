use std::{
    hash::Hash,
    ops::{Add, Sub},
};

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
