#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use manta::{decode::decode, encode::encode};
use std::mem::size_of_val;
use std::path::Path;

static BUFFER_WINDOW: usize = 6;
static LOOKAHEAD_WINDOW: usize = 4;

fn assert_roundtrip(input: &str) {
    let encoded = encode(input, BUFFER_WINDOW, LOOKAHEAD_WINDOW);
    assert!(encoded.is_ok());

    let decoded = decode(encoded.unwrap());
    assert!(decoded.is_ok());

    assert_eq!(input, decoded.unwrap());
}

fn report_memory(input: &str, filename: &Path) {
    let input_size = size_of_val(input);
    let encoded: Vec<(usize, usize, u8)> =
        encode(input, BUFFER_WINDOW, LOOKAHEAD_WINDOW).unwrap().0;
    let encoded_size = size_of_val(&*encoded);
    println!("\n File at {:?}:", filename);
    println!("\t input size: {}", input_size,);
    println!("\t encoded size: {}", encoded_size,);
    println!("\t ratio: {} \n", input_size as f32 / encoded_size as f32);
}

// ------------------------
// Simple correctness tests
// ------------------------
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

// ---------------
// Research Corpus
// ---------------
// from: https://corpus.canterbury.ac.nz/descriptions/
// Some files were excluded because they failed to parse as valid UTF-8,
// a requirement for the datatest framework, unfortunately.

// #[datatest::files("tests/data/cantrbry", {
//     input in r"^(.*)",
// })]
// #[test]
// fn cantrbry(input: &str) {
//     assert_roundtrip(input)
// }

// #[datatest::files("tests/data/artificl", {
//     input in r"^(.*)",
// })]
// #[test]
// fn artificl(input: &str) {
//     assert_roundtrip(input)
// }

// #[datatest::files("tests/data/large", {
//     input in r"^(.*)",
// })]
// #[test]
// fn large(input: &str) {
//     assert_roundtrip(input)
// }

// #[datatest::files("tests/data/misc", {
//     input in r"^(.*)",
// })]
// #[test]
// fn misc(input: &str) {
//     assert_roundtrip(input)
// }

// ----------------
// Efficiency Tests
// ----------------
#[datatest::files("tests/data/artificl", {
    input in r"^(.*)",
    filename = r"${1}",
})]
#[test]
fn artificl_memory_efficiency(input: &str, filename: &Path) {
    report_memory(input, filename);
}
