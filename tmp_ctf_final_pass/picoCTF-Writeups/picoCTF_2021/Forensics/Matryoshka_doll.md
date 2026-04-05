# Matryoshka doll

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SUSIE/PANDU

Description:
Matryoshka dolls are a set of wooden dolls of decreasing size placed one inside another. 
What's the final one? 

Image: this

Hints:
1. Wait, you can hide files inside files? But how do you find them?
2. Make sure to submit the flag as picoCTF{XXXXX}
```

Challenge link: [https://play.picoctf.org/practice/challenge/129](https://play.picoctf.org/practice/challenge/129)

## Solution

Hhm, hiding file within files, sounds like tools such as [Binwalk](https://github.com/ReFirmLabs/binwalk) will be useful...

But first let's check what type of file it is with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll]
└─$ file dolls.jpg            
dolls.jpg: PNG image data, 594 x 1104, 8-bit/color RGBA, non-interlaced
```

Let's see if any files are embedded with `binwalk`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll]
└─$ binwalk -e dolls.jpg 

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             PNG image, 594 x 1104, 8-bit/color RGBA, non-interlaced
3226          0xC9A           TIFF image data, big-endian, offset of first image directory: 8
272492        0x4286C         Zip archive data, at least v2.0 to extract, compressed size: 378956, uncompressed size: 383938, name: base_images/2_c.jpg
651614        0x9F15E         End of Zip archive, footer length: 22
```

Aha, an embedded Zip-file.

The files are unpacked in a separate directory

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll]
└─$ cd _dolls.jpg.extracted     

┌──(kali㉿kali)-[/mnt/…/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted]
└─$ ls -la
total 371
drwxrwxrwx 1 root root      0 Aug 13 09:24 .
drwxrwxrwx 1 root root      0 Aug 13 09:24 ..
-rwxrwxrwx 1 root root 379144 Aug 13 09:24 4286C.zip
drwxrwxrwx 1 root root      0 Aug 13 09:24 base_images

┌──(kali㉿kali)-[/mnt/…/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted]
└─$ cd base_images         

┌──(kali㉿kali)-[/mnt/…/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images]
└─$ ls -la
total 375
drwxrwxrwx 1 root root      0 Aug 13 09:24 .
drwxrwxrwx 1 root root      0 Aug 13 09:24 ..
-rwxrwxrwx 1 root root 383938 Mar 15  2021 2_c.jpg
```

Another zip-file... Ok, let's try the same thing again but with an added `-M` to recursively scan extracted files

```bash
┌──(kali㉿kali)-[/mnt/…/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images]
└─$ binwalk -e -M 2_c.jpg    

Scan Time:     2023-08-13 09:32:47
Target File:   /mnt/hgfs/CTFs/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images/2_c.jpg
MD5 Checksum:  f407f8aea8d5f8ffaf8cfd567f063cdd
Signatures:    411

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             PNG image, 526 x 1106, 8-bit/color RGBA, non-interlaced
3226          0xC9A           TIFF image data, big-endian, offset of first image directory: 8
187707        0x2DD3B         Zip archive data, at least v2.0 to extract, compressed size: 196043, uncompressed size: 201445, name: base_images/3_c.jpg
383805        0x5DB3D         End of Zip archive, footer length: 22
383916        0x5DBAC         End of Zip archive, footer length: 22


Scan Time:     2023-08-13 09:32:48
Target File:   /mnt/hgfs/CTFs/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images/_2_c.jpg.extracted/base_images/3_c.jpg
MD5 Checksum:  783ef3f85e2f73120323623e3acd8547
Signatures:    411

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             PNG image, 428 x 1104, 8-bit/color RGBA, non-interlaced
3226          0xC9A           TIFF image data, big-endian, offset of first image directory: 8
123606        0x1E2D6         Zip archive data, at least v2.0 to extract, compressed size: 77651, uncompressed size: 79808, name: base_images/4_c.jpg
201423        0x312CF         End of Zip archive, footer length: 22


Scan Time:     2023-08-13 09:32:48
Target File:   /mnt/hgfs/CTFs/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images/_2_c.jpg.extracted/base_images/_3_c.jpg.extracted/base_images/4_c.jpg
MD5 Checksum:  0d871e3f9784f4bf15fb00a790625ac9
Signatures:    411

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
0             0x0             PNG image, 320 x 768, 8-bit/color RGBA, non-interlaced
3226          0xC9A           TIFF image data, big-endian, offset of first image directory: 8
79578         0x136DA         Zip archive data, at least v2.0 to extract, compressed size: 64, uncompressed size: 81, name: flag.txt
79786         0x137AA         End of Zip archive, footer length: 22


Scan Time:     2023-08-13 09:32:48
Target File:   /mnt/hgfs/CTFs/picoCTF/picoCTF_2021/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images/_2_c.jpg.extracted/base_images/_3_c.jpg.extracted/base_images/_4_c.jpg.extracted/flag.txt
MD5 Checksum:  481f410040a86d20306fd0e7b362582b
Signatures:    411

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------

```

And there at the end is a `flag.txt` file. Hopefully that is the flag.

Let's verify

```bash
┌──(kali㉿kali)-[/mnt/…/Forensics/Matryoshka_doll/_dolls.jpg.extracted/base_images]
└─$ cat _2_c.jpg.extracted/base_images/_3_c.jpg.extracted/base_images/_4_c.jpg.extracted/flag.txt
picoCTF{<REDACTED>}
```

Yes, that's the flag.

For additional information, please see the references below.

## References

- [Binwalk - GitHub](https://github.com/ReFirmLabs/binwalk)
- [Binwalk - Kali Tools](https://www.kali.org/tools/binwalk/)
- [binwalk - Linux manual page](https://manpages.debian.org/testing/binwalk/binwalk.1.en.html)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
