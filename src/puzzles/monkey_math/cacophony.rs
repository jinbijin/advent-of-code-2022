use std::collections::HashMap;

use crate::common::operation::Operation;

use super::{
    monkey::Monkey,
    monkey_job::{Job, MonkeyJob},
};

#[derive(Debug, Clone, Copy)]
pub enum CacophonicState {
    Yell(i64),
    WaitLeft {
        lhs: Monkey,
        op: Operation,
        rhs: i64,
    },
    WaitRight {
        lhs: i64,
        op: Operation,
        rhs: Monkey,
    },
    WaitBoth {
        lhs: Monkey,
        op: Operation,
        rhs: Monkey,
    },
}

pub struct Cacophony {
    /// A list of monkeys in their current state
    monkeys: HashMap<Monkey, CacophonicState>,
    /// This maps a monkey ("awaited") to the list of monkeys that are waiting for it ("waiters")
    awaited: HashMap<Monkey, Vec<Monkey>>,
}

impl Cacophony {
    pub fn new() -> Cacophony {
        Cacophony {
            monkeys: HashMap::new(),
            awaited: HashMap::new(),
        }
    }

    pub fn get_root(&self) -> Option<i64> {
        match self.monkeys.get(&Monkey::root()) {
            Some(CacophonicState::Yell(yell)) => Some(*yell),
            _ => None,
        }
    }

    pub fn process(&mut self, monkey_jobs: Vec<MonkeyJob>) {
        for monkey_job in monkey_jobs {
            self.add(monkey_job);
        }
    }

    fn add(&mut self, monkey_job: MonkeyJob) {
        let monkey = monkey_job.monkey;
        let cacophonic_state = match monkey_job.job {
            Job::Yell(yell) => CacophonicState::Yell(yell),
            Job::Wait { lhs, rhs, op } => match (self.monkeys.get(&lhs), self.monkeys.get(&rhs)) {
                (Some(CacophonicState::Yell(lhs)), Some(CacophonicState::Yell(rhs))) => {
                    CacophonicState::Yell(op.operation()(lhs, rhs))
                }
                (Some(CacophonicState::Yell(lhs)), _) => {
                    CacophonicState::WaitRight { lhs: *lhs, op, rhs }
                }
                (_, Some(CacophonicState::Yell(rhs))) => {
                    CacophonicState::WaitLeft { lhs, op, rhs: *rhs }
                }
                (_, _) => CacophonicState::WaitBoth { lhs, op, rhs },
            },
        };

        self.monkeys.insert(monkey, cacophonic_state);
        match cacophonic_state {
            CacophonicState::Yell(yell) => self.resolve_waiting_monkeys(vec![(monkey, yell)]),
            CacophonicState::WaitLeft { lhs, op: _, rhs: _ } => self.await_monkey(monkey, lhs),
            CacophonicState::WaitRight { lhs: _, op: _, rhs } => self.await_monkey(monkey, rhs),
            CacophonicState::WaitBoth { lhs, op: _, rhs } => {
                self.await_monkey(monkey, lhs);
                self.await_monkey(monkey, rhs);
            }
        }
    }

    fn await_monkey(&mut self, waiter: Monkey, awaited: Monkey) {
        if let Some(queue) = self.awaited.get_mut(&awaited) {
            queue.push(waiter);
        } else {
            self.awaited.insert(awaited, vec![waiter]);
        }
    }

    fn resolve_waiting_monkeys(&mut self, monkey_yells: Vec<(Monkey, i64)>) {
        let mut recurse_for: Vec<(Monkey, i64)> = Vec::new();

        for (monkey, yell) in monkey_yells {
            if let Some(waiters) = self.awaited.remove(&monkey) {
                for waiter in waiters {
                    if let Some(cacophonic_state) = self.monkeys.remove(&waiter) {
                        let new_state = match cacophonic_state {
                            CacophonicState::Yell(yell) => CacophonicState::Yell(yell),
                            CacophonicState::WaitLeft { lhs, op, rhs } => {
                                if lhs == monkey {
                                    let yell = op.operation()(yell, rhs);
                                    recurse_for.push((waiter, yell));
                                    CacophonicState::Yell(yell)
                                } else {
                                    CacophonicState::WaitLeft { lhs, op, rhs }
                                }
                            }
                            CacophonicState::WaitRight { lhs, op, rhs } => {
                                if rhs == monkey {
                                    let yell = op.operation()(lhs, yell);
                                    recurse_for.push((waiter, yell));
                                    CacophonicState::Yell(yell)
                                } else {
                                    CacophonicState::WaitRight { lhs, op, rhs }
                                }
                            }
                            CacophonicState::WaitBoth { lhs, op, rhs } => {
                                if lhs == monkey {
                                    CacophonicState::WaitRight { lhs: yell, op, rhs }
                                } else if rhs == monkey {
                                    CacophonicState::WaitLeft { lhs, op, rhs: yell }
                                } else {
                                    CacophonicState::WaitBoth { lhs, op, rhs }
                                }
                            }
                        };

                        self.monkeys.insert(waiter, new_state);
                    }
                }
            }
        }

        if recurse_for.len() == 0 {
            return;
        }

        // Hoping for tail call optimization
        self.resolve_waiting_monkeys(recurse_for);
    }
}
