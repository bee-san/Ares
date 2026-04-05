# hideme

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Forensics, steganography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: GEOFFREY NJOGU

Description:
Every file gets a flag.

The SOC analyst saw one image been sent back and forth between two people.  
They decided to investigate and found out that there was more than what meets the eye here.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/350](https://play.picoctf.org/practice/challenge/350)

## Solution

In steganography challenges there are a number of checks that are more or less "standard practice". These include:

1. Checking for metadata with [ExifTool](https://exiftool.org/)
2. Checking for embedded [strings](https://en.wikipedia.org/wiki/Strings_(Unix))
3. Checking for embedded Zip-files with tools such as [Binwalk](https://github.com/ReFirmLabs/binwalk)

### Checking for metadata

Let's start by checking for metadata

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Forensics\hideme>exiftool flag.png
ExifTool Version Number         : 12.44
File Name                       : flag.png
Directory                       : .
File Size                       : 43 kB
File Modification Date/Time     : 2023:07:19 07:00:20+02:00
File Access Date/Time           : 2023:07:19 07:00:25+02:00
File Creation Date/Time         : 2023:07:19 07:00:17+02:00
File Permissions                : -rw-rw-rw-
File Type                       : PNG
File Type Extension             : png
MIME Type                       : image/png
Image Width                     : 512
Image Height                    : 504
Bit Depth                       : 8
Color Type                      : RGB with Alpha
Compression                     : Deflate/Inflate
Filter                          : Adaptive
Interlace                       : Noninterlaced
Warning                         : [minor] Trailer data after PNG IEND chunk
Image Size                      : 512x504
Megapixels                      : 0.258
```

Hhm, we see that there is data embedded after the PNG-file, i.e. after the IEND chunk.

### Checking for embedded strings

Continue with checking for strings. In this case I'm using a [Windows version of strings from Sysinternals](https://learn.microsoft.com/en-us/sysinternals/downloads/strings).

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Forensics\hideme>strings -n 8 flag.png

Strings v2.53 - Search for ANSI and Unicode strings in binary images.
Copyright (C) 1999-2016 Mark Russinovich
Sysinternals - www.sysinternals.com

?bH2u.9n
N~cN.C/p
H@H5ES\h
B \GUO<W
E@d}iX^g
|3.8g3MyO
secret/UT
secret/flag.pngUT
secret/UT
secret/flag.pngUT
```

No flag here either but something that looks like file paths.

### Checking for embedded Zip-files

Now lets check for embedded Zip-files or other interesting files with `binwalk`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Forensics/hideme]
└─$ binwalk flag.png              

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             PNG image, 512 x 504, 8-bit/color RGBA, non-interlaced
41            0x29            Zlib compressed data, compressed
39739         0x9B3B          Zip archive data, at least v1.0 to extract, name: secret/
39804         0x9B7C          Zip archive data, at least v2.0 to extract, compressed size: 2876, uncompressed size: 3029, name: secret/flag.png
42915         0xA7A3          End of Zip archive, footer length: 22

```

Yes, there is an embedded zip-file and the file names matches what we saw earlier in the strings output.

Extract the zip-file with `binwalk -e flag.png`.

The extracted files will be written to a directory with the format of '_target-file-name.extracted'.  
Let's check it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Forensics/hideme]
└─$ ls -la _flag.png.extracted 
total 46
drwxrwxrwx 1 root root     0 Jul 26 01:46 .
drwxrwxrwx 1 root root     0 Jul 26 01:46 ..
-rwxrwxrwx 1 root root     0 Jul 26 01:46 29
-rwxrwxrwx 1 root root 42896 Jul 26 01:46 29.zlib
-rwxrwxrwx 1 root root  3198 Jul 26 01:46 9B3B.zip
drwxrwxrwx 1 root root     0 Mar 15 22:01 secret

┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Forensics/hideme]
└─$ ls -la _flag.png.extracted/secret 
total 3
drwxrwxrwx 1 root root    0 Mar 15 22:01 .
drwxrwxrwx 1 root root    0 Jul 26 01:46 ..
-rwxrwxrwx 1 root root 3029 Mar 15 22:01 flag.png
```

Ah, in the secrets subdirectory there is indeed a flag.png file.

### Get the flag

To view the flag.png file in Kali linux you need a program such as `feh` or `eog`.
These were not installed in the version I was using and needed to be installed (with 'sudo apt install xxx').

Viewing the flag.png file reveals the flag.

For additional information, please see the references below.

## References

- [Binwalk - GitHub](https://github.com/ReFirmLabs/binwalk)
- [Binwalk - Kali Tools](https://www.kali.org/tools/binwalk/)
- [binwalk - Linux manual page](https://manpages.debian.org/testing/binwalk/binwalk.1.en.html)
- [Exif - Wikipedia](https://en.wikipedia.org/wiki/Exif)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
- [strings (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Strings_(Unix))
