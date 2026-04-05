# Glitch Cat

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, nc, shell, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Our flag printing service has started glitching!

$ nc saturn.picoctf.net 50363

Hints:
1. ASCII is one of the most common encodings used in programming
2. We know that the glitch output is valid Python, somehow!
3. Press Ctrl and c on your keyboard to close your connection and return to the command prompt.
```

Challenge link: [https://play.picoctf.org/practice/challenge/242](https://play.picoctf.org/practice/challenge/242)

## Solution

Connect to the flag printing service

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Glitch_Cat]
└─$ nc saturn.picoctf.net 50363
'picoCTF{gl17ch_m3_n07_' + chr(0x61) + chr(0x34) + chr(0x33) + chr(0x39) + chr(0x32) + chr(0x64) + chr(0x32) + chr(0x65) + '}'
```

The first part of the flag looks correct, but the last part looks rather like python code.

Let's try to execute it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Glitch_Cat]
└─$ python -c "print('picoCTF{gl17ch_m3_n07_' + chr(0x61) + chr(0x34) + chr(0x33) + chr(0x39) + chr(0x32) + chr(0x64) + chr(0x32) + chr(0x65) + '}')"
picoCTF{<REDACTED>}
```

And we get the complete flag (but redacted here).  

The plus operator can also "add" strings together. This is called concatenation.  
The `chr` function returns the ASCII-character of the value.  
Numbers preceded with '0x' are in hexadecimal.

For additional information, please see the references below.

### References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [chr()-function - Python](https://docs.python.org/3/library/functions.html#chr)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [nc - Linux man page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python Operators - W3Schools](https://www.w3schools.com/python/python_operators.asp)
