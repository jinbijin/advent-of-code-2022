mod sensor_reading;

use crate::{
    input::{puzzle_input::PuzzleInput, puzzle_part::PuzzlePart},
    parse::{error::ParseContentsError, lines::ByLines},
};

use self::sensor_reading::SensorReading;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// PERF
// For part 2, if I really want to, I could take a polygon-based approach to find the coordinates,
// instead of scanning all 4000000 lines.
// TARGET
// Part 2: ~1 ms (is ~20 s)
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn beacon_exclusion_zone(input: PuzzleInput) -> Result<String, ParseContentsError> {
    let ByLines(readings) = input.file_contents.parse::<ByLines<SensorReading>>()?;
    let scale = scale();
    let answer = match input.puzzle_part {
        PuzzlePart::Part1 => sensor_reading::get_covered_position_count(readings, scale),
        PuzzlePart::Part2 => sensor_reading::scan(readings, scale),
    };

    Ok(answer.to_string())
}

#[cfg(test)]
fn scale() -> isize {
    10
}

#[cfg(not(test))]
fn scale() -> isize {
    2000000
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    const INPUT_TEXT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn example_1() -> Result<(), Box<dyn Error>> {
        let output = beacon_exclusion_zone(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part1,
        })?;

        assert_eq!("26", output);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<(), Box<dyn Error>> {
        let output = beacon_exclusion_zone(PuzzleInput {
            file_contents: INPUT_TEXT.to_string(),
            puzzle_part: PuzzlePart::Part2,
        })?;

        assert_eq!("56000011", output);
        Ok(())
    }
}
