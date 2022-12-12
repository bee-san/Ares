use std::{fs::File, io::Read};

use crate::{
    cli_pretty_printing::{
        panic_failure_both_input_and_fail_provided, panic_failure_no_input_provided,
    },
    config::Config,
};
/// This doc string acts as a help message when the usees run '--help' in CLI mode
/// as do all doc strings on fields
use clap::Parser;
use lemmeknow::Identifier;
use log::trace;

/// The struct for Clap CLI arguments
#[derive(Parser)]
#[command(author = "Bee <bee@skerritt.blog>", about, long_about = None)]
pub struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[arg(short, long)]
    text: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Turn off human checker, perfect for APIs where you don't want input from humans
    #[arg(short, long)]
    disable_human_checker: bool,

    /// Set timeout, if it is not decrypted after this time, it will return an error.
    /// Default is 5 seconds.
    // If we want to call it `timeout`, the short argument contends with the one for Text `ares -t`.
    // I propose we just call it `cracking_timeout`.
    #[arg(short, long)]
    cracking_timeout: Option<u32>,
    /// Run in API mode, this will return the results instead of printing them
    /// Default is False
    #[arg(short, long)]
    api_mode: Option<bool>,
    /// Opens a file for decoding
    /// Used instead of `--t`
    #[arg(short, long)]
    file: Option<String>,
}

/// Parse CLI Arguments turns a Clap Opts struct, seen above
/// Into a library Struct for use within the program
/// The library struct can be found in the [config](../config) folder.
pub fn parse_cli_args() -> (String, Config) {
    let mut opts: Opts = Opts::parse();
    let min_log_level = match opts.verbose {
        0 => "Warn",
        1 => "Info",
        2 => "Debug",
        _ => "Trace",
    };
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, min_log_level),
    );

    if opts.text.is_none() && opts.file.is_none() {
        panic_failure_no_input_provided();
    }
    if opts.file.is_some() && opts.text.is_some() {
        panic_failure_both_input_and_fail_provided();
    }
    let input_text: String = if opts.file.is_some() {
        // TODO pretty match on the errors to provide better output
        // Else it'll panic
        let mut file = File::open(opts.file.unwrap()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // We can just put the file into the `Opts.text` and the program will work as normal
        contents
    } else {
        opts.text.unwrap().clone()
    };
    // Else we are using text
    opts.text = None;
    opts.file = None;

    trace!("Program was called with CLI 😉");
    trace!("Parsed the arguments");

    cli_args_into_config_struct(opts, input_text)
}

/// Turns our CLI arguments into a config stuct
fn cli_args_into_config_struct(opts: Opts, text: String) -> (String, Config) {
    (
        text,
        Config {
            verbose: opts.verbose,
            lemmeknow_config: Identifier::default(),
            // default is false, we want default to be true
            human_checker_on: !opts.disable_human_checker,
            // These if statements act as defaults
            timeout: if opts.cracking_timeout.is_none() {
                5
            } else {
                opts.cracking_timeout.unwrap()
            },
            api_mode: opts.api_mode.is_some(),
        },
    )
}
