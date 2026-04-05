# basic-mod2

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG

Description:
A new modular challenge! Download the message here.

Take each number mod 41 and find the modular inverse for the result. 

Then map to the following character set: 1-26 are the alphabet, 27-36 are the decimal digits, 
and 37 is an underscore.

Wrap your decrypted message in the picoCTF flag format (i.e. picoCTF{decrypted_message})

Hints:
1. Do you know what the modular inverse is?
2. The inverse modulo z of x is the number, y that when multiplied by x is 1 modulo z
3. It's recommended to use a tool to find the modular inverses
```

Challenge link: [https://play.picoctf.org/practice/challenge/254](https://play.picoctf.org/practice/challenge/254)

## Solution

The code for this challenge is almost identical to the [previous challenge](basic-mod1.md).

For my implementation of modular inverse I found code on [StackOverflow](https://stackoverflow.com/questions/4798654/modular-multiplicative-inverse-function-in-python). It's very easy now in Python 3.8 and later...

```python
#!/usr/bin/python

# Read the encoded flag as string
with open("message.txt", 'r') as fh:
    enc_string = fh.read().strip()

# Convert to array of numbers
enc_numbers = map(int, enc_string.split())

# Create decode array
base_37 = []
for i in range(26):
    base_37 += chr(ord('A') + i)
for i in range(10):
    base_37 += chr(ord('0') + i)
base_37 += '_'

# Decode flag and print it
flag = []
for x in enc_numbers:
    flag += base_37[pow(x, -1, 41) - 1]
print('picoCTF{%s}' % "".join(flag))
```

Then make the script executable and run it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Basic_Mod2]
└─$ chmod +x get_flag.py  

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Basic_Mod2]
└─$ ./get_flag.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Modulo - Wikipedia](https://en.wikipedia.org/wiki/Modulo)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
