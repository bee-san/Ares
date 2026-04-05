# Based

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX FULTON/DANIEL TUNITIS
 
Description:
To get truly 1337, you must understand different data encodings, such as hexadecimal or binary. 

Can you get the flag from this program to prove you are on the way to becoming 1337? 

Connect with nc jupiter.challenges.picoctf.org 29221.

Hints:
1. I hear python can convert things.
2. It might help to have multiple windows open.
```

Challenge link: [https://play.picoctf.org/practice/challenge/35](https://play.picoctf.org/practice/challenge/35)

## Solution

Let's connect to the server and see what happens

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Based]
└─$ nc jupiter.challenges.picoctf.org 29221
Let us see how data is stored
light
Please give the 01101100 01101001 01100111 01101000 01110100 as a word.
...
you have 45 seconds.....

Input:
Too slow!
```

Solving this manually will be tedious so lets automate it with the help of [pwntools](https://docs.pwntools.com/en/stable/index.html).
Also note the 5 letter word `light` is printed before the question. This looks like a bug that print the correct answer!

### Write a Python script

The final script looks like this

```python
#!/usr/bin/python

from pwn import *

PORT = 29221
SERVER = 'jupiter.challenges.picoctf.org'

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "warning"

io = remote(SERVER, PORT)

# Please give the <binary strings separated by space> as a word.
# There seems to be a bug that prints the correct answer before the question!
io.recvuntil(b'Let us see how data is stored\n')
answer = io.recvlineS(keepends = False)
log.info(f"First question answer: {answer}")
io.sendlineafter(b'Input:\n', answer.encode('ascii'))   

# Please give me the <octal strings separated by space> as a word.
io.recvuntil(b'Please give me the  ')
octal_str = io.recvuntil(b'as a word.\n').rstrip().decode()
octal_array = octal_str.split()[:-3]
answer = ""
for item in octal_array:
    answer += chr(int(item, 8))
log.info(f"Second question answer: {answer}")
io.sendlineafter(b'Input:\n', answer.encode('ascii'))

# Please give me the <hex string> as a word.
io.recvuntil(b'Please give me the ')
hex_str = io.recvuntil(b'as a word.\n').rstrip().decode()
answer = bytearray.fromhex(hex_str.split()[0]).decode()
log.info(f"Third question answer: {answer}")
io.sendlineafter(b'Input:\n', answer.encode('ascii'))

Youve_beaten = io.recvuntilS(b'\n')
flag = io.recvuntilS(b'\n')
print(flag)

io.close()
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Based]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
Flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Binary number - Wikipedia](https://en.wikipedia.org/wiki/Binary_numbers)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Octal - Wikipedia](https://en.wikipedia.org/wiki/Octal)
