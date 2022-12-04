use advent_of_code_2022::input::PuzzleArgs;
use std::{convert::Infallible, env, error::Error, process};

fn main() -> Result<(), Infallible> {
    body().map_err(|err| handle_error(err))
}

fn body() -> Result<(), Box<dyn Error>> {
    PuzzleArgs::build(&mut env::args())?.run_solution()?;
    Ok(())
}

fn handle_error(error: Box<dyn Error>) -> ! {
    eprintln!("{}", error);
    let mut inner_error_option = error.source();
    while let Some(inner_error) = inner_error_option {
        eprintln!("{}", inner_error);
        inner_error_option = inner_error.source();
    }
    process::exit(1);
}
