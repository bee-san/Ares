# Invisible WORDs

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2023, Forensics, steganography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Do you recognize this cyberpunk baddie? We don't either. AI art generators are all the rage nowadays, 
which makes it hard to get a reliable known cover image. But we know you'll figure it out. 

The suspect is believed to be trafficking in classics. That probably won't help crack the stego, 
but we hope it will give motivation to bring this criminal to justice!
Download the image here

Hints:
1. Something doesn't quite add up with this image...
2. How's the image quality?
```

Challenge link: [https://play.picoctf.org/practice/challenge/354](https://play.picoctf.org/practice/challenge/354)

## Solution

### Basic analysis of the image

Let's start with some basic analysis of the image

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ file output.bmp 
output.bmp: PC bitmap, Windows 98/2000 and newer format, 960 x 540 x 32, cbSize 2073738, bits offset 138

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ exiftool output.bmp                                             
ExifTool Version Number         : 12.52
File Name                       : output.bmp
Directory                       : .
File Size                       : 2.1 MB
File Modification Date/Time     : 2023:11:05 04:58:45-05:00
File Access Date/Time           : 2023:11:05 04:58:50-05:00
File Inode Change Date/Time     : 2023:11:05 04:58:45-05:00
File Permissions                : -rwxrwxrwx
File Type                       : BMP
File Type Extension             : bmp
MIME Type                       : image/bmp
BMP Version                     : Windows V5
Image Width                     : 960
Image Height                    : 540
Planes                          : 1
Bit Depth                       : 32
Compression                     : Bitfields
Image Length                    : 2073600
Pixels Per Meter X              : 11811
Pixels Per Meter Y              : 11811
Num Colors                      : Use BitDepth
Num Important Colors            : All
Red Mask                        : 0x00007c00
Green Mask                      : 0x000003e0
Blue Mask                       : 0x0000001f
Alpha Mask                      : 0x00000000
Color Space                     : sRGB
Rendering Intent                : Proof (LCS_GM_GRAPHICS)
Image Size                      : 960x540
Megapixels                      : 0.518
```

Nothing that stands out.

Let's try a hex view with `xxd`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ xxd -l 0x200 output.bmp       
00000000: 424d 8aa4 1f00 0000 0000 8a00 0000 7c00  BM............|.
00000010: 0000 c003 0000 1c02 0000 0100 2000 0300  ............ ...
00000020: 0000 00a4 1f00 232e 0000 232e 0000 0000  ......#...#.....
00000030: 0000 0000 0000 007c 0000 e003 0000 1f00  .......|........
00000040: 0000 0000 0000 4247 5273 0000 0000 0000  ......BGRs......
00000050: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000060: 0000 0000 0000 0000 0000 0000 0000 0000  ................
00000070: 0000 0000 0000 0000 0000 0200 0000 0000  ................
00000080: 0000 0000 0000 0000 0000 3867 504b 9552  ..........8gPK.R
00000090: 0304 c618 1400 ce3d 0000 104a 0800 6f56  .......=...J..oV
000000a0: 6f13 1016 7056 723e 229b 0e3a 64e7 d55a  o...pVr>"..:d..Z
000000b0: b095 ab39 0200 2d4e 82d8 693d 0600 ef41  ...9..-N..i=...A
000000c0: 1c00 4e49 1c00 314a 5a6e af1d 4a68 b452  ..NI..1JZn..Jh.R
000000d0: 626d 1146 746c 0909 626e b44a 4e30 ac35  bm.Ftl..bn.JN0.5
000000e0: 5a57 4f3e 6c75 4a29 4c58 a318 526c 5642  ZWO>luJ)LX..RlVB
000000f0: 6333 d65a 5175 3146 6448 0821 6830 ad1d  c3.ZQu1FdH.!h0..
00000100: 5554 b301 0900 6b45 0391 904d 7e12 f20d  UT....kE...M~...
00000110: 6491 0c31 7e12 143e 6475 cd41 780b 6124  d..1~..>du.Ax.a$
00000120: 0001 4f25 0400 840c 0000 4f32 0004 292d  ..O%......O2..)-
00000130: 0000 1042 0000 f14d 8cfd b149 cd8e b746  ...B...M...I...F
00000140: 2469 b45a 7225 4e3a 0aee ef4d 13c8 0424  $i.Zr%N:...M...$
00000150: 77b0 fc7b c06c b352 c886 7a63 a5a3 b556  w..{.l.R..zc...V
00000160: ab8a 534a bc24 d56a d317 313e 81a8 b35a  ..SJ.$.j..1>...Z
00000170: df74 b035 7665 f33d 55a1 f666 32d9 ae2d  .t.5ve.=U..f2..-
00000180: 09a2 304a 7151 0221 5033 f041 5373 9732  ..0JqQ.!P3.ASs.2
00000190: d374 ad3d 3555 f85a 2bfd f74a 714b f552  .t.=5U.Z+..JqK.R
000001a0: abc1 b44a 00f5 4408 10bd 3046 b80d 9439  ...J..D...0F...9
000001b0: 746f 534a 7bd1 7326 ebd9 ce14 cd62 7031  toSJ{.s&.....bp1
000001c0: 167c 262d 937a 6b39 8179 b545 8591 3562  .|&-.zk9.y.E..5b
000001d0: 738e 923a 887c d12d 9f79 9125 44b1 5546  s..:.|.-.y.%D.UF
000001e0: efbd ce35 4d32 b45a 32c2 8c35 dd4c 2d19  ...5M2.Z2..5.L-.
000001f0: f5fb 0d25 919f e828 2347 cd35 8efc 5256  ...%...(#G.5..RV
```

After the [BMP file header](https://en.wikipedia.org/wiki/BMP_file_format) and the Windows bitmap header there is a `PK`
that suggests a [ZIP file](https://en.wikipedia.org/wiki/ZIP_(file_format)) header.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ xxd -s 0x8c -l 0x10 output.bmp
0000008c: 504b 9552 0304 c618 1400 ce3d 0000 104a  PK.R.......=...J
```

But there are additional bytes in the middle of the `504b0304` or `PK\3\4` ZIP-header.

### Write a python extraction script - part 1

Let's write a small python script called `extract_zip.py` to try to extract the ZIP

```python
#!/usr/bin/python

with open("output.bmp", 'rb') as bmp:
    zip = open("extracted.zip", "wb")
    bmp.seek(0x8c)
    while data := bmp.read(4):
        zip.write(data[0:2])
zip.close()
```

Then we make the script executable and run it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ chmod +x extract_zip.py 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ ./extract_zip.py          
                 
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ file extracted.zip 
extracted.zip: Zip archive data, at least v2.0 to extract, compression method=deflate
```

This looks promising. Let's try to unzip it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ unzip extracted.zip          
Archive:  extracted.zip
  End-of-central-directory signature not found.  Either this file is not
  a zipfile, or it constitutes one disk of a multi-part archive.  In the
  latter case the central directory and zipfile comment will be found on
  the last disk(s) of this archive.
note:  extracted.zip may be a plain executable, not an archive
unzip:  cannot find zipfile directory in one of extracted.zip or
        extracted.zip.zip, and cannot find extracted.zip.ZIP, period.
```

Hhm, something is wrong. Let's investigate the resulting file.

### Write a python extraction script - part 2

Looking at the result, we can see that additional data is included after the EOCD (End of central directory) at the end of the zip-file.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ xxd -l 0x100 -s 0x295c0 extracted.zip
000295c0: a48f dd2f 2ef1 459b e841 fa90 ec48 fee7  .../..E..A...H..
000295d0: 78a4 5f4d dcd7 f161 a629 7309 f296 d171  x._M...a.)s....q
000295e0: d0b3 5c67 6508 b183 f97c eb39 3eda 49a8  ..\ge....|.9>.I.
000295f0: 1499 2b97 5f9f 7754 5dd3 a7c1 f79e 7753  ..+._.wT].....wS
00029600: 3fc7 324a ff7f 504b 0102 1e03 1400 0000  ?.2J..PK........
00029610: 0800 6f13 7056 229b 64e7 b095 0200 82d8  ..o.pV".d.......
00029620: 0600 1c00 1800 0000 0000 0100 0000 a481  ................
00029630: 0000 0000 5a6e 4a68 626d 746c 626e 4e30  ....ZnJhbmtlbnN0
00029640: 5a57 6c75 4c58 526c 6333 5175 6448 6830  ZWluLXRlc3QudHh0
00029650: 5554 0500 0391 7e12 6475 780b 0001 0400  UT....~.dux.....
00029660: 0000 0004 0000 0000 504b 0506 0000 0000  ........PK......
00029670: 0100 0100 6200 0000 0696 0200 0000 ff00  ....b...........
00029680: ff00 ff00 ff00 ff00 ff00 ff00 ff00 ff00  ................
00029690: ff00 ff00 ff00 ff00 ff00 ff00 ff00 ff00  ................
000296a0: ff00 ff00 ff00 ff00 ff00 ff00 ff00 ff00  ................
000296b0: ff00 ff00 ff00 ff00 ff00 ff00 ff00 ff00  ................
```

Let's modify the script to quit extracting when the '\xff\x00' data begins

```python
#!/usr/bin/python

with open("output.bmp", 'rb') as bmp:
    zip = open("extracted2.zip", "wb")
    bmp.seek(0x8c)
    while data := bmp.read(4):
        if data[0:2] == b'\xff\x00':
            break
        else:
            zip.write(data[0:2])
zip.close()
```

Let's try again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ ./extract_zip2.py

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ unzip extracted2.zip 
Archive:  extracted2.zip
  inflating: ZnJhbmtlbnN0ZWluLXRlc3QudHh0  
```

Success!

Let's check the extracted file which name looks [base64](https://en.wikipedia.org/wiki/Base64) encoded

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ file ZnJhbmtlbnN0ZWluLXRlc3QudHh0 
ZnJhbmtlbnN0ZWluLXRlc3QudHh0: Unicode text, UTF-8 (with BOM) text, with CRLF, LF line terminators
    
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ echo ZnJhbmtlbnN0ZWluLXRlc3QudHh0 | base64 -d
frankenstein-test.txt  

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ head ZnJhbmtlbnN0ZWluLXRlc3QudHh0    
The Project Gutenberg eBook of Frankenstein, by Mary Wollstonecraft Shelley

This eBook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it under the terms
of the Project Gutenberg License included with this eBook or online at
www.gutenberg.org. If you are not located in the United States, you
will have to check the laws of the country where you are located before
using this eBook.
```

### Get the flag

Finally, let's grep for the flag with a [RegEx](https://en.wikipedia.org/wiki/Regular_expression) for the flag format

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Forensics/Invisible_WORDs]
└─$ grep -oE 'picoCTF{.*}' ZnJhbmtlbnN0ZWluLXRlc3QudHh0 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [BMP file format - Wikipedia](https://en.wikipedia.org/wiki/BMP_file_format)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Regular expression - Wikipedia](https://en.wikipedia.org/wiki/Regular_expression)
- [Steganography - Wikipedia](https://en.wikipedia.org/wiki/Steganography)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
- [ZIP (file format) - Wikipedia](https://en.wikipedia.org/wiki/ZIP_(file_format))
