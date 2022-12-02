use super::common::{RpsResult, RpsType};

pub struct RpsMatchResultMap {
    pub opponent_choice: RpsType,
    pub own_choice: RpsType,
    pub result: RpsResult,
}

pub const RPS_MATCH_RESULT_MAPPING: [RpsMatchResultMap; 9] = [
    RpsMatchResultMap {
        opponent_choice: RpsType::Rock,
        own_choice: RpsType::Rock,
        result: RpsResult::Draw,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Rock,
        own_choice: RpsType::Paper,
        result: RpsResult::Win,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Rock,
        own_choice: RpsType::Scissors,
        result: RpsResult::Loss,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Paper,
        own_choice: RpsType::Rock,
        result: RpsResult::Loss,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Paper,
        own_choice: RpsType::Paper,
        result: RpsResult::Draw,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Paper,
        own_choice: RpsType::Scissors,
        result: RpsResult::Win,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Scissors,
        own_choice: RpsType::Rock,
        result: RpsResult::Win,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Scissors,
        own_choice: RpsType::Paper,
        result: RpsResult::Loss,
    },
    RpsMatchResultMap {
        opponent_choice: RpsType::Scissors,
        own_choice: RpsType::Scissors,
        result: RpsResult::Draw,
    },
];
