use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use crate::common::operation::{Operation, ParseOperationError};

use super::monkey::{Monkey, ParseMonkeyError};

#[derive(Debug)]
pub enum ParseJobError {
    InvalidFormat,
    InvalidMonkey,
    InvalidOperation,
    InvalidYell,
}

impl From<ParseMonkeyError> for ParseJobError {
    fn from(_: ParseMonkeyError) -> Self {
        Self::InvalidMonkey
    }
}

impl From<ParseOperationError> for ParseJobError {
    fn from(_: ParseOperationError) -> Self {
        Self::InvalidOperation
    }
}

impl From<ParseIntError> for ParseJobError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidYell
    }
}

impl Display for ParseJobError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidYell => write!(f, "invalid yell"),
            Self::InvalidMonkey => write!(f, "invalid monkey"),
            Self::InvalidOperation => write!(f, "invalid operation"),
        }
    }
}

pub enum Job {
    Yell(i64),
    Wait {
        lhs: Monkey,
        rhs: Monkey,
        op: Operation,
    },
}

impl FromStr for Job {
    type Err = ParseJobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts.len() == 1 {
            let yell = parts[0].parse::<i64>()?;
            Ok(Job::Yell(yell))
        } else if parts.len() == 3 {
            let lhs = parts[0].parse::<Monkey>()?;
            let op = parts[1].parse::<Operation>()?;
            let rhs = parts[2].parse::<Monkey>()?;
            Ok(Job::Wait { lhs, rhs, op })
        } else {
            Err(Self::Err::InvalidFormat)
        }
    }
}

#[derive(Debug)]
pub enum ParseMonkeyJobError {
    InvalidFormat,
    InvalidMonkey,
    InvalidJob(ParseJobError),
}

impl From<ParseMonkeyError> for ParseMonkeyJobError {
    fn from(_: ParseMonkeyError) -> Self {
        ParseMonkeyJobError::InvalidMonkey
    }
}

impl From<ParseJobError> for ParseMonkeyJobError {
    fn from(err: ParseJobError) -> Self {
        ParseMonkeyJobError::InvalidJob(err)
    }
}

impl Display for ParseMonkeyJobError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidMonkey => write!(f, "invalid monkey"),
            Self::InvalidJob(err) => write!(f, "invalid job: {}", err),
        }
    }
}

impl Error for ParseMonkeyJobError {}

pub struct MonkeyJob {
    pub monkey: Monkey,
    pub job: Job,
}

impl FromStr for MonkeyJob {
    type Err = ParseMonkeyJobError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (monkey, job) = match s.split_once(": ") {
            Some(pair) => Ok(pair),
            None => Err(Self::Err::InvalidFormat),
        }?;
        let monkey = monkey.parse::<Monkey>()?;
        let job = job.parse::<Job>()?;

        Ok(MonkeyJob { monkey, job })
    }
}
