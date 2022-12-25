mod snafu;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::ParseContentsError, input::puzzle_input::PuzzleInput,
    parse::lines::ByLines, puzzles::full_of_hot_air::snafu::Snafu,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn full_of_hot_air(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(snafus) = input.file_contents.parse::<ByLines<Snafu>>()?;
    let answer = snafus.into_iter().sum::<Snafu>();
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use crate::input::puzzle_part::PuzzlePart;

    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = full_of_hot_air(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("2=-1=0", output);
        Ok(())
    }
}
