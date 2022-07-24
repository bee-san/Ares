//! This module contains all the code for decoders
//! Think of a decoder as a decryption method that doesn't require a key
//! The `interface.rs` defines what each decoder looks like.
//! Once you have made a decoder you need to add it to the filtration system's
//! mod.rs file
//! you will also need to make it a public module in this file.

pub mod base64_decoder;
pub mod hash_decoder;
pub mod interface;
pub mod reverse_decoder;
