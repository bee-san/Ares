# Trivial Flag Transfer Protocol

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
Figure out how they moved the flag.

Hints:
1. What are some other ways to hide data?
```

Challenge link: [https://play.picoctf.org/practice/challenge/103](https://play.picoctf.org/practice/challenge/103)

## Solution

The challenge name suggests that we should focus on the [Trivial File Transfer Protocol (TFTP)](https://en.wikipedia.org/wiki/Trivial_File_Transfer_Protocol).

Open up the PCAP-file in [Wireshark](https://www.wireshark.org/).

Wireshark can extract any files transfered with TFTP for us. In the `File`-menu, select `Export Objects -> TFTP`.  
Then click `Save All`.

### Analyse the transfered files

We now have six files

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ file *          
instructions.txt: ASCII text
picture1.bmp:     PC bitmap, Windows 3.x format, 605 x 454 x 24, image size 824464, resolution 5669 x 5669 px/m, cbSize 824518, bits offset 54
picture2.bmp:     PC bitmap, Windows 3.x format, 4032 x 3024 x 24, image size 36578304, resolution 5669 x 5669 px/m, cbSize 36578358, bits offset 54
picture3.bmp:     PC bitmap, Windows 3.x format, 807 x 605 x 24, image size 1466520, resolution 5669 x 5669 px/m, cbSize 1466574, bits offset 54
plan:             ASCII text
program.deb:      Debian binary package (format 2.0), with control.tar.gz, data compression xz
```

Let's start with the text files

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ cat instructions.txt                       
GSGCQBRFAGRAPELCGBHEGENSSVPFBJRZHFGQVFTHVFRBHESYNTGENAFSRE.SVTHERBHGNJNLGBUVQRGURSYNTNAQVJVYYPURPXONPXSBEGURCYNA

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ cat plan                                           
VHFRQGURCEBTENZNAQUVQVGJVGU-QHRQVYVTRAPR.PURPXBHGGURCUBGBF
```

Hhm, this looks like [ROT13](https://en.wikipedia.org/wiki/ROT13).

### Decode the text files

We can decode them with a prepackaged `rot13` tool from either [hxtools](https://manpages.debian.org/testing/hxtools/hxtools.7.en.html) or [bsdgames](https://wiki.linuxquestions.org/wiki/BSD_games).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ rot13 plan    
IUSEDTHEPROGRAMANDHIDITWITH-DUEDILIGENCE.CHECKOUTTHEPHOTOS

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ rot13 instructions.txt 
TFTPDOESNTENCRYPTOURTRAFFICSOWEMUSTDISGUISEOURFLAGTRANSFER.FIGUREOUTAWAYTOHIDETHEFLAGANDIWILLCHECKBACKFORTHEPLAN
```

It's a bit hard to read without spaces but the files contain:

```text
plan: I USED THE PROGRAM AND HID IT WITH - DUE DILIGENCE. CHECK OUT THE PHOTOS

instructions.txt: TFTP DOESNT ENCRYPT OUR TRAFFIC SO WE MUST DISGUISE OUR FLAG TRANSFER.
   FIGURE OUT AWAY TO HIDE THE FLAG AND I WILL CHECK BACK FOR THE PLAN
```

### Unpack the program.deb file

Analysing the `program.deb` file I saw that it contains a `control.tar.gz` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ head -n 4 program.deb
!<arch>
debian-binary   1413331375  0     0     100644  4         `
2.0
control.tar.gz  1413331375  0     0     100644  1250      `
```

We can unpack it with `ar` and then `gunzip` and untar it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ ar x program.deb 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ gunzip control.tar.gz         
gzip: control.tar: Value too large for defined data type

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ tar xvf control.tar 
./
./md5sums
./control
```

Check out the `md5sums` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ cat md5sums 
71bdab1263ab4b8d28f34afa5f0ab121  usr/bin/steghide
11db80c2a5dbb9c6107853b08aeacc49  usr/share/doc/steghide/ABOUT-NLS.gz
57deb17212483b49f89587180d4d67d4  usr/share/doc/steghide/BUGS
72c7831222483f5c6d96ac2a8ca7ad48  usr/share/doc/steghide/CREDITS
adbb29f44a5e5eefda3c3d756cc15ab1  usr/share/doc/steghide/HISTORY
fe7cac39a1a1ef0975d24dfcf02f09b7  usr/share/doc/steghide/LEAME.gz
85587b9213ca2301eb450aad574d5f87  usr/share/doc/steghide/README.gz
a9e03fa8166b8fa918c81db1855b68d1  usr/share/doc/steghide/TODO
09d7710e276a06c4a3f3bc81b3b86a41  usr/share/doc/steghide/changelog.Debian.amd64.gz
e454b20fdc2208f8170e28b90b6d43f7  usr/share/doc/steghide/changelog.Debian.gz
1a2e10366a3a55d7a4cb5fc3c87a6bf7  usr/share/doc/steghide/changelog.gz
df8c0ea893b3f6f64a917824c6c9d224  usr/share/doc/steghide/copyright
fc53645374c583f11f628331be710d9a  usr/share/locale/de/LC_MESSAGES/steghide.mo
b8ceabc96f9bffd9157103e1a86be33f  usr/share/locale/es/LC_MESSAGES/steghide.mo
87ee9a19bb49b217dad67b5a889bb1d1  usr/share/locale/fr/LC_MESSAGES/steghide.mo
dbc3a8e974ccf7e91da81aca4a5c1605  usr/share/locale/ro/LC_MESSAGES/steghide.mo
921a5afd279097e4ed359ce3767068f5  usr/share/man/man1/steghide.1.gz
```

And the `control` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ file control
control: ASCII text

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ cat control
Package: steghide
Source: steghide (0.5.1-9.1)
Version: 0.5.1-9.1+b1
Architecture: amd64
Maintainer: Ola Lundqvist <opal@debian.org>
Installed-Size: 426
Depends: libc6 (>= 2.2.5), libgcc1 (>= 1:4.1.1), libjpeg62-turbo (>= 1:1.3.1), libmcrypt4, libmhash2, libstdc++6 (>= 4.9), zlib1g (>= 1:1.1.4)
Section: misc
Priority: optional
Description: A steganography hiding tool
 Steghide is steganography program which hides bits of a data file
 in some of the least significant bits of another file in such a way
 that the existence of the data file is not visible and cannot be proven.
 .
 Steghide is designed to be portable and configurable and features hiding
 data in bmp, wav and au files, blowfish encryption, MD5 hashing of
 passphrases to blowfish keys, and pseudo-random distribution of hidden bits
 in the container data.
```

Ok, so `steghide` have been used to hide the flag in one of the pictures.

### Get the flag

First I tried to extract without a password (that is an empty password)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture1.bmp
Enter passphrase: 
steghide: could not extract any data with that passphrase!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture2.bmp
Enter passphrase: 
steghide: could not extract any data with that passphrase!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture3.bmp
Enter passphrase: 
steghide: could not extract any data with that passphrase!
```

But that didn't work.

So I went back to the text files looking for clues about the password.  
From the `plan` file

```text
IUSEDTHEPROGRAMANDHIDITWITH-DUEDILIGENCE.CHECKOUTTHEPHOTOS
```

So I tried the password `DUEDILIGENCE` and that worked

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture1.bmp
Enter passphrase: 
steghide: could not extract any data with that passphrase!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture2.bmp
Enter passphrase: 
steghide: could not extract any data with that passphrase!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ steghide extract -sf picture3.bmp
Enter passphrase: 
wrote extracted data to "flag.txt".
   
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Trivial_Flag_Transfer_Protocol]
└─$ cat flag.txt 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ar - Linux manual page](https://man7.org/linux/man-pages/man1/ar.1.html)
- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gunzip - Linux manual page](https://linux.die.net/man/1/gunzip)
- [head - Linux manual page](https://man7.org/linux/man-pages/man1/head.1.html)
- [MD5 - Wikipedia](https://en.wikipedia.org/wiki/MD5)
- [ROT13 - Wikipedia](https://en.wikipedia.org/wiki/ROT13)
- [steghide - Homepage](https://steghide.sourceforge.net/)
- [steghide - Kali Tools](https://www.kali.org/tools/steghide/)
- [tar - Linux manual page](https://man7.org/linux/man-pages/man1/tar.1.html)
- [Trivial File Transfer Protocol - Wikipedia](https://en.wikipedia.org/wiki/Trivial_File_Transfer_Protocol)
- [Wireshark - Homepage](https://www.wireshark.org/)
