/*
keyforge95
Copyright 2024 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/keyforge95/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

pub(crate) fn validate_block(block: &str) -> bool {
    // First determine the block of the product key per length of the block
    if block.len() == 3 {
        // Block A
        !((3..=9).contains(&(block.parse::<i32>().unwrap() / 111)) // This block must not contain a lucky number between 333 and 999
            && block.parse::<i32>().unwrap() % 111 == 0)
    } else if block.len() == 5 {
        // Block B
        (0..=366).contains(&block[0..=2].parse::<u16>().unwrap()) // The first three digits must be a number between 0 and 366
            && (4..=93).contains(&block[3..=4].parse::<u8>().unwrap()) // The last two digits must be a number between 4 and 93
    } else if block.len() == 7 || block.len() == 8 {
        // Block C & D
        matches!(
            (
                if block.len() == 8 {
                    false // Nothing happens here for oem keys (block D)
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

pub(crate) fn validate_format(product_key: &str) -> bool {
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
                product_key[5..=9] == *"-OEM-",           // Check if the second block is "-OEM-"
            ),
            (true, true, true, true, true)
        )
    } else {
        false
    }
}
