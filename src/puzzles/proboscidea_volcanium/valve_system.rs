use std::collections::HashMap;

use super::valve::Valve;

pub struct ValveSystem {
    pub distance: HashMap<String, HashMap<String, usize>>,
    pub flow_rate: HashMap<String, usize>,
}

impl From<Vec<Valve>> for ValveSystem {
    fn from(valves: Vec<Valve>) -> Self {
        let mut distance: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut flow_rate: HashMap<String, usize> = HashMap::new();

        for valve in valves.iter() {
            flow_rate.insert(valve.name.to_string(), valve.flow_rate);

            let mut diagonal: HashMap<String, usize> = HashMap::new();
            diagonal.insert(valve.name.to_string(), 0);
            distance.insert(valve.name.to_string(), diagonal);
        }

        for i in 0..(valves.len()) {
            for valve in valves.iter() {
                // For each A (valve.name) -> B (connected_to) ...
                for connected_to in valve.connected_to.iter() {
                    // ... and for each X (key of distance) -> A of distance i and not computed distance X -> B ...
                    for distance_map in distance.values_mut() {
                        if distance_map.get(&valve.name) == Some(&i)
                            && distance_map.get(connected_to) == None
                        {
                            // ... the distance X -> B is (i + 1)
                            distance_map.insert(connected_to.to_string(), i + 1);
                        }
                    }
                }
            }
        }

        ValveSystem {
            distance,
            flow_rate,
        }
    }
}
