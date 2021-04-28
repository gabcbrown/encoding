mod decode;
mod encode;

use decode::decode;
use encode::encode;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Input to be encoded
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("Encoding input string: {:?}", opts.input);
    let encoded = encode(opts.input).unwrap();
    println!("Encoded: {:?}", encoded);
    println!("Decoding");
    let decoded = decode(encoded).unwrap();
    println!("Decoded: {:?}", decoded);
}
