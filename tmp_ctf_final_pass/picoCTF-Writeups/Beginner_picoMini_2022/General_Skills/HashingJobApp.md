# HashingJobApp

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, hashing, nc, shell, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
If you want to hash with the best, beat this test!

nc saturn.picoctf.net 55823

Hints:
1. You can use a commandline tool or web app to hash text
2. Press Ctrl and c on your keyboard to close your connection and return to the command prompt.
```

Challenge link: [https://play.picoctf.org/practice/challenge/243](https://play.picoctf.org/practice/challenge/243)

## Solution

### Manual solution

Let's start by connecting to the server

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/HashingJobApp]
└─$ nc saturn.picoctf.net 55823
Please md5 hash the text between quotes, excluding the quotes: 'Greenpeace'
Answer: 
```

You are expected to hash the text. This can be done with an online service such as [Tools 4 noobs](https://www.tools4noobs.com/online_tools/hash/) or the tool `md5sum` as shown below.

Open a separate windows and run

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/HashingJobApp]
└─$ echo -n 'Greenpeace' | md5sum                                                  
7628ecff54896cb076074261828e6623  -
```

Note that the `-n` parameter is important. This prevents a trailing newline (which is the default) to be added which will change the hash.  
Copy the hash (the long hexadecimal number) and paste it in as the answer.

You need to be rather fast or otherwise the server disconnects you

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/HashingJobApp]
└─$ nc saturn.picoctf.net 55823
Please md5 hash the text between quotes, excluding the quotes: 'Helen Keller'
Answer: 
Time's up. Press Ctrl-C to disconnect. Feel free to reconnect and try again.
```

After you have been disconnected, new text will be randomly selected.

After three correct hashes are provided, you get the flag.

### Automated solution with pwntools

A timed challenge like this is nice to automate with [pwntools](https://docs.pwntools.com/en/stable/index.html).

Let's write a small Python script called `pwn_solve.py` that solves this challenge for us

```python
#!/usr/bin/python
# -*- coding: latin-1 -*-

import hashlib
from pwn import *
import re

SERVER = 'saturn.picoctf.net'
PORT = 55823

def generateHash(word): 
    return hashlib.md5(word.encode()).hexdigest()

conn = remote(SERVER, PORT)
for i in range(3):
    question = conn.recvline().decode().strip()
    word = re.findall('Please md5 hash the text between quotes, excluding the quotes: \'(.*)\'', question)[0]
    answer = conn.recvline()
    hash = generateHash(word).encode()
    conn.sendline(hash)
    hashline = conn.recvline()
    correct = conn.recvline()
flag = conn.recvline().decode().strip()
conn.close()
print(flag)
```

Then I run my virtual Python environment with pwntools to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/HashingJobApp]
└─$ ~/python_venvs/pwntools/bin/python3  ./pwn_solve.py
[+] Opening connection to saturn.picoctf.net on port 55823: Done
[*] Closed connection to saturn.picoctf.net port 55823
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [echo - Linux man page](https://linux.die.net/man/1/echo)
- [hashlib module - Python](https://docs.python.org/3/library/hashlib.html)
- [MD5 - Wikipedia](https://en.wikipedia.org/wiki/MD5)
- [md5sum - Linux man page](https://linux.die.net/man/1/md5sum)
- [nc - Linux man page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
