/// By having all of our print statements in one file it allows us to align what they look like
/// and make sure each one is up to our standards. Previously a rogue print statement that went off at an edge case
/// would look a bit ugly and not the same UI as others.
/// We can also do things like check for logic or share information / functions which would be a bit messy in the main code.
#[cfg(test)]
mod tests;
use crate::storage;
use crate::DecoderResult;
use std::env;
use std::fs::write;
use text_io::read;

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

    let decoded_path_coloured = ansi_term::Colour::Yellow.bold().paint(&decoded_path);
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
        println!("{:2.0}% of the plaintext is invisible characters, would you like to save to a file instead? (y/N)", (invis_char_percentage * 100.0));
        let reply: String = read!("{}\n");
        let result = reply.to_ascii_lowercase().starts_with('y');
        if result {
            println!(
                "Please enter a filename: (default: {}/ares_text.txt)",
                env::var("HOME").unwrap_or_default()
            );
            let mut file_path: String = read!("{}\n");
            if file_path.is_empty() {
                file_path = format!("{}/ares_text.txt", env::var("HOME").unwrap_or_default());
            }
            println!(
                "Outputting plaintext to file: {}\n\n{}",
                file_path, decoded_path_string
            );
            write(file_path, &plaintext[0]).expect("Error writing to file.");
            return;
        }
    }
    println!(
        "The plaintext is: \n{}\nand {}",
        ansi_term::Colour::Yellow.bold().paint(&plaintext[0]),
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
    let decoded_times_int = depth * (decoders.components.len() as u32 + 25);

    let time_took = calculate_time_took(decoded_times_int);

    // TODO add colour to the times
    println!("\n🥳 Ares has decoded {decoded_times_int} times.\nIf you would have used Ciphey, it would have taken you {time_took}\n");
}

/// Whenever the human checker checks for text, this function is run.
/// The human checker checks to see if API mdoe is runnign inside of it
/// rather than doing it here at the printing level
pub fn human_checker_check(description: &str, text: &str) {
    println!(
        "🕵️ I think the plaintext is {}.\nPossible plaintext: '{}' (y/N): ",
        ansi_term::Colour::Yellow.bold().paint(description),
        ansi_term::Colour::Yellow.bold().paint(text)
    )
}

/// When Ares has failed to decode something, print this message
pub fn failed_to_decode() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }

    println!("⛔️ Ares has failed to decode the text.");
    println!("If you want more help, please ask in #coded-messages in our Discord http://discord.skerritt.blog");
}
/// Calculate how long it would take to decode this in Ciphey
fn calculate_time_took(decoded_times_int: u32) -> String {
    // TODO if we grab how long the programs been running for (see timer) we can make some nice stats like:
    // * How many decodings / second we did
    // * How much longer it'd take in Ciphey
    // We'll guess Ciphey can do 8 a second. No science here, it's arbitrary based on my opinion
    let ciphey_decodings_a_second = 5;
    // Calculate how long it'd take in Ciphey
    let ciphey_how_long_to_decode_in_seconds = decoded_times_int / ciphey_decodings_a_second;
    if ciphey_how_long_to_decode_in_seconds > 60 {
        // If it took
        if ciphey_how_long_to_decode_in_seconds / 60 == 1 {
            // Handle case where it's each 1 minute
            // TODO 1 minutes is still broken for me
            format!("{} minute", ciphey_how_long_to_decode_in_seconds / 60)
        } else {
            // 1.26 minutes sounds good in English
            // So we do not need to handle special case here
            format!("{} minutes", ciphey_how_long_to_decode_in_seconds / 60)
        }
    } else {
        format!("{ciphey_how_long_to_decode_in_seconds} seconds")
    }
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
        println!("{seconds_spent_running} seconds have passed. {time_left} remaining");
    }
}

/// The input given to Ares is already plaintext
/// So we do not need to do anything
pub fn return_early_because_input_text_is_plaintext() {
    let config = crate::config::get_config();
    if config.api_mode {
        return;
    }
    println!("Your input text is the plaintext 🥳");
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

/// Creates a statement with optional styling
pub fn statement(text: &str, style: Option<&str>) -> String {
    match style {
        Some("informational") => format!("ℹ️  {}", text),
        _ => text.to_string(),
    }
}

/// Parse an RGB color string in the format "R,G,B" or "R, G, B"
pub fn parse_rgb(rgb: &str) -> Result<(u8, u8, u8), String> {
    let parts: Vec<&str> = rgb.split(',').map(|s| s.trim()).collect();
    if parts.len() != 3 {
        return Err("Invalid RGB format. Expected 'R,G,B' or 'R, G, B'".to_string());
    }

    let r = parts[0].parse::<u8>().map_err(|_| "Invalid red value")?;
    let g = parts[1].parse::<u8>().map_err(|_| "Invalid green value")?;
    let b = parts[2].parse::<u8>().map_err(|_| "Invalid blue value")?;

    Ok((r, g, b))
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
        assert!(result.is_ok());
    }
}

/// Print a success message
pub fn success(text: &str) -> String {
    format!("✅ {}", text)
}

/// Print a warning message
pub fn warning(text: &str) -> String {
    format!("⚠️  {}", text)
}
