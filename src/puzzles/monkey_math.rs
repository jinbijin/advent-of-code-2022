mod cacophony;
mod monkey;
mod monkey_job;
mod symphony;

use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines},
    puzzles::monkey_math::{cacophony::Cacophony, monkey_job::MonkeyJob},
};

use self::symphony::Symphony;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn monkey_math(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(monkey_jobs) = input.file_contents.parse::<ByLines<MonkeyJob>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => {
            let mut cacophony = Cacophony::new();
            cacophony.process(monkey_jobs);
            match cacophony.get_root() {
                Some(root) => root,
                None => 0,
            }
        }
        PuzzlePart::Part2 => {
            let mut symphony = Symphony::new();
            symphony.process(monkey_jobs);
            match symphony.get_human() {
                Some(human) => human,
                None => 0,
            }
        }
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = monkey_math(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("152", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = monkey_math(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("301", output);
        Ok(())
    }
}
