# Secret of the Polyglot

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Forensics, file_format, polyglot
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL

Description:
The Network Operations Center (NOC) of your local institution picked up a suspicious file, 
they're getting conflicting information on what type of file it is. They've brought you in 
as an external expert to examine the file. Can you extract all the information from this 
strange file?

Download the suspicious file here.

Hints:
1. This problem can be solved by just opening the file in different ways
```

Challenge link: [https://play.picoctf.org/practice/challenge/423](https://play.picoctf.org/practice/challenge/423)

## Solution

### Basic file analysis

We start with some basic analysis of the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Secret_of_the_Polyglot]
└─$ file flag2of2-final.pdf 
flag2of2-final.pdf: PNG image data, 50 x 50, 8-bit/color RGBA, non-interlaced
```

Even though the file extension is pdf, `file` identifies the file as a [PNG-file](https://en.wikipedia.org/wiki/PNG).

Looking at the beginning of the file with `xxd` we can see that the file is indeed a PNG-file.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Secret_of_the_Polyglot]
└─$ xxd flag2of2-final.pdf| head -n 25 
00000000: 8950 4e47 0d0a 1a0a 0000 000d 4948 4452  .PNG........IHDR
00000010: 0000 0032 0000 0032 0806 0000 001e 3f88  ...2...2......?.
00000020: b100 0001 8569 4343 5049 4343 2070 726f  .....iCCPICC pro
00000030: 6669 6c65 0000 2891 7d91 3d48 c340 1cc5  file..(.}.=H.@..
00000040: 5f53 a5a2 1511 3b88 0866 a80e 6241 54c4  _S....;..f..bAT.
00000050: 51ab 5084 0aa1 5668 d5c1 e4d2 2f68 d290  Q.P...Vh..../h..
00000060: a4b8 380a ae05 073f 16ab 0e2e ceba 3ab8  ..8....?......:.
00000070: 0a82 e007 88ab 8b93 a28b 94f8 bfa4 d022  ..............."
00000080: d683 e37e bcbb f7b8 7b07 08d5 22d3 acb6  ...~....{..."...
00000090: 7140 d36d 3311 8b8a a9f4 aa18 7845 1704  q@.m3.......xE..
000000a0: f462 1443 32b3 8c39 498a a3e5 f8ba 878f  .b.C2..9I.......
000000b0: af77 119e d5fa dc9f a35b cd58 0cf0 89c4  .w.......[.X....
000000c0: b3cc 306d e20d e2e9 4ddb e0bc 4f1c 6279  ..0m....M...O.by
000000d0: 5925 3e27 1e33 e982 c48f 5c57 3c7e e39c  Y%>'.3....\W<~..
000000e0: 7359 e099 2133 9998 270e 118b b926 569a  sY..!3..'....&V.
000000f0: 98e5 4d8d 788a 38ac 6a3a e50b 298f 55ce  ..M.x.8.j:..).U.
00000100: 5b9c b562 99d5 efc9 5f18 cce8 2bcb 5ca7  [..b...._...+.\.
00000110: 3988 1816 b104 0922 1494 5140 1136 22b4  9......"..Q@.6".
00000120: eaa4 5848 d07e b485 7fc0 f54b e452 c855  ..XH.~.....K.R.U
00000130: 0023 c702 4ad0 20bb 7ef0 3ff8 ddad 959d  .#..J. .~.?.....
00000140: 9cf0 9282 51a0 fdc5 713e 8681 c02e 50ab  ....Q...q>....P.
00000150: 38ce f7b1 e3d4 4e00 ff33 70a5 37fc a52a  8.....N..3p.7..*
00000160: 30f3 497a a5a1 858f 809e 6de0 e2ba a129  0.Iz......m....)
00000170: 7bc0 e50e d0ff 64c8 a6ec 4a7e 9a42 360b  {.....d...J~.B6.
00000180: bc9f d137 a581 be5b a073 cdeb adbe 8fd3  ...7...[.s......
```

If we open the file with `feh` we can see the first part of the flag.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Secret_of_the_Polyglot]
└─$ feh flag2of2-final.pdf &
[1] 8105
```

However, looking at the end of the file we can see that it is also a [PDF-file](https://en.wikipedia.org/wiki/PDF).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Secret_of_the_Polyglot]
└─$ xxd flag2of2-final.pdf| tail -n 50
00000a10: 6974 6c65 3e3c 7264 663a 416c 743e 3c72  itle><rdf:Alt><r
00000a20: 6466 3a6c 6920 786d 6c3a 6c61 6e67 3d27  df:li xml:lang='
00000a30: 782d 6465 6661 756c 7427 3e55 6e74 6974  x-default'>Untit
00000a40: 6c65 643c 2f72 6466 3a6c 693e 3c2f 7264  led</rdf:li></rd
00000a50: 663a 416c 743e 3c2f 6463 3a74 6974 6c65  f:Alt></dc:title
00000a60: 3e3c 2f72 6466 3a44 6573 6372 6970 7469  ></rdf:Descripti
00000a70: 6f6e 3e0a 3c2f 7264 663a 5244 463e 0a3c  on>.</rdf:RDF>.<
00000a80: 2f78 3a78 6d70 6d65 7461 3e0a 2020 2020  /x:xmpmeta>.    
00000a90: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000aa0: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000ab0: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000ac0: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000ad0: 2020 2020 0a20 2020 2020 2020 2020 2020      .           
00000ae0: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000af0: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000b00: 2020 2020 2020 2020 2020 2020 2020 2020                  
00000b10: 2020 2020 2020 2020 2020 2020 200a 3c3f               .<?
00000b20: 7870 6163 6b65 7420 656e 643d 2777 273f  xpacket end='w'?
00000b30: 3e0a 656e 6473 7472 6561 6d0a 656e 646f  >.endstream.endo
00000b40: 626a 0a32 2030 206f 626a 0a3c 3c2f 5072  bj.2 0 obj.<</Pr
00000b50: 6f64 7563 6572 2847 504c 2047 686f 7374  oducer(GPL Ghost
00000b60: 7363 7269 7074 2031 302e 3031 2e32 290a  script 10.01.2).
00000b70: 2f43 7265 6174 696f 6e44 6174 6528 443a  /CreationDate(D:
00000b80: 3230 3234 3033 3132 3030 3034 3332 5a30  20240312000432Z0
00000b90: 3027 3030 2729 0a2f 4d6f 6444 6174 6528  0'00')./ModDate(
00000ba0: 443a 3230 3234 3033 3132 3030 3034 3332  D:20240312000432
00000bb0: 5a30 3027 3030 2729 3e3e 656e 646f 626a  Z00'00')>>endobj
00000bc0: 0a78 7265 660a 3020 3130 0a30 3030 3030  .xref.0 10.00000
00000bd0: 3030 3030 3020 3635 3533 3520 6620 0a30  00000 65535 f .0
00000be0: 3030 3030 3030 3536 3320 3030 3030 3020  000000563 00000 
00000bf0: 6e20 0a30 3030 3030 3031 3936 3920 3030  n .0000001969 00
00000c00: 3030 3020 6e20 0a30 3030 3030 3030 3530  000 n .000000050
00000c10: 3420 3030 3030 3020 6e20 0a30 3030 3030  4 00000 n .00000
00000c20: 3030 3336 3320 3030 3030 3020 6e20 0a30  00363 00000 n .0
00000c30: 3030 3030 3030 3138 3220 3030 3030 3020  000000182 00000 
00000c40: 6e20 0a30 3030 3030 3030 3334 3520 3030  n .0000000345 00
00000c50: 3030 3020 6e20 0a30 3030 3030 3030 3635  000 n .000000065
00000c60: 3620 3030 3030 3020 6e20 0a30 3030 3030  6 00000 n .00000
00000c70: 3030 3632 3720 3030 3030 3020 6e20 0a30  00627 00000 n .0
00000c80: 3030 3030 3030 3732 3020 3030 3030 3020  000000720 00000 
00000c90: 6e20 0a74 7261 696c 6572 0a3c 3c20 2f53  n .trailer.<< /S
00000ca0: 697a 6520 3130 202f 526f 6f74 2031 2030  ize 10 /Root 1 0
00000cb0: 2052 202f 496e 666f 2032 2030 2052 0a2f   R /Info 2 0 R./
00000cc0: 4944 205b 3c32 3445 3841 4241 3338 3145  ID [<24E8ABA381E
00000cd0: 3245 4344 4345 4443 3133 3538 3144 3546  2ECDCEDC13581D5F
00000ce0: 3244 3030 353e 3c32 3445 3841 4241 3338  2D005><24E8ABA38
00000cf0: 3145 3245 4344 4345 4443 3133 3538 3144  1E2ECDCEDC13581D
00000d00: 3546 3244 3030 353e 5d0a 3e3e 0a73 7461  5F2D005>].>>.sta
00000d10: 7274 7872 6566 0a32 3039 350a 2525 454f  rtxref.2095.%%EO
00000d20: 460a  
```

Files that contains valid forms of multiple formats at the same time are called [polyglots](https://en.wikipedia.org/wiki/Polyglot_(computing)). Hence, the challenge name.

Opening the file in a web browser such as `firefox` gives us the second part of the flag.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Secret_of_the_Polyglot]
└─$ firefox flag2of2-final.pdf &
[1] 8604
```

Finally, we manually append the two parts to get the entire flag.

For additional information, please see the references below.

## References

- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [PDF - Wikipedia](https://en.wikipedia.org/wiki/PDF)
- [PNG - Wikipedia](https://en.wikipedia.org/wiki/PNG)
- [Polyglot (computing) - Wikipedia](https://en.wikipedia.org/wiki/Polyglot_(computing))
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
