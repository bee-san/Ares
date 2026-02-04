//! Ciphey CLI entry point
//!
//! This binary provides both a traditional CLI and a TUI interface for Ciphey.
//! The TUI is used by default when running interactively in a terminal,
//! but can be disabled with the `--no-tui` flag.

use std::io::IsTerminal;

use ciphey::cli::parse_cli_args;
use ciphey::cli_pretty_printing::{failed_to_decode, program_exiting_successful_decoding};
use ciphey::config::{config_exists, create_config_from_setup, set_global_config};
use ciphey::perform_cracking;
use ciphey::tui::{run_setup_wizard, run_tui};

fn main() {
    // Set up human panic for better crash reports
    human_panic::setup_panic!();

    // Check if this is first run and we're in a terminal
    let is_first_run = !config_exists();
    let is_terminal = std::io::stdout().is_terminal();

    // Run TUI setup wizard on first run if in terminal
    if is_first_run && is_terminal {
        match run_setup_wizard() {
            Ok(Some(setup_config)) => {
                // User completed setup - create config from their choices
                create_config_from_setup(setup_config);
            }
            Ok(None) => {
                // User skipped setup - create default config
                create_config_from_setup(std::collections::HashMap::new());
            }
            Err(e) => {
                eprintln!("Setup wizard error: {}", e);
                // Fall back to creating default config
                create_config_from_setup(std::collections::HashMap::new());
            }
        }
    } else if is_first_run {
        // Not in terminal but first run - use CLI setup
        let first_run_config = ciphey::cli::run_first_time_setup();
        create_config_from_setup(first_run_config);
    }

    // Turn CLI arguments into a library object
    let (text, config, use_tui) = parse_cli_args();

    // Determine if we should use TUI:
    // - TUI is requested (--no-tui not passed)
    // - stdout is a terminal (not piped)
    // - Not in API mode
    // - Not in top_results mode
    let should_use_tui = use_tui && is_terminal && !config.api_mode && !config.top_results;

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
