/*
keyforge95
Copyright 2024 Nando Lawson

Licensed under the GPL, Version 3 <https://opensource.org/license/gpl-3-0>.
This file may not be copied, modified, or distributed except according to those terms.
*/

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
)]
#![doc = include_str!("../README.md")] // Adding the README to the documentation

// Dependencies
use core::ops::RangeInclusive;
use std::process::exit;
use rand::Rng;

//Public Functions

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
///     "901-2345678"
/// ];
/// for test_case in test_cases {
///      assert_eq!(validate_product_key(test_case), true);
/// }
/// ```
/// 
/// ```
/// use keyforge95::validate_product_key;
/// let test_cases: [&str; 5] = [ // This keys should be invalid
///     "111-1111112",
///     "ABC-DEF-GHI",
///     "333-3333333",
///     "111-7777769",
///     "123-4567-89-00-AB4"
/// ];
/// for test_case in test_cases {
///     assert_eq!(validate_product_key(test_case), false);
/// }
/// ```
#[must_use]
pub fn validate_product_key(product_key: &str) -> bool {
    if validate_format(product_key) { // Check if the product key format is valid
        matches!((
            validate_block(&product_key[0..=2]), // Check if first block is valid
            validate_block(&product_key[4..=10]) // Check if second block is valid
        ), (true, true))
    } else {false}
}

/// Generates a valid product key
///
/// # Example
///
/// ```
/// use keyforge95::generate_product_key;
/// for _ in 0..10 {
///     let product_key: String = generate_product_key();
///     assert_eq!(product_key.len(), 11);
///     assert_eq!(product_key.chars().nth(3).unwrap(), '-');
/// }
/// ```
#[must_use]
pub fn generate_product_key() -> String {
    // Use generate_block() for product key generation and print it with the right format
    format!("{}-{}", generate_block("a"), generate_block("b"))
}

// Functions

fn validate_format(product_key: &str) -> bool {
    if product_key.len() == 11 { // The length of the product key must be 11 digits
        matches!((
            product_key[0..=2].chars().all(char::is_numeric), // Block must not contain anything else than numbers
            product_key[4..=10].chars().all(char::is_numeric), // Same rule for block b
            product_key.chars().nth(3) == Some('-'), // The fourth character must be a tie rope
        ), (true, true, true))
    } else {false}
}

fn validate_block(block: &str) -> bool {
    // First determine the block of the product key per length of the block
    if block.len() == 3 { // Block a
        !((3..=9).contains(&(block.parse::<i32>().unwrap() / 111)) // This block must not contain a lucky number between 333 and 999
            && block.parse::<i32>().unwrap() % 111 == 0)
    } else if block.len() == 7 { // Block b
        matches!((
            block.contains('9'), // The number 9 is not allowed for this block
            block.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() % 7 == 0 // The sum of this block must be divisible by 7 with no remainder
        ), (false, true))
    } else { // Only used if the block size is neither 3 nor 7
        false
    }
}

fn generate_block(choice: &str) -> String {
    // Determine which block of the product key will be generated
    let range: RangeInclusive<u32> = if choice == "a" {
        000..=998 // The number range for block a
    } else {
        0_000_000..=8_888_888 // The number range for block b
    };
    let length: usize = if choice == "a" {
        3 // The length of block a
    } else if choice == "b" {
        7 // The length of block b
    } else {
        eprintln!("Invalid choice: {choice}");
        exit(1);
    };
    // Generate a block and validate it
    loop { // Loop this operation if it fails
        let block: String = format!("{:0length$}", rand::thread_rng().gen_range(range.clone())); // Generate a block of the product key
           if validate_block(&block) {
            return block // Exit the loop if the block validates successfully
        }
    }
}

// Tests

#[test]
fn test_validate_format() {
    let product_key: &str = "000-0000000"; // This key should be formatted correctly
    assert_eq!(validate_product_key(product_key), true);
    let test_cases: [&str; 5] = [ // This keys should be formatted incorrectly
        "000-00000000",
        "0000-0000000",
        "0-0",
        "A00-B000000",
        "A-A-A-A-A"
    ];
    for test_case in test_cases {
        assert_eq!(validate_format(test_case), false);
    }
}
#[test]
fn test_validate_block() {
    let test_cases: [&str; 5] = [ // This blocks should be valid
        "111",
        "334",
        "998",
        "1111111",
        "8888888"
    ];
    for test_case in test_cases {
        assert_eq!(validate_block(test_case), true);
    }
    let test_cases: [&str; 5] = [ // This blocks should be invalid
        "333",
        "999",
        "0",
        "9999999",
        "000000"
    ];
    for test_case in test_cases {
        assert_eq!(validate_block(test_case), false);
    }
}
#[test]
fn test_generate_block() {
    for _ in 0..10 { // Generates both blocks
        assert_eq!(generate_block("a").len(), 3); // First block
        assert_eq!(generate_block("b").len(), 7); // Second block
    }
}