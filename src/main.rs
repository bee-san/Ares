//! Ciphey CLI entry point
//!
//! This binary provides both a traditional CLI and a TUI interface for Ciphey.
//! The TUI is used by default when running interactively in a terminal,
//! but can be disabled with the `--no-tui` flag.

use std::io::IsTerminal;

use ciphey::cli::parse_cli_args;
use ciphey::cli_pretty_printing::{failed_to_decode, program_exiting_successful_decoding};
use ciphey::config::set_global_config;
use ciphey::perform_cracking;
use ciphey::tui::run_tui;

fn main() {
    // Set up human panic for better crash reports
    human_panic::setup_panic!();

    // Turn CLI arguments into a library object
    let (text, config, use_tui) = parse_cli_args();

    // Determine if we should use TUI:
    // - TUI is requested (--no-tui not passed)
    // - stdout is a terminal (not piped)
    // - Not in API mode
    // - Not in top_results mode
    let should_use_tui =
        use_tui && std::io::stdout().is_terminal() && !config.api_mode && !config.top_results;

    if should_use_tui {
        // Run TUI mode
        if let Err(e) = run_tui(&text, config) {
            eprintln!("TUI error: {}", e);
            std::process::exit(1);
        }
    } else {
        // Classic CLI mode
        set_global_config(config);
        let config = ciphey::config::get_config();
        let result = perform_cracking(&text, config.clone());

        match result {
            Some(result) => {
                program_exiting_successful_decoding(result);
            }
            None => {
                failed_to_decode();
            }
        }
    }
}
