/*
When the user provides CLI input, we need to parse it for:
- Text or file?
- Verbose mode to level

and so on.
*/

// build new library_input

use crate::api_library_input_struct::LibraryInput;

fn main() {
    let options = LibraryInput::default();
}
