//! This module contains all the code for decoders
//! Think of a decoder as a decryption method that doesn't require a key
//! The `interface.rs` defines what each decoder looks like.
//! Once you have made a decoder you need to add it to the filtration system's
//! mod.rs file
//! you will also need to make it a public module in this file.

/// The a1z26_decoder module decodes A1Z26
pub mod a1z26_decoder;
/// The atbash_decoder module decodes atbash
pub mod atbash_decoder;
/// The base32_decoder module decodes base32
pub mod base32_decoder;
/// The base58_bitcoin_decoder module decodes base58 bitcoin
pub mod base58_bitcoin_decoder;
/// The base58_monero_decoder module decodes base58 monero
pub mod base58_monero_decoder;
/// The binary_decoder module decodes binary
pub mod binary_decoder;
/// The hexadecimal_decoder module decodes hexadecimal
pub mod hexadecimal_decoder;

/// The base58_ripple_decoder module decodes base58 ripple
pub mod base58_ripple_decoder;

/// The base58_flickr decoder module decodes base58 flickr
pub mod base58_flickr_decoder;

/// The base64_decoder module decodes base64
/// It is public as we use it in some tests.
pub mod base64_decoder;
/// The base64_url_decoder module decodes base64 url
pub mod base64_url_decoder;
/// The base65536 module decodes base65536
pub mod base65536_decoder;
/// The base91_decoder module decodes base91
pub mod base91_decoder;
/// The citrix_ctx1_decoder module decodes citrix ctx1
pub mod citrix_ctx1_decoder;
/// The crack_results module defines the CrackResult
/// Each and every decoder return same CrackResult
pub mod crack_results;
/// The url_decoder module decodes url
pub mod url_decoder;

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

/// For the caesar cipher decoder
pub mod caesar_decoder;

/// For the railfence cipher decoder
pub mod railfence_decoder;

/// For the z85 cipher decoder
pub mod z85_decoder;
