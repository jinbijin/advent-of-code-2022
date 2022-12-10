use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseSignalChangeError {
    EmptyLine,
    InvalidOperation(String),
    AddMissingValue,
    AddInvalidValue(String),
}

impl Display for ParseSignalChangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLine => write!(f, "unexpected empty line"),
            Self::InvalidOperation(value) => write!(f, "invalid operation '{}'", value),
            Self::AddMissingValue => write!(f, "missing value for add operation"),
            Self::AddInvalidValue(value) => {
                write!(f, "invalid value '{}' for add operation", value)
            }
        }
    }
}

impl Debug for ParseSignalChangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseSignalChangeError {}

pub enum SignalChange {
    Noop,
    Add { register: String, value: i32 },
}

impl SignalChange {
    pub fn get_value_changes(&self, register_name: &str) -> Vec<i32> {
        match self {
            Self::Noop => vec![0],
            Self::Add { register, value } => {
                if register.as_str() == register_name {
                    vec![0, *value]
                } else {
                    vec![0, 0]
                }
            }
        }
    }
}

impl FromStr for SignalChange {
    type Err = ParseSignalChangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let operation = tokens.next();

        if let Some(operation) = operation {
            if operation == "noop" {
                Ok(Self::Noop)
            } else if operation.starts_with("add") {
                let register = operation[3..].to_string();
                let count = tokens.next();
                if let Some(count) = count {
                    match count.parse::<i32>() {
                        Ok(value) => Ok(Self::Add { register, value }),
                        Err(_) => Err(Self::Err::AddInvalidValue(count.to_string())),
                    }
                } else {
                    Err(Self::Err::AddMissingValue)
                }
            } else {
                Err(Self::Err::InvalidOperation(operation.to_string()))
            }
        } else {
            Err(Self::Err::EmptyLine)
        }
    }
}
