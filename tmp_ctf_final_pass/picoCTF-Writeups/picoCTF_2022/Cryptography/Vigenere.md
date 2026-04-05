# Vigenere

- [Challenge information](#challenge-information)
- [Online solver solution](#online-solver-solution)
- [Python solution](#python-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
Can you decrypt this message?

Decrypt this message using this key "CYLAB".

Hints:
1. https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
```

Challenge link: [https://play.picoctf.org/practice/challenge/316](https://play.picoctf.org/practice/challenge/316)

The message given looks like this

```text
rgnoDVD{O0NU_WQ3_G1G3O3T3_A1AH3S_f85729e7}
```

There probably are more ways to solve this challenge, but here are two solutions.

## Online solver solution

You can use an online solver such as [Rumkin](https://rumkin.com/tools/cipher/vigenere/) to solve this challenge.

Set the 'Operating Mode' to `Decrypt` and set the 'Cipher key' to `CYLAB`.  
Then enter the cipher text in the large text field and you get the flag at the bottom of the window.

## Python solution

In addition, let's write a small Python script called `solve.py` to decode this

```python
#!/usr/bin/python
# -*- coding: latin-1 -*-

import string

cipher_text = "rgnoDVD{O0NU_WQ3_G1G3O3T3_A1AH3S_f85729e7}"
key = 'CYLAB'

universe = string.ascii_uppercase
uni_len = len(universe)

flag = ''
k_len = len(key)

i = 0
for c in cipher_text:
    if c.islower():
        txt_index = universe.index(c.upper())
        key_index = universe.index(key[i % k_len])
        i += 1
        flag += universe[(txt_index - key_index) % uni_len].lower()
    elif c.isupper():
        txt_index = universe.index(c)
        key_index = universe.index(key[i % k_len])
        i += 1
        flag += universe[(txt_index - key_index) % uni_len]
    else:
        flag += c    

print(flag)
```

Then make the script executable and run it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Vigenere]
└─$ chmod +x solve.py       

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Vigenere]
└─$ ./solve.py         
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Vigenère cipher - Wikipedia](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)
