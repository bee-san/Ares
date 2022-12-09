// TODO align the happy path to the left
// Currently we are aligning the happy path to the right
// So config.api, if it's not on we do something
// We should invert this to align the happy path to the left
// https://medium.com/@matryer/line-of-sight-in-code-186dd7cdea88

/// The output macro is used to print the output of the program.
/// If the API mode is on, it will not print.
#[macro_export]
macro_rules! program_exiting_successful_decoding {
    ($result:expr) => {
        let config = ares::config::get_config();
        if !config.api_mode {
            let plaintext = $result.text;
            // calculate path
            let decoded_path =  $result
                .path
                .iter()
                .map(|c| c.decoder)
                .collect::<Vec<_>>()
                .join(" → ");

            // TODO if text is longer than 1 line on terminal this will suck
            // Perhaps if its over say 30 chars we do not do it up to length
            // However it's helpful when the plaintext has whitespace in it, like this:
            // 👇👇👇👇👇👇👇👇👇👇👇👇👇👇👇👇👇
            // ```hello there```
            // 👆👆👆👆👆👆👆👆👆👆👆👆👆👆👆👆👆
            // You can clearly see it has white space haha. Maybe we should detect this and warn about it?
            let point_down = "👇".repeat(plaintext[0].len());
            let point_up = "👆".repeat(plaintext[0].len());

            let decoded_path_string = if !decoded_path.contains("→") {
                // handles case where only 1 decoder is used
                format!("the decoder used is {}", decoded_path)
            } else {
                format!("the decoders used and their order is {}", decoded_path)
            };
            println!(
                "The plaintext is: \n{}\n{}\n{}\nand {}", point_down,
                ansi_term::Colour::Yellow.bold().paint(&plaintext[0]), point_up, decoded_path_string
            );
        }
    };
}

/// The output macro is used to print the output of the program.
#[macro_export]
macro_rules! decoded_how_many_times {
    ($depth:expr) => {
        let config = $crate::config::get_config();
        if !config.api_mode {

            // Gets how many decoders we have
            // Then we add 25 for Caesar, and roughly 25 for Binary
            let decoders = filter_and_get_decoders();
            let decoded_times_int = $depth * (decoders.components.len() as u32 + 25 + 25);
            let decoded_times_str = ansi_term::Colour::Yellow
                .bold()
                .paint(format!("{} times", decoded_times_int));

            let time_took = $crate::cli_pretty_printing::calculate_time_took(decoded_times_int);

            // TODO add colour to the times
            println!("\n{} Ares has decoded {} times.\nIf you would have used Ciphey, it would have taken you {}\n", "🥳", decoded_times_str, time_took);
        }
    };
}

/// The output macro is used to print the output of the program.
#[macro_export]
macro_rules! human_checker_check {
    ($description:expr, $text:expr) => {
        println!(
            "{} I think the plaintext is {}.\nPossible plaintext: '{}' (y/N): ",
            "🕵️ ",
            ansi_term::Colour::Yellow
                .bold()
                .paint($description)
                .to_string(),
            ansi_term::Colour::Yellow.bold().paint($text).to_string()
        )
    };
}

/// When Ares has failed to decode something, print this message
pub fn failed_to_decode() {
    let config = crate::config::get_config();
    if !config.api_mode {
        // The program can roughly do 45 decodings a second
        // Currently it is not possible to get this info at this stage of the program from the decoding level
        // TODO fix this
        let ares_decodings = config.timeout * 45;
        let time_took = calculate_time_took(ares_decodings);
        println!("Ares has failed to decode the text after trying {} decodings. If you would have used Ciphey, it would have taken you {}", ares_decodings, time_took);
        println!("If you want more help, please ask in #coded-messages in our Discord http://discord.skerritt.blog")
    }
}
/// Calculate how long it would take to decode this in Ciphey
pub fn calculate_time_took(decoded_times_int: u32) -> String {
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
