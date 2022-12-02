use advent_of_code_2022::{
    parse,
    puzzles::{
        calorie_counting,
        rock_paper_scissors::{self, RpsDesiredResult, RpsMatch},
        PuzzleInput,
    },
};
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let config = PuzzleInput::build(&mut env::args())?;
    match config {
        PuzzleInput::CalorieCounting(file_name, count) => solve_calorie_counting(file_name, count),
        PuzzleInput::RockPaperScissors(file_name) => solve_rock_paper_scissors(file_name),
        PuzzleInput::RockPaperScissorsReverse(file_name) => {
            solve_rock_paper_scissors_reverse(file_name)
        }
    }?;
    Ok(())
}

fn solve_calorie_counting(file_name: String, count: usize) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;
    let calories_lines = parse::parse_as_newline_separated_option::<i32>(contents)?;
    let calories = calorie_counting::sum_of_top_group_sums(&mut calories_lines.into_iter(), count);
    println!("The top {} elves carry {} calories.", count, calories);
    Ok(())
}

fn solve_rock_paper_scissors(file_name: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;
    let rps_matches = parse::parse_as_newline_separated::<RpsMatch>(contents)?;
    let score = rock_paper_scissors::get_tournament_score(&mut rps_matches.into_iter());
    println!("The total score is {}", score);
    Ok(())
}

fn solve_rock_paper_scissors_reverse(file_name: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;
    let rps_matches = parse::parse_as_newline_separated::<RpsDesiredResult>(contents)?;
    let score = rock_paper_scissors::get_tournament_score(&mut rps_matches.into_iter());
    println!("The total score is {}", score);
    Ok(())
}
