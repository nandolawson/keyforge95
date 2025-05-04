use crate::modals::{
    Error::{self, InvalidFormat},
    KeyType,
};

/// Tests if a product key is valid
///
/// If the given key is valid, the function will return either ``KeyType::Oem`` or ``KeyType::Retail``.
///
/// # Examples
/// ```
/// use keyforge95::*;
/// let test_cases: [&str; 5] = [ // This keys should be valid
///     "111-1111111",
///     "000-0000000",
///     "332-3333333",
///     "334-7777777",
///     "33693-OEM-0000000-00000"
/// ];
/// for test_case in test_cases {
///      assert!(validate_product_key(test_case).is_ok());
/// }
/// ```
///
/// ```
/// use keyforge95::*;
/// let test_cases: [&str; 5] = [ // This keys should be invalid
///     "111-1111112",
///     "ABC-DEF-GHI",
///     "333-3333333",
///     "33703-OEM-1234569-00000",
///     "123-4567-89-00-AB4"
/// ];
/// for test_case in test_cases {
///     assert!(validate_product_key(test_case).is_err());
/// }
/// ```
///
/// # Errors
/// This function can return two errors: ``InvalidFormat`` and ``InvalidKey``.
///
/// - ``InvalidFormat``: Product keys for Windows 95 and other compatible products can be either OEM or retail keys. If the format of the given key does not belong to either of them, this error will appear.
/// - ``InvalidKey``: If the key has the right format, but isn't valid, the function will result this error.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn validate_product_key(product_key: &str) -> Result<KeyType, Error> {
    // Check if the product key format is valid
    if !validate_format(product_key) {
        return Err(InvalidFormat);
    }
    match product_key.len() {
        11 => (validate_block(&product_key[0..=2]) && validate_block(&product_key[4..=10]))
            .then_some(KeyType::Retail)
            .ok_or(Error::InvalidKey), // Retail product key
        23 => (validate_block(&product_key[0..=4]) && validate_block(&product_key[10..=17]))
            .then_some(KeyType::Oem)
            .ok_or(Error::InvalidKey), // OEM product key
        _ => unreachable!(),
    }
}

pub(crate) fn validate_block(block: &str) -> bool {
    // First determine the block of the product key per length of the block
    match block.len() {
        3 => {
            // Block A
            !((3..=9).contains(&(block.parse::<i32>().unwrap() / 111)) // This block must not contain a lucky number between 333 and 999
                && block.parse::<i32>().unwrap() % 111 == 0)
        }
        5 => {
            // Block B
            (0..=366).contains(&block[0..=2].parse::<u16>().unwrap()) // The first three digits must be a number between 0 and 366
                && (4..=93).contains(&block[3..=4].parse::<u8>().unwrap()) // The last two digits must be a number between 4 and 93
        }
        7 | 8 => {
            // Block C & D
            (block.len() == 8 || !block.contains('9')) && // The number 9 is not allowed for retail keys
            block[0..=6]
                .chars()
                .filter_map(|c: char| c.to_digit(10))
                .sum::<u32>()
                % 7
                == 0 // The sum of this block must be divisible by 7 with no remainder
        }
        _ => unreachable!(), // Only used if the block size is neither one of the above ones
    }
}

pub(crate) fn validate_format(product_key: &str) -> bool {
    // The length of the product key must be 11 or 23 digits
    match product_key.len() {
        11 => {
            // Retail
            product_key[0..=2].chars().all(char::is_numeric) && // Block must not contain anything else than numbers
            product_key[4..=10].chars().all(char::is_numeric) && // Same rule for this block
            product_key.chars().nth(3) == Some('-') // The fourth character must be a tie rope
        }
        23 => {
            // OEM
            product_key[0..=4].chars().all(char::is_numeric) && // Block must not contain anything else than numbers
            product_key[10..=16].chars().all(char::is_numeric) && // Same rule for this block
            product_key[18..=22].chars().all(char::is_numeric) && // Same rule for this block
            product_key[5..=9] == *"-OEM-" && // Check if the second block is "-OEM-"
            product_key.chars().nth(17) == Some('-') // The seventeenth character must be a tie rope
        }
        _ => false, // Invalid length, always false
    }
}
