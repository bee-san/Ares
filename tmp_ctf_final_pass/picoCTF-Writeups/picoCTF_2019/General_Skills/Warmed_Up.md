# Warmed Up

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SANJAY C/DANNY TUNITIS

Description:
What is 0x3D (base 16) in decimal (base 10)?

Hints:
1. Submit your answer in our flag format. For example, if your answer was '22', 
   you would submit 'picoCTF{22}' as the flag.
```

Challenge link: [https://play.picoctf.org/practice/challenge/58](https://play.picoctf.org/practice/challenge/58)

## Solution

### Convert in Python

We can use an interactive Python session to do the work for us with the [str function](https://docs.python.org/3/library/functions.html#func-str) and the fact that Python understands [hexadecimal numbers](https://en.wikipedia.org/wiki/Hexadecimal)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Warmed_Up]
└─$ python
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> print('picoCTF{' + str(0x3d) + '}')
picoCTF{61}
>>> exit()
```

### Convert with bc

Alternatively, we can use the tool `bc` to do the convertion. Install it with `sudo apt install bc` if it isn't installed already.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Warmed_Up]
└─$ echo "ibase=16; 3D" | bc
61
```

In this case you need to construct the complete flag manually.

### Convert directly in bash

Finally, we can convert directly in bash

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Warmed_Up]
└─$ echo $((16#3d))
61
```

As before, you need to construct the complete flag manually in this case.

For additional information, please see the references below.

## References

- [bc - Linux manual page](https://man7.org/linux/man-pages/man1/bc.1p.html)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [str function - Python](https://docs.python.org/3/library/functions.html#func-str)
