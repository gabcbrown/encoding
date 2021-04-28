mod decode;
mod encode;

use decode::decode;
use encode::encode;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Input to be encoded
    input: String,
    #[clap(short, long, default_value = "6")]
    lookback_limit: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("Encoding input string: {:?}", opts.input);
    let encoded = encode(opts.input, opts.lookback_limit).unwrap();
    println!("Encoded: {:?}", encoded);
    println!("Decoding");
    let decoded = decode(encoded).unwrap();
    println!("Decoded: {:?}", decoded);
}
