use std::{cmp, collections::HashMap};

#[derive(Clone, PartialEq, Eq)]
pub struct TargetValve {
    pub target: String,
    pub distance: usize,
    pub flow_rate: usize,
}

impl TargetValve {
    pub fn flow_rate_contribution(&self, time_left: usize) -> usize {
        (time_left - self.distance - 1) * self.flow_rate
    }

    pub fn reachable(
        &self,
        distance_map: &HashMap<String, HashMap<String, usize>>,
        closed_valves: &HashMap<String, usize>,
        time_left: usize,
    ) -> Vec<TargetValve> {
        match distance_map.get(&self.target) {
            Some(distances_from_target) => distances_from_target
                .iter()
                .filter(|(_, distance)| distance < &&time_left)
                .filter_map(
                    |(target, distance)| match closed_valves.get_key_value(target) {
                        None => None,
                        Some((target, flow_rate)) => {
                            if flow_rate > &0 && target != &self.target {
                                Some(TargetValve {
                                    target: target.to_string(),
                                    distance: *distance,
                                    flow_rate: *flow_rate,
                                })
                            } else {
                                None
                            }
                        }
                    },
                )
                .collect::<Vec<TargetValve>>(),
            None => unreachable!("Key is always in hash map."),
        }
    }
}

pub enum TargetValvePair {
    Both(TargetValve, TargetValve),
    Scout(TargetValve),
    Elephant(TargetValve),
}

impl TargetValvePair {
    pub fn active_valves(&self) -> Vec<&TargetValve> {
        match self {
            Self::Both(x, y) => {
                let mut result: Vec<&TargetValve> = Vec::new();
                if x.distance == self.min_distance() {
                    result.push(x);
                }
                if y.distance == self.min_distance() {
                    result.push(y);
                }
                result
            }
            Self::Scout(x) => vec![x],
            Self::Elephant(y) => vec![y],
        }
    }

    pub fn min_distance(&self) -> usize {
        match self {
            Self::Both(x, y) => cmp::min(x.distance, y.distance),
            Self::Scout(x) => x.distance,
            Self::Elephant(y) => y.distance,
        }
    }

    pub fn activated_flow_rate(&self) -> usize {
        match self {
            Self::Both(x, y) => {
                if x.distance == self.min_distance()
                    && y.distance == self.min_distance()
                    && x.target != y.target
                {
                    x.flow_rate + y.flow_rate
                } else if x.distance == self.min_distance() {
                    x.flow_rate
                } else if y.distance == self.min_distance() {
                    y.flow_rate
                } else {
                    unreachable!("One of them is smallest")
                }
            }
            Self::Scout(x) => x.flow_rate,
            Self::Elephant(y) => y.flow_rate,
        }
    }

    pub fn reachable_by_scout(
        &self,
        distance_map: &HashMap<String, HashMap<String, usize>>,
        closed_valves: &HashMap<String, usize>,
        time_left: usize,
    ) -> Vec<Option<TargetValve>> {
        match self {
            Self::Both(x, y) => {
                if x.distance == self.min_distance() {
                    let mut result = x
                        .reachable(distance_map, closed_valves, time_left)
                        .into_iter()
                        .map(|x| Some(x))
                        .collect::<Vec<Option<TargetValve>>>();
                    result.push(None);
                    result
                } else {
                    let flow_rate = if x.target != y.target { x.flow_rate } else { 0 };
                    vec![Some(TargetValve {
                        target: x.target.clone(),
                        distance: x.distance - self.min_distance() - 1,
                        flow_rate,
                    })]
                }
            }
            Self::Scout(x) => {
                let mut result = x
                    .reachable(distance_map, closed_valves, time_left)
                    .into_iter()
                    .map(|x| Some(x))
                    .collect::<Vec<Option<TargetValve>>>();
                result.push(None);
                result
            }
            Self::Elephant(_) => vec![None],
        }
    }

    pub fn reachable_by_elephant(
        &self,
        distance_map: &HashMap<String, HashMap<String, usize>>,
        closed_valves: &HashMap<String, usize>,
        time_left: usize,
    ) -> Vec<Option<TargetValve>> {
        match self {
            Self::Both(x, y) => {
                if y.distance == self.min_distance() {
                    let mut result = y
                        .reachable(distance_map, closed_valves, time_left)
                        .into_iter()
                        .map(|x| Some(x))
                        .collect::<Vec<Option<TargetValve>>>();
                    result.push(None);
                    result
                } else {
                    let flow_rate = if x.target != y.target { y.flow_rate } else { 0 };
                    vec![Some(TargetValve {
                        target: y.target.clone(),
                        distance: y.distance - self.min_distance() - 1,
                        flow_rate,
                    })]
                }
            }
            Self::Scout(_) => vec![None],
            Self::Elephant(y) => {
                let mut result = y
                    .reachable(distance_map, closed_valves, time_left)
                    .into_iter()
                    .map(|x| Some(x))
                    .collect::<Vec<Option<TargetValve>>>();
                result.push(None);
                result
            }
        }
    }

    fn flow_rate_contribution(&self, time_left: usize) -> usize {
        match self {
            Self::Both(x, y) => {
                x.flow_rate_contribution(time_left) + y.flow_rate_contribution(time_left)
            }
            Self::Scout(x) => x.flow_rate_contribution(time_left),
            Self::Elephant(y) => y.flow_rate_contribution(time_left),
        }
    }
}

pub fn merge_targets(
    scout_targets: &Vec<Option<TargetValve>>,
    elephant_targets: &Vec<Option<TargetValve>>,
    time_left: usize,
) -> Vec<TargetValvePair> {
    let mut targets = scout_targets
        .iter()
        .flat_map(|x| elephant_targets.iter().map(|y| (x.clone(), y.clone())))
        .filter_map(|(x, y)| match x {
            Some(x) => match y {
                Some(y) => {
                    if x.target != y.target {
                        Some(TargetValvePair::Both(x, y))
                    } else {
                        None
                    }
                }
                None => Some(TargetValvePair::Scout(x)),
            },
            None => match y {
                Some(y) => Some(TargetValvePair::Elephant(y)),
                None => None,
            },
        })
        .collect::<Vec<TargetValvePair>>();
    targets.sort_by(|x, y| {
        (y.flow_rate_contribution(time_left)).cmp(&(x.flow_rate_contribution(time_left)))
    });

    targets
}
