#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use manta::{decode::decode, encode::encode};

// Basic tests
#[datatest::files("tests/data/ascii", {
    input in r"^(.*)\.txt",
})]
#[test]
fn ascii(input: &str) {
    let encoded = encode(input, 6, 4);
    assert!(encoded.is_ok());
    let decoded = decode(encoded.unwrap());
    assert!(decoded.is_ok());
    assert_eq!(input, decoded.unwrap());
}
