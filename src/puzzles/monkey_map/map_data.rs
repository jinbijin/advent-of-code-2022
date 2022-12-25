use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::common::position::Position;

use super::map_tile::{MapTile, ParseMapTileError};

#[derive(Debug)]
pub enum ParseMapError {
    InvalidTile,
}

impl From<ParseMapTileError> for ParseMapError {
    fn from(_: ParseMapTileError) -> Self {
        Self::InvalidTile
    }
}

impl Display for ParseMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTile => write!(f, "invalid tile"),
        }
    }
}

impl Error for ParseMapError {}

pub struct MapData(pub HashMap<Position<usize>, MapTile>);

impl FromStr for MapData {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashMap<Position<usize>, MapTile> = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    let tile: MapTile = c.try_into()?;
                    tiles.insert(Position { x, y }, tile);
                }
            }
        }

        Ok(MapData(tiles))
    }
}
