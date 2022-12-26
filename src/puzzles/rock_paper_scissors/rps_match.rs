use std::collections::HashMap;

use super::strategy::{RpsStrategy, RpsTarget, RpsType};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RpsResult {
    Loss,
    Draw,
    Win,
}

pub struct RpsMatch {
    pub opponent_choice: RpsType,
    pub own_choice: RpsType,
    pub result: RpsResult,
}

impl From<RpsTarget> for RpsResult {
    fn from(value: RpsTarget) -> Self {
        match value {
            RpsTarget::X => RpsResult::Loss,
            RpsTarget::Y => RpsResult::Draw,
            RpsTarget::Z => RpsResult::Win,
        }
    }
}

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

pub struct RpsTargetMap {
    to_result_map: HashMap<(RpsType, RpsType), RpsResult>,
    to_own_choice_map: HashMap<(RpsType, RpsResult), RpsType>,
}

impl RpsTargetMap {
    pub fn new() -> Self {
        let mut to_result_map: HashMap<(RpsType, RpsType), RpsResult> = HashMap::new();
        let mut to_own_choice_map: HashMap<(RpsType, RpsResult), RpsType> = HashMap::new();

        for mapping in RPS_MATCH_RESULT_MAPPING {
            to_result_map.insert(
                (mapping.opponent_choice, mapping.own_choice),
                mapping.result,
            );
            to_own_choice_map.insert(
                (mapping.opponent_choice, mapping.result),
                mapping.own_choice,
            );
        }

        RpsTargetMap {
            to_result_map,
            to_own_choice_map,
        }
    }

    pub fn map_target_as_type(&self, strategy: RpsStrategy) -> RpsMatch {
        let opponent_choice = strategy.opponent_choice;
        let own_choice: RpsType = strategy.target.into();
        match self.to_result_map.get(&(opponent_choice, own_choice)) {
            Some(result) => RpsMatch {
                opponent_choice,
                own_choice,
                result: *result,
            },
            None => unreachable!("by definition of mapping"),
        }
    }

    pub fn map_target_as_result(&self, strategy: RpsStrategy) -> RpsMatch {
        let opponent_choice = strategy.opponent_choice;
        let result: RpsResult = strategy.target.into();
        match self.to_own_choice_map.get(&(opponent_choice, result)) {
            Some(own_choice) => RpsMatch {
                opponent_choice,
                own_choice: *own_choice,
                result,
            },
            None => unreachable!("by definition of mapping"),
        }
    }
}
