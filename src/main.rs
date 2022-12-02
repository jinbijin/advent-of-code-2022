use advent_of_code_2022::puzzles::PuzzleInput;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    PuzzleInput::build(&mut env::args())?.run_solution()
}
