use crate::common::increment::{Decrement, Increment};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Direction3Kind {
    Increment,
    Decrement,
}

impl Direction3Kind {
    pub fn all() -> [Direction3Kind; 2] {
        [Direction3Kind::Increment, Direction3Kind::Decrement]
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction3Kind::Increment => Direction3Kind::Decrement,
            Direction3Kind::Decrement => Direction3Kind::Increment,
        }
    }

    pub fn action<T>(self) -> Box<dyn Fn(T) -> T>
    where
        T: Increment + Decrement,
    {
        match self {
            Direction3Kind::Increment => Box::new(|x| x.increment()),
            Direction3Kind::Decrement => Box::new(|x| x.decrement()),
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Direction3 {
    X(Direction3Kind),
    Y(Direction3Kind),
    Z(Direction3Kind),
}

impl Direction3 {
    pub fn all() -> [Direction3; 6] {
        [
            Direction3::X(Direction3Kind::Increment),
            Direction3::X(Direction3Kind::Decrement),
            Direction3::Y(Direction3Kind::Increment),
            Direction3::Y(Direction3Kind::Decrement),
            Direction3::Z(Direction3Kind::Increment),
            Direction3::Z(Direction3Kind::Decrement),
        ]
    }

    pub fn opposite(self) -> Direction3 {
        match self {
            Direction3::X(kind) => Direction3::X(kind.opposite()),
            Direction3::Y(kind) => Direction3::Y(kind.opposite()),
            Direction3::Z(kind) => Direction3::Z(kind.opposite()),
        }
    }
}
