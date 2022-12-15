use std::cmp::min;

use crate::{common::position::Position, contents::grid::Grid};

pub enum DirectionFrom {
    Top,
    Bottom,
    Left,
    Right,
}

impl DirectionFrom {
    fn all() -> Vec<DirectionFrom> {
        vec![
            DirectionFrom::Top,
            DirectionFrom::Bottom,
            DirectionFrom::Left,
            DirectionFrom::Right,
        ]
    }
}

pub struct TreetopGrid(pub Grid<1, 0, usize>);

impl TreetopGrid {
    pub fn visible(&self, position: Position<usize>) -> bool {
        DirectionFrom::all()
            .into_iter()
            .any(|from| self.visible_from(position, from))
    }

    pub fn scenic_score(&self, position: Position<usize>) -> usize {
        DirectionFrom::all()
            .into_iter()
            .map(|from| self.visible_tree_count_from(position, from))
            .product()
    }

    fn visible_from(&self, position: Position<usize>, from: DirectionFrom) -> bool {
        let compare_with = self.trees_from_direction(position, from);

        self.is_higher_than(position, &mut compare_with.into_iter())
    }

    fn visible_tree_count_from(&self, position: Position<usize>, from: DirectionFrom) -> usize {
        let compare_with = self.trees_from_direction(position, from);

        self.visible_tree_count(position, compare_with)
    }

    fn trees_from_direction(
        &self,
        position: Position<usize>,
        from: DirectionFrom,
    ) -> Vec<Position<usize>> {
        match from {
            DirectionFrom::Left => (0..position.x)
                .rev()
                .map(|x| Position { x, y: position.y })
                .collect::<Vec<Position<usize>>>(),
            DirectionFrom::Right => ((position.x + 1)..(self.0.width()))
                .map(|x| Position { x, y: position.y })
                .collect::<Vec<Position<usize>>>(),
            DirectionFrom::Top => (0..position.y)
                .rev()
                .map(|y| Position { x: position.x, y })
                .collect::<Vec<Position<usize>>>(),
            DirectionFrom::Bottom => ((position.y + 1)..(self.0.height()))
                .map(|y| Position { x: position.x, y })
                .collect::<Vec<Position<usize>>>(),
        }
    }

    fn is_higher_than<T>(&self, position: Position<usize>, compare_with: &mut T) -> bool
    where
        T: Iterator<Item = Position<usize>>,
    {
        compare_with.all(|p| self.0.get_value(position) > self.0.get_value(p))
    }

    fn visible_tree_count(
        &self,
        position: Position<usize>,
        compare_with: Vec<Position<usize>>,
    ) -> usize {
        let count = compare_with.len();
        let visible_trees = compare_with
            .into_iter()
            .take_while(|p| self.0.get_value(position) > self.0.get_value(*p))
            .count()
            + 1;
        min(count, visible_trees)
    }
}
