# Custom encryption

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Cryptography, browser_webshell_solvable, ASCII_encoding, XOR
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: NGIRIMANA SCHADRACK
 
Description:
Can you get sense of this code file and write the function that will decode the 
given encrypted file content.

Find the encrypted file here flag_info and code file might be good to analyze 
and get the flag.

Hints:
1. Understanding encryption algorithm to come up with decryption algorithm.
```

Challenge link: [https://play.picoctf.org/practice/challenge/412](https://play.picoctf.org/practice/challenge/412)

## Solution

### Analysis of the custom encryption

The given `enc_flag` file contains the following:

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/Custom_encryption]
└─$ cat enc_flag                        
a = 94
b = 29
cipher is: [260307, 491691, 491691, 2487378, 2516301, 0, 1966764, 1879995, 1995687, 1214766, 0, 2400609, 607383, 144615, 1966764, 0, 636306, 2487378, 28923, 1793226, 694152, 780921, 173538, 173538, 491691, 173538, 751998, 1475073, 925536, 1417227, 751998, 202461, 347076, 491691]
```

And the python source code looks like this

```python
from random import randint
import sys


def generator(g, x, p):
    return pow(g, x) % p


def encrypt(plaintext, key):
    cipher = []
    for char in plaintext:
        cipher.append(((ord(char) * key*311)))
    return cipher


def is_prime(p):
    v = 0
    for i in range(2, p + 1):
        if p % i == 0:
            v = v + 1
    if v > 1:
        return False
    else:
        return True


def dynamic_xor_encrypt(plaintext, text_key):
    cipher_text = ""
    key_length = len(text_key)
    for i, char in enumerate(plaintext[::-1]):
        key_char = text_key[i % key_length]
        encrypted_char = chr(ord(char) ^ ord(key_char))
        cipher_text += encrypted_char
    return cipher_text


def test(plain_text, text_key):
    p = 97
    g = 31
    if not is_prime(p) and not is_prime(g):
        print("Enter prime numbers")
        return
    a = randint(p-10, p)
    b = randint(g-10, g)
    print(f"a = {a}")
    print(f"b = {b}")
    u = generator(g, a, p)
    v = generator(g, b, p)
    key = generator(v, a, p)
    b_key = generator(u, b, p)
    shared_key = None
    if key == b_key:
        shared_key = key
    else:
        print("Invalid key")
        return
    semi_cipher = dynamic_xor_encrypt(plain_text, text_key)
    cipher = encrypt(semi_cipher, shared_key)
    print(f'cipher is: {cipher}')


if __name__ == "__main__":
    message = sys.argv[1]
    test(message, "trudeau")
```

From the code we can see that the encryption is a combination of:

- Reversing the order of the characters in the  `dynamic_xor_encrypt` function (`enumerate(plaintext[::-1]`),
- [XOR](https://en.wikipedia.org/wiki/Exclusive_or) performed in the `dynamic_xor_encrypt` function (`ord(char) ^ ord(key_char)`) and
- Multiplication of the characters [ASCII-values](https://en.wikipedia.org/wiki/ASCII) performed in the `encrypt` function (`ord(char) * key * 311)`)

To decrypt, we need to perform the reverse operations in the reverse order:

- Division with the same values in a `decrypt` function
- XOR with the same key in a `dynamic_xor_decrypt` function (the reverse of XOR is XOR)
- Reversing the order of the characters

### Create a python decoding script

The final decryption script looks like this

```python
#!/usr/bin/python

# Given in the enc_flag file
a = 94
b = 29
cipher = [260307, 491691, 491691, 2487378, 2516301, 0, 1966764, 1879995, 1995687, 1214766, 0, 2400609, 607383, 144615, 1966764, 0, 636306, 2487378, 28923, 1793226, 694152, 780921, 173538, 173538, 491691, 173538, 751998, 1475073, 925536, 1417227, 751998, 202461, 347076, 491691]

# Passed to test function from main
text_key = "trudeau"

# Unchanged function
def generator(g, x, p):
    return pow(g, x) % p

# Division instead of multiplication
def decrypt(ciphertext, key):
    plain = []
    for num in ciphertext:
        plain.append(chr(int(num / key / 311)))
    return plain

# No reversing of the order of the text chars
def dynamic_xor_decrypt(ciphertext, key):
    text = ""
    key_length = len(key)
    for i, char in enumerate(ciphertext):
        key_char = key[i % key_length]
        decrypted_char = chr(ord(char) ^ ord(key_char))
        text += decrypted_char
    return text

if __name__ == "__main__":

    # Unchanged code from test function
    p = 97
    g = 31
    u = generator(g, a, p)
    v = generator(g, b, p)
    key = generator(v, a, p)
    b_key = generator(u, b, p)
    shared_key = None
    if key == b_key:
        shared_key = key
    else:
        print("Invalid key")
        exit(1)  # Exit instead of return
    
    plain = decrypt(cipher, shared_key)
    flag = dynamic_xor_decrypt(plain, text_key)
    print(flag[::-1])
```

## Get the flag

Finally we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/Custom_encryption]
└─$ python custom_decryption.py 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
