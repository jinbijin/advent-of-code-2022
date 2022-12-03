mod lib;

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
    vec::IntoIter,
};

use crate::file::{self, FileErrorCollection};

use self::lib::{Rucksack, VectorChunkIterator};

pub enum ParseRucksackReorganizationArgsError {
    InvalidValue(String),
}

impl Display for ParseRucksackReorganizationArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self::InvalidValue(value) = self;
        write!(
            f,
            "Invalid option '{}' for puzzle 'rucksack_reorganization'",
            value
        )
    }
}

impl Debug for ParseRucksackReorganizationArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl Error for ParseRucksackReorganizationArgsError {}

pub enum RucksackReorganizationArgs {
    Compartments,
    RucksackGroups,
}

impl FromStr for RucksackReorganizationArgs {
    type Err = ParseRucksackReorganizationArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compartments" => Ok(Self::Compartments),
            "rucksack_groups" => Ok(Self::RucksackGroups),
            _ => Err(Self::Err::InvalidValue(s.to_string())),
        }
    }
}

pub fn main(
    file_contents: String,
    args: &RucksackReorganizationArgs,
) -> Result<Box<dyn Display>, FileErrorCollection<<Rucksack as FromStr>::Err>> {
    let rucksacks = file::parse_lines::<Rucksack>(file_contents)?;
    let answer = match args {
        RucksackReorganizationArgs::Compartments => rucksacks
            .into_iter()
            .map(|rucksack| lib::find_common_item(rucksack.compartments()).priority())
            .sum::<i32>(),
        RucksackReorganizationArgs::RucksackGroups => {
            VectorChunkIterator::<3, Rucksack, IntoIter<Rucksack>> {
                iterator: &mut rucksacks.into_iter(),
            }
            .map(|group| lib::find_common_item(group).priority())
            .sum::<i32>()
        }
    };

    Ok(Box::new(answer))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    const INPUT_TEXT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(
            INPUT_TEXT.to_string(),
            &RucksackReorganizationArgs::Compartments,
        )?;

        assert_eq!("157", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(
            INPUT_TEXT.to_string(),
            &RucksackReorganizationArgs::RucksackGroups,
        )?;

        assert_eq!("70", output.to_string());
        Ok(())
    }
}
