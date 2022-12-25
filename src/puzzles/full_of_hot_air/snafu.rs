use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    iter::Sum,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug)]
pub enum ParseSnafuError {
    InvalidDigit,
}

impl Display for ParseSnafuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDigit => write!(f, "invalid digit"),
        }
    }
}

impl Error for ParseSnafuError {}

pub struct Snafu(pub isize);

impl FromStr for Snafu {
    type Err = ParseSnafuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_value: isize = 0;

        for c in s.chars() {
            parsed_value *= 5;
            match c {
                '=' => parsed_value -= 2,
                '-' => parsed_value -= 1,
                '0' => {}
                '1' => parsed_value += 1,
                '2' => parsed_value += 2,
                _ => return Err(ParseSnafuError::InvalidDigit),
            }
        }

        Ok(Snafu(parsed_value))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Snafu(value) = self;
        let mut value = *value;
        let mut chars: Vec<char> = Vec::new();

        // This implementation does not work for negative integers
        while value != 0 {
            value += 2;
            chars.push(match value % 5 {
                0 => '=',
                1 => '-',
                2 => '0',
                3 => '1',
                4 => '2',
                _ => unreachable!(),
            });
            value /= 5;
        }

        for c in chars.iter().rev() {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        Snafu(value)
    }
}

impl Add<Snafu> for Snafu {
    type Output = Snafu;

    fn add(self, rhs: Snafu) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

impl AddAssign<Snafu> for Snafu {
    fn add_assign(&mut self, rhs: Snafu) {
        self.0 += rhs.0;
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut snafu = Snafu(0);

        for i in iter {
            snafu += i;
        }

        snafu
    }
}
