use super::{blueprint::Blueprint, collection_state::CollectionState, resources::Resources};

pub struct Collector<'a> {
    state_stack: Vec<CollectionState<'a>>,
    total_minutes: usize,
    current_max: usize,
}

impl<'a> Collector<'a> {
    fn next_state(&mut self) -> Option<()> {
        let mut current_state = self.state_stack.pop()?;
        match current_state.next() {
            Some(next_state) => {
                self.state_stack.push(current_state);
                self.state_stack.push(next_state);

                Some(())
            }
            None => self.next_state(),
        }
    }
}

impl<'a> Iterator for Collector<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while self.next_state().is_some() {
            let current_state = match self.state_stack.last() {
                Some(state) => state,
                None => unreachable!("if next_state is Some, then it contains at least two items"),
            };
            let geode_count = current_state.geode_count();
            if current_state.start_of_minute() > self.total_minutes {
                self.state_stack.pop();
                if geode_count > self.current_max {
                    self.current_max = geode_count;
                }
                return Some(geode_count);
            }
            let potential = current_state.potential_geode_count();
            if potential < self.current_max {
                return Some(potential);
            }
        }
        None
    }
}

pub trait AsCollector {
    fn collector<'a>(&'a self, total_minutes: usize) -> Collector<'a>;
}

impl AsCollector for Blueprint {
    fn collector<'a>(&'a self, total_minutes: usize) -> Collector<'a> {
        Collector {
            state_stack: vec![CollectionState::new(
                self,
                1,
                Resources::new(0, 0, 0, 0),
                Resources::new(1, 0, 0, 0),
                total_minutes,
            )],
            total_minutes,
            current_max: 0,
        }
    }
}
