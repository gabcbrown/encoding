#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use manta::{decode::decode, encode::encode};

fn assert_roundtrip(input: &str) {
    let encoded = encode(input, 6, 4);
    assert!(encoded.is_ok());

    let decoded = decode(encoded.unwrap());
    assert!(decoded.is_ok());

    assert_eq!(input, decoded.unwrap());
}

// Correctness tests
#[datatest::files("tests/data/ascii", {
    input in r"^(.*)\.txt",
})]
#[test]
fn small_ascii(input: &str) {
    assert_roundtrip(input)
}

#[datatest::files("tests/data/unicode", {
    input in r"^(.*)\.txt",
})]
#[test]
fn small_unicode(input: &str) {
    assert_roundtrip(input)
}

// Research Corpus
// from: https://corpus.canterbury.ac.nz/descriptions/
// Some files were excluded because they failed to parse as valid UTF-8, a requirement for the datatest framework, unfortunately.

#[datatest::files("tests/data/cantrbry", {
    input in r"^(.*)",
})]
#[test]
fn cantrbry(input: &str) {
    assert_roundtrip(input)
}

#[datatest::files("tests/data/artificl", {
    input in r"^(.*)",
})]
#[test]
fn artificl(input: &str) {
    assert_roundtrip(input)
}

#[datatest::files("tests/data/large", {
    input in r"^(.*)",
})]
#[test]
fn large(input: &str) {
    assert_roundtrip(input)
}

#[datatest::files("tests/data/misc", {
    input in r"^(.*)",
})]
#[test]
fn misc(input: &str) {
    assert_roundtrip(input)
}
