use super::{
    pair_decision::PairDecision, target_valve::TargetValvePair, valve_system::ValveSystem,
};

pub struct PairCrawler<'a> {
    valve_system: &'a ValveSystem,
    decisions: Vec<PairDecision>,
    calls: usize,
    current_max: usize,
}

impl<'a> PairCrawler<'a> {
    fn next_decision(&mut self) -> Option<usize> {
        match self.decisions.pop() {
            None => None,
            Some(mut latest_decision) => match latest_decision.next() {
                None => self.next_decision(),
                Some(target_valve) => Some(self.populate(Some((latest_decision, target_valve)))),
            },
        }
    }

    fn populate(&mut self, current: Option<(PairDecision, TargetValvePair)>) -> usize {
        let mut decision = match current {
            None => PairDecision::new(self.valve_system),
            Some((latest_decision, target_valve)) => {
                let new_decision = latest_decision
                    .move_to_valve_and_open(&self.valve_system.distance, &target_valve);
                self.decisions.push(latest_decision);
                new_decision
            }
        };

        if decision.potential_pressure_released() < self.current_max {
            return decision.potential_pressure_released();
        }

        match decision.next() {
            None => decision.total_pressure_released(),
            Some(target_valve) => self.populate(Some((decision, target_valve))),
        }
    }
}

impl<'a> Iterator for PairCrawler<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.calls += 1;
        let result = if self.decisions.len() > 0 {
            self.next_decision()
        } else {
            Some(self.populate(None))
        };
        if let Some(new_result) = result {
            if new_result > self.current_max {
                self.current_max = new_result;
            }
        }
        if self.calls % 1000000 == 0 {
            dbg!(self.current_max);
        }
        result
    }
}

pub trait AsPairCrawler {
    fn pair_crawler<'a>(&'a self) -> PairCrawler<'a>;
}

impl AsPairCrawler for ValveSystem {
    fn pair_crawler<'a>(&'a self) -> PairCrawler<'a> {
        PairCrawler {
            valve_system: self,
            decisions: Vec::new(),
            calls: 0,
            current_max: 0,
        }
    }
}
