use ares::cli::parse_cli_args;
use ares::cli_pretty_printing::program_exiting_successful_decoding;
use ares::perform_cracking;

fn main() {
    // Turn CLI arguments into a library object
    let (text, config) = parse_cli_args();
    let result = perform_cracking(&text, config);

    if text.is_empty() {
        ares::cli_pretty_printing::input_is_empty();
        return;
    }
    match result {
        // TODO: As result have array of CrackResult used,
        // we can print in better way with more info
        Some(result) => {
            program_exiting_successful_decoding(result);
        }
        None => ares::cli_pretty_printing::failed_to_decode(),
    }
}
