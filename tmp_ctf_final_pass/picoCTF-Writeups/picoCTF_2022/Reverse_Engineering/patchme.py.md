# patchme.py

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Can you get the flag?

Run this Python program in the same directory as this encrypted flag.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/287](https://play.picoctf.org/practice/challenge/287)

## Solution

Let's start by looking at the Python source code (with some empty lines removed)

```python
### THIS FUNCTION WILL NOT HELP YOU FIND THE FLAG --LT ########################
def str_xor(secret, key):
    #extend key to secret length
    new_key = key
    i = 0
    while len(new_key) < len(secret):
        new_key = new_key + key[i]
        i = (i + 1) % len(key)        
    return "".join([chr(ord(secret_c) ^ ord(new_key_c)) for (secret_c,new_key_c) in zip(secret,new_key)])
###############################################################################

flag_enc = open('flag.txt.enc', 'rb').read()

def level_1_pw_check():
    user_pw = input("Please enter correct password for flag: ")
    if( user_pw == "ak98" + \
                   "-=90" + \
                   "adfjhgj321" + \
                   "sleuth9000"):
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), "utilitarian")
        print(decryption)
        return
    print("That password is incorrect")

level_1_pw_check()
```

In the `level_1_pw_check` function we see a password comparision for some strings concatenated together.
The plus operator just adds the strings together resulting in the string `ak98-=90adfjhgj321sleuth9000`.

Use this as the password when running the script and you get the flag

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Reverse_Engineering/Patchme.py]
└─$ python patchme.flag.py
Please enter correct password for flag: ak98-=90adfjhgj321sleuth9000
Welcome back... your flag, user:
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python Tutorial - 7 Ways to Concatenate Strings in Python](https://www.pythontutorial.net/python-string-methods/python-string-concatenation/)
- [Python - Common string operations](https://docs.python.org/3/library/string.html)
