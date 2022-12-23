use std::collections::HashMap;

use crate::common::{direction::Direction, position::Position};

use super::{
    edge::{Edge, GlueOrientation, Glueing},
    map_data::MapData,
    map_tile::MapTile,
    travel_instruction::TravelInstruction,
};

pub struct CrazyMap {
    tiles: HashMap<Position<usize>, MapTile>,
    glueing: Glueing,
}

impl CrazyMap {
    pub fn from(MapData(tiles): MapData, glueing: Glueing) -> Self {
        CrazyMap { tiles, glueing }
    }
}

pub struct TransverserState {
    position: Position<usize>,
    direction: Direction,
}

impl TransverserState {
    fn password(&self) -> usize {
        1000 * (self.position.y + 1) + 4 * (self.position.x + 1) + self.direction.code()
    }
}

pub struct CrazyMapTransverser<'a> {
    map: &'a CrazyMap,
    state: TransverserState,
}

impl<'a> CrazyMapTransverser<'a> {
    pub fn password(&self) -> usize {
        self.state.password()
    }

    pub fn follow(&mut self, travel_instruction: TravelInstruction) {
        match travel_instruction {
            TravelInstruction::Move(distance) => {
                for _ in 0..distance {
                    self.step();
                }
            }
            TravelInstruction::TurnLeft => self.state.direction = self.state.direction.turn_left(),
            TravelInstruction::TurnRight => {
                self.state.direction = self.state.direction.turn_right()
            }
        }
    }

    fn step(&mut self) {
        let next_tile = self.next_tile();
        if self.get_tile(next_tile.position) == Some(MapTile::Empty) {
            self.state = next_tile;
        }
    }

    fn next_tile(&self) -> TransverserState {
        match self.state.direction {
            Direction::Up => self.next_tile_up(),
            Direction::Left => self.next_tile_left(),
            Direction::Down => self.next_tile_down(),
            Direction::Right => self.next_tile_right(),
        }
    }

    fn next_tile_up(&self) -> TransverserState {
        let Position { x, y } = self.state.position;
        let resolution = self.map.glueing.resolution;
        match self.map.glueing.map.keys().find(|edge| {
            edge.position.x == x / resolution
                && edge.position.y == y / resolution
                && edge.direction == Direction::Up
                && y % resolution == 0
        }) {
            Some(edge) => {
                let (other_edge, orientation) = match self.map.glueing.map.get(edge) {
                    Some(edge) => edge,
                    None => unreachable!(),
                };
                TransverserState {
                    position: self.tile_at_edge(*other_edge, *orientation, x % resolution),
                    direction: other_edge.direction.opposite(),
                }
            }
            None => TransverserState {
                position: Position { x, y: y - 1 },
                direction: self.state.direction,
            },
        }
    }

    fn next_tile_down(&self) -> TransverserState {
        let Position { x, y } = self.state.position;
        let resolution = self.map.glueing.resolution;
        match self.map.glueing.map.keys().find(|edge| {
            edge.position.x == x / resolution
                && edge.position.y == y / resolution
                && edge.direction == Direction::Down
                && y % resolution == resolution - 1
        }) {
            Some(edge) => {
                let (other_edge, orientation) = match self.map.glueing.map.get(edge) {
                    Some(edge) => edge,
                    None => unreachable!(),
                };
                TransverserState {
                    position: self.tile_at_edge(*other_edge, *orientation, x % resolution),
                    direction: other_edge.direction.opposite(),
                }
            }
            None => TransverserState {
                position: Position { x, y: y + 1 },
                direction: self.state.direction,
            },
        }
    }

    fn next_tile_left(&self) -> TransverserState {
        let Position { x, y } = self.state.position;
        let resolution = self.map.glueing.resolution;
        match self.map.glueing.map.keys().find(|edge| {
            edge.position.x == x / resolution
                && edge.position.y == y / resolution
                && edge.direction == Direction::Left
                && x % resolution == 0
        }) {
            Some(edge) => {
                let (other_edge, orientation) = match self.map.glueing.map.get(edge) {
                    Some(edge) => edge,
                    None => unreachable!(),
                };
                TransverserState {
                    position: self.tile_at_edge(*other_edge, *orientation, y % resolution),
                    direction: other_edge.direction.opposite(),
                }
            }
            None => TransverserState {
                position: Position { x: x - 1, y },
                direction: self.state.direction,
            },
        }
    }

    fn next_tile_right(&self) -> TransverserState {
        let Position { x, y } = self.state.position;
        let resolution = self.map.glueing.resolution;
        match self.map.glueing.map.keys().find(|edge| {
            edge.position.x == x / resolution
                && edge.position.y == y / resolution
                && edge.direction == Direction::Right
                && x % resolution == resolution - 1
        }) {
            Some(edge) => {
                let (other_edge, orientation) = match self.map.glueing.map.get(edge) {
                    Some(edge) => edge,
                    None => unreachable!(),
                };
                TransverserState {
                    position: self.tile_at_edge(*other_edge, *orientation, y % resolution),
                    direction: other_edge.direction.opposite(),
                }
            }
            None => TransverserState {
                position: Position { x: x + 1, y },
                direction: self.state.direction,
            },
        }
    }

    fn tile_at_edge(
        &self,
        edge: Edge,
        orientation: GlueOrientation,
        index: usize,
    ) -> Position<usize> {
        let resolution = self.map.glueing.resolution;
        let corrected_index = match orientation {
            GlueOrientation::Aligned => index,
            GlueOrientation::Opposite => resolution - 1 - index,
        };
        let base = Position {
            x: edge.position.x * resolution,
            y: edge.position.y * resolution,
        };
        let offset = match edge.direction {
            Direction::Up => Position {
                x: corrected_index,
                y: 0,
            },
            Direction::Left => Position {
                x: 0,
                y: corrected_index,
            },
            Direction::Down => Position {
                x: corrected_index,
                y: resolution - 1,
            },
            Direction::Right => Position {
                x: resolution - 1,
                y: corrected_index,
            },
        };

        Position {
            x: base.x + offset.x,
            y: base.y + offset.y,
        }
    }

    fn get_tile(&self, position: Position<usize>) -> Option<MapTile> {
        self.map.tiles.get(&position).copied()
    }
}

pub trait TransverseCrazyMap {
    fn transverse(&self) -> CrazyMapTransverser;
}

impl TransverseCrazyMap for CrazyMap {
    fn transverse(&self) -> CrazyMapTransverser {
        let x = match self
            .tiles
            .iter()
            .filter_map(|(p, _)| if p.y == 0 { Some(p.x) } else { None })
            .min()
        {
            Some(min) => min,
            None => unreachable!(),
        };
        CrazyMapTransverser {
            map: self,
            state: TransverserState {
                position: Position { x, y: 0 },
                direction: Direction::Right,
            },
        }
    }
}
