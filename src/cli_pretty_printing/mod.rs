// TODO align the happy path to the left
// Currently we are aligning the happy path to the right
// So config.api, if it's not on we do something
// We should invert this to align the happy path to the left
// https://medium.com/@matryer/line-of-sight-in-code-186dd7cdea88

use crate::DecoderResult;

/// The output function is used to print the output of the program.
/// If the API mode is on, it will not print.
pub fn program_exiting_successful_decoding(result: DecoderResult) -> (){
    let config = crate::config::get_config();
    if config.api_mode {
        return ();
    }
    let plaintext = result.text;
    // calculate path
    let decoded_path =  result
        .path
        .iter()
        .map(|c| c.decoder)
        .collect::<Vec<_>>()
        .join(" → ");

    let decoded_path_string = if !decoded_path.contains("→") {
        // handles case where only 1 decoder is used
        format!("the decoder used is {}", decoded_path)
    } else {
        format!("the decoders used are {}", decoded_path)
    };
    println!(
        "The plaintext is: \n{}\nand {}",
        ansi_term::Colour::Yellow.bold().paint(&plaintext[0]), decoded_path_string
    );
    return ();
}


/// The output function is used to print the output of the program.
pub fn decoded_how_many_times(depth: u32) -> () {
    let config = crate::config::get_config();
    if config.api_mode {
        return ();
    }

    // Gets how many decoders we have
    // Then we add 25 for Caesar, and roughly 25 for Binary
    let decoders = crate::filtration_system::filter_and_get_decoders();
    let decoded_times_int = depth * (decoders.components.len() as u32 + 25 + 25);
    let decoded_times_str = ansi_term::Colour::Yellow
        .bold()
        .paint(format!("{} times", decoded_times_int));

    let time_took = calculate_time_took(decoded_times_int);

    // TODO add colour to the times
    println!("\n{} Ares has decoded {} times.\nIf you would have used Ciphey, it would have taken you {}\n", "🥳", decoded_times_str, time_took);
    return ();
}

/// Whenever the human checker checks for text, this function is run.
/// The human checker checks to see if API mdoe is runnign inside of it
/// rather than doing it here at the printing level
pub fn human_checker_check(description: &str, text: &str) {
    println!(
        "{} I think the plaintext is {}.\nPossible plaintext: '{}' (y/N): ",
        "🕵️ ",
        ansi_term::Colour::Yellow
            .bold()
            .paint(description)
            .to_string(),
        ansi_term::Colour::Yellow.bold().paint(text).to_string()
    )
}

/// When Ares has failed to decode something, print this message
pub fn failed_to_decode() -> () {
    let config = crate::config::get_config();
    if config.api_mode {
        return ();
    }
    // The program can roughly do 45 decodings a second
    // Currently it is not possible to get this info at this stage of the program from the decoding level
    // TODO fix this
    let ares_decodings = config.timeout * 45;
    let time_took = calculate_time_took(ares_decodings);
    println!("⛔️ Ares has failed to decode the text. If you would have used Ciphey, it would have taken you {}\n", time_took);
    println!("If you want more help, please ask in #coded-messages in our Discord http://discord.skerritt.blog");
    return ();
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
        format!("{} seconds", ciphey_how_long_to_decode_in_seconds)
    }
}

/// Every second the timer ticks once
/// If the timer hits our countdown, we exit the program.
/// This function prints the countdown to let the user know the program is still running.
pub fn countdown_until_program_ends(seconds_spent_running: u32, duration: u32) -> () {
    let config = crate::config::get_config();
    if config.api_mode {
        return ();
    }
    if seconds_spent_running % 5 == 0 && seconds_spent_running != 0 {
        let time_left = duration - seconds_spent_running;
        if time_left <= 0{
            return ();
        }
        println!(
            "{} seconds have passed. {} remaining",
            seconds_spent_running, time_left
            
        );
    }
    return ();
}

/// The input given to Ares is already plaintext
/// So we do not need to do anything
pub fn return_early_because_input_text_is_plaintext() -> () {
    let config = crate::config::get_config();
    if config.api_mode {
        return ();
    }
    println!("Your input text is the plaintext 🥳");
    return ();
}