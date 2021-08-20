use ares::crack;
use clap::Clap;

mod cli_input_parser;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Bee <bee@skerritt.blog>")]
struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long)]
    text: String,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    crack(&opts.text);
}
