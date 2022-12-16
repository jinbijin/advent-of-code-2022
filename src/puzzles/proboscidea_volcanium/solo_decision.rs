use std::collections::HashMap;

use super::{target_valve::TargetValve, valve_system::ValveSystem};

pub struct SoloDecision {
    closed_valves: HashMap<String, usize>,
    targets: Box<dyn Iterator<Item = TargetValve>>,
    time_left: usize,
    flow_rate: usize,
    pressure_released: usize,
}

impl Iterator for SoloDecision {
    type Item = TargetValve;

    fn next(&mut self) -> Option<Self::Item> {
        self.targets.next()
    }
}

impl SoloDecision {
    pub fn new(valve_system: &ValveSystem) -> Self {
        let time_left = 30;
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
            let reachable_closed_valves = distances_from_start
                .iter()
                .filter(|(_, distance)| distance < &&time_left)
                .filter_map(
                    |(target, distance)| match closed_valves.get_key_value(target) {
                        None => None,
                        Some((target, flow_rate)) => {
                            if flow_rate > &0 && target != &location {
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
                .collect::<Vec<TargetValve>>()
                .into_iter();
            let targets = Box::new(reachable_closed_valves);
            SoloDecision {
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
        target_valve: &TargetValve,
    ) -> Self {
        let time_taken = target_valve.distance + 1;
        let time_left = self.time_left - time_taken;
        let pressure_released = self.pressure_released + self.flow_rate * time_taken;
        let flow_rate = self.flow_rate + target_valve.flow_rate;
        let location = target_valve.target.clone();
        let mut closed_valves = self.closed_valves.clone();
        closed_valves.remove(&location);

        if let Some(distances_from_target) = distance_map.get(&location) {
            let reachable_closed_valves = distances_from_target
                .iter()
                .filter(|(_, distance)| distance < &&time_left)
                .filter_map(
                    |(target, distance)| match closed_valves.get_key_value(target) {
                        None => None,
                        Some((target, flow_rate)) => {
                            if flow_rate > &0 && target != &location {
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
                .collect::<Vec<TargetValve>>()
                .into_iter();
            let targets = Box::new(reachable_closed_valves);
            SoloDecision {
                closed_valves,
                targets,
                time_left,
                flow_rate,
                pressure_released,
            }
        } else {
            unreachable!("Assuming target is always in hashmap")
        }
    }

    pub fn total_pressure_released(&self) -> usize {
        self.flow_rate * self.time_left + self.pressure_released
    }
}
