use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

enum ParseArgumentError {
    InvalidValue(String),
}

impl Display for ParseArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(value) => write!(f, "invalid operation argument '{}'", value),
        }
    }
}

impl Debug for ParseArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseArgumentError {}

enum Argument {
    Parameter,
    Constant(usize),
}

impl Argument {
    fn as_fn(&self) -> Box<dyn Fn(usize) -> usize> {
        match self {
            Self::Parameter => Box::new(|x| x),
            Self::Constant(value) => {
                let value = *value;
                Box::new(move |_| value)
            }
        }
    }
}

impl FromStr for Argument {
    type Err = ParseArgumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Parameter)
        } else if let Ok(constant) = s.parse::<usize>() {
            Ok(Self::Constant(constant))
        } else {
            Err(Self::Err::InvalidValue(s.to_string()))
        }
    }
}

enum ParseOperationKindError {
    InvalidValue(String),
}

impl Display for ParseOperationKindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(value) => write!(f, "invalid operation kind '{}'", value),
        }
    }
}

impl Debug for ParseOperationKindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseOperationKindError {}

enum OperationKind {
    Plus,
    Times,
}

impl OperationKind {
    fn as_fn(&self) -> Box<dyn Fn(usize, usize) -> usize> {
        match self {
            Self::Plus => Box::new(|x, y| x + y),
            Self::Times => Box::new(|x, y| x * y),
        }
    }
}

impl FromStr for OperationKind {
    type Err = ParseOperationKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Times),
            "+" => Ok(Self::Plus),
            _ => Err(Self::Err::InvalidValue(s.to_string())),
        }
    }
}

pub enum ParseOperationError {
    InvalidFormat(String),
    InvalidArgument(String),
    InvalidOperationKind(String),
}

impl Display for ParseOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => write!(f, "line '{}' is in invalid format", value),
            Self::InvalidOperationKind(value) => write!(f, "invalid operation kind '{}'", value),
            Self::InvalidArgument(value) => write!(f, "invalid argument '{}'", value),
        }
    }
}

impl Debug for ParseOperationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseOperationError {}

pub struct Operation {
    lhs: Argument,
    op: OperationKind,
    rhs: Argument,
}

impl Operation {
    pub fn as_fn(&self) -> Box<dyn Fn(usize) -> usize> {
        let lhs = self.lhs.as_fn();
        let op = self.op.as_fn();
        let rhs = self.rhs.as_fn();
        Box::new(move |x| op(lhs(x), rhs(x)))
    }
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = "  Operation: new = ";
        if s.starts_with(prefix) {
            let parts = s[(prefix.len())..].split(' ').collect::<Vec<&str>>();
            if parts.len() == 3 {
                let lhs = parts[0].parse::<Argument>().map_err(
                    |ParseArgumentError::InvalidValue(value)| Self::Err::InvalidArgument(value),
                )?;
                let op = parts[1].parse::<OperationKind>().map_err(
                    |ParseOperationKindError::InvalidValue(value)| {
                        Self::Err::InvalidOperationKind(value)
                    },
                )?;
                let rhs = parts[2].parse::<Argument>().map_err(
                    |ParseArgumentError::InvalidValue(value)| Self::Err::InvalidArgument(value),
                )?;
                Ok(Operation { lhs, op, rhs })
            } else {
                Err(Self::Err::InvalidFormat(s.to_string()))
            }
        } else {
            Err(Self::Err::InvalidFormat(s.to_string()))
        }
    }
}
