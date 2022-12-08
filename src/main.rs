use ares::cli::parse_cli_args;
use ares::perform_cracking;

fn main() {
    // Turn CLI arguments into a library object
    let (text, config) = parse_cli_args();
    let result = perform_cracking(&text, config);
    match result {
        // TODO: As result have array of CrackResult used,
        // we can print in better way with more info
        Some(result) => {
            println!("PLAINTEXT: {:?}", result.text);
            println!(
                "DECODERS USED: {}",
                result
                    .path
                    .iter()
                    .map(|c| c.decoder)
                    .collect::<Vec<_>>()
                    .join(" → ")
            )
        }
        None => println!("FAILED 😭"),
    }
}
