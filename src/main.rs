use ares::perform_cracking;
mod cli_input_parser;

use clap::Parser;

use log::trace;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser)]
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
    let min_log_level = match opts.verbose {
        0 => "Warn",
        1 => "Info",
        2 => "Debug",
        3 | _ => "Trace",
    };
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, min_log_level),
    );
    trace!("Program was called with CLI 😉");
    trace!("Parsed the arguments");
    println!("{:?}", opts.text);
    println!("{:?}", opts.verbose);

    let result = perform_cracking(&opts.text);
    match result {
        Some(result) => println!("SUCCESSFUL {:?}", result),
        None => println!("FAILED 😭"),
    }
}
