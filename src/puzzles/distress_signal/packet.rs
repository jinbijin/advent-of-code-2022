use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParsePacketError {
    InvalidItem(String),
}

impl Display for ParsePacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidItem(value) => write!(f, "invalid integer '{}'", value),
        }
    }
}

impl Debug for ParsePacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParsePacketError {}

// Also allows for a packet to be a single integer
// TODO learn how to use the Borrow trait for this
#[derive(Debug, Clone)]
pub enum Packet {
    Constant(usize),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') && s.ends_with(']') {
            let entries = split_lists(&s[1..(s.len() - 1)])
                .into_iter()
                .map(|item| Self::from_str(item))
                .collect::<Result<Vec<Packet>, ParsePacketError>>()?;
            Ok(Self::List(entries))
        } else {
            match s.parse::<usize>() {
                Ok(constant) => Ok(Self::Constant(constant)),
                Err(_) => Err(Self::Err::InvalidItem(s.to_string())),
            }
        }
    }
}

fn split_lists(s: &str) -> Vec<&str> {
    let mut last_split = 0;
    let mut bracket_depth = 0;
    let mut current = 0;
    let mut items: Vec<&str> = Vec::new();
    for c in s.chars() {
        if c == '[' {
            bracket_depth += 1;
        } else if c == ']' {
            bracket_depth -= 1;
        } else if c == ',' && bracket_depth == 0 {
            items.push(&s[last_split..current]);
            last_split = current + 1;
        }

        current += c.len_utf8();
    }
    if last_split < current {
        items.push(&s[last_split..current]);
    }
    items
}

impl Packet {
    pub fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Constant(x), Self::Constant(y)) => x.cmp(y),
            (Self::Constant(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Constant(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::List(xs), Self::List(ys)) => {
                let mut i: usize = 0;
                let mut result = Ordering::Equal;
                while result == Ordering::Equal {
                    if xs.len() == i && ys.len() == i {
                        return Ordering::Equal;
                    } else if xs.len() == i {
                        return Ordering::Less;
                    } else if ys.len() == i {
                        return Ordering::Greater;
                    } else {
                        result = xs[i].cmp(&ys[i]);
                    }
                    i += 1;
                }
                result
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}
