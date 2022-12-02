pub mod calorie_counting;
pub mod rock_paper_scissors;

use std::str::FromStr;

pub enum PuzzleInput {
    CalorieCounting(String, usize),
    RockPaperScissors(String),
    RockPaperScissorsReverse(String),
}

impl PuzzleInput {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<PuzzleInput, &'static str> {
        args.next();
        let puzzle = match_argument::<String>(args)?;
        match_puzzle(&puzzle, args, "Unrecognized value found for 'puzzle'.")
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
        let file_name = match_argument::<String>(args)?;
        Ok(PuzzleInput::CalorieCounting(file_name, count))
    } else if puzzle == "rock_paper_scissors" {
        let file_name = match_argument::<String>(args)?;
        Ok(PuzzleInput::RockPaperScissors(file_name))
    } else if puzzle == "rock_paper_scissors_reverse" {
        let file_name = match_argument::<String>(args)?;
        Ok(PuzzleInput::RockPaperScissorsReverse(file_name))
    } else {
        Err(error_message)
    }
}
