# CanYouSee

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Forensics, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
How about some hide and seek?

Download this file here.

Hints:
1. How can you view the information about the picture?
2. If something isn't in the expected form, maybe it deserves attention?
```

Challenge link: [https://play.picoctf.org/practice/challenge/408](https://play.picoctf.org/practice/challenge/408)

## Solution

### Unpacking and basic analysis

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/CanYouSee]
└─$ unzip unknown.zip  
Archive:  unknown.zip
  inflating: ukn_reality.jpg         

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/CanYouSee]
└─$ file ukn_reality.jpg 
ukn_reality.jpg: JPEG image data, JFIF standard 1.01, resolution (DPI), density 72x72, segment length 16, baseline, precision 8, 4308x2875, components 3

```

We have a JPEG-file. Use a tool such as `eog` of `feh` to view it on Linux.

Next, we check for [Exif](https://en.wikipedia.org/wiki/Exif) [metdata](https://en.wikipedia.org/wiki/Metadata) with `exiftool`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/CanYouSee]
└─$ exiftool ukn_reality.jpg
ExifTool Version Number         : 12.52
File Name                       : ukn_reality.jpg
Directory                       : .
File Size                       : 2.3 MB
File Modification Date/Time     : 2024:02:15 23:40:14+01:00
File Access Date/Time           : 2024:02:15 23:40:14+01:00
File Inode Change Date/Time     : 2024:02:15 23:40:14+01:00
File Permissions                : -rwxrwxrwx
File Type                       : JPEG
File Type Extension             : jpg
MIME Type                       : image/jpeg
JFIF Version                    : 1.01
Resolution Unit                 : inches
X Resolution                    : 72
Y Resolution                    : 72
XMP Toolkit                     : Image::ExifTool 11.88
Attribution URL                 : cGljb0NURntNRTc0RDQ3QV9ISUREM05fZGVjYTA2ZmJ9Cg==
Image Width                     : 4308
Image Height                    : 2875
Encoding Process                : Baseline DCT, Huffman coding
Bits Per Sample                 : 8
Color Components                : 3
Y Cb Cr Sub Sampling            : YCbCr4:2:0 (2 2)
Image Size                      : 4308x2875
Megapixels                      : 12.4
```

The attribution URL data looks like it is [base64-encoded](https://en.wikipedia.org/wiki/Base64).

### Get the flag

Finally, we use `base64` to decode it and get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/CanYouSee]
└─$ echo "cGljb0NURntNRTc0RDQ3QV9ISUREM05fZGVjYTA2ZmJ9Cg==" | base64 -d
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Exif - Wikipedia](https://en.wikipedia.org/wiki/Exif)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [JPEG - Wikipedia](https://en.wikipedia.org/wiki/JPEG)
- [Metadata - Wikipedia](https://en.wikipedia.org/wiki/Metadata)
