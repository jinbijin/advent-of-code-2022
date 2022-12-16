use super::{solo_decision::SoloDecision, target_valve::TargetValve, valve_system::ValveSystem};

pub struct SoloCrawler<'a> {
    valve_system: &'a ValveSystem,
    decisions: Vec<SoloDecision>,
    calls: usize,
}

impl<'a> SoloCrawler<'a> {
    fn next_decision(&mut self) -> Option<usize> {
        match self.decisions.pop() {
            None => None,
            Some(mut latest_decision) => match latest_decision.next() {
                None => self.next_decision(),
                Some(target_valve) => Some(self.populate(Some((latest_decision, target_valve)))),
            },
        }
    }

    fn populate(&mut self, current: Option<(SoloDecision, TargetValve)>) -> usize {
        let mut decision = match current {
            None => SoloDecision::new(self.valve_system),
            Some((latest_decision, target_valve)) => {
                let new_decision = latest_decision
                    .move_to_valve_and_open(&self.valve_system.distance, &target_valve);
                self.decisions.push(latest_decision);
                new_decision
            }
        };

        match decision.next() {
            None => decision.total_pressure_released(),
            Some(target_valve) => self.populate(Some((decision, target_valve))),
        }
    }
}

impl<'a> Iterator for SoloCrawler<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.calls += 1;
        if self.decisions.len() > 0 {
            self.next_decision()
        } else {
            Some(self.populate(None))
        }
    }
}

pub trait AsSoloCrawler {
    fn solo_crawler<'a>(&'a self) -> SoloCrawler<'a>;
}

impl AsSoloCrawler for ValveSystem {
    fn solo_crawler<'a>(&'a self) -> SoloCrawler<'a> {
        SoloCrawler {
            valve_system: self,
            decisions: Vec::new(),
            calls: 0,
        }
    }
}
