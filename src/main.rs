use ares::perform_cracking;
use ares::cli::parse_cli_args;

fn main() {
   // Turn CLI arguments into a library object
    let arguments = parse_cli_args();    
    let result = perform_cracking(&"test", &arguments);
    match result {
        Some(result) => println!("SUCCESSFUL {:?}", result),
        None => println!("FAILED 😭"),
    }
}
