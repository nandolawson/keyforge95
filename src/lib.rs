/*
keyforge95
Copyright 2025 Nando Lawson

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

mod modals;
#[cfg(test)]
mod tests;
mod validation;

pub use crate::{
    modals::{
        Error::{InvalidFormat, InvalidKey},
        KeyType::{Oem, Retail},
    },
    validation::validate_product_key,
};

#[cfg(feature = "generation")]
mod generation;

#[cfg(feature = "generation")]
pub use generation::generate_product_key;
