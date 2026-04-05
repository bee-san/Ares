# HideToSee

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SUNDAY JACOB NWANYIM

Description:
How about some hide and seek heh?

Look at this image here.

Hints:
1. Download the image and try to extract it.
```

Challenge link: [https://play.picoctf.org/practice/challenge/351](https://play.picoctf.org/practice/challenge/351)

## Solution

The challange name (Hide something) suggests that there is steganography involved.  
Also the name of the given file (atbash.jpg) suggests that the [Atbash substitution cipher](https://en.wikipedia.org/wiki/Atbash) is used to encode the flag.

In steganography challenges there are a number of checks that are more or less "standard practice". These include:

1. Checking for metadata with [ExifTool](https://exiftool.org/)
2. Checking for embedded [strings](https://en.wikipedia.org/wiki/Strings_(Unix))
3. Checking for "forensically" embedded Zip-files with tools such as [Binwalk](https://github.com/ReFirmLabs/binwalk)
4. Checking for "steganography" hidden files with tools such as [steghide](https://steghide.sourceforge.net/)

Let's start by running through these standard checks one-by-one until we find the flag.

### Checking for metadata

Checking for metadata with `exiftool`

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Cryptography\HideToSee>exiftool atbash.jpg
ExifTool Version Number         : 12.44
File Name                       : atbash.jpg
Directory                       : .
File Size                       : 51 kB
File Modification Date/Time     : 2023:07:19 07:38:06+02:00
File Access Date/Time           : 2023:07:19 07:38:33+02:00
File Creation Date/Time         : 2023:07:19 07:38:03+02:00
File Permissions                : -rw-rw-rw-
File Type                       : JPEG
File Type Extension             : jpg
MIME Type                       : image/jpeg
JFIF Version                    : 1.01
Resolution Unit                 : None
X Resolution                    : 1
Y Resolution                    : 1
Image Width                     : 465
Image Height                    : 455
Encoding Process                : Baseline DCT, Huffman coding
Bits Per Sample                 : 8
Color Components                : 3
Y Cb Cr Sub Sampling            : YCbCr4:2:0 (2 2)
Image Size                      : 465x455
Megapixels                      : 0.212
```

Nope, nothing of interest.

### Checking for embedded strings

Continue with checking for strings. In this case I'm using a [Windows version of strings from Sysinternals](https://learn.microsoft.com/en-us/sysinternals/downloads/strings).

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Cryptography\HideToSee>strings -n 8 atbash.jpg

Strings v2.53 - Search for ANSI and Unicode strings in binary images.
Copyright (C) 1999-2016 Mark Russinovich
Sysinternals - www.sysinternals.com

 , #&')*)
-0-(0%()(
((((((((((((((((((((((((((((((((((((((((((((((((((
%&'()*456789:CDEFGHIJSTUVWXYZcdefghijstuvwxyz
&'()*56789:CDEFGHIJSTUVWXYZcdefghijstuvwxyz
K+ #9=O_j
&~oc_$xb
o-X43(e#
Tpr:sYz_
me^sZ"$udv
4*Kzq_.x
TW2.M V
WzwdI^V<
<Mkmsw9U
h(&2[x/K
```

Nope, nothing of interest here either.

### Checking for embedded Zip-files

Now let's check for embedded Zip-files or other interesting files with `binwalk`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Cryptography/HideToSee]
└─$ binwalk atbash.jpg 

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             JPEG image data, JFIF standard 1.01
```

Nope, fail again.

### Checking for hidden files

The previous check looked for "forensically" embedded/hidden files. This check looks for "steganography" hidden/embedded files with tools such as `steghide`.

Let's use the 'extract' command in `steghide` and specifying the stegofile with -sf.

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Cryptography\HideToSee>steghide extract -sf atbash.jpg
Enter passphrase:
wrote extracted data to "encrypted.txt".
```

Since we don't have any password just press enter when prompted.
Yes, there is indeed a hidden file call `encrypted.txt`.

Let's view it

```bash
Z:\CTFs\picoCTF\picoCTF_2023\Cryptography\HideToSee>type encrypted.txt
krxlXGU{zgyzhs_xizxp_92533667}
```

Ah, a flag most likely scrambled with the Atbash cipher.

### Get the flag

To view the flag in plaintext you can use one of these sites

- The [Atbash cipher recipe from CyberChef](https://cyberchef.org/#recipe=Atbash_Cipher())
- The [Atbash cipher function at Crypto Corner](https://crypto.interactive-maths.com/atbash-cipher.html)

For additional information, please see the references below.

## References

- [Atbash - Wikipedia](https://en.wikipedia.org/wiki/Atbash)
- [Binwalk - GitHub](https://github.com/ReFirmLabs/binwalk)
- [Binwalk - Kali Tools](https://www.kali.org/tools/binwalk/)
- [Exif - Wikipedia](https://en.wikipedia.org/wiki/Exif)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [Steganography - Wikipedia](https://en.wikipedia.org/wiki/Steganography)
- [steghide - Homepage](https://steghide.sourceforge.net/)
- [steghide - Kali Tools](https://www.kali.org/tools/steghide/)
- [strings (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Strings_(Unix))
