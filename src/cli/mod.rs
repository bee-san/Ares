use crate::config::Config;
/// This doc string acts as a help message when the usees run '--help' in CLI mode
/// as do all doc strings on fields
use clap::Parser;
use lemmeknow::Identifier;
use log::{debug, trace};

/// The struct for Clap CLI arguments
#[derive(Parser)]
#[clap(version = "1.0", author = "Bee <bee@skerritt.blog>")]
pub struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long)]
    text: String,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    /// Turn off human checker
    #[clap(short, long)]
    disable_human_checker: bool,

    /// Maximum number of decodings to perform on a string
    #[clap(short, long, default_value = "10000")]
    max_depth: u32,
}

/// Parse CLI Arguments turns a Clap Opts struct, seen above
/// Into a library Struct for use within the program
/// The library struct can be found in the [config](../config) folder.
pub fn parse_cli_args() -> (String, Config) {
    let opts: Opts = Opts::parse();
    let min_log_level = match opts.verbose {
        0 => "Warn",
        1 => "Info",
        2 => "Debug",
        _ => "Trace",
    };
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, min_log_level),
    );
    trace!("Program was called with CLI 😉");
    trace!("Parsed the arguments");
    debug!("{:?}", opts.text);
    debug!("{:?}", opts.verbose);

    cli_args_into_config_struct(opts)
}

/// Turns our CLI arguments into a config stuct
fn cli_args_into_config_struct(opts: Opts) -> (String, Config) {
    (
        opts.text,
        Config {
            verbose: opts.verbose,
            lemmeknow_config: Identifier::default(),
            // default is false, we want default to be true
            human_checker_on: !opts.disable_human_checker,
            offline_mode: true,
            max_depth: Some(opts.max_depth),
        },
    )
}
