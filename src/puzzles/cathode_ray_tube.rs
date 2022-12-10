mod signal_change;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use self::signal_change::SignalChange;
use crate::{
    common::vector_chunks::AsVectorChunks,
    contents::{convert::AsParseLines, errors::ParseContentsError},
    input::puzzle_input::PuzzleInput,
    input::puzzle_part::PuzzlePart,
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn cathode_ray_tube(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let signal_changes = input
        .file_contents
        .as_str()
        .parse_lines::<Vec<SignalChange>>()?;
    let signal_strengths = signal_changes
        .into_iter()
        .flat_map(|c| c.get_value_changes("x").into_iter())
        .scan(1, |state, x| {
            let result = Some(*state);
            *state += x;
            result
        });
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => signal_strengths
            .enumerate()
            .filter_map(|(index, value)| {
                if index % 40 == 19 {
                    let index_value = match i32::try_from(index) {
                        Ok(value) => value + 1,
                        Err(_) => 0,
                    };
                    Some(index_value * value)
                } else {
                    None
                }
            })
            .sum::<i32>()
            .to_string(),
        PuzzlePart::Part2 => signal_strengths
            .enumerate()
            .map(|(index, value)| {
                let position = match i32::try_from(index % 40) {
                    Ok(value) => value,
                    Err(_) => 0,
                };
                if position - value >= -1 && position - value <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .vector_chunks::<40>()
            .map(|chunk| {
                let mut result = chunk.into_iter().collect::<String>();
                result.push_str("\n");
                result
            })
            .collect::<String>(),
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = cathode_ray_tube(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("13140", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = cathode_ray_tube(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(expected, output);
        Ok(())
    }
}
