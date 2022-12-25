use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[cfg(feature = "wasm")]
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct True;

impl Display for True {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "true")
    }
}

impl Error for True {}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

#[cfg(feature = "wasm")]
impl Serialize for True {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}
