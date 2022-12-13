mod packet;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::{
    contents::convert::contents::{AsParseContents, ParseContentsError},
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
};

use self::packet::Packet;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn distress_signal(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let packet_groups = input
        .file_contents
        .as_str()
        .parse_contents::<Vec<Vec<Packet>>>()?;
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => packet_groups
            .into_iter()
            .enumerate()
            .filter_map(|(index, packet_group)| {
                if packet_group[0].cmp(&packet_group[1]).is_lt() {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .sum::<usize>(),
        PuzzlePart::Part2 => {
            let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Constant(2)])]);
            let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Constant(6)])]);
            let mut packets = packet_groups.into_iter().flatten().collect::<Vec<Packet>>();
            packets.push(divider_1.clone());
            packets.push(divider_2.clone());
            packets.sort();
            packets
                .into_iter()
                .enumerate()
                .filter_map(|(index, item)| {
                    if item == divider_1 || item == divider_2 {
                        Some(index + 1)
                    } else {
                        None
                    }
                })
                .product::<usize>()
        }
    };
    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = distress_signal(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("13", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = distress_signal(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("140", output);
        Ok(())
    }
}
