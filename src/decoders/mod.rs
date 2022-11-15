//! This module contains all the code for decoders
//! Think of a decoder as a decryption method that doesn't require a key
//! The `interface.rs` defines what each decoder looks like.
//! Once you have made a decoder you need to add it to the filtration system's
//! mod.rs file
//! you will also need to make it a public module in this file.

/// The base32_decoder module decodes base32
pub mod base32_decoder;
/// The base58_bitcoin_decoder module decodes base58 bitcoin
pub mod base58_bitcoin_decoder;
/// The base64_decoder module decodes base64
/// It is public as we use it in some tests.
pub mod base64_decoder;
/// The base91_decoder module decodes base91
pub mod base91_decoder;
/// The crack_results module defines the CrackResult
/// Each and every decoder return same CrackResult
pub mod crack_results;

/// The interface module defines the interface for decoders
/// Each and every decoder has the same struct & traits
pub mod interface;

/// The reverse_decoder module decodes reverse text
/// Stac -> Cats
/// It is public as we use it in some tests.
pub mod reverse_decoder;

/// The morse_code module decodes morse code
/// It is public as we use it in some tests.
pub mod morse_code;
