/*
keyforge95
Copyright 2024 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/keyforge95/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub enum Choice {
    A,
    B,
    C,
    D,
    E,
}

use Choice::{A, B, C, D, E};

/// Generates a valid product key
///
/// # Example
///
/// ```
/// use keyforge95::generate_product_key;
/// for _ in 0..10 {
///     let product_key: String = generate_product_key("retail"); // Both, "retail" and "oem" are available
///     assert_eq!(product_key.len(), 11);
///     assert_eq!(product_key.chars().nth(3).unwrap(), '-');
/// }
/// ```
/// # Panics
///
/// Will panic if no argument or any argument other than "retail" or "oem" is used.
#[must_use]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn generate_product_key(key_type: &str) -> String {
    // Use generate_block() for product key generation and print it with the right format
    match key_type {
        "retail" => {
            format!("{}-{}", generate_block(&A), generate_block(&C))
        }
        "oem" => {
            format!(
                "{}-OEM-{}-{}",
                generate_block(&B),
                generate_block(&D),
                generate_block(&E)
            )
        }
        _ => panic!("Invalid choice: {key_type}. Only 'retail' or 'oem' allowed."),
    }
}

pub(crate) fn generate_block(choice: &Choice) -> String {
    use rand_core::{OsRng, RngCore};

    // Determine which block of the product key will be generated
    match choice {
        B => format!(
            "{:03}{:02}",
            OsRng.next_u32() % 367,
            4 + (OsRng.next_u32() % 90)
        ),
        E => format!("{:05}", OsRng.next_u32() % 100_000),
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
                let block: String = format!("{:0length$}", OsRng.next_u32() % (max_value + 1)); // Generate a block of the product
                if validate_block(&block)
                    || (matches!(choice, D) && validate_block(&format!("{block}-")))
                {
                    return block;
                }
            }
        }
    }
}
