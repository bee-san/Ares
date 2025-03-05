//! CLI Pretty Printing Module
//!
//! This module provides a unified interface for all CLI output formatting in Ares.
//! By centralising all print statements here, we ensure:
//! - Consistent visual appearance across the application
//! - Standardised color schemes and formatting
//! - Proper handling of API mode vs CLI mode
//! - Centralised error message formatting
//!
//! # Color Scheme
//! The module uses a configurable color scheme with roles:
//! - Informational: General information and status updates
//! - Warning: Non-critical warnings and cautions
//! - Success: Successful operations and confirmations
//! - Question: Interactive prompts and user queries
//! - Statement: Standard output and neutral messages
//!
//! # Usage
//! ```rust
//! use ares::cli_pretty_printing::{success, warning};
//!
//! // Print a success message
//! println!("{}", success("Operation completed successfully"));
//!
//! // Print a warning message
//! println!("{}", warning("Please check your input"));
//! ```

#[cfg(test)]
mod tests;
use crate::storage;
use crate::DecoderResult;
use colored::Colorize;
use std::env;
use std::fs::write;
use text_io::read;

/// Parse RGB string in format "r,g,b" to RGB values.
///
/// The input string should be in the format "r,g,b" where r, g, and b are integers between 0 and 255.
/// Spaces around numbers are allowed. This function is used internally by the color formatting
/// functions to convert config-specified RGB strings into usable values.
///
/// # Arguments
/// * `rgb` - The RGB string to parse in format "r,g,b"
///
/// # Returns
/// * `Option<(u8, u8, u8)>` - The parsed RGB values if valid, None if invalid
///
/// # Examples
/// ```
/// use ares::cli_pretty_printing::parse_rgb;
///
/// // Valid formats:
/// assert!(parse_rgb("255,0,0").is_some());     // Pure red
/// assert!(parse_rgb("0, 255, 0").is_some());   // Pure green with spaces
/// assert!(parse_rgb("0,0,255").is_some());     // Pure blue
/// ```
///
/// # Errors
/// Returns None if:
/// - The string is not in the correct format (must have exactly 2 commas)
/// - Any value cannot be parsed as a u8 (must be 0-255)
pub fn parse_rgb(rgb: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = rgb.split(',').collect();
    if parts.len() != 3 {
        eprintln!("Invalid RGB format: '{}'. Expected format: 'r,g,b' where r,g,b are numbers between 0-255", rgb);
        return None;
    }

    let r = match parts[0].trim().parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!(
                "Invalid red value '{}': must be a number between 0-255",
                parts[0]
            );
            return None;
        }
    };

    let g = match parts[1].trim().parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!(
                "Invalid green value '{}': must be a number between 0-255",
                parts[1]
            );
            return None;
        }
    };

    let b = match parts[2].trim().parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!(
                "Invalid blue value '{}': must be a number between 0-255",
                parts[2]
            );
            return None;
        }
    };

    Some((r, g, b))
}

/// Colors a string based on its role using RGB values from the config.
///
/// This function is the core color formatting function that all other color
/// functions use. It retrieves colors from the global config and applies them
/// based on the specified role.
///
/// # Arguments
/// * `text` - The text to be colored
/// * `role` - The role determining which color to use (e.g., "informational", "warning")
///
/// # Returns
/// * `String` - The text colored according to the role's RGB values
///
/// # Role Colors
/// - informational: Used for general information
/// - warning: Used for warnings and cautions
/// - success: Used for success messages
/// - question: Used for interactive prompts
/// - statement: Used for neutral messages
fn color_string(text: &str, role: &str) -> String {
    let config = crate::config::get_config();

    // Get the RGB color string, defaulting to statement color if not found
    let rgb = match config.colourscheme.get(role) {
        Some(color) => color.clone(),
        None => config
            .colourscheme
            .get("statement")
            .cloned()
            .unwrap_or_else(|| "255,255,255".to_string()),
    };

    if let Some((r, g, b)) = parse_rgb(&rgb) {
        text.truecolor(r, g, b).bold().to_string()
    } else {
        // Default to statement color if RGB parsing fails
        if let Some(statement_rgb) = config.colourscheme.get("statement") {
            if let Some((r, g, b)) = parse_rgb(statement_rgb) {
                return text.truecolor(r, g, b).bold().to_string();
            }
        }
        text.white().to_string()
    }
}

/// Colors text based on its role, defaulting to statement color if no role is specified.
///
/// # Arguments
/// * `text` - The text to be colored
/// * `role` - Optional role to determine color choice. If None, uses statement color
///
/// # Returns
/// * `String` - The colored text string
///
/// # Examples
/// ```
/// use ares::cli_pretty_printing::statement;
///
/// let info = statement("Status update", Some("informational"));
/// let neutral = statement("Regular text", None);
/// assert!(!info.is_empty());
/// assert!(!neutral.is_empty());
/// ```
pub fn statement(text: &str, role: Option<&str>) -> String {
    match role {
        Some(r) => color_string(text, r),
        None => color_string(text, "statement"),
    }
}

/// Colors text using the warning color from config.
///
/// Used for non-critical warnings and cautions that don't prevent
/// program execution but require user attention.
///
/// # Arguments
/// * `text` - The warning message to be colored
///
/// # Returns
/// * `String` - The text colored in the warning color
#[allow(dead_code)]
pub fn warning(text: &str) -> String {
    color_string(text, "warning")
}

/// Colors text using the success color from config.
///
/// Used for messages indicating successful operations or positive outcomes.
///
/// # Arguments
/// * `text` - The success message to be colored
///
/// # Returns
/// * `String` - The text colored in the success color
pub fn success(text: &str) -> String {
    color_string(text, "success")
}

/// Colors text using the warning color from config for error messages.
///
/// Note: Uses warning color since error is not defined in the color scheme.
/// Used for error messages that indicate operation failure.
///
/// # Arguments
/// * `text` - The error message to be colored
///
/// # Returns
/// * `String` - The text colored in the warning color
#[allow(dead_code)]
fn error(text: &str) -> String {
    color_string(text, "warning")
}

/// Colors text using the question color from config.
///
/// Used for interactive prompts and user queries to make them
/// stand out from regular output.
///
/// # Arguments
/// * `text` - The question or prompt to be colored
///
/// # Returns
/// * `String` - The text colored in the question color
fn question(text: &str) -> String {
    color_string(text, "question")
}

/// Prints the final output of a successful decoding operation.
///
/// This function handles the presentation of decoded text, including special
/// handling for invisible characters and file output options.
///
/// # Arguments
/// * `result` - The DecoderResult containing the decoded text and metadata
///
/// # Behavior
/// - Checks for API mode and returns early if enabled
/// - Formats the decoder path with arrows
/// - Handles invisible character detection and file output
/// - Presents the decoded text with appropriate formatting
///
/// # Panics
/// Panics if there is an error writing to file when output_method is set to a file
pub fn program_exiting_successful_decoding(result: DecoderResult) {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    let plaintext = result.text;
    // calculate path
    let decoded_path = result
        .path
        .iter()
        .map(|c| c.decoder)
        .collect::<Vec<_>>()
        .join(" → ");

    let decoded_path_coloured = statement(&decoded_path, Some("informational"));
    let decoded_path_string = if !decoded_path.contains('→') {
        // handles case where only 1 decoder is used
        format!("the decoder used is {decoded_path_coloured}")
    } else {
        format!("the decoders used are {decoded_path_coloured}")
    };
    /// If 30% of the characters are invisible characters, then prompt the
    /// user to save the resulting plaintext into a file
    const INVIS_CHARS_DETECTION_PERCENTAGE: f64 = 0.3;
    let mut invis_chars_found: f64 = 0.0;
    for char in plaintext[0].chars() {
        if storage::INVISIBLE_CHARS
            .iter()
            .any(|invis_chars| *invis_chars == char)
        {
            invis_chars_found += 1.0;
        }
    }

    // If the percentage of invisible characters in the plaintext exceeds
    // the detection percentage, prompt the user asking if they want to
    // save the plaintext into a file
    let invis_char_percentage = invis_chars_found / plaintext[0].len() as f64;
    if invis_char_percentage > INVIS_CHARS_DETECTION_PERCENTAGE {
        let invis_char_percentage_string = format!("{:2.0}%", invis_char_percentage * 100.0);
        println!(
            "{}",
            question(
                &format!(
                    "{} of the plaintext is invisible characters, would you like to save to a file instead? (y/N)", 
                    invis_char_percentage_string.white().bold()
                )
            )
        );
        let reply: String = read!("{}\n");
        let result = reply.to_ascii_lowercase().starts_with('y');
        if result {
            println!(
                "Please enter a filename: (default: {}/ares_text.txt)",
                env::var("HOME").unwrap_or_default().white().bold()
            );
            let mut file_path: String = read!("{}\n");
            if file_path.is_empty() {
                file_path = format!("{}/ares_text.txt", env::var("HOME").unwrap_or_default());
            }
            println!(
                "Outputting plaintext to file: {}\n\n{}",
                statement(&file_path, None),
                decoded_path_string
            );
            write(file_path, &plaintext[0]).expect("Error writing to file.");
            return;
        }
    }
    println!(
        "The plaintext is:\n{}\n{}",
        success(&plaintext[0]),
        decoded_path_string
    );
}

/// Prints the number of decoding attempts performed.
///
/// # Arguments
/// * `depth` - The depth of decoding attempts
///
/// # Note
/// This function automatically calculates the total number of attempts
/// based on the available decoders and the depth parameter.
pub fn decoded_how_many_times(depth: u32) {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }

    // Gets how many decoders we have
    // Then we add 25 for Caesar
    let decoders = crate::filtration_system::filter_and_get_decoders(&DecoderResult::default());
    let decoded_times_int = depth * (decoders.components.len() as u32 + 40); //TODO 40 is how many decoders we have. Calculate automatically
    println!(
        "\n🥳 Ares has decoded {} times.\n",
        statement(&decoded_times_int.to_string(), None)
    );
}

/// Prompts the user to verify potential plaintext during human checking.
///
/// # Arguments
/// * `description` - Description of why this might be plaintext
/// * `text` - The potential plaintext to verify
///
/// # Note
/// This function is only called when human checking is enabled and
/// not in API mode.
pub fn human_checker_check(description: &str, text: &str) {
    println!(
        "🕵️ I think the plaintext is {}.\nPossible plaintext: '{}' (y/N): ",
        statement(description, Some("informational")),
        statement(text, Some("informational"))
    );
}

/// Prints a failure message when decoding was unsuccessful.
///
/// This function provides user guidance by suggesting Discord support
/// when automated decoding fails.
///
/// # Note
/// This message is suppressed in API mode.
pub fn failed_to_decode() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }

    println!(
        "{}",
        warning("⛔️ Ares has failed to decode the text.\nIf you want more help, please ask in #coded-messages in our Discord http://discord.skerritt.blog")
    );
}

/// Updates the user on decoding progress with a countdown timer.
///
/// # Arguments
/// * `seconds_spent_running` - Number of seconds elapsed
/// * `duration` - Total duration allowed for decoding
///
/// # Note
/// Progress updates are shown every 5 seconds until the duration is reached.
pub fn countdown_until_program_ends(seconds_spent_running: u32, duration: u32) {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    if seconds_spent_running % 5 == 0 && seconds_spent_running != 0 {
        let time_left = duration - seconds_spent_running;
        if time_left == 0 {
            return;
        }
        println!(
            "{} seconds have passed. {} remaining",
            statement(&seconds_spent_running.to_string(), None),
            statement(&time_left.to_string(), None)
        );
    }
}

/// Indicates that the input is already plaintext.
///
/// This function is called when the input passes plaintext detection
/// and no decoding is necessary.
pub fn return_early_because_input_text_is_plaintext() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    println!("{}", success("Your input text is the plaintext 🥳"));
}

/// Handles the error case of receiving both file and text input.
///
/// # Panics
/// This function always panics with a message explaining the input conflict.
/// Only used in CLI mode.
pub fn panic_failure_both_input_and_fail_provided() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    panic!("Failed -- both file and text were provided. Please only use one.")
}

/// Handles the error case of receiving no input.
///
/// # Panics
/// This function always panics with a message explaining the missing input.
/// Only used in CLI mode.
pub fn panic_failure_no_input_provided() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    panic!("Failed -- no input was provided. Please use -t for text or -f for files.")
}

/// Warns about unknown configuration keys.
///
/// # Arguments
/// * `key` - The unknown configuration key that was found
///
/// # Note
/// This warning is suppressed in API mode.
pub fn warning_unknown_config_key(key: &str) {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    eprintln!(
        "{}",
        warning(&format!(
            "Unknown configuration key found in config file: {}",
            key
        ))
    );
}

#[test]
fn test_parse_rgb() {
    let test_cases = vec![
        "255,0,0",   // Pure red
        "0, 255, 0", // Pure green with spaces
        "0,0,255",   // Pure blue
    ];

    for case in test_cases {
        let result = parse_rgb(case);
        assert!(result.is_some());
    }
}
