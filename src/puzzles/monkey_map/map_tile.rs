#[derive(Debug)]
pub enum ParseMapTileError {
    InvalidTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapTile {
    Empty,
    Wall,
}

impl TryFrom<char> for MapTile {
    type Error = ParseMapTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err(Self::Error::InvalidTile),
        }
    }
}
