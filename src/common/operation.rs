use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

pub enum ParseOperationError {
    InvalidOperation,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn operation<T, Rhs, Output>(self) -> Box<dyn Fn(T, Rhs) -> Output>
    where
        T: Add<Rhs, Output = Output>
            + Sub<Rhs, Output = Output>
            + Mul<Rhs, Output = Output>
            + Div<Rhs, Output = Output>,
    {
        match self {
            Self::Add => Box::new(|x, y| x + y),
            Self::Subtract => Box::new(|x, y| x - y),
            Self::Multiply => Box::new(|x, y| x * y),
            Self::Divide => Box::new(|x, y| x / y),
        }
    }

    pub fn find_lhs<T, Rhs, Output>(self) -> Box<dyn Fn(T, Rhs) -> Output>
    where
        T: Add<Rhs, Output = Output>
            + Sub<Rhs, Output = Output>
            + Mul<Rhs, Output = Output>
            + Div<Rhs, Output = Output>,
    {
        match self {
            Self::Add => Box::new(|tgt, rhs| tgt - rhs),
            Self::Subtract => Box::new(|tgt, rhs| tgt + rhs),
            Self::Multiply => Box::new(|tgt, rhs| tgt / rhs),
            Self::Divide => Box::new(|tgt, rhs| tgt * rhs),
        }
    }

    pub fn find_rhs<T>(self) -> Box<dyn Fn(T, T) -> T>
    where
        T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
    {
        match self {
            Self::Add => Box::new(|tgt, lhs| tgt - lhs),
            Self::Subtract => Box::new(|tgt, lhs| lhs - tgt),
            Self::Multiply => Box::new(|tgt, lhs| tgt / lhs),
            Self::Divide => Box::new(|tgt, lhs| lhs * tgt),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(Self::Err::InvalidOperation),
        }
    }
}
