use std::iter::Sum;

pub trait Scorable<T>
where
    T: Sum,
{
    fn score(&self) -> T;
}

#[derive(PartialEq, Clone, Copy)]
pub enum RpsType {
    Rock,
    Paper,
    Scissors,
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

#[derive(PartialEq, Clone, Copy)]
pub enum RpsResult {
    Loss,
    Draw,
    Win,
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

pub trait RpsMatchWithResult {
    fn own_choice(&self) -> Option<RpsType>;
    fn result(&self) -> Option<RpsResult>;
}

impl<T> Scorable<i32> for T
where
    T: RpsMatchWithResult,
{
    fn score(&self) -> i32 {
        let own_score = match self.own_choice() {
            Some(rps_type) => rps_type.score(),
            None => 0, // Should not occur
        };
        let result_score = match self.result() {
            Some(rps_result) => rps_result.score(),
            None => 0, // Should not occur
        };
        own_score + result_score
    }
}
