#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use chrono::naive::NaiveTime;
use chrono::prelude::*;
use encode::{decode::decode, encode::encode};
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

fn report_memory_and_time(input: &str, filename: &Path) {
    let input_size = size_of_val(input);

    let encoding_start = Utc::now();
    let encoded = encode(input, BUFFER_WINDOW, LOOKAHEAD_WINDOW).unwrap();
    let encoding_end = Utc::now();
    let encoding_duration = encoding_end - encoding_start;

    let encoded_size = size_of_val(&*encoded.0);

    let decoding_start = Utc::now();
    let _decoded = decode(encoded);
    let decoding_end = Utc::now();
    let decoding_duration = decoding_end - decoding_start;

    println!("\n File at {:?}:", filename);
    println!(
        "\t Encoding time: \t\t{}",
        NaiveTime::from_hms(0, 0, 0)
            .overflowing_add_signed(encoding_duration)
            .0
    );
    println!(
        "\t Decoding time: \t\t{}",
        NaiveTime::from_hms(0, 0, 0)
            .overflowing_add_signed(decoding_duration)
            .0
    );
    println!("\t Input size: \t\t\t{} B", input_size,);
    println!("\t Encoded size: \t\t\t{} B", encoded_size,);
    println!(
        "\t Ratio: \t\t\t{} \n",
        input_size as f32 / encoded_size as f32
    );
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

// ----------------
// Efficiency Tests
// ----------------
#[datatest::files("tests/data/artificl", {
    input in r"^(.*)",
    filename = r"${1}",
})]
#[test]
fn artificl_efficiency(input: &str, filename: &Path) {
    report_memory_and_time(input, filename);
}

#[datatest::files("tests/data/large", {
    input in r"^(.*)",
    filename = r"${1}",
})]
#[test]
fn large_efficiency(input: &str, filename: &Path) {
    report_memory_and_time(input, filename);
}
