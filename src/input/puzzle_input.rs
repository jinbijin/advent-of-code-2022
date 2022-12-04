use super::puzzle_part::PuzzlePart;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
pub struct PuzzleInput {
    pub puzzle_part: PuzzlePart,
    pub file_contents: String,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl PuzzleInput {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(file_contents: String, puzzle_part: PuzzlePart) -> PuzzleInput {
        PuzzleInput {
            puzzle_part,
            file_contents,
        }
    }
}
