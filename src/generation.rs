use crate::modals::{
    Choice::{A, B, C, D, E},
    KeyType::{Oem, Retail},
};

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
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
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
    let rng = || {
        let mut buf = [0u8; 4];
        getrandom::fill(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    };
    let (max_value, length) = match choice {
        A => (998, 3),
        B => return format!("{:03}{:02}", rng() % 367, 4 + rng() % 90),
        C => (8_888_888, 7),
        D => (9_999_999, 7),
        E => return format!("{:05}", rng() % 100_000),
    };
    loop {
        let block = format!("{:0length$}", rng() % (max_value + 1));
        if crate::validation::validate_block(&format!(
            "{}{}",
            block,
            if matches!(choice, D) { "-" } else { "" }
        )) {
            return block;
        }
    }
}
