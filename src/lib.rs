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

// Dependencies

use rand_core::{OsRng, RngCore};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//Public Functions

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
            format!("{}-{}", generate_block('a'), generate_block('c'))
        }
        "oem" => {
            format!(
                "{}-OEM-{}-{:05}",
                generate_block('b'),
                generate_block('d'),
                OsRng.next_u32() % 100_000
            )
        }
        _ => {
            panic!("Invalid choice: {key_type}. Only 'retail' or 'oem' allowed.");
        }
    }
}

/// Tests if a product key is valid
///
/// # Examples
/// ```
/// use keyforge95::validate_product_key;
/// let test_cases: [&str; 5] = [ // This keys should be valid
///     "111-1111111",
///     "000-0000000",
///     "332-3333333",
///     "334-7777777",
///     "33693-OEM-0000000-00000"
/// ];
/// for test_case in test_cases {
///      assert!(validate_product_key(test_case));
/// }
/// ```
///
/// ```
/// use keyforge95::validate_product_key;
/// let test_cases: [&str; 5] = [ // This keys should be invalid
///     "111-1111112",
///     "ABC-DEF-GHI",
///     "333-3333333",
///     "33703-OEM-1234569-00000",
///     "123-4567-89-00-AB4"
/// ];
/// for test_case in test_cases {
///     assert!(!validate_product_key(test_case));
/// }
/// ```
#[must_use]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn validate_product_key(product_key: &str) -> bool {
    if validate_format(product_key) {
        // Check if the product key format is valid
        if product_key.len() == 11 {
            matches!(
                // Retail product key
                (
                    validate_block(&product_key[0..=2]), // Check if first block is valid
                    validate_block(&product_key[4..=10])  // Check if second block is valid
                ),
                (true, true)
            )
        } else if product_key.len() == 23 {
            // OEM product key
            matches!(
                (
                    validate_block(&product_key[0..=4]), // Check if first block is valid
                    validate_block(&product_key[10..=17])  // Check if second block is valid
                ),
                (true, true)
            )
        } else {
            false
        }
    } else {
        false
    }
}

// Functions

fn generate_block(choice: char) -> String {
    // Determine which block of the product key will be generated
    if choice == 'b' {
        format!(
            "{:03}{:02}",
            OsRng.next_u32() % 367,
            4 + (OsRng.next_u32() % 90)
        ) // Generate block c of the product key
    } else {
        let max_value: u32 = if choice == 'a' {
            998 // Number range for block a
        } else if choice == 'c' {
            8_888_888 // Number range for block c
        } else if choice == 'd' {
            9_999_999 // Number range for block d
        } else {
            panic!("Invalid choice: {choice}. Only 'a', 'b' 'c' and 'd' allowed.");
        };
        let length: usize = if choice == 'a' {
            3 // Length of block a
        } else if choice == 'c' || choice == 'd' {
            7 // Length of block c
        } else {
            panic!("Invalid choice: {choice}. Only 'a', 'b', 'c' and 'd' allowed.");
        };
        // Generate a block and validate it
        loop {
            // Loop this operation if it fails
            let block: String = format!("{:0length$}", OsRng.next_u32() % (max_value + 1)); // Generate a block of the product
            if choice == 'd' {
                if validate_block(format!("{}-", &block).as_str()) {
                    return block; // Exit the loop if the block validates successfully
                }
            } else if validate_block(&block) {
                return block; // Exit the loop if the block validates successfully
            }
        }
    }
}

fn validate_block(block: &str) -> bool {
    // First determine the block of the product key per length of the block
    if block.len() == 3 {
        // Block a
        !((3..=9).contains(&(block.parse::<i32>().unwrap() / 111)) // This block must not contain a lucky number between 333 and 999
            && block.parse::<i32>().unwrap() % 111 == 0)
    } else if block.len() == 5 {
        // Block b
        (0..=366).contains(&block[0..=2].parse::<u16>().unwrap()) // The first three digits must be a number between 0 and 366
            && (4..=93).contains(&block[3..=4].parse::<u8>().unwrap()) // The last two digits must be a number between 4 and 93
    } else if block.len() == 7 || block.len() == 8 {
        // Block c
        matches!(
            (
                if block.len() == 8 {
                    false // Nothing happens here for oem keys (block d)
                } else {
                    block.contains('9') // The number 9 is not allowed for retail keys
                },
                block[0..=6]
                    .chars()
                    .filter_map(|c: char| c.to_digit(10))
                    .sum::<u32>()
                    % 7
                    == 0 // The sum of this block must be divisible by 7 with no remainder
            ),
            (false, true)
        )
    } else {
        // Only used if the block size is neither one of the above ones
        false
    }
}

fn validate_format(product_key: &str) -> bool {
    // The length of the product key must be 11 or 23 digits
    if product_key.len() == 11 {
        // Retail product key
        matches!(
            (
                product_key[0..=2].chars().all(char::is_numeric), // Block must not contain anything else than numbers
                product_key[4..=10].chars().all(char::is_numeric), // Same rule for this block
                product_key.chars().nth(3) == Some('-'), // The fourth character must be a tie rope
            ),
            (true, true, true)
        )
    } else if product_key.len() == 23 {
        // OEM product key
        matches!(
            (
                product_key[0..=4].chars().all(char::is_numeric), // Block must not contain anything else than numbers
                product_key[10..=16].chars().all(char::is_numeric), // Same rule for this block
                product_key[18..=22].chars().all(char::is_numeric), // Same rule for this block
                product_key.chars().nth(17) == Some('-'), // The seventeenth character must be a tie rope
                product_key[5..=9].to_string() == "-OEM-", // Check if the second block is "-OEM-"
            ),
            (true, true, true, true, true)
        )
    } else {
        false
    }
}

// Tests

#[test]
fn test_generate_block() {
    for _ in 0..10 {
        // Generates all blocks
        assert_eq!(generate_block('a').len(), 3); // First block
        assert_eq!(generate_block('b').len(), 5); // Second block
        assert_eq!(generate_block('c').len(), 7); // Third block
        assert_eq!(generate_block('d').len(), 7); // Fourth block
    }
}

#[test]
fn test_validate_block() {
    let test_cases: [&str; 6] = [
        // These blocks are valid
        "334", "998", "1111111", "8888888", "36693", "00004",
    ];
    for test_case in test_cases {
        assert!(validate_block(test_case));
    }
    let test_cases: [&str; 7] = [
        // This blocks are invalid
        "333", "999", "0", "9999999", "000000", "36793", "36694",
    ];
    for test_case in test_cases {
        assert!(!validate_block(test_case));
    }
}

#[test]
fn test_validate_format() {
    let test_cases: [&str; 2] = [
        // This key should be formatted correctly
        "000-0000000",
        "00004-OEM-0000000-00000",
    ];
    for test_case in test_cases {
        assert!(validate_format(test_case));
    }
    let test_cases: [&str; 5] = [
        // These keys are formatted incorrectly
        "000-00000000",
        "0000-0000000",
        "0-0",
        "A00-B000000",
        "A-A-A-A-A",
    ];
    for test_case in test_cases {
        assert!(!validate_format(test_case));
    }
}
