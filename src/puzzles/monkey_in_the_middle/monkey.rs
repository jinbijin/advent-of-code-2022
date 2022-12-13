use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use crate::contents::convert::{
    contents::{AsParseSections, FromContents, ParseContentsError},
    sections::{CustomSectionError, FromLines},
};

use super::{
    divisor::{Divisor, ParseDivisorError},
    if_false_throw_to::{IfFalseThrowTo, ParseIfFalseThrowToError},
    if_true_throw_to::{IfTrueThrowTo, ParseIfTrueThrowToError},
    monkey_name::{MonkeyName, ParseMonkeyNameError},
    operation::{Operation, ParseOperationError},
    starting_items::{ParseStartingItemsError, StartingItems},
};

pub enum ParseMonkeyError {
    InvalidMonkeyNameFormat(String),
    InvalidStartingItemFormat(String),
    InvalidStartingItem(String),
    InvalidOperationFormat(String),
    InvalidOperationKind(String),
    InvalidOperationArgument(String),
    InvalidDivisorFormat(String),
    InvalidIfFalseThrowToFormat(String),
    InvalidIfTrueThrowToFormat(String),
    UnexpectedEndOfLine(String),
}

impl Display for ParseMonkeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMonkeyNameFormat(value) => {
                write!(f, "invalid monkey name format for '{}'", value)
            }
            Self::InvalidStartingItemFormat(value) => {
                write!(f, "invalid starting item format for '{}'", value)
            }
            Self::InvalidStartingItem(value) => write!(f, "invalid starting item '{}'", value),
            Self::InvalidOperationFormat(value) => {
                write!(f, "invalid operation format for '{}'", value)
            }
            Self::InvalidOperationKind(value) => write!(f, "invalid operation kind '{}'", value),
            Self::InvalidOperationArgument(value) => {
                write!(f, "invalid operation argument '{}'", value)
            }
            Self::InvalidDivisorFormat(value) => write!(f, "invalid test format for '{}'", value),
            Self::InvalidIfTrueThrowToFormat(value) => {
                write!(f, "invalid if true throw to format for '{}'", value)
            }
            Self::InvalidIfFalseThrowToFormat(value) => {
                write!(f, "invalid if false throw to format for '{}'", value)
            }
            Self::UnexpectedEndOfLine(while_reading) => write!(
                f,
                "unexpected end of section while reading '{}'",
                while_reading
            ),
        }
    }
}

impl Debug for ParseMonkeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseMonkeyError {}

impl CustomSectionError for ParseMonkeyError {}

impl From<ParseMonkeyNameError> for ParseMonkeyError {
    fn from(err: ParseMonkeyNameError) -> Self {
        match err {
            ParseMonkeyNameError::InvalidFormat(value) => Self::InvalidMonkeyNameFormat(value),
        }
    }
}

impl From<ParseStartingItemsError> for ParseMonkeyError {
    fn from(err: ParseStartingItemsError) -> Self {
        match err {
            ParseStartingItemsError::InvalidFormat(value) => Self::InvalidStartingItemFormat(value),
            ParseStartingItemsError::InvalidItem(value) => Self::InvalidStartingItem(value),
        }
    }
}

impl From<ParseOperationError> for ParseMonkeyError {
    fn from(err: ParseOperationError) -> Self {
        match err {
            ParseOperationError::InvalidArgument(value) => Self::InvalidOperationArgument(value),
            ParseOperationError::InvalidFormat(value) => Self::InvalidOperationFormat(value),
            ParseOperationError::InvalidOperationKind(value) => Self::InvalidOperationKind(value),
        }
    }
}

impl From<ParseDivisorError> for ParseMonkeyError {
    fn from(err: ParseDivisorError) -> Self {
        match err {
            ParseDivisorError::InvalidFormat(value) => Self::InvalidDivisorFormat(value),
        }
    }
}

impl From<ParseIfTrueThrowToError> for ParseMonkeyError {
    fn from(err: ParseIfTrueThrowToError) -> Self {
        match err {
            ParseIfTrueThrowToError::InvalidFormat(value) => {
                Self::InvalidIfTrueThrowToFormat(value)
            }
        }
    }
}

impl From<ParseIfFalseThrowToError> for ParseMonkeyError {
    fn from(err: ParseIfFalseThrowToError) -> Self {
        match err {
            ParseIfFalseThrowToError::InvalidFormat(value) => {
                Self::InvalidIfFalseThrowToFormat(value)
            }
        }
    }
}

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

impl FromLines for (String, Monkey) {
    type Err = ParseMonkeyError;

    fn from_lines(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let monkey_key = match lines.next() {
            Some(line) => match line.parse::<MonkeyName>() {
                Ok(MonkeyName(monkey_name)) => Ok(monkey_name),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!(
                "monkey name"
            ))),
        }?;

        let items = match lines.next() {
            Some(line) => match line.parse::<StartingItems>() {
                Ok(StartingItems(items)) => Ok(items
                    .into_iter()
                    .map(|worry_level| MonkeyItem { worry_level })
                    .collect::<VecDeque<MonkeyItem>>()),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!(
                "starting items"
            ))),
        }?;

        let operation = match lines.next() {
            Some(line) => match line.parse::<Operation>() {
                Ok(operation) => Ok(operation.as_fn()),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!("operation"))),
        }?;

        let divisor = match lines.next() {
            Some(line) => match line.parse::<Divisor>() {
                Ok(Divisor(divisor)) => Ok(divisor),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!("test"))),
        }?;

        let throw_to_if_true = match lines.next() {
            Some(line) => match line.parse::<IfTrueThrowTo>() {
                Ok(IfTrueThrowTo(name)) => Ok(name),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!(
                "throw to if true"
            ))),
        }?;

        let throw_to_if_false = match lines.next() {
            Some(line) => match line.parse::<IfFalseThrowTo>() {
                Ok(IfFalseThrowTo(name)) => Ok(name),
                Err(err) => Err(err.into()),
            },
            None => Err(ParseMonkeyError::UnexpectedEndOfLine(format!(
                "throw to if false"
            ))),
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
    fn from_contents(s: &str) -> Result<Self, ParseContentsError> {
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
