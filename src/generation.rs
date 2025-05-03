/*
keyforge95
Copyright 2025 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/keyforge95/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

use crate::modals::{
    Choice::{A, B, C, D, E},
    KeyType::{Oem, Retail},
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Generates a valid product key
/// 
/// This function is available via the feature `generation`
///
/// # Example
///
/// ```
/// use keyforge95::*;
/// for _ in 0..10 {
///     let product_key: String = generate_product_key(Retail); // Both, "Retail" and "Oem" are available
///     assert_eq!(product_key.len(), 11);
///     assert_eq!(product_key.chars().nth(3).unwrap(), '-');
/// }
/// ```
#[must_use]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn generate_product_key(key_type: crate::modals::KeyType) -> String {
    // Use generate_block() for product key generation and print it with the right format
    match key_type {
        Retail => {
            format!("{}-{}", generate_block(A), generate_block(C))
        }
        Oem => {
            format!(
                "{}-OEM-{}-{}",
                generate_block(B),
                generate_block(D),
                generate_block(E)
            )
        }
    }
}

pub(crate) fn generate_block(choice: crate::modals::Choice) -> String {
    use rand_core::{OsRng, RngCore};
    let rng = || OsRng.next_u32();
    // Determine which block of the product key will be generated
    match choice {
        B => format!("{:03}{:02}", rng() % 367, 4 + (rng() % 90)),
        E => format!("{:05}", rng() % 100_000),
        _ => {
            let max_value: u32 = match choice {
                A => 998,       // Number range for block A
                C => 8_888_888, // Number range for block C
                D => 9_999_999, // Number range for block D
                _ => 0,         // Dummy value for block B & E
            };
            let length: usize = match choice {
                A => 3,     // Length of block A
                C | D => 7, // Length of block C & D
                _ => 0,     // Dummy value for block B & E
            };
            // Generate a block and validate it
            loop {
                use crate::validation::validate_block;
                // Loop this operation if it fails
                let block: String = format!("{:0length$}", rng() % (max_value + 1)); // Generate a block of the product
                if validate_block(&format!(
                    "{}{}",
                    block,
                    if matches!(choice, D) { "-" } else { "" }
                )) {
                    return block;
                }
            }
        }
    }
}
