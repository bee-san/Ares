# information

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SUSIE

Description:
Files can always be changed in a secret way. 
Can you find the flag? cat.jpg
 
Hints:
1. Look at the details of the file
2. Make sure to submit the flag as picoCTF{XXXXX}
```

Challenge link: [https://play.picoctf.org/practice/challenge/186](https://play.picoctf.org/practice/challenge/186)

## Solution

In steganography oriented forensics challenges there are a number of checks that are more or less "standard practice".  
These include:

1. Checking for metadata with [ExifTool](https://exiftool.org/)
2. Checking for embedded [strings](https://en.wikipedia.org/wiki/String_(computer_science))
3. Checking for embedded Zip-files with tools such as [Binwalk](https://github.com/ReFirmLabs/binwalk)

Let's start checking them one by one until we find the flag.

First, check for metadata with `exiftool`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/information]
└─$ exiftool cat.jpg 
ExifTool Version Number         : 12.52
File Name                       : cat.jpg
Directory                       : .
File Size                       : 878 kB
File Modification Date/Time     : 2022:04:17 04:35:09-04:00
File Access Date/Time           : 2023:08:04 13:52:19-04:00
File Inode Change Date/Time     : 2022:04:17 04:35:09-04:00
File Permissions                : -rwxrwxrwx
File Type                       : JPEG
File Type Extension             : jpg
MIME Type                       : image/jpeg
JFIF Version                    : 1.02
Resolution Unit                 : None
X Resolution                    : 1
Y Resolution                    : 1
Current IPTC Digest             : 7a78f3d9cfb1ce42ab5a3aa30573d617
Copyright Notice                : PicoCTF
Application Record Version      : 4
XMP Toolkit                     : Image::ExifTool 10.80
License                         : cGljb0NURnt0aGVfbTN0YWRhdGFfMXNfbW9kaWZpZWR9
Rights                          : PicoCTF
Image Width                     : 2560
Image Height                    : 1598
Encoding Process                : Baseline DCT, Huffman coding
Bits Per Sample                 : 8
Color Components                : 3
Y Cb Cr Sub Sampling            : YCbCr4:2:0 (2 2)
Image Size                      : 2560x1598
Megapixels                      : 4.1
```

Hhm, the licence information (`cGljb0NURnt0aGVfbTN0YWRhdGFfMXNfbW9kaWZpZWR9`) looks suspicious.  
It is long, contains only letters and numbers and could be [Base64 encoded data](https://en.wikipedia.org/wiki/Base64).

### Decode the flag

To decode the flag we could use an online site such as [CyberChef](https://gchq.github.io/CyberChef/) with the 'From Base64' recipe. Enter 'base64' in the `Operations` search bar, then drag and drop it to the `Recipe`. Copy the license data to the `Input` pane and press `BAKE`.

Alternatively, you can use the `base64` tool like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/information]
└─$ echo "cGljb0NURnt0aGVfbTN0YWRhdGFfMXNfbW9kaWZpZWR9" | base64 -d                           
picoCTF{<REDACTED>}     
```

And there we have the flag.

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [Binwalk - GitHub](https://github.com/ReFirmLabs/binwalk)
- [Binwalk - Kali Tools](https://www.kali.org/tools/binwalk/)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
