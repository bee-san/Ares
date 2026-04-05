# extensions

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author:  SANJAY C/DANNY

Description:
This is a really weird text file TXT? Can you find the flag?

Hints:
1. How do operating systems know what kind of file it is? (It's not just the ending!
2. Make sure to submit the flag as picoCTF{XXXXX}
```

Challenge link: [https://play.picoctf.org/practice/challenge/52](https://play.picoctf.org/practice/challenge/52)

## Solution

Let's start by checking the file with `file`.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Extensions]
└─$ file flag.txt 
flag.txt: PNG image data, 1697 x 608, 8-bit/color RGB, non-interlaced
```

Ah, it's a PNG picture file, not a text file.

To view the flag use a tool such as `eog` of `feh`.

For additional information, please see the references below.

## References

- [feh - Linux manual page](https://linux.die.net/man/1/feh)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [Filename extension - Wikipedia](https://en.wikipedia.org/wiki/Filename_extension)
