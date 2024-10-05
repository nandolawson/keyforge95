/*
keyforge95
Copyright 2024 Nando Lawson

Licensed under the GPL, Version 3 <https://github.com/nandolawson/keyforge95/blob/main/LICENSE>.
This file may not be copied, modified, or distributed except according to those terms.
*/

#[test]
fn test_generate_block() {
    use crate::generation::generate_block;
    use crate::Choice::{A, B, C, D, E};
    for _ in 0..10 {
        // Generates all blocks
        assert_eq!(generate_block(A).len(), 3); // First block
        assert_eq!(generate_block(B).len(), 5); // Second block
        assert_eq!(generate_block(C).len(), 7); // Third block
        assert_eq!(generate_block(D).len(), 7); // Fourth block
        assert_eq!(generate_block(E).len(), 5); // Fifth block
    }
}

#[test]
fn test_validate_block() {
    use crate::validation::validate_block;
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
    use crate::validation::validate_format;
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
