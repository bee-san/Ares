# interencdec

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Cryptography, base64, browser_webshell_solvable, caesar
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: NGIRIMANA SCHADRACK
 
Description:
Can you get the real meaning from this file.

Download the file here.

Hints:
1. Engaging in various decoding processes is of utmost importance
```

Challenge link: [https://play.picoctf.org/practice/challenge/418](https://play.picoctf.org/practice/challenge/418)

## Solution

## Base64-decoding

The given `enc_file` file contains the following:

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/interencdec]
└─$ cat enc_flag               
YidkM0JxZGtwQlRYdHFhR3g2YUhsZmF6TnFlVGwzWVROclh6ZzJhMnd6TW1zeWZRPT0nCg==
```

The padding characters (`=`) at the end reveals that this is likely [base64-encoded data](https://en.wikipedia.org/wiki/Base64).  

Let's decode it with `base64`:

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/interencdec]
└─$ cat enc_flag | base64 -d
b'd3BqdkpBTXtqaGx6aHlfazNqeTl3YTNrXzg2a2wzMmsyfQ=='
```

Still base64-endoded but in python byte-format.  

Another round of decoding:

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/interencdec]
└─$ echo "d3BqdkpBTXtqaGx6aHlfazNqeTl3YTNrXzg2a2wzMmsyfQ==" | base64 -d
wpjvJAM{jhlzhy_k3jy9wa3k_86kl32k2}  
```

Now this looks like a rotation cipher like [Caesar](https://en.wikipedia.org/wiki/Caesar_cipher) or [ROT13](https://en.wikipedia.org/wiki/ROT13). The caesar cipher rotates 3 positions whereas ROT13 rotates 13 positions.

## Get the flag - caesar tool solution

We can try to bruteforce the cipher with the `caesar` tool from the bsdgames package.  
The tool uses English letter frequency statistics to crack the cipher.
Install it with `sudo apt install bsdgames` if needed.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/interencdec]
└─$ echo "d3BqdkpBTXtqaGx6aHlfazNqeTl3YTNrXzg2a2wzMmsyfQ==" | base64 -d | caesar 
picoCTF{<REDACTED>}  
```

Success, we get the flag.

## Get the flag - python script solution

A more manual approach is to brute-force the cipher with a python script

```python
#!/usr/bin/python

import string

alphabet = string.ascii_lowercase
alpha_len = len(alphabet)

def shift(cipher_text, key):
    result = ''
    for c in cipher_text:
        if c.islower():
            result += alphabet[(alphabet.index(c) + key) % alpha_len]
        elif c.isupper():
            result += alphabet[(alphabet.index(c.lower()) + key) % alpha_len].upper()
        else:
            result += c
    return result

# Encrypted data after base64-decoding (twice)
enc_data = 'wpjvJAM{jhlzhy_k3jy9wa3k_86kl32k2}'

for i in range(1, alpha_len+1):
    plain = shift(enc_data, i)
    if ('picoCTF' in plain):
        print("ROT-%02d: %s" % (i, plain))
```

This also gives us the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Cryptography/interencdec]
└─$ ./solve.py                                                        
ROT-19: picoCTF{<REDACTED>}
```

and tells us that the rotation used was 19 characters.

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [caesar - Linux manual page](https://manpages.debian.org/testing/bsdgames/caesar.6.en.html)
- [Caesar cipher - Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [ROT13 - Wikipedia](https://en.wikipedia.org/wiki/ROT13)
