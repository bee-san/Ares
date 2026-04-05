# strings it

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
Can you find the flag in file without running it?

Hints:
1. strings
```

Challenge link: [https://play.picoctf.org/practice/challenge/37](https://play.picoctf.org/practice/challenge/37)

## Solution

This is basically a tutorial to usage of `strings` and `grep`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Strings_it]
└─$ strings -n 8 strings | grep picoCTF
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
