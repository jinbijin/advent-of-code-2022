use std::collections::HashSet;

use crate::common::three_d::{
    direction3::{Direction3, Direction3Kind},
    position3::{Position3, Surface3},
};

pub struct BoulderCollection {
    boulders: HashSet<Position3<isize>>,
    surface: HashSet<Surface3<isize>>,
}

impl BoulderCollection {
    pub fn face_count(&self) -> usize {
        self.surface.len()
    }

    pub fn external_face_count(&self) -> usize {
        if self.surface.len() == 0 {
            return 0;
        }

        let max_x = if let Some(max_x) = self.surface.iter().map(|s| s.position.x).max() {
            max_x
        } else {
            unreachable!("by check on self.surface.len()");
        };

        let initial_face = if let Some(face) = self.surface.iter().find(|s| {
            s.position.x == max_x && s.direction == Direction3::X(Direction3Kind::Increment)
        }) {
            *face
        } else {
            unreachable!("by check on self.surface.len()");
        };

        let mut external_surface: HashSet<Surface3<isize>> = HashSet::new();
        let mut must_continue = true;
        external_surface.insert(initial_face);

        while must_continue {
            let attached_faces = self
                .surface
                .iter()
                .filter(|face| {
                    !external_surface.contains(face)
                        && external_surface
                            .iter()
                            .any(|external_face| self.is_attached_to(face, external_face))
                })
                .collect::<Vec<&Surface3<isize>>>();

            for attached_face in attached_faces.iter() {
                external_surface.insert(**attached_face);
            }

            must_continue = attached_faces.len() > 0;
        }

        external_surface.len()
    }

    fn is_attached_to(&self, first: &Surface3<isize>, second: &Surface3<isize>) -> bool {
        if first.position == second.position
            && first.direction != second.direction
            && first.direction != second.direction.opposite()
        {
            let opposite_corner = if let Some(position) = first
                .position
                .opposite_corner(&first.direction, &second.direction)
            {
                position
            } else {
                unreachable!("we just checked that")
            };

            return !self.boulders.contains(&opposite_corner);
        }

        if first
            .adjacent()
            .iter()
            .any(|x| x.position == second.position && x.direction == second.direction)
        {
            return true;
        }

        if first.opposite().position == second.opposite().position
            && first.direction != second.direction
            && first.direction != second.direction.opposite()
        {
            return true;
        }

        false
    }
}

impl From<Vec<Position3<isize>>> for BoulderCollection {
    fn from(positions: Vec<Position3<isize>>) -> Self {
        let mut boulders: HashSet<Position3<isize>> = HashSet::new();
        let mut surface: HashSet<Surface3<isize>> = HashSet::new();

        for position in positions {
            let is_new = boulders.insert(position);
            if is_new {
                for face in position.faces() {
                    if surface.contains(&face.opposite()) {
                        surface.remove(&face.opposite());
                    } else {
                        surface.insert(face);
                    }
                }
            }
        }

        BoulderCollection { boulders, surface }
    }
}
