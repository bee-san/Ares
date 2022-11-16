use ares::cli::parse_cli_args;
use ares::perform_cracking;

fn main() {
    // Turn CLI arguments into a library object
    let (text, config) = parse_cli_args();
    let result = perform_cracking(&text, config);
    match result {
        Some(result) => {
            println!("SUCCESSFUL 😁");
            println!("PLAINTEXT: {:?}", result.text);
            println!("DECODERS USED: {}", result.path.join(" -> "))
        },
        None => println!("FAILED 😭"),
    }
}
