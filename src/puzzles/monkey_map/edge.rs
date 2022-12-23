use std::collections::HashMap;

use crate::common::{direction::Direction, position::Position};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    pub position: Position<usize>,
    pub direction: Direction,
}

impl Edge {
    pub fn new(x: usize, y: usize, direction: Direction) -> Edge {
        Edge {
            position: Position { x, y },
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GlueOrientation {
    Aligned,
    Opposite,
}

#[derive(Debug, Clone)]
pub struct Glueing {
    pub map: HashMap<Edge, (Edge, GlueOrientation)>,
    pub resolution: usize,
}
