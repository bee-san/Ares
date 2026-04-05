# PW Crack 5

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: Beginner picoMini 2022, General Skills, password_cracking, hashing
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES
 
Description:
Can you crack the password to get the flag?
 
Download the password checker here and you'll need the encrypted flag and the hash in the same directory too. 

Here's a dictionary with all possible passwords based on the password conventions we've seen so far.

Hints:
1. Opening a file in Python is crucial to using the provided dictionary.
2. You may need to trim the whitespace from the dictionary word before hashing. 
   Look up the Python string function, strip
3. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/249](https://play.picoctf.org/practice/challenge/249)

## Solution

Let's start with analysing the Python script. The script looks like this (with some empty lines removed)

```python
import hashlib

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

flag_enc = open('level5.flag.txt.enc', 'rb').read()
correct_pw_hash = open('level5.hash.bin', 'rb').read()

def hash_pw(pw_str):
    pw_bytes = bytearray()
    pw_bytes.extend(pw_str.encode())
    m = hashlib.md5()
    m.update(pw_bytes)
    return m.digest()

def level_5_pw_check():
    user_pw = input("Please enter correct password for flag: ")
    user_pw_hash = hash_pw(user_pw)
    
    if( user_pw_hash == correct_pw_hash ):
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), user_pw)
        print(decryption)
        return
    print("That password is incorrect")

level_5_pw_check()
```

Just like the previous challenges we brute force the solution.

First we change the `level_5_pw_check` function slightly

```python
def level_5_pw_check(user_pw):
    user_pw_hash = hash_pw(user_pw)
    
    if( user_pw_hash == correct_pw_hash ):
        print("Correct password is: %s" % user_pw)
        print("Welcome back... your flag, user:")
        decryption = str_xor(flag_enc.decode(), user_pw)
        print(decryption)
        return
```

Then we also add code to iterate through the dictionary

```python
pos_pw_list = open("dictionary.txt", "r").readlines()

for pw in pos_pw_list:
    level_5_pw_check(pw.strip())
```

We can leave the rest of the code unchanged.

Finally, we run the brute forcer to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/PW_Crack_5]
└─$ python pw_crack_5_get_flag.py

Correct password is: 9581
Welcome back... your flag, user:
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [Brute-force attack - Wikipedia](https://en.wikipedia.org/wiki/Brute-force_attack)
- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [hashlib module - Python](https://docs.python.org/3/library/hashlib.html)
- [MD5 - Wikipedia](https://en.wikipedia.org/wiki/MD5)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python - List Comprehension - W3Schools](https://www.w3schools.com/python/python_lists_comprehension.asp)
- [zip() in Python - GeeksforGeeks](https://www.geeksforgeeks.org/zip-in-python/)
