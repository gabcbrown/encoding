use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    /// Input to be encoded
    input: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Test(Test),
}

/// Tool for running encoding/decoding test suite
#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Using input file: {}", opts.input);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
    }

    // more program logic goes here...
}
