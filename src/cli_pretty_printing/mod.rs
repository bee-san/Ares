/// The output macro is used to print the output of the program.
/// If the API mode is on, it will not print.
#[macro_export]
macro_rules! output {
    ($name:expr) => {
        if !config.api_mode {
            println!(
                "{} {}",
                ansi_term::Colour::RGB(0, 255, 9).bold().paint("[>]"),
                $name
            );
        }
    };
}

/// The output macro is used to print the output of the program.
#[macro_export]
macro_rules! decoded_how_many_times {
    ($depth:expr) => {
        let config = crate::config::get_config();
        if !config.api_mode {
            // Gets how many decoders we have
            // Then we add 25 for Caesar, and roughly 3 for Binary
            let decoders = filter_and_get_decoders();
            let decoded = $depth * (decoders.components.len() as u32 + 25 + 3);
            let decoded_times = ansi_term::Colour::Yellow
                .bold()
                .paint(format!("{} times", decoded));
            // TODO add colour to the times
            println!("\n{} Ares has decoded {}\n", "🥳", decoded_times);
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
