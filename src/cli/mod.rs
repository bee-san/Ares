/// This doc string acts as a help message when the uses runs '--help'
/// as do all doc strings on fields

use clap::Parser;
use log::{debug, trace};
use crate::config::Config;
use std::rc::Rc;

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
}

/// Parse CLI Arguments turns a Clap Opts struct, seen above 
/// Into a library Struct for use within the program 
/// The library struct can be found in the [config](../config) folder.
pub fn parse_cli_args() -> Rc<Config> {
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
fn cli_args_into_config_struct(opts: Opts) -> Rc<Config>{
    todo!()   
}
