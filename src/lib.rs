/*
keyforge95
Copyright 2024 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/keyforge95/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://docs.rs/-/rustdoc.static/favicon-32x32-422f7d1d52889060.png",
    issue_tracker_base_url = "https://github.com/nandolawson/keyforge95/issues",
    html_no_source
)]
#![doc = include_str!("../README.md")] // Adding the README to the documentation

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

enum Choice {
    A,
    B,
    C,
    D,
    E,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum KeyType {
    OEM,
    Retail,
}

mod generation;
#[cfg(test)]
mod tests;
mod validation;

pub mod prelude {
    pub use crate::{
        generation::generate_product_key,
        validation::validate_product_key,
        KeyType::{Retail, OEM},
    };
}
