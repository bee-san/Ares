/// By having all of our print statements in one file it allows us to align what they look like
/// and make sure each one is up to our standards. Previously a rogue print statement that went off at an edge case
/// would look a bit ugly and not the same UI as others.
/// We can also do things like check for logic or share information / functions which would be a bit messy in the main code.
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
/// Spaces around numbers are allowed.
///
/// # Examples
/// ```
/// // Valid formats:
/// "255,0,0"     // Pure red
/// "0, 255, 0"   // Pure green with spaces
/// "0,0,255"     // Pure blue
/// ```
///
/// Returns None if:
/// - The string is not in the correct format (must have exactly 2 commas)
/// - Any value cannot be parsed as a u8 (must be 0-255)
fn parse_rgb(rgb: &str) -> Option<(u8, u8, u8)> {
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

/// Color a string based on its role using RGB values from the config
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

/// Color text as informational or statement
/// If role is None, text will be colored using statement color from config
/// If role is Some("informational"), text will use the informational color from config
fn statement(text: &str, role: Option<&str>) -> String {
    match role {
        Some(r) => color_string(text, r),
        None => color_string(text, "statement"),
    }
}

/// Color text as a warning using the warning color from config
#[allow(dead_code)]
fn warning(text: &str) -> String {
    color_string(text, "warning")
}

/// Color text as success using the success color from config
fn success(text: &str) -> String {
    color_string(text, "success")
}

/// Color text as error using the warning color from config
/// Note: Uses warning color since error is not defined in the color scheme
#[allow(dead_code)]
fn error(text: &str) -> String {
    color_string(text, "warning")
}

/// Color text as a question using the question color from config
fn question(text: &str) -> String {
    color_string(text, "question")
}

/// The output function is used to print the output of the program.
/// If the API mode is on, it will not print.
///
/// # Panics
///
/// Panics if there is an error writing to file when output_method is set to a
/// file
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

/// The output function is used to print the output of the program.
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

/// Whenever the human checker checks for text, this function is run.
/// The human checker checks to see if API mdoe is runnign inside of it
/// rather than doing it here at the printing level
pub fn human_checker_check(description: &str, text: &str) {
    println!(
        "🕵️ I think the plaintext is {}.\nPossible plaintext: '{}' (y/N): ",
        statement(description, Some("informational")),
        statement(text, Some("informational"))
    );
}

/// When Ares has failed to decode something, print this message
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

/// Every second the timer ticks once
/// If the timer hits our countdown, we exit the program.
/// This function prints the countdown to let the user know the program is still running.
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

/// The input given to Ares is already plaintext
/// So we do not need to do anything
pub fn return_early_because_input_text_is_plaintext() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    println!("{}", success("Your input text is the plaintext 🥳"));
}

/// The user has provided both textual input and file input
/// # Panics
/// This function panics and is only used in the CLI.
pub fn panic_failure_both_input_and_fail_provided() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    panic!("Failed -- both file and text were provided. Please only use one.")
}

/// The user has not provided any input.
/// # Panics
/// This function panics and is only used in the CLI.
pub fn panic_failure_no_input_provided() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    panic!("Failed -- no input was provided. Please use -t for text or -f for files.")
}

/// Print a warning when an unknown configuration key is found
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
