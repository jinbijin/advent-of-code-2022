use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub enum ParseSupplyInputLineError {
    CrateLayerIncorrectlySeparated,
    CrateLayerInvalidCrate { index: usize, value: String },
    CrateLayerDoesNotFitBelowPrevious(Vec<usize>),
    StackMappingNotFound,
    StackMappingNotFollowedByEmptyLine,
    MoveInstructionInvalidFormat,
    MoveInstructionCountParseError(String),
    MoveInstructionFromParseError(String),
    MoveInstructionToParseError(String),
}

impl Display for ParseSupplyInputLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CrateLayerIncorrectlySeparated => {
                write!(f, "contains non-space character where space is expected")
            }
            Self::CrateLayerInvalidCrate { index, value } => write!(
                f,
                "invalid value {} for crate in column {}",
                value,
                index + 1
            ),
            Self::CrateLayerDoesNotFitBelowPrevious(indices) => write!(
                f,
                "unexpected empty spaces under crates in columns {}",
                indices
                    .iter()
                    .map(|index| { index.to_string() })
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::StackMappingNotFound => write!(f, "did not find stack mapping"),
            Self::StackMappingNotFollowedByEmptyLine => {
                write!(f, "expected empty line after stack mapping")
            }
            Self::MoveInstructionInvalidFormat => write!(
                f,
                "does not satisfy format 'move <count> from <from> to <to>'"
            ),
            Self::MoveInstructionCountParseError(value) => write!(f, "invalid count '{}'", value),
            Self::MoveInstructionFromParseError(value) => write!(f, "invalid from '{}'", value),
            Self::MoveInstructionToParseError(value) => write!(f, "invalid to '{}'", value),
        }
    }
}

impl Debug for ParseSupplyInputLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseSupplyInputLineError {}

enum ParseSupplyInputMode {
    ParsingCrateArrangement,
    ExpectEmptyLine,
    ParsingMoveInstruction,
}

pub enum SupplyInputLine {
    CrateLayerLine(Vec<Option<char>>),
    StackMappingLine(Vec<char>),
    EmptyLine,
    MoveInstructionLine { count: usize, from: char, to: char },
}

pub struct ParseSupplyInputIterator<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    iterator: T,
    mode: ParseSupplyInputMode,
    previous_crate_layer: Option<Vec<Option<char>>>,
}

impl<'a, T> Iterator for ParseSupplyInputIterator<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    type Item = Result<SupplyInputLine, ParseSupplyInputLineError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mode {
            ParseSupplyInputMode::ParsingCrateArrangement => {
                let line = self.iterator.next();
                if let Some(line) = line {
                    if is_stack_mapping_line(&line) {
                        self.mode = ParseSupplyInputMode::ExpectEmptyLine;
                        self.previous_crate_layer = None;
                        return Some(Ok(SupplyInputLine::StackMappingLine(
                            line.chars().filter(|c| *c != ' ').collect::<Vec<char>>(),
                        )));
                    } else if is_candidate_crate_layer_line(&line) {
                        let crates = line
                            .chars()
                            .enumerate()
                            .filter_map(|(index, c)| if index % 4 == 1 { Some(c) } else { None })
                            .map(|c| if c == ' ' { None } else { Some(c) })
                            .collect::<Vec<Option<char>>>();
                        self.previous_crate_layer = Some(crates.clone());
                        return Some(Ok(SupplyInputLine::CrateLayerLine(crates)));
                    } else {
                        return Some(Err(
                            ParseSupplyInputLineError::CrateLayerIncorrectlySeparated,
                        ));
                    }
                } else {
                    return Some(Err(ParseSupplyInputLineError::StackMappingNotFound));
                }
            }
            ParseSupplyInputMode::ExpectEmptyLine => {
                let line = self.iterator.next();
                if let Some(line) = line {
                    if line.is_empty() {
                        self.mode = ParseSupplyInputMode::ParsingMoveInstruction;
                        self.previous_crate_layer = None;
                        return Some(Ok(SupplyInputLine::EmptyLine));
                    } else {
                        return Some(Err(
                            ParseSupplyInputLineError::StackMappingNotFollowedByEmptyLine,
                        ));
                    }
                } else {
                    return Some(Err(
                        ParseSupplyInputLineError::StackMappingNotFollowedByEmptyLine,
                    ));
                };
            }
            ParseSupplyInputMode::ParsingMoveInstruction => {
                let line = self.iterator.next();
                if let Some(line) = line {
                    let parts = line.split(' ').collect::<Vec<&str>>();
                    if parts.len() != 6
                        || parts[0] != "move"
                        || parts[2] != "from"
                        || parts[4] != "to"
                    {
                        return Some(Err(ParseSupplyInputLineError::MoveInstructionInvalidFormat));
                    }
                    let count = match parts[1].parse::<usize>() {
                        Ok(value) => value,
                        Err(_) => {
                            return Some(Err(
                                ParseSupplyInputLineError::MoveInstructionCountParseError(
                                    parts[1].to_string(),
                                ),
                            ));
                        }
                    };
                    let from = match parts[3].parse::<char>() {
                        Ok(value) => value,
                        Err(_) => {
                            return Some(Err(
                                ParseSupplyInputLineError::MoveInstructionFromParseError(
                                    parts[3].to_string(),
                                ),
                            ));
                        }
                    };
                    let to = match parts[5].parse::<char>() {
                        Ok(value) => value,
                        Err(_) => {
                            return Some(Err(
                                ParseSupplyInputLineError::MoveInstructionFromParseError(
                                    parts[5].to_string(),
                                ),
                            ));
                        }
                    };
                    return Some(Ok(SupplyInputLine::MoveInstructionLine { count, from, to }));
                } else {
                    return None;
                };
            }
        }
    }
}

impl<'a, T> From<T> for ParseSupplyInputIterator<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn from(iterator: T) -> Self {
        ParseSupplyInputIterator {
            iterator,
            mode: ParseSupplyInputMode::ParsingCrateArrangement,
            previous_crate_layer: None,
        }
    }
}

fn is_stack_mapping_line(value: &str) -> bool {
    value.len() % 4 == 3
        && value
            .chars()
            .enumerate()
            .all(|(index, value)| (index % 4 == 1 && value != ' ') || value == ' ')
}

fn is_candidate_crate_layer_line(value: &str) -> bool {
    value.len() % 4 == 3
        && value
            .chars()
            .enumerate()
            .filter_map(
                |(index, value)| {
                    if index % 4 == 3 {
                        Some(value)
                    } else {
                        None
                    }
                },
            )
            .all(|value| value == ' ')
}
