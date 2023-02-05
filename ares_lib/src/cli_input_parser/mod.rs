/*
When the user provides CLI input, we need to parse it for:
- Text or file?
- Verbose mode to level

and so on.
*/

// build new library_input

use crate::api_library_input_struct::LibraryInput;

/// This creates a new LibraryInput struct and sets it to a default.
/// added _ before name to let clippy know that they aren't used
fn _main() {
    let _options = LibraryInput::default();
}
