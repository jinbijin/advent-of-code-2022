mod elf_diffuser;
mod elf_distribution;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError, SingleSection},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    puzzles::unstable_diffusion::{elf_diffuser::AsDiffuser, elf_distribution::ElfDistribution},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn unstable_diffusion(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let SingleSection(elf_distribution) = input
        .file_contents
        .as_str()
        .parse_contents::<SingleSection<ElfDistribution>>()?;
    let mut diffuser = elf_distribution.diffuser();
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => {
            for _ in 0..10 {
                diffuser.diffuse();
            }
            diffuser.covered_ground()
        }
        PuzzlePart::Part2 => {
            let mut count: isize = 0;
            while let Some(_) = diffuser.diffuse() {
                count += 1;
            }
            count + 1
        }
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = unstable_diffusion(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("110", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = unstable_diffusion(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("20", output);
        Ok(())
    }
}
