# Lookey here

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Forensics, grep
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES / MUBARAK MIKAIL

Description:
Attackers have hidden information in a very large mass of data in the past, maybe they are still doing it.

Download the data here.

Hints:
1. Download the file and search for the flag based on the known prefix.
```

Challenge link: [https://play.picoctf.org/practice/challenge/279](https://play.picoctf.org/practice/challenge/279)

## Solution

The most efficient way to get the flag is to use `grep` with `-o` to only output the matched text  
and `-E` to say that your pattern is an extended regular expression

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Lookey_here]
└─$ grep -oE 'picoCTF{.*}' anthem.flag.txt
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
