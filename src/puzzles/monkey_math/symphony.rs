use std::collections::{HashMap, HashSet};

use crate::{common::operation::Operation, puzzles::monkey_math::monkey_job::Job};

use super::{monkey::Monkey, monkey_job::MonkeyJob};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SymphonicState {
    Equal {
        lhs: Monkey,
        rhs: Monkey,
    },
    Invert {
        tgt: i64,
        lhs: Monkey,
        rhs: Monkey,
        op: Operation,
    },
    EvaluateLeft {
        tgt: Monkey,
        lhs: Monkey,
        rhs: i64,
        op: Operation,
    },
    EvaluateRight {
        tgt: Monkey,
        lhs: i64,
        rhs: Monkey,
        op: Operation,
    },
    None {
        tgt: Monkey,
        lhs: Monkey,
        rhs: Monkey,
        op: Operation,
    },
}

// This code is not very "symphonic", but whatever
pub struct Symphony {
    resolved: HashMap<Monkey, i64>,
    symphonic_states: HashSet<SymphonicState>,
    /// This maps a monkey to the symphonic states it is a part in
    unresolved: HashMap<Monkey, HashSet<SymphonicState>>,
}

impl Symphony {
    pub fn new() -> Symphony {
        Symphony {
            resolved: HashMap::new(),
            symphonic_states: HashSet::new(),
            unresolved: HashMap::new(),
        }
    }

    pub fn get_human(&mut self) -> Option<i64> {
        self.resolved.get(&Monkey::human()).copied()
    }

    pub fn process(&mut self, monkey_jobs: Vec<MonkeyJob>) {
        for monkey_job in monkey_jobs {
            self.add(monkey_job);
        }
    }

    fn add(&mut self, monkey_job: MonkeyJob) {
        let monkey = monkey_job.monkey;
        let job = monkey_job.job;

        if monkey == Monkey::root() {
            match job {
                Job::Wait { lhs, rhs, op: _ } => {
                    self.add_root(lhs, rhs);
                }
                Job::Yell(_) => unreachable!(),
            }
            return;
        }

        if monkey == Monkey::human() {
            return;
        }

        match job {
            Job::Yell(yell) => self.add_resolved(vec![(monkey, yell)]),
            Job::Wait { lhs, rhs, op } => self.try_resolve_and_add(monkey, lhs, rhs, op),
        };
    }

    fn add_root(&mut self, lhs: Monkey, rhs: Monkey) {
        match (self.resolved.get(&lhs), self.resolved.get(&rhs)) {
            (Some(_), Some(_)) => unreachable!(),
            (Some(lhs), None) => self.add_resolved(vec![(rhs, *lhs)]),
            (None, Some(rhs)) => self.add_resolved(vec![(lhs, *rhs)]),
            (None, None) => {
                let symphonic_state = SymphonicState::Equal { lhs, rhs };
                self.symphonic_states.insert(symphonic_state);
                self.add_symphonic_state_to_index(lhs, symphonic_state);
                self.add_symphonic_state_to_index(rhs, symphonic_state);
            }
        };
    }

    fn add_resolved(&mut self, monkey_yells: Vec<(Monkey, i64)>) {
        let mut recurse_to: Vec<(Monkey, i64)> = Vec::new();

        for (monkey, yell) in monkey_yells {
            self.resolved.insert(monkey, yell);
            if let Some(states_to_resolve) = self.unresolved.remove(&monkey) {
                for state_to_resolve in states_to_resolve {
                    match state_to_resolve {
                        SymphonicState::Equal { lhs, rhs } => {
                            if lhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((rhs, yell));
                            } else if rhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((lhs, yell));
                            };
                        }
                        SymphonicState::Invert { tgt, lhs, rhs, op } => {
                            if lhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((rhs, op.find_rhs()(tgt, yell)));
                            } else if rhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((lhs, op.find_lhs()(tgt, yell)));
                            };
                        }
                        SymphonicState::EvaluateLeft { tgt, lhs, rhs, op } => {
                            if tgt == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                recurse_to.push((lhs, op.find_lhs()(yell, rhs)));
                            } else if lhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                recurse_to.push((tgt, op.operation()(yell, rhs)));
                            };
                        }
                        SymphonicState::EvaluateRight { tgt, lhs, rhs, op } => {
                            if tgt == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((rhs, op.find_rhs()(yell, lhs)));
                            } else if rhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);
                                recurse_to.push((tgt, op.operation()(lhs, yell)));
                            };
                        }
                        SymphonicState::None { tgt, lhs, rhs, op } => {
                            if tgt == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);

                                let symphonic_state = SymphonicState::Invert {
                                    tgt: yell,
                                    lhs,
                                    rhs,
                                    op,
                                };
                                self.symphonic_states.insert(symphonic_state);
                                self.add_symphonic_state_to_index(lhs, symphonic_state);
                                self.add_symphonic_state_to_index(rhs, symphonic_state);
                            } else if lhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);

                                let symphonic_state = SymphonicState::EvaluateRight {
                                    tgt,
                                    lhs: yell,
                                    rhs,
                                    op,
                                };
                                self.symphonic_states.insert(symphonic_state);
                                self.add_symphonic_state_to_index(tgt, symphonic_state);
                                self.add_symphonic_state_to_index(rhs, symphonic_state);
                            } else if rhs == monkey {
                                self.symphonic_states.remove(&state_to_resolve);
                                self.remove_symphonic_state_from_index(tgt, state_to_resolve);
                                self.remove_symphonic_state_from_index(lhs, state_to_resolve);
                                self.remove_symphonic_state_from_index(rhs, state_to_resolve);

                                let symphonic_state = SymphonicState::EvaluateLeft {
                                    tgt,
                                    lhs,
                                    rhs: yell,
                                    op,
                                };
                                self.symphonic_states.insert(symphonic_state);
                                self.add_symphonic_state_to_index(tgt, symphonic_state);
                                self.add_symphonic_state_to_index(lhs, symphonic_state);
                            }
                        }
                    };
                }
            }
        }

        if recurse_to.len() == 0 {
            return;
        }

        self.add_resolved(recurse_to);
    }

    fn try_resolve_and_add(&mut self, tgt: Monkey, lhs: Monkey, rhs: Monkey, op: Operation) {
        match (
            self.resolved.get(&tgt),
            self.resolved.get(&lhs),
            self.resolved.get(&rhs),
        ) {
            (Some(_), Some(_), Some(_)) => unreachable!(),
            (Some(tgt), Some(lhs), None) => {
                self.add_resolved(vec![(rhs, op.find_rhs()(*tgt, *lhs))])
            }
            (Some(tgt), None, Some(rhs)) => {
                self.add_resolved(vec![(lhs, op.find_lhs()(*tgt, *rhs))])
            }
            (None, Some(lhs), Some(rhs)) => {
                self.add_resolved(vec![(tgt, op.operation()(*lhs, *rhs))])
            }
            (Some(tgt), None, None) => {
                let symphonic_state = SymphonicState::Invert {
                    tgt: *tgt,
                    lhs,
                    rhs,
                    op,
                };
                self.symphonic_states.insert(symphonic_state);
                self.add_symphonic_state_to_index(lhs, symphonic_state);
                self.add_symphonic_state_to_index(rhs, symphonic_state);
            }
            (None, Some(lhs), None) => {
                let symphonic_state = SymphonicState::EvaluateRight {
                    tgt,
                    lhs: *lhs,
                    rhs,
                    op,
                };
                self.symphonic_states.insert(symphonic_state);
                self.add_symphonic_state_to_index(tgt, symphonic_state);
                self.add_symphonic_state_to_index(rhs, symphonic_state);
            }
            (None, None, Some(rhs)) => {
                let symphonic_state = SymphonicState::EvaluateLeft {
                    tgt,
                    lhs,
                    rhs: *rhs,
                    op,
                };
                self.symphonic_states.insert(symphonic_state);
                self.add_symphonic_state_to_index(tgt, symphonic_state);
                self.add_symphonic_state_to_index(lhs, symphonic_state);
            }
            (None, None, None) => {
                let symphonic_state = SymphonicState::None { tgt, lhs, rhs, op };
                self.symphonic_states.insert(symphonic_state);
                self.add_symphonic_state_to_index(tgt, symphonic_state);
                self.add_symphonic_state_to_index(lhs, symphonic_state);
                self.add_symphonic_state_to_index(rhs, symphonic_state);
            }
        };
    }

    fn add_symphonic_state_to_index(&mut self, monkey: Monkey, symphonic_state: SymphonicState) {
        if let Some(states) = self.unresolved.get_mut(&monkey) {
            states.insert(symphonic_state);
        } else {
            let mut states: HashSet<SymphonicState> = HashSet::new();
            states.insert(symphonic_state);
            self.unresolved.insert(monkey, states);
        }
    }

    fn remove_symphonic_state_from_index(
        &mut self,
        monkey: Monkey,
        symphonic_state: SymphonicState,
    ) {
        if let Some(states) = self.unresolved.get_mut(&monkey) {
            states.remove(&symphonic_state);
        }
    }
}
