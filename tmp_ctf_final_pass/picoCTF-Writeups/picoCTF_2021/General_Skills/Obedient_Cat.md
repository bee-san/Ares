# Obedient Cat

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL
  
Description:
This file has a flag in plain sight (aka "in-the-clear"). 
Download flag.
 
Hints:
1. Any hints about entering a command into the Terminal (such as the next one), will start with 
   a '$'... everything after the dollar sign will be typed (or copy and pasted) into your Terminal.
2. To get the file accessible in your shell, enter the following in the Terminal prompt: 
   $ wget https://mercury.picoctf.net/static/fb851c1858cc762bd4eed569013d7f00/flag
3. $ man cat
```

Challenge link: [https://play.picoctf.org/practice/challenge/147](https://play.picoctf.org/practice/challenge/147)

## Solution

picoCTF is a VERY beginner friendly CTF and this must be one of the easiest challenges ever.

As suggested in the hints use either `wget` or your browser (Right-click and select Save link as...) to download
the flag file.

Then use `cat` (on Linux), `type` (on Windows) or any text editor to view the flag.

For additional information, please see the references below.

## References

- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [wget - Linux manual page](https://man7.org/linux/man-pages/man1/wget.1.html)
- [type - Windows Command](https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/cc732507(v=ws.11))
