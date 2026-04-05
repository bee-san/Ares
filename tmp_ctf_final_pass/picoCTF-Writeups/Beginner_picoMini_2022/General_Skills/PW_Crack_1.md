# PW Crack 1

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
1. To view the file in the webshell, do: $ nano level1.py
2. To exit nano, press Ctrl and x and follow the on-screen prompts.
3. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/245](https://play.picoctf.org/practice/challenge/245)

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

flag_enc = open('level1.flag.txt.enc', 'rb').read()

def level_1_pw_check():
    user_pw = input("Please enter correct password for flag: ")
    if( user_pw == "691d"):
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), user_pw)
        print(decryption)
        return
    print("That password is incorrect")

level_1_pw_check()
```

The most interesting part is of course the IF statement where we see the password in plain text

```python
<---snip--->
    user_pw = input("Please enter correct password for flag: ")
    if( user_pw == "691d"):
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), user_pw)
        print(decryption)
        return
<---snip--->
```

With knowledge of the password (`691d`) we can run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/PW_Crack_1]
└─$ python level1.py
Please enter correct password for flag: 691d
Welcome back... your flag, user:
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python - List Comprehension - W3Schools](https://www.w3schools.com/python/python_lists_comprehension.asp)
- [XOR cipher - Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher)
- [zip() in Python - GeeksforGeeks](https://www.geeksforgeeks.org/zip-in-python/)
