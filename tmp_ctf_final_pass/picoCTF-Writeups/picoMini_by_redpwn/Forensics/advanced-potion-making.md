# advanced-potion-making

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoMini by redpwn, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: BIGC

Description:
Ron just found his own copy of advanced potion making, but its been corrupted by some kind of spell. 
Help him recover it!

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/205](https://play.picoctf.org/practice/challenge/205)

## Solution

### Analyse the file

Let's start by checking the given file with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Forensics/advanced-potion-making]
└─$ file advanced-potion-making 
advanced-potion-making: data
```

Hhm, let's check it in hex form and see if we can recognise anything interesting

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Forensics/advanced-potion-making]
└─$ xxd -l 100 advanced-potion-making 
00000000: 8950 4211 0d0a 1a0a 0012 1314 4948 4452  .PB.........IHDR
00000010: 0000 0990 0000 04d8 0802 0000 0004 2de7  ..............-.
00000020: 7800 0000 0173 5247 4200 aece 1ce9 0000  x....sRGB.......
00000030: 0004 6741 4d41 0000 b18f 0bfc 6105 0000  ..gAMA......a...
00000040: 0009 7048 5973 0000 1625 0000 1625 0149  ..pHYs...%...%.I
00000050: 5224 f000 0076 3949 4441 5478 5eec fd61  R$...v9IDATx^..a
00000060: 72e3 4c94 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Forensics/advanced-potion-making]
└─$ xxd advanced-potion-making | tail
00007610: 0000 0000 00c8 084b 0000 0000 0000 2023  .......K...... #
00007620: 2c01 0000 0000 0080 8cb0 0400 0000 0000  ,...............
00007630: 0032 c212 0000 0000 0000 c808 4b00 0000  .2..........K...
00007640: 0000 0020 232c 0100 0000 0000 808c b004  ... #,..........
00007650: 0000 0000 0000 32c2 1200 0000 0000 00c8  ......2.........
00007660: 084b 0000 0000 0000 2023 2c01 0000 0000  .K...... #,.....
00007670: 0080 8cb0 0400 0000 0000 0032 c212 0000  ...........2....
00007680: 0000 0000 c808 4b00 0000 0000 0020 727b  ......K...... r{
00007690: fbda 8182 5b04 4dfe 0000 0000 4945 4e44  ....[.M.....IEND
000076a0: ae42 6082  
```

I recognise `IHDR` and `IEND` as chunk types of PNG picture files.  
They normally begin with these 8 bytes `89 50 4E 47 0D 0A 1A 0A`.

### Fix the image file

Let's change the first bytes in the file with a hex editor such as [010 Editor](https://www.sweetscape.com/010editor/) and try to view it.  
Nope, it is still corrupted.

To get some help with analysing the file I used `pngcheck` (which I had to install on my Kali Linux)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Forensics/advanced-potion-making]
└─$ pngcheck -v advanced-potion-making.png
zlib warning:  different version (expected 1.2.13, using 1.2.11)

File: advanced-potion-making.png (30372 bytes)
  chunk IHDR at offset 0x0000c, length 1184532:  EOF while reading data
ERRORS DETECTED in advanced-potion-making.png
```

The `IHDR` seems to have an invalid (too large) length. According to the [PNG-specification](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) the length of a chunk is located just before its type.  

Comparing with a known good PNG-file I saw that these 4 bytes were `00 00 00 0D`. So I changed these bytes as well.  
The beginning of the file now looks like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Forensics/advanced-potion-making]
└─$ xxd -l 32 advanced-potion-making.png 
00000000: 8950 4e47 0d0a 1a0a 0000 000d 4948 4452  .PNG........IHDR
00000010: 0000 0990 0000 04d8 0802 0000 0004 2de7  ..............-.
```

And voila, the file is now viewable. But only a solid red "background" is shown.

### Get the flag

Time to bring out the stego tools. I used [StegSolve](https://github.com/Giotino/stegsolve/releases) which is also available as an [online service](https://georgeom.net/StegOnline/upload).

The flag can be found encoded in `Red plane 0`.

For additional information, please see the references below.

### References

- [010 Editor - Homepage](https://www.sweetscape.com/010editor/)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [PNG - Wikipedia](https://en.wikipedia.org/wiki/PNG)
- [PNG (Portable Network Graphics) Specification, Version 1.2](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
- [pngcheck - GitHub](https://github.com/pnggroup/pngcheck)
- [pngcheck - Homepage](https://www.libpng.org/pub/png/apps/pngcheck.html)
- [pngcheck - Linux manual page](https://manpages.ubuntu.com/manpages/focal/man1/pngcheck.1.html)
- [stegsolve 1.4 - GitHub](https://github.com/Giotino/stegsolve)
- [tail - Linux manual page](https://man7.org/linux/man-pages/man1/tail.1.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
