# Rust fixme 1

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
1. Cargo is Rust's package manager and will make your life easier. 
   See the getting started page here
2. println!
3. Rust has some pretty great compiler error messages. Read them maybe?
```

Challenge link: [https://play.picoctf.org/practice/challenge/461](https://play.picoctf.org/practice/challenge/461)

## Solution

We start by unpacking the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_1]
└─$ tar xvfz fixme1.tar.gz 
fixme1/
fixme1/Cargo.toml
fixme1/Cargo.lock
fixme1/src/
fixme1/src/main.rs
```

### Study the source code

Then we navigate to the `src` directory and check the source code

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_1]
└─$ cd fixme1/src                                                     

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_1/fixme1/src]
└─$ cat main.rs        
use xor_cryptor::XORCryptor;

fn main() {
    // Key for decryption
    let key = String::from("CSUCKS") // How do we end statements in Rust?

    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "01", "1c", "7e", "59", "63", "e1", "61", "25", "7f", "5a", "60", "50", "11", "38", "1f", "3a", "60", "e9", "62", "20", "0c", "e6", "50", "d3", "35"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    // Create decrpytion object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        ret; // How do we return in rust?
    }
    let xrc = res.unwrap();

    // Decrypt flag and print it out
    let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);
    println!(
        ":?", // How do we print out a variable in the println function? 
        String::from_utf8_lossy(&decrypted_buffer)
    );
}  
```

We can see that the program decrypts the XOR-encrypted flag and prints it.  
The encryption key is `CSUCKS`.

There is also a couple of comments pin-pointing us to the errors:

- `// How do we end statements in Rust?`, with semi-colons.
- `// How do we return in rust?`, with `return`
- `// How do we print out a variable in the println function?`, with the `{}`-characters

### Fix and build the program

Next, we fix and build the program. If you don't have `cargo` installed already this can be done with `sudo apt install cargo`.

```bash
┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_1/fixme1/src]
└─$ vi main.rs           

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_1/fixme1/src]
└─$ cat main.rs 
use xor_cryptor::XORCryptor;

fn main() {
    // Key for decryption
    let key = String::from("CSUCKS");

    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "01", "1c", "7e", "59", "63", "e1", "61", "25", "7f", "5a", "60", "50", "11", "38", "1f", "3a", "60", "e9", "62", "20", "0c", "e6", "50", "d3", "35"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    // Create decrpytion object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return;
    }
    let xrc = res.unwrap();

    // Decrypt flag and print it out
    let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);
    println!(
        "{}",
        String::from_utf8_lossy(&decrypted_buffer)
    );
}

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_1/fixme1/src]
└─$ cd ..      

┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_1/fixme1]
└─$ cargo build
   Compiling crossbeam-utils v0.8.20
   Compiling rayon-core v1.12.1
   Compiling either v1.13.0
   Compiling crossbeam-epoch v0.9.18
   Compiling crossbeam-deque v0.8.5
   Compiling rayon v1.10.0
   Compiling xor_cryptor v1.2.3
   Compiling rust_proj v0.1.0 (/mnt/hgfs/CTFs/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_1/fixme1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.76s
```

Great, we have fixed all the errors!

### Get the flag

Finally, we run the program and get the decrypted flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_1/fixme1]
└─$ cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rust_proj`
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Cargo Tutorial - Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Println Macro - Rust Programming Language](https://doc.rust-lang.org/std/macro.println.html)
- [Rust (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
- [XOR cipher - Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher)
