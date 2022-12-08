use std::{cmp::min, str::FromStr};

use crate::file::{FileErrorCollection, FileParseResult};

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

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

pub struct DigitGrid {
    pub width: usize,
    pub height: usize,
    pub digits: Vec<usize>,
}

impl FromStr for DigitGrid {
    type Err = FileErrorCollection;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let height = lines.len();
        let width = lines[0].len();
        let digits: Result<Vec<usize>, FileErrorCollection> = lines
            .into_iter()
            .flat_map(|line| line.chars().map(|c| c.to_string().parse::<usize>()))
            .collect::<FileParseResult<usize>>()
            .into();
        let digits = digits?;

        Ok(DigitGrid {
            width,
            height,
            digits,
        })
    }
}

impl DigitGrid {
    pub fn visible(&self, coords: Coordinates) -> bool {
        DirectionFrom::all()
            .into_iter()
            .any(|from| self.visible_from(coords, from))
    }

    pub fn scenic_score(&self, coords: Coordinates) -> usize {
        DirectionFrom::all()
            .into_iter()
            .map(|from| self.visible_tree_count_from(coords, from))
            .product()
    }

    fn visible_from(&self, coords: Coordinates, from: DirectionFrom) -> bool {
        let compare_with = self.trees_from_direction(coords, from);

        self.is_higher_than(coords, &mut compare_with.into_iter())
    }

    fn visible_tree_count_from(&self, coords: Coordinates, from: DirectionFrom) -> usize {
        let compare_with = self.trees_from_direction(coords, from);

        self.visible_tree_count(coords, compare_with)
    }

    fn trees_from_direction(&self, coords: Coordinates, from: DirectionFrom) -> Vec<Coordinates> {
        match from {
            DirectionFrom::Left => (0..coords.col)
                .rev()
                .map(|col| Coordinates {
                    row: coords.row,
                    col,
                })
                .collect::<Vec<Coordinates>>(),
            DirectionFrom::Right => ((coords.col + 1)..self.width)
                .map(|col| Coordinates {
                    row: coords.row,
                    col,
                })
                .collect::<Vec<Coordinates>>(),
            DirectionFrom::Top => (0..coords.row)
                .rev()
                .map(|row| Coordinates {
                    row,
                    col: coords.col,
                })
                .collect::<Vec<Coordinates>>(),
            DirectionFrom::Bottom => ((coords.row + 1)..self.height)
                .map(|row| Coordinates {
                    row,
                    col: coords.col,
                })
                .collect::<Vec<Coordinates>>(),
        }
    }

    fn is_higher_than<T>(&self, coords: Coordinates, compare_with: &mut T) -> bool
    where
        T: Iterator<Item = Coordinates>,
    {
        compare_with.all(|p| self.get_value(coords) > self.get_value(p))
    }

    fn visible_tree_count(&self, coords: Coordinates, compare_with: Vec<Coordinates>) -> usize {
        let count = compare_with.len();
        let visible_trees = compare_with
            .into_iter()
            .take_while(|p| self.get_value(coords) > self.get_value(*p))
            .count()
            + 1;
        min(count, visible_trees)
    }

    fn get_value(&self, coords: Coordinates) -> usize {
        self.digits[coords.row * self.width + coords.col]
    }
}
