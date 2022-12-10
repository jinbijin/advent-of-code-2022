#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::{convert::AsParseContents, errors::ParseContentsError},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn calorie_counting(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let calorie_counts = input
        .file_contents
        .as_str()
        .parse_contents::<Vec<Vec<i32>>>()?;
    let mut calorie_sums = calorie_counts
        .into_iter()
        .map(|group| group.into_iter().sum())
        .collect::<Vec<i32>>();

    // PERF (N: #groups, K: top # needed)
    // Current: N log N
    // Optimal: N log K
    calorie_sums.sort_by(|x, y| y.cmp(x));
    let count = match input.puzzle_part {
        PuzzlePart::Part1 => 1,
        PuzzlePart::Part2 => 3,
    };
    let answer = calorie_sums.into_iter().take(count).sum::<i32>();
    Ok(answer.to_string())
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
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("24000", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = calorie_counting(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("45000", output);
        Ok(())
    }
}
