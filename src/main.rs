use clap::Clap;
use manta::{decode::decode, encode::encode};

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Input to be encoded
    input: String,
    #[clap(short, long, default_value = "6")]
    search_buffer_size: usize,
    #[clap(short, long, default_value = "4")]
    lookahead_buffer_size: usize,
}

pub fn main() {
    let opts: Opts = Opts::parse();

    println!("Encoding input string: {:?}", opts.input);
    let encoded = encode(
        &opts.input,
        opts.search_buffer_size,
        opts.lookahead_buffer_size,
    )
    .unwrap();
    println!("Encoded: {:?}", encoded);
    let decoded = decode(encoded).unwrap();
    println!("Decoded: {:?}", decoded);
}
