use std::{
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseMonkeyError {
    InvalidLength,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Monkey([char; 4]);

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() == 4 {
            Ok(Monkey([chars[0], chars[1], chars[2], chars[3]]))
        } else {
            Err(Self::Err::InvalidLength)
        }
    }
}

impl Monkey {
    pub fn root() -> Monkey {
        Monkey(['r', 'o', 'o', 't'])
    }

    pub fn human() -> Monkey {
        Monkey(['h', 'u', 'm', 'n'])
    }
}
