# Ares: Next Generation Decoding Tool

## Overview

Ares is the next generation of automatic decoding and cracking tools, built by the same team that created [Ciphey](https://github.com/ciphey/ciphey). It's designed to be faster, more efficient, and more extensible than its predecessor, with the goal of eventually replacing Ciphey entirely.

Ares can automatically detect and decode various types of encoded or encrypted text, including (but not limited to) Base64, Hexadecimal, Caesar cipher, ROT13, URL encoding, and many more. It uses advanced algorithms and heuristics to identify the encoding type and apply the appropriate decoding method, often handling multiple layers of encoding automatically.

## Key Features

### Speed and Performance

Ares is significantly faster than its predecessor, with performance improvements of up to 700%. For every decode operation that Ciphey could perform, Ares can do approximately 7 in the same timeframe. This dramatic speed increase is achieved through:

- Efficient Rust implementation
- Multithreading support via [Rayon](https://github.com/rayon-rs/rayon)
- Optimized search algorithms
- Improved plaintext detection

### Library-First Architecture

Ares is designed with a library-first approach, separating core functionality from the CLI interface. This architecture enables:

- Easy integration into other applications
- Building additional tools on top of Ares (e.g., Discord bots)
- Better testing and maintainability
- Cleaner separation of concerns

### Advanced Search Algorithms

Ares employs sophisticated search algorithms to efficiently navigate the space of possible decodings:

- **A* Search**: Uses heuristics to prioritize the most promising decoders, enhanced with Cipher Identifier for statistical analysis of ciphertext
- **BFS (Breadth-First Search)**: Systematically explores all possible decodings

These algorithms allow Ares to handle multi-level encodings (e.g., Base64 → Hex → ROT13) efficiently, a capability that was limited in Ciphey due to performance constraints.

### Timeout Mechanism

One significant improvement over Ciphey is the built-in timeout mechanism. Ares will automatically stop processing after a configurable timeout period (default: 5 seconds for CLI, 10 seconds for Discord bot), ensuring that it doesn't run indefinitely on inputs it cannot decode.

### Comprehensive Documentation and Testing

Ares emphasizes code quality with:

- Extensive test coverage (over 120 tests)
- Documentation tests to ensure examples stay up-to-date
- Enforced documentation on all major components

## How Ares Identifies Plaintext

Ares uses a sophisticated system to determine whether decoded text is valid plaintext. This is a critical component of the system, as it determines when to stop the decoding process. The plaintext detection system includes several checkers:

### 1. Athena Checker

The Athena checker is the main orchestrator that runs multiple sub-checkers in sequence:

1. **Regex Checker** (if configured): Checks if the text matches a user-provided regular expression
2. **LemmeKnow Checker**: Uses the [LemmeKnow](https://github.com/swanandx/lemmeknow) library (a Rust version of [PyWhat](https://github.com/bee-san/pyWhat)) to identify if the text matches known patterns like IP addresses, URLs, etc.
3. **English Checker**: Determines if the text is valid English using the [gibberish-or-not](https://crates.io/crates/gibberish-or-not) library

### 2. Human Checker (Optional)

For interactive use, Ares can optionally ask a human to verify if the decoded text is valid plaintext. This is particularly useful for ambiguous cases or specialized content that automated checkers might not recognize correctly.

### 3. Plaintext Preprocessing

Before checking if text is valid plaintext, Ares performs normalization:
- Converting to lowercase
- Removing punctuation
- Handling edge cases like very short strings

## Decoding Process

The decoding process in Ares follows these general steps:

1. **Initial Plaintext Check**: First, Ares checks if the input is already plaintext using the Athena checker. If it is, Ares returns early with the input as the result.

2. **Search Algorithm Initialization**: If the input is not plaintext, Ares initializes the search algorithm (A* by default) with the input text as the starting point.

3. **Decoder Selection**: The filtration system selects appropriate decoders to try based on the input characteristics.

4. **Iterative Decoding**: The search algorithm iteratively applies decoders to the input and any intermediate results, checking after each step if plaintext has been found.

5. **Result or Timeout**: The process continues until either:
   - Valid plaintext is found (success)
   - All possible decodings have been exhausted (failure)
   - The configured timeout is reached (failure)

## Invisible Characters Detection

Ares includes a feature to detect invisible Unicode characters in decoded plaintext. This is particularly useful for steganography or obfuscated text. When a significant percentage (>30%) of characters in the decoded text are invisible, Ares offers to save the result to a file instead of displaying it in the terminal, where such characters might not render correctly.

## Supported Decoders

Ares supports a growing list of decoders, including:

- Base64, Base32, Base58 (various flavors), Base91, Base65536
- Hexadecimal
- URL encoding
- Caesar cipher and ROT47
- Atbash cipher
- A1Z26 encoding
- Morse code
- Binary
- Braille
- Rail fence cipher
- Reverse text
- Z85
- And more being added regularly

## Usage

### Discord Bot

The simplest way to use Ares is through the Discord bot. Join the [Discord Server](http://discord.skerritt.blog), go to the #bots channel, and use the `$ares` command. Type `$help` for more information.

### CLI Installation

To install the CLI version:

```bash
cargo install project_ares
```

Then use it with the `ares` command.

### Docker

You can also build and run Ares using Docker:

```bash
git clone https://github.com/bee-san/Ares
cd Ares
docker build .
```

## Configuration

Ares provides several configuration options:

- **Timeout**: Maximum time to spend trying to decode (default: 5 seconds)
- **Human Checker**: Enable/disable human verification of results
- **Regex Pattern**: Specify a regex pattern to match against decoded text
- **Verbosity**: Control the level of output detail

## Future Development

Ares is under active development, with plans to:

- Add more decoders (aiming to match and exceed Ciphey's ~50 decoders)
- Improve plaintext detection accuracy
- Enhance A* search with adaptive learning and contextual heuristics
- Enhance performance further
- Add more configuration options
- Expand platform support

## Contributing

Contributions to Ares are welcome! Whether it's adding new decoders, improving existing ones, enhancing documentation, or fixing bugs, your help is appreciated. Check the [GitHub repository](https://github.com/bee-san/Ares) for more information on how to contribute.