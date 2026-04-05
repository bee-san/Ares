# Codebook

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, shell, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES
  
Description:
Run the Python script code.py in the same directory as codebook.txt.

Download code.py
Download codebook.txt

Hints:
1. On the webshell, use ls to see if both files are in the directory you are in
2. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/238](https://play.picoctf.org/practice/challenge/238)

## Solution

Most of the time you just make sure the script is executable and then run it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Codebook]
└─$ chmod +x code.py                                                        

┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Codebook]
└─$ ./code.py 
./code.py: 2: import: not found
./code.py: 3: import: not found
./code.py: 7: Syntax error: "(" unexpected
```

But in this case that doesn't work. The reason for this is that the script doesn't contain a so called 'shebang' - a special comment specifying what kind of program/interpreter that should execute the script. It normally looks something like this `#!/usr/bin/python3`.

Let's display the first lines of the script with `head` to verify this.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Codebook]
└─$ head code.py 

import random
import sys

def str_xor(secret, key):
    #extend key to secret length
    new_key = key
    i = 0
```

Yes. the shebang is missing and we need to explicitly say that Python should run the script like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Codebook]
└─$ python code.py 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [chmod — Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [Executing Python Scripts With a Shebang - Real Python](https://realpython.com/python-shebang/)
- [head — Linux manual page](https://man7.org/linux/man-pages/man1/head.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Shebang (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Shebang_(Unix))
