use std::{
    collections::{HashMap, VecDeque},
    num::ParseIntError,
};

use crate::contents::{
    convert::{AsParseSections, FromContents, FromSection},
    errors::{self, ParseLineError},
};

pub struct MonkeyItem {
    worry_level: usize,
}

impl MonkeyItem {
    fn inspect(&mut self, operation: &Box<dyn Fn(usize) -> usize>) {
        self.worry_level = operation(self.worry_level);
    }

    fn release(&mut self, reduce_by: Option<usize>) {
        match reduce_by {
            Some(reduce_by) => self.worry_level %= reduce_by,
            None => self.worry_level /= 3,
        };
    }

    fn test(&self, divisor: usize) -> bool {
        self.worry_level % divisor == 0
    }
}

pub struct Monkey {
    items: VecDeque<MonkeyItem>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisor: usize,
    throw_to_if_true: String,
    throw_to_if_false: String,
    items_thrown: usize,
}

impl Monkey {
    fn inspect_throw(&mut self, item: &mut MonkeyItem, reduce_by: Option<usize>) -> String {
        self.items_thrown += 1;
        item.inspect(&self.operation);
        item.release(reduce_by);
        if item.test(self.divisor) {
            self.throw_to_if_true.clone()
        } else {
            self.throw_to_if_false.clone()
        }
    }
}

impl FromSection for (String, Monkey) {
    type Err = Vec<ParseLineError>; // Needed for now because of implementation of FromSections, TODO use own error type

    fn from_section(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let monkey_key = match lines.next() {
            Some(line) => {
                let expected_line_start = "Monkey ";
                if line.starts_with(expected_line_start) && line.ends_with(":") {
                    Ok(line[(expected_line_start.len())..(line.len() - 1)].to_string())
                } else {
                    Err(vec![ParseLineError::new(
                        0,
                        "invalid format for monkey".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(0, "expected monkey".to_string())]),
        }?;

        let items = match lines.next() {
            Some(line) => {
                let expected_line_start = "  Starting items: ";
                if line.starts_with(expected_line_start) {
                    line[(expected_line_start.len())..]
                        .split(", ")
                        .map(|item| {
                            item.parse::<usize>()
                                .map(|worry_level| MonkeyItem { worry_level })
                        })
                        .collect::<Result<VecDeque<MonkeyItem>, ParseIntError>>()
                        .map_err(|_| {
                            vec![ParseLineError::new(
                                1,
                                "invalid number found in starting items".to_string(),
                            )]
                        })
                } else {
                    Err(vec![ParseLineError::new(
                        1,
                        "invalid format for starting items".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(
                1,
                "expected starting items".to_string(),
            )]),
        }?;

        let operation = match lines.next() {
            Some(line) => {
                let expected_line_start = "  Operation: new = ";
                if line.starts_with(expected_line_start) {
                    let parts = line[(expected_line_start.len())..]
                        .split(' ')
                        .collect::<Vec<&str>>();
                    if parts.len() == 3 {
                        parse_expression(parts[0], parts[1], parts[2])
                            .map_err(|err| vec![ParseLineError::new(2, err)])
                    } else {
                        Err(vec![ParseLineError::new(
                            2,
                            "invalid format for operation".to_string(),
                        )])
                    }
                } else {
                    Err(vec![ParseLineError::new(
                        2,
                        "invalid format for operation".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(
                2,
                "expected operation".to_string(),
            )]),
        }?;

        let divisor = match lines.next() {
            Some(line) => {
                let expected_line_start = "  Test: divisible by ";
                if line.starts_with(expected_line_start) {
                    line[(expected_line_start.len())..]
                        .parse::<usize>()
                        .map_err(|_| vec![ParseLineError::new(3, "invalid divisor".to_string())])
                } else {
                    Err(vec![ParseLineError::new(
                        3,
                        "invalid format for test".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(3, "expected test".to_string())]),
        }?;

        let throw_to_if_true = match lines.next() {
            Some(line) => {
                let expected_line_start = "    If true: throw to monkey ";
                if line.starts_with(expected_line_start) {
                    Ok(line[(expected_line_start.len())..].to_string())
                } else {
                    Err(vec![ParseLineError::new(
                        4,
                        "invalid format for if true".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(4, "expected if true".to_string())]),
        }?;

        let throw_to_if_false = match lines.next() {
            Some(line) => {
                let expected_line_start = "    If false: throw to monkey ";
                if line.starts_with(expected_line_start) {
                    Ok(line[(expected_line_start.len())..].to_string())
                } else {
                    Err(vec![ParseLineError::new(
                        5,
                        "invalid format for if false".to_string(),
                    )])
                }
            }
            None => Err(vec![ParseLineError::new(5, "expected if true".to_string())]),
        }?;

        Ok((
            monkey_key,
            Monkey {
                items,
                operation,
                divisor,
                throw_to_if_true,
                throw_to_if_false,
                items_thrown: 0,
            },
        ))
    }
}

pub struct MonkeyCollection {
    monkey_keys: Vec<String>,
    monkeys: HashMap<String, Monkey>,
}

impl MonkeyCollection {
    pub fn round(&mut self, relieved_after_inspection: bool) {
        let reduce_by = if relieved_after_inspection {
            None
        } else {
            Some(self.get_divisors().iter().product::<usize>())
        };
        let keys = self.monkey_keys.clone();
        for key in keys.iter() {
            self.inspect_throw_all(key, reduce_by);
        }
    }

    pub fn get_sorted_throw_counts(&self) -> Vec<usize> {
        let mut throw_counts = self
            .monkeys
            .values()
            .map(|monkey| monkey.items_thrown)
            .collect::<Vec<usize>>();
        throw_counts.sort_by(|a, b| b.cmp(a));
        throw_counts
    }

    fn get_divisors(&self) -> Vec<usize> {
        self.monkeys
            .values()
            .map(|monkey| monkey.divisor)
            .collect::<Vec<usize>>()
    }

    fn inspect_throw_all(&mut self, key: &String, reduce_by: Option<usize>) {
        while let Some(()) = self.inspect_throw_item(key, reduce_by) {}
    }

    fn inspect_throw_item(&mut self, key: &String, reduce_by: Option<usize>) -> Option<()> {
        if let Some(monkey) = self.monkeys.get_mut(key) {
            if let Some(mut item) = monkey.items.pop_front() {
                let target = monkey.inspect_throw(&mut item, reduce_by);
                if let Some(other_monkey) = self.monkeys.get_mut(&target) {
                    other_monkey.items.push_back(item);
                    return Some(());
                }
            }
        }
        None
    }
}

impl FromContents for MonkeyCollection {
    fn from_contents(s: &str) -> Result<Self, errors::ParseContentsError> {
        let monkeys_with_keys = s.parse_sections::<Vec<(String, Monkey)>>()?;
        let mut monkey_keys: Vec<String> = Vec::new();
        let mut monkeys: HashMap<String, Monkey> = HashMap::new();

        for (key, monkey) in monkeys_with_keys {
            monkey_keys.push(key.clone());
            monkeys.insert(key, monkey);
        }

        Ok(MonkeyCollection {
            monkey_keys,
            monkeys,
        })
    }
}

fn parse_expression(lhs: &str, op: &str, rhs: &str) -> Result<Box<dyn Fn(usize) -> usize>, String> {
    if lhs == "old" && rhs == "old" {
        if op == "*" {
            return Ok(Box::new(move |x| x * x));
        } else if op == "+" {
            return Ok(Box::new(move |x| x + x));
        }
    } else if lhs == "old" {
        let rhs = rhs
            .parse::<usize>()
            .map_err(|_| "cannot parse rhs".to_string())?;
        if op == "*" {
            return Ok(Box::new(move |x| x * rhs));
        } else if op == "+" {
            return Ok(Box::new(move |x| x + rhs));
        }
    } else if rhs == "old" {
        let lhs = lhs
            .parse::<usize>()
            .map_err(|_| "cannot parse lhs".to_string())?;
        if op == "*" {
            return Ok(Box::new(move |x| lhs * x));
        } else if op == "+" {
            return Ok(Box::new(move |x| lhs + x));
        }
    } else {
        let lhs = lhs
            .parse::<usize>()
            .map_err(|_| "cannot parse lhs".to_string())?;
        let rhs = rhs
            .parse::<usize>()
            .map_err(|_| "cannot parse rhs".to_string())?;
        if op == "*" {
            return Ok(Box::new(move |_| lhs * rhs));
        } else if op == "+" {
            return Ok(Box::new(move |_| lhs + rhs));
        }
    }
    Err("cannot parse operation".to_string())
}
