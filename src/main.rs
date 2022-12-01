use advent_of_code_2022::{
    parse,
    puzzles::{calorie_counting, PuzzleInput},
};
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let config = PuzzleInput::build(&mut env::args())?;
    match config {
        PuzzleInput::CalorieCounting(file_name, count) => solve_calorie_counting(file_name, count),
    }?;
    Ok(())
}

fn solve_calorie_counting(file_name: String, count: usize) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name)?;
    let calories_lines = parse::parse_as_newline_separated::<i32>(contents)?;
    let calories =
        calorie_counting::sum_of_top_group_sums(&mut calories_lines.into_iter(), count);
    println!("The top {} elves carry {} calories.", count, calories);
    Ok(())
}
