pub mod calorie_counting;
pub mod rock_paper_scissors;
pub mod rucksack_reorganization;

use std::{error::Error, fs, str::FromStr};

use self::{
    calorie_counting::CalorieCountingArgs,
    rock_paper_scissors::{RockPaperScissorsArgType, RockPaperScissorsArgs},
    rucksack_reorganization::RucksackReorganizationArgs,
};

pub enum PuzzleInput {
    CalorieCounting(CalorieCountingArgs),
    RockPaperScissors(RockPaperScissorsArgs),
    RucksackReorganization(RucksackReorganizationArgs),
}

impl PuzzleInput {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<PuzzleInput, &'static str> {
        args.next();
        let puzzle = match_argument::<String>(args)?;
        match_puzzle(&puzzle, args, "Unrecognized value found for 'puzzle'.")
    }

    pub fn run_solution(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("input/{}.txt", self.file_name());
        let file_contents = fs::read_to_string(file_name)?;
        let output = match self {
            Self::CalorieCounting(args) => calorie_counting::main(file_contents, args),
            Self::RockPaperScissors(args) => rock_paper_scissors::main(file_contents, args),
            Self::RucksackReorganization(args) => {
                rucksack_reorganization::main(file_contents, args)
            }
        }?;
        println!("The answer is: {}", output);
        Ok(())
    }

    pub fn file_name(&self) -> &str {
        match self {
            Self::CalorieCounting(_) => "calorie_counting",
            Self::RockPaperScissors(_) => "rock_paper_scissors",
            Self::RucksackReorganization(_) => "rucksack_reorganization",
        }
    }
}

fn match_argument<'a, T>(args: &mut impl Iterator<Item = String>) -> Result<T, &'a str>
where
    T: FromStr,
{
    match args.next() {
        Some(arg) => arg.parse::<T>().map_err(|_err| "Could not parse argument."),
        None => Err("Missing argument."),
    }
}

fn match_puzzle<'a>(
    puzzle: &String,
    args: &mut impl Iterator<Item = String>,
    error_message: &'a str,
) -> Result<PuzzleInput, &'a str> {
    let puzzle = puzzle.to_lowercase();
    if puzzle == "calorie_counting" {
        let count = match_argument::<usize>(args)?;
        Ok(PuzzleInput::CalorieCounting(CalorieCountingArgs { count }))
    } else if puzzle == "rock_paper_scissors" {
        let arg_type = match match_argument::<String>(args)?.as_str() {
            "regular" => Ok(RockPaperScissorsArgType::Regular),
            "reverse" => Ok(RockPaperScissorsArgType::Reverse),
            _ => Err(error_message),
        }?;

        Ok(PuzzleInput::RockPaperScissors(RockPaperScissorsArgs {
            arg_type,
        }))
    } else if puzzle == "rucksack_reorganization" {
        let arg_type = match match_argument::<String>(args)?.as_str() {
            "compartments" => Ok(RucksackReorganizationArgs::Compartments),
            "rucksack_groups" => Ok(RucksackReorganizationArgs::RucksackGroups),
            _ => Err(error_message),
        }?;

        Ok(PuzzleInput::RucksackReorganization(arg_type))
    } else {
        Err(error_message)
    }
}
