# First Grep

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX FULTON/DANNY TUNITIS

Description:
Can you find the flag in file? 

This would be really tedious to look through manually, something tells me there is a better way.
 
Hints:
1. grep tutorial
```

Challenge link: [https://play.picoctf.org/practice/challenge/85](https://play.picoctf.org/practice/challenge/85)

## Solution

This is basically a very easy tutorial for `grep`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/First_Grep]
└─$ grep picoCTF file
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [Grep and Regular Expressions!](https://ryanstutorials.net/linuxtutorial/grep.php)
