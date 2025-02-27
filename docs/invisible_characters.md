# Invisible Characters Detection

## Overview

Ares now includes a feature to detect invisible characters in decoded plaintext and offer to save the result to a file. This is particularly useful when dealing with steganography or obfuscated text that uses invisible Unicode characters.

## What are Invisible Characters?

Invisible characters are Unicode characters that don't display visibly in text but still take up space or affect text rendering. These include:

- Spaces and various space-like characters (U+0020, U+00A0, U+2000-U+200A, etc.)
- Zero-width characters (U+200B, U+200C, U+200D, etc.)
- Control characters
- Formatting characters
- Various other special Unicode characters

These characters are often used in steganography (hiding messages within other messages) or for obfuscation purposes.

## How the Detection Works

When Ares successfully decodes a message, it analyzes the resulting plaintext to determine what percentage of the characters are invisible. The detection process works as follows:

1. The system maintains a list of known invisible characters in `src/storage/invisible_chars/chars.txt`
2. When plaintext is decoded, each character is checked against this list
3. If more than 30% of the characters in the plaintext are invisible, the user is prompted with options:
   - Save the plaintext to a file (recommended for invisible character-heavy content)
   - Display the plaintext in the terminal (which may not render invisible characters properly)

## Why This Feature is Useful

Invisible characters can be difficult to work with in terminal output:

1. They're hard to see (by definition)
2. They can break formatting or be lost when copying text
3. They might not render consistently across different terminals

By saving to a file, users can:
- Preserve all characters exactly as decoded
- Open the file in specialized editors that can visualize invisible characters
- Process the file with other tools for further analysis

## Implementation Details

The feature is implemented in the following components:

- `src/storage/mod.rs`: Defines the `INVISIBLE_CHARS` static variable that loads the list of invisible characters
- `src/storage/invisible_chars/chars.txt`: Contains the list of Unicode invisible characters
- `src/cli_pretty_printing/mod.rs`: Contains the logic to detect invisible characters and prompt the user

The detection threshold is set at 30% by default, which can be adjusted in the code if needed.

## Example Usage

When a decoded message contains a significant number of invisible characters:

```
75% of the plaintext is invisible characters, would you like to save to a file instead? (y/N)
```

If the user selects 'y':

```
Please enter a filename: (default: /home/user/ares_text.txt)
```

The user can then specify a custom filename or accept the default.

```
Outputting plaintext to file: /home/user/ares_text.txt

the decoders used are Base64 → Hex
```

## Testing

The invisible characters detection feature includes comprehensive tests:

- Tests for loading the invisible characters list
- Tests for detecting various percentages of invisible characters
- Tests for handling edge cases

These tests ensure the feature works reliably across different scenarios.