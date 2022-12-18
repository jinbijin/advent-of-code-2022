use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    str::FromStr,
};

use crate::common::increment::{Decrement, Increment};

use super::direction3::{Direction3, Direction3Kind};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Position3<T>
where
    T: Hash + Copy + PartialEq + Eq,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Position3<T>
where
    T: Hash + Copy + PartialEq + Eq + Increment + Decrement,
{
    pub fn faces(self) -> [Surface3<T>; 6] {
        Direction3::all().map(|direction| Surface3 {
            position: self,
            direction,
        })
    }

    pub fn opposite_corner(&self, first: &Direction3, second: &Direction3) -> Option<Self> {
        match (first, second) {
            (Direction3::X(kind_x), Direction3::Y(kind_y)) => Some(Position3 {
                x: kind_x.action()(self.x),
                y: kind_y.action()(self.y),
                z: self.z,
            }),
            (Direction3::Y(kind_y), Direction3::X(kind_x)) => Some(Position3 {
                x: kind_x.action()(self.x),
                y: kind_y.action()(self.y),
                z: self.z,
            }),
            (Direction3::X(kind_x), Direction3::Z(kind_z)) => Some(Position3 {
                x: kind_x.action()(self.x),
                y: self.y,
                z: kind_z.action()(self.z),
            }),
            (Direction3::Z(kind_z), Direction3::X(kind_x)) => Some(Position3 {
                x: kind_x.action()(self.x),
                y: self.y,
                z: kind_z.action()(self.z),
            }),
            (Direction3::Y(kind_y), Direction3::Z(kind_z)) => Some(Position3 {
                x: self.x,
                y: kind_y.action()(self.y),
                z: kind_z.action()(self.z),
            }),
            (Direction3::Z(kind_z), Direction3::Y(kind_y)) => Some(Position3 {
                x: self.x,
                y: kind_y.action()(self.y),
                z: kind_z.action()(self.z),
            }),
            _ => None,
        }
    }
}

pub enum ParsePosition3Error<T>
where
    T: FromStr,
{
    InvalidFormat,
    InvalidCoordinate { error: T::Err },
}

impl<T> Display for ParsePosition3Error<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCoordinate { error } => error.fmt(f),
            Self::InvalidFormat => write!(f, "invalid format for Position3"),
        }
    }
}

impl<T> Debug for ParsePosition3Error<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl<T> Error for ParsePosition3Error<T>
where
    T: FromStr,
    T::Err: Display,
{
}

impl<T> FromStr for Position3<T>
where
    T: Hash + Copy + PartialEq + Eq + FromStr,
{
    type Err = ParsePosition3Error<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|part| {
                part.trim()
                    .parse::<T>()
                    .map_err(|error| ParsePosition3Error::InvalidCoordinate { error })
            })
            .collect::<Result<Vec<T>, Self::Err>>()?;
        if parts.len() == 3 {
            Ok(Position3 {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            Err(Self::Err::InvalidFormat)
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Surface3<T>
where
    T: Hash + Copy + PartialEq + Eq,
{
    pub position: Position3<T>,
    pub direction: Direction3,
}

impl<T> Surface3<T>
where
    T: Hash + Copy + PartialEq + Eq + Increment + Decrement,
{
    pub fn adjacent(self) -> Vec<Self> {
        match self.direction {
            Direction3::X(kind) => {
                let y_adjacent = Direction3Kind::all().map(|kind_1| Surface3 {
                    position: Position3 {
                        x: self.position.x,
                        y: kind_1.action()(self.position.y),
                        z: self.position.z,
                    },
                    direction: Direction3::X(kind),
                });
                let z_adjacent = Direction3Kind::all().map(|kind_2| Surface3 {
                    position: Position3 {
                        x: self.position.x,
                        y: self.position.y,
                        z: kind_2.action()(self.position.z),
                    },
                    direction: Direction3::X(kind),
                });
                [y_adjacent, z_adjacent].concat()
            }
            Direction3::Y(kind) => {
                let x_adjacent = Direction3Kind::all().map(|kind_1| Surface3 {
                    position: Position3 {
                        x: kind_1.action()(self.position.x),
                        y: self.position.y,
                        z: self.position.z,
                    },
                    direction: Direction3::Y(kind),
                });
                let z_adjacent = Direction3Kind::all().map(|kind_2| Surface3 {
                    position: Position3 {
                        x: self.position.x,
                        y: self.position.y,
                        z: kind_2.action()(self.position.z),
                    },
                    direction: Direction3::Y(kind),
                });
                [x_adjacent, z_adjacent].concat()
            }
            Direction3::Z(kind) => {
                let x_adjacent = Direction3Kind::all().map(|kind_1| Surface3 {
                    position: Position3 {
                        x: kind_1.action()(self.position.x),
                        y: self.position.y,
                        z: self.position.z,
                    },
                    direction: Direction3::Z(kind),
                });
                let y_adjacent = Direction3Kind::all().map(|kind_2| Surface3 {
                    position: Position3 {
                        x: self.position.x,
                        y: kind_2.action()(self.position.y),
                        z: self.position.z,
                    },
                    direction: Direction3::Z(kind),
                });
                [x_adjacent, y_adjacent].concat()
            }
        }
    }

    pub fn opposite(self) -> Self {
        match self.direction {
            Direction3::X(kind) => Surface3 {
                position: Position3 {
                    x: kind.action()(self.position.x),
                    y: self.position.y,
                    z: self.position.z,
                },
                direction: Direction3::X(kind.opposite()),
            },
            Direction3::Y(kind) => Surface3 {
                position: Position3 {
                    x: self.position.x,
                    y: kind.action()(self.position.y),
                    z: self.position.z,
                },
                direction: Direction3::Y(kind.opposite()),
            },
            Direction3::Z(kind) => Surface3 {
                position: Position3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: kind.action()(self.position.z),
                },
                direction: Direction3::Z(kind.opposite()),
            },
        }
    }
}
