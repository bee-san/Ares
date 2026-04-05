# tunn3l v1s10n

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY

Description:

We found this file. Recover the flag.
 
Hints:
1. Weird that it won't display right...
```

Challenge link: [https://play.picoctf.org/practice/challenge/112](https://play.picoctf.org/practice/challenge/112)

## Solution

### Analyse the file

Let's start with checking the file type with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/tunn3l_v1s10n]
└─$ file tunn3l_v1s10n  
tunn3l_v1s10n: data
```

Hhm, not much help there. Lets check the first bytes with `xxd`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/tunn3l_v1s10n]
└─$ xxd -g 1 -l 48 tunn3l_v1s10n        
00000000: 42 4d 8e 26 2c 00 00 00 00 00 ba d0 00 00 ba d0  BM.&,...........
00000010: 00 00 6e 04 00 00 32 01 00 00 01 00 18 00 00 00  ..n...2.........
00000020: 00 00 58 26 2c 00 25 16 00 00 25 16 00 00 00 00  ..X&,.%...%.....
```

After some research (a.k.a. Googling) I find that the magic bytes `0x42 0x4D` is for a [BMP image file](https://en.wikipedia.org/wiki/BMP_file_format).

Now, we need to figure out what fields in the BMP header is corrupt and fix them.

### Fixing the BMP header

Fixing the header is made considerably easier with this help:

- A [specification of the file format](https://en.wikipedia.org/wiki/BMP_file_format)
- At least one known good file of the same format
- A tool which can parse the headers for you such as the [010 Editor](https://www.sweetscape.com/010editor/) with its [binary templates](https://www.sweetscape.com/010editor/templates.html)

Reading from the beginning of the header the following values seems corrupt/wrong:

**The offset** on offset 0xA-0xD, should be `36 00 00 00` instead of `BA D0 00 00`.
Without additional headers the offset should be 0x36 or decimal 54.

**The header size** on offset 0xE-0x11, should be `28 00 00 00` instead of `BA D0 00 00`.
The is standard according to the specification.

I didn't see it first but the fields actually says `BAD`...

After the changes, the headers looks like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/tunn3l_v1s10n]
└─$ xxd -g 1 -l 48 tunn3l_v1s10n.bmp 
00000000: 42 4d 8e 26 2c 00 00 00 00 00 36 00 00 00 28 00  BM.&,.....6...(.
00000010: 00 00 6e 04 00 00 32 01 00 00 01 00 18 00 00 00  ..n...2.........
00000020: 00 00 58 26 2c 00 25 16 00 00 25 16 00 00 00 00  ..X&,.%...%.....
```

And now it is recognized as a BMP image by `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/tunn3l_v1s10n]
└─$ file tunn3l_v1s10n.bmp 
tunn3l_v1s10n.bmp: PC bitmap, Windows 3.x format, 1134 x 306 x 24, image size 2893400, resolution 5669 x 5669 px/m, cbSize 2893454, bits offset 54
```

The image can now be viewed but only contains a fake flag (`notaflag{sorry}`).

### Get the flag

Let's try to increase the height of the image by changing offset 0x16 - 0x19 to `52 03 00 00` instead of `32 01 00 00`.
This increases the height to decimal 850 from decimal 306.

Viewing the modified image displays a real flag at the top of the image.

For additional information, please see the references below.

## References

- [010 Editor - Homepage](https://www.sweetscape.com/010editor/)
- [BMP file format - Wikipedia](https://en.wikipedia.org/wiki/BMP_file_format)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
