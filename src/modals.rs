use std::fmt::{Debug, Formatter, Result};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "generation")]
#[derive(Clone, Copy, Debug)]
pub(crate) enum Choice {
    A,
    B,
    C,
    D,
    E,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum Error {
    InvalidKey,
    InvalidFormat,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            Error::InvalidKey => "Invalid key",
            Error::InvalidFormat => "Invalid format",
        };
        write!(f, "{message}")
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Debug)]
pub enum KeyType {
    Oem,
    Retail,
}
