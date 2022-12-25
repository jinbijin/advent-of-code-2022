use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct ParseContentsError {
    error_description: String,
}

impl ParseContentsError {
    pub fn new<TError>(error: TError) -> ParseContentsError
    where
        TError: Error,
    {
        ParseContentsError {
            error_description: error.to_string(),
        }
    }
}

impl Display for ParseContentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.error_description)
    }
}

impl Debug for ParseContentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Error for ParseContentsError {}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ParseContentsError {
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.to_string()
    }
}
