# PW Crack 2

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, password_cracking
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES
  
Description:
Can you crack the password to get the flag?

Download the password checker here and you'll need the encrypted flag in the same directory too.

Hints:
1. Does that encoding look familiar?
2. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/246](https://play.picoctf.org/practice/challenge/246)

## Solution

Let's start with analysing the Python script. The script looks like this (with some empty lines removed)

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

flag_enc = open('level2.flag.txt.enc', 'rb').read()

def level_2_pw_check():
    user_pw = input("Please enter correct password for flag: ")
    if( user_pw == chr(0x34) + chr(0x65) + chr(0x63) + chr(0x39) ):
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), user_pw)
        print(decryption)
        return
    print("That password is incorrect")

level_2_pw_check()
```

As in the previous challenge, the interesting part is the IF statement

```python
<---snip--->
    user_pw = input("Please enter correct password for flag: ")
    if( user_pw == chr(0x34) + chr(0x65) + chr(0x63) + chr(0x39) ):
        print("Welcome back... your flag, user:")
<---snip--->
```

The password is encoded as [ASCII-numbers](https://en.wikipedia.org/wiki/ASCII). To get the password in plain text you can either

- Lookup the characters manually in [an ASCII-table](https://www.ascii-code.com/)
- Use an interactive Python session to give you the answer (see below)

Using Python to get the password

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/PW_Crack_2]
└─$ python          
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> print(chr(0x34) + chr(0x65) + chr(0x63) + chr(0x39))
4ec9
>>> quit()
```

So the password is `4ec9`. Finally, we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/PW_Crack_2]
└─$ python level2.py
Please enter correct password for flag: 4ec9
Welcome back... your flag, user:
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [chr()-function - Python](https://docs.python.org/3/library/functions.html#chr)
- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python - List Comprehension - W3Schools](https://www.w3schools.com/python/python_lists_comprehension.asp)
- [XOR cipher - Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher)
- [zip() in Python - GeeksforGeeks](https://www.geeksforgeeks.org/zip-in-python/)
