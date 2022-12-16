use std::collections::HashMap;

use super::{
    target_valve::{self, TargetValve, TargetValvePair},
    valve_system::ValveSystem,
};

pub struct PairDecision {
    closed_valves: HashMap<String, usize>,
    targets: Box<dyn Iterator<Item = TargetValvePair>>,
    time_left: usize,
    flow_rate: usize,
    pressure_released: usize,
}

impl Iterator for PairDecision {
    type Item = TargetValvePair;

    fn next(&mut self) -> Option<Self::Item> {
        self.targets.next()
    }
}

impl PairDecision {
    pub fn new(valve_system: &ValveSystem) -> Self {
        let time_left = 26;
        let pressure_released = 0;
        let flow_rate = 0;
        let location = format!("AA");
        let closed_valves = valve_system
            .flow_rate
            .iter()
            .filter_map(|(name, flow_rate)| {
                if flow_rate > &0 {
                    Some((name.to_string(), *flow_rate))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, usize>>();

        if let Some(distances_from_start) = valve_system.distance.get("AA") {
            let mut reachable_closed_valves = distances_from_start
                .iter()
                .filter(|(_, distance)| distance < &&time_left)
                .filter_map(
                    |(target, distance)| match closed_valves.get_key_value(target) {
                        None => None,
                        Some((target, flow_rate)) => {
                            if flow_rate > &0 && target != &location {
                                Some(Some(TargetValve {
                                    target: target.to_string(),
                                    distance: *distance,
                                    flow_rate: *flow_rate,
                                }))
                            } else {
                                None
                            }
                        }
                    },
                )
                .collect::<Vec<Option<TargetValve>>>();
            reachable_closed_valves.push(None);

            let targets = target_valve::merge_targets(
                &reachable_closed_valves,
                &reachable_closed_valves,
                time_left,
            );
            let targets = Box::new(targets.into_iter());
            PairDecision {
                closed_valves,
                targets,
                time_left,
                flow_rate,
                pressure_released,
            }
        } else {
            unreachable!("AA is always in input")
        }
    }

    pub fn move_to_valve_and_open(
        &self,
        distance_map: &HashMap<String, HashMap<String, usize>>,
        target_valves: &TargetValvePair,
    ) -> Self {
        let min_distance = target_valves.min_distance();

        let time_taken = min_distance + 1;
        let time_left = self.time_left - time_taken;
        let pressure_released = self.pressure_released + self.flow_rate * time_taken;
        let flow_rate = self.flow_rate + target_valves.activated_flow_rate();

        let mut closed_valves = self.closed_valves.clone();
        for active_valve in target_valves.active_valves() {
            closed_valves.remove(&active_valve.target);
        }

        let self_reachable =
            target_valves.reachable_by_scout(distance_map, &closed_valves, time_left);
        let other_reachable =
            target_valves.reachable_by_elephant(distance_map, &closed_valves, time_left);

        let targets = target_valve::merge_targets(&self_reachable, &other_reachable, time_left);
        let targets = Box::new(targets.into_iter());
        PairDecision {
            closed_valves,
            targets,
            time_left,
            flow_rate,
            pressure_released,
        }
    }

    pub fn potential_pressure_released(&self) -> usize {
        let potential_flow_rate = self.flow_rate + self.closed_valves.values().sum::<usize>();
        potential_flow_rate * self.time_left + self.pressure_released
    }

    pub fn total_pressure_released(&self) -> usize {
        self.flow_rate * self.time_left + self.pressure_released
    }
}
