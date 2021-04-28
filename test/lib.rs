use manta::{decode::decode, encode::encode};

#[test]
fn it_runs() {
    let sample = "ababcbababaa";
    let encoded = encode(sample, 6, 4).unwrap();
    let decoded = decode(encoded).unwrap();
    assert_eq!(sample, decoded);
}
