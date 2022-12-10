mod command_line;
mod file_tree;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use self::{command_line::CommandLine, file_tree::Directory};
use crate::{
    contents::{convert::AsParseLines, errors::ParseContentsError},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn no_space_left_on_device(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let command_lines = input
        .file_contents
        .as_str()
        .parse_lines::<Vec<CommandLine>>()?;
    let file_system = command_lines.into_iter().collect::<Directory>();
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => {
            let sizes = file_system.get_directories_smaller_than(100000);
            sizes.iter().map(|(_, size)| size).sum::<usize>()
        }
        PuzzlePart::Part2 => {
            let main_directory_size = file_system.get_size();
            let target_size = main_directory_size - 40000000;
            let mut size = file_system.get_directories_larger_than(target_size);
            size.sort_by(|(_, x), (_, y)| x.cmp(y));
            let (_, size) = size[0];
            size
        }
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = no_space_left_on_device(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("95437", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = no_space_left_on_device(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("24933642", output);
        Ok(())
    }
}
