# Rust fixme 3

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: General Skills, picoCTF 2025
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: Taylor McCampbell

Description:
Have you heard of Rust? Fix the syntax errors in this Rust file to print the flag!
Download the Rust code here.

Hints:
1. Read the comments...darn it!
```

Challenge link: [https://play.picoctf.org/practice/challenge/463](https://play.picoctf.org/practice/challenge/463)

## Solution

We start by unpacking the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_3]
└─$ tar xvfz fixme3.tar.gz
fixme3/
fixme3/Cargo.toml
fixme3/Cargo.lock
fixme3/src/
fixme3/src/main.rs
```

### Study the source code

Then we navigate to the `src` directory and check the source code

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_3]
└─$ cd fixme3/src  

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_3/fixme3/src]
└─$ cat main.rs  
use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer: Vec<u8>, borrowed_string: &mut String) {
    // Key for decryption
    let key = String::from("CSUCKS");

    // Editing our borrowed value
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decryption object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return;
    }
    let xrc = res.unwrap();

    // Did you know you have to do "unsafe operations in Rust?
    // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
    // Even though we have these memory safe languages, sometimes we need to do things outside of the rules
    // This is where unsafe rust comes in, something that is important to know about in order to keep things in perspective
    
    // unsafe {
        // Decrypt the flag operations 
        let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);

        // Creating a pointer 
        let decrypted_ptr = decrypted_buffer.as_ptr();
        let decrypted_len = decrypted_buffer.len();
        
        // Unsafe operation: calling an unsafe function that dereferences a raw pointer
        let decrypted_slice = std::slice::from_raw_parts(decrypted_ptr, decrypted_len);

        borrowed_string.push_str(&String::from_utf8_lossy(decrypted_slice));
    // }
    println!("{}", borrowed_string);
}

fn main() {
    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "12", "90", "7e", "53", "63", "e1", "01", "35", "7e", "59", "60", "f6", "03", "86", "7f", "56", "41", "29", "30", "6f", "08", "c3", "61", "f9", "35"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    let mut party_foul = String::from("Using memory unsafe languages is a: ");
    decrypt(encrypted_buffer, &mut party_foul);
}   
```

Too use unsafe functionality in Rust, we need to explicitly add the `unsafe` keyword.  
Unsafe functionality include:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of a `union`

The `unsafe` keyword is already present in the source, but needs to be "uncommented".

### Fix and build the program

Next, we fix and build the program. If you don't have `cargo` installed already this can be done with `sudo apt install cargo`.

```bash
┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_3/fixme3/src]
└─$ vi main.rs 

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_3/fixme3/src]
└─$ cat main.rs 
use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer: Vec<u8>, borrowed_string: &mut String) {
    // Key for decryption
    let key = String::from("CSUCKS");

    // Editing our borrowed value
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decryption object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return;
    }
    let xrc = res.unwrap();

    // Did you know you have to do "unsafe operations in Rust?
    // https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
    // Even though we have these memory safe languages, sometimes we need to do things outside of the rules
    // This is where unsafe rust comes in, something that is important to know about in order to keep things in perspective
    
    unsafe {
        // Decrypt the flag operations 
        let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);

        // Creating a pointer 
        let decrypted_ptr = decrypted_buffer.as_ptr();
        let decrypted_len = decrypted_buffer.len();
        
        // Unsafe operation: calling an unsafe function that dereferences a raw pointer
        let decrypted_slice = std::slice::from_raw_parts(decrypted_ptr, decrypted_len);

        borrowed_string.push_str(&String::from_utf8_lossy(decrypted_slice));
    }
    println!("{}", borrowed_string);
}

fn main() {
    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "12", "90", "7e", "53", "63", "e1", "01", "35", "7e", "59", "60", "f6", "03", "86", "7f", "56", "41", "29", "30", "6f", "08", "c3", "61", "f9", "35"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    let mut party_foul = String::from("Using memory unsafe languages is a: ");
    decrypt(encrypted_buffer, &mut party_foul);
}

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_3/fixme3/src]
└─$ cd ..        

┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_3/fixme3]
└─$ cargo build
   Compiling crossbeam-utils v0.8.20
   Compiling rayon-core v1.12.1
   Compiling either v1.13.0
   Compiling crossbeam-epoch v0.9.18
   Compiling crossbeam-deque v0.8.5
   Compiling rayon v1.10.0
   Compiling xor_cryptor v1.2.3
   Compiling rust_proj v0.1.0 (/mnt/hgfs/CTFs/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_3/fixme3)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.91s
```

### Get the flag

Finally, we run the program and get the decrypted flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_3/fixme3]
└─$ cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/rust_proj`
Using memory unsafe languages is a: PARTY FOUL! Here is your flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Cargo Tutorial - Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Unsafe Rust - Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html)
- [Rust (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
