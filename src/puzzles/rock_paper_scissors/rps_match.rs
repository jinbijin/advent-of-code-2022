use super::{
    mapping,
    strategy::{RpsStrategy, RpsTarget, RpsType},
};

#[derive(PartialEq, Clone, Copy)]
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

pub fn match_with_target_as_type(strategy: RpsStrategy) -> RpsMatch {
    let opponent_choice = strategy.opponent_choice;
    let own_choice: RpsType = strategy.target.into();
    let result = if let Some(map) = mapping::RPS_MATCH_RESULT_MAPPING
        .iter()
        .find(|map| map.opponent_choice == opponent_choice && map.own_choice == own_choice)
    {
        map.result
    } else {
        unreachable!("Cannot be reached due to mapping definition.")
    };
    RpsMatch {
        opponent_choice,
        own_choice,
        result,
    }
}

pub fn match_with_target_as_result(strategy: RpsStrategy) -> RpsMatch {
    let opponent_choice = strategy.opponent_choice;
    let result: RpsResult = strategy.target.into();
    let own_choice = if let Some(map) = mapping::RPS_MATCH_RESULT_MAPPING
        .iter()
        .find(|map| map.opponent_choice == opponent_choice && map.result == result)
    {
        map.own_choice
    } else {
        unreachable!("Cannot be reached due to mapping definition.")
    };
    RpsMatch {
        opponent_choice,
        own_choice,
        result,
    }
}
