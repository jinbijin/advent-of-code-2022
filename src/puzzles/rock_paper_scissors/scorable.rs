use std::iter::Sum;

use super::{
    rps_match::{RpsMatch, RpsResult},
    strategy::RpsType,
};

pub trait Scorable<T>
where
    T: Sum,
{
    fn score(&self) -> T;
}

impl Scorable<i32> for RpsType {
    fn score(&self) -> i32 {
        match &self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Scorable<i32> for RpsResult {
    fn score(&self) -> i32 {
        match &self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Scorable<i32> for RpsMatch {
    fn score(&self) -> i32 {
        self.own_choice.score() + self.result.score()
    }
}
