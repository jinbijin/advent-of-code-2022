use crate::common::direction::Direction;

pub enum ParseBasinTileError {
    InvalidTile,
}

pub enum BasinTile {
    Wall,
    Empty,
    Blizzard(Direction),
}

impl TryFrom<char> for BasinTile {
    type Error = ParseBasinTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            '>' => Ok(Self::Blizzard(Direction::Right)),
            '<' => Ok(Self::Blizzard(Direction::Left)),
            '^' => Ok(Self::Blizzard(Direction::Up)),
            'v' => Ok(Self::Blizzard(Direction::Down)),
            _ => Err(Self::Error::InvalidTile),
        }
    }
}
