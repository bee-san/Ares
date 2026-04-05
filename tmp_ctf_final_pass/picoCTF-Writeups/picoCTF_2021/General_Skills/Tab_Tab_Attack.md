# Tab, Tab, Attack

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
Using tabcomplete in the Terminal will add years to your life, esp. when dealing with 
long rambling directory structures and filenames: Addadshashanammu.zip
 
Hints:
1. After `unzip`ing, this problem can be solved with 11 button-presses...(mostly Tab)...
```

Challenge link: [https://play.picoctf.org/practice/challenge/176](https://play.picoctf.org/practice/challenge/176)

## Solution

### Likely intented solution

Based on the challenge name this is the likely intended solution. The challenge is mainly an exercise in how to use [tab completion](https://en.wikipedia.org/wiki/Command-line_completion).

First we need to unpack the file with `unzip`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Tab_Tab_Attack]
└─$ unzip Addadshashanammu.zip 
Archive:  Addadshashanammu.zip
   creating: Addadshashanammu/
   creating: Addadshashanammu/Almurbalarammi/
   creating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/
   creating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/
   creating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/
   creating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/Onnissiralis/
   creating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku/
  inflating: Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku/fang-of-haynekhtnamet  
```

Then we need to change directory with `cd` to the find the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Tab_Tab_Attack]
└─$ cd Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku 

┌──(kali㉿kali)-[/mnt/…/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku]
└─$ ls
fang-of-haynekhtnamet
```

Let's check what kind of file it is with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku]
└─$ file fang-of-haynekhtnamet 
fang-of-haynekhtnamet: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=fcea24fb5379795a123bb860267d815e889a6d23, not stripped
```

Ah, a 64-bit ELF binary.

Why not run it?

```bash
┌──(kali㉿kali)-[/mnt/…/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku]
└─$ ./fang-of-haynekhtnamet                                                                                  
*ZAP!* picoCTF{<REDACTED>}
```

And there is the flag!

### The smarter solution

A smarter solution is to unpack the zip-file without recreating the directory structure (`-j` parameter)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Tab_Tab_Attack]
└─$ unzip -j Addadshashanammu.zip 
Archive:  Addadshashanammu.zip
  inflating: fang-of-haynekhtnamet

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Tab_Tab_Attack]
└─$ file fang-of-haynekhtnamet                                  
fang-of-haynekhtnamet: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=fcea24fb5379795a123bb860267d815e889a6d23, not stripped
```

Then we run the program to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Tab_Tab_Attack]
└─$ ./fang-of-haynekhtnamet 
*ZAP!* picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [cd - Linux manual page](https://man7.org/linux/man-pages/man1/cd.1p.html)
- [Command-line completion - Wikipedia](https://en.wikipedia.org/wiki/Command-line_completion)
- [ls - Linux manual page](https://man7.org/linux/man-pages/man1/ls.1.html)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
