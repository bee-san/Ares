# Easy1

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX FULTON/DANNY

Description:
The one time pad can be cryptographically secure, but not when you know the key. 
Can you solve this? 

We've given you the encrypted flag, key, and a table to help UFJKXQZQUNB with the key of SOLVECRYPTO. 
Can you use this table to solve it?.

Hints:
1. Submit your answer in our flag format. For example, if your answer was 'hello', 
   you would submit 'picoCTF{HELLO}' as the flag.
2. Please use all caps for the message.
```

Challenge link: [https://play.picoctf.org/practice/challenge/43](https://play.picoctf.org/practice/challenge/43)

## Solution

There are several ways to solve this challenge and here are two of them.

### Use an online decoder service

You can use an online decoder service such as [Braingle](https://www.braingle.com/brainteasers/codes/onetimepad.php) or [Rumkin](https://rumkin.com/tools/cipher/one-time-pad/) to solve this challenge.

In Braingle, use `UFJKXQZQUNB` as `PLAINTEXT / CIPHERTEXT` and `SOLVECRYPTO` as `ONE-TIME PAD`. Click `Decipher` to get the flag.

In Rumkim, set `Operating mode` to `Decrypt`, set `SOLVECRYPTO` as `The pad` and `UFJKXQZQUNB` as `Text to encode or decode`.

### Write a Python decoder

Alternatively, you can write a Python script to do the decoding

```python
#!/usr/bin/python

def decode(chiffer, key):
    return chr((ord(chiffer) - ord(key))%26 + ord('A'))

chiffer = 'UFJKXQZQUNB'
key = 'SOLVECRYPTO'

result = ''
for pos in range(0, len(chiffer)):
    result += decode(chiffer[pos], key[pos])
print(f"picoCTF{{{result}}}")
```

Then we make sure the script is executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Easy1]
└─$ chmod +x decode.py   

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Easy1]
└─$ ./decode.py       
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [Modulo - Wikipedia](https://en.wikipedia.org/wiki/Modulo)
- [One-time pad - Wikipedia](https://en.wikipedia.org/wiki/One-time_pad)
- [One Time Pad Decoder - Rumkin](https://rumkin.com/tools/cipher/one-time-pad/)
- [One-time Pad Cipher Decoder - Braingle](https://www.braingle.com/brainteasers/codes/onetimepad.php)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
