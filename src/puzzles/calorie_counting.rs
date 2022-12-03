mod lib;

use crate::file::{self, FileErrorCollection};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

pub struct CalorieCountingArgs {
    pub count: usize,
}

impl FromStr for CalorieCountingArgs {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.parse::<usize>()?;
        Ok(Self { count })
    }
}

pub fn main(
    file_contents: String,
    args: &CalorieCountingArgs,
) -> Result<Box<dyn Display>, FileErrorCollection<<i32 as FromStr>::Err>> {
    let calories_lines = file::parse_optional_lines::<i32>(file_contents)?;
    let calories = lib::sum_of_top_group_sums(&mut calories_lines.into_iter(), args.count);
    Ok(Box::new(calories))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn example_1_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &CalorieCountingArgs { count: 1 })?;

        assert_eq!("24000", output.to_string());
        Ok(())
    }

    #[test]
    fn example_2_should_be_correct() -> Result<(), Box<dyn Error>> {
        let output = main(INPUT_TEXT.to_string(), &CalorieCountingArgs { count: 3 })?;

        assert_eq!("45000", output.to_string());
        Ok(())
    }
}
