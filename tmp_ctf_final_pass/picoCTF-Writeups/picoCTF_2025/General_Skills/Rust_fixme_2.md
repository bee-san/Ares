# Rust fixme 2

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
The Rust saga continues? I ask you, can I borrow that, pleeeeeaaaasseeeee?
Download the Rust code here.

Hints:
1. https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
```

Challenge link: [https://play.picoctf.org/practice/challenge/462](https://play.picoctf.org/practice/challenge/462)

## Solution

We start by unpacking the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_2]
└─$ tar xvfz fixme2.tar.gz
fixme2/
fixme2/Cargo.toml
fixme2/Cargo.lock
fixme2/src/
fixme2/src/main.rs
```

### Study the source code

Then we navigate to the `src` directory and check the source code

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_2]
└─$ cd fixme2/src 

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_2/fixme2/src]
└─$ cat main.rs  
use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer:Vec<u8>, borrowed_string: &String){ // How do we pass values to a function that we want to change?

    // Key for decryption
    let key = String::from("CSUCKS");

    // Editing our borrowed value
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decrpytion object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return; // How do we return in rust?
    }
    let xrc = res.unwrap();

    // Decrypt flag and print it out
    let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);
    borrowed_string.push_str(&String::from_utf8_lossy(&decrypted_buffer));
    println!("{}", borrowed_string);
}


fn main() {
    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "01", "1c", "7e", "59", "63", "e1", "61", "25", "0d", "c4", "60", "f2", "12", "a0", "18", "03", "51", "03", "36", "05", "0e", "f9", "42", "5b"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    let party_foul = String::from("Using memory unsafe languages is a: "); // Is this variable changeable?
    decrypt(encrypted_buffer, &party_foul); // Is this the correct way to pass a value to a function so that it can be changed?
} 
```

As in the [previous challenge](Rust_fixme_1.md) there are comments pin-pointing us to the errors:

- `// How do we pass values to a function that we want to change?`, with `mut` as in mutable.
- `// How do we return in rust?`, no error. (Mistakenly left over from previous challenge?)
- `// Is this variable changeable?`, add `mut`
- `// Is this the correct way to pass a value to a function so that it can be changed?`, again add `mut`

### Fix and build the program

Next, we fix and build the program. If you don't have `cargo` installed already this can be done with `sudo apt install cargo`.

```bash
┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_2/fixme2/src]
└─$ vi main.rs 

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_2/fixme2/src]
└─$ cat main.rs 
use xor_cryptor::XORCryptor;

fn decrypt(encrypted_buffer:Vec<u8>, borrowed_string: &mut String){

    // Key for decryption
    let key = String::from("CSUCKS");

    // Editing our borrowed value
    borrowed_string.push_str("PARTY FOUL! Here is your flag: ");

    // Create decrpytion object
    let res = XORCryptor::new(&key);
    if res.is_err() {
        return;
    }
    let xrc = res.unwrap();

    // Decrypt flag and print it out
    let decrypted_buffer = xrc.decrypt_vec(encrypted_buffer);
    borrowed_string.push_str(&String::from_utf8_lossy(&decrypted_buffer));
    println!("{}", borrowed_string);
}


fn main() {
    // Encrypted flag values
    let hex_values = ["41", "30", "20", "63", "4a", "45", "54", "76", "01", "1c", "7e", "59", "63", "e1", "61", "25", "0d", "c4", "60", "f2", "12", "a0", "18", "03", "51", "03", "36", "05", "0e", "f9", "42", "5b"];

    // Convert the hexadecimal strings to bytes and collect them into a vector
    let encrypted_buffer: Vec<u8> = hex_values.iter()
        .map(|&hex| u8::from_str_radix(hex, 16).unwrap())
        .collect();

    let mut party_foul = String::from("Using memory unsafe languages is a: ");
    decrypt(encrypted_buffer, &mut party_foul);
}

┌──(kali㉿kali)-[/mnt/…/General_Skills/Rust_fixme_2/fixme2/src]
└─$ cd ..  

┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_2/fixme2]
└─$ cargo build
   Compiling crossbeam-utils v0.8.20
   Compiling rayon-core v1.12.1
   Compiling either v1.13.0
   Compiling crossbeam-epoch v0.9.18
   Compiling crossbeam-deque v0.8.5
   Compiling rayon v1.10.0
   Compiling xor_cryptor v1.2.3
   Compiling rust_proj v0.1.0 (/mnt/hgfs/CTFs/picoCTF/picoCTF_2025/General_Skills/Rust_fixme_2/fixme2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.98s
```

Great, no errors.

### Get the flag

Finally, we run the program and get the decrypted flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2025/General_Skills/Rust_fixme_2/fixme2]
└─$ cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rust_proj`
Using memory unsafe languages is a: PARTY FOUL! Here is your flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Cargo Tutorial - Rust Programming Language](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [References and Borrowing - Rust Programming Language](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
