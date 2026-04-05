# MacroHard WeakEdge

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MADSTACKS

Description:
I've hidden a flag in this file. Can you find it? 
Forensics is fun.pptm

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/130](https://play.picoctf.org/practice/challenge/130)

## Solution

The pptm (rather than just ppt) file extension and the name of the challenge hints that there are macros involved so let's check that first.

### Checking for macros

Checking for macros with `olevba` which is part of [oletools](https://github.com/decalage2/oletools)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ ~/python_venvs/oletools/bin/olevba Forensics_is_fun.pptm 
XLMMacroDeobfuscator: pywin32 is not installed (only is required if you want to use MS Excel)
olevba 0.60.1 on Python 3.11.4 - http://decalage.info/python/oletools
===============================================================================
FILE: Forensics_is_fun.pptm
Type: OpenXML
WARNING  For now, VBA stomping cannot be detected for files in memory
-------------------------------------------------------------------------------
VBA MACRO Module1.bas 
in file: ppt/vbaProject.bin - OLE stream: 'VBA/Module1'
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
Sub not_flag()
    Dim not_flag As String
    not_flag = "sorry_but_this_isn't_it"
End Sub
No suspicious keyword or IOC found.
```

Nope, no flag there.

### Check for exif data

Next, I checked for exif data with [ExifTool](https://exiftool.org/)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ exiftool Forensics_is_fun.pptm                          
ExifTool Version Number         : 12.52
File Name                       : Forensics_is_fun.pptm
Directory                       : .
File Size                       : 100 kB
File Modification Date/Time     : 2022:04:25 13:13:56-04:00
File Access Date/Time           : 2023:08:04 13:52:19-04:00
File Inode Change Date/Time     : 2022:04:25 13:13:56-04:00
File Permissions                : -rwxrwxrwx
File Type                       : PPTM
File Type Extension             : pptm
MIME Type                       : application/vnd.ms-powerpoint.presentation.macroEnabled.12
Zip Required Version            : 20
Zip Bit Flag                    : 0x0006
Zip Compression                 : Deflated
Zip Modify Date                 : 1980:01:01 00:00:00
Zip CRC                         : 0xa0517e97
Zip Compressed Size             : 674
Zip Uncompressed Size           : 10660
Zip File Name                   : [Content_Types].xml
Preview Image                   : (Binary data 2278 bytes, use -b option to extract)           <--- Note #1
Title                           : Forensics is fun
Creator                         : John
Last Modified By                : John
Revision Number                 : 2
Create Date                     : 2020:10:23 18:21:24Z
Modify Date                     : 2020:10:23 18:35:27Z
Total Edit Time                 : 4 minutes
Words                           : 7
Application                     : Microsoft Office PowerPoint
Presentation Format             : Widescreen
Paragraphs                      : 2
Slides                          : 58
Notes                           : 0
Hidden Slides                   : 1                              <---- Note #2
MM Clips                        : 0
Scale Crop                      : No
Heading Pairs                   : Fonts Used, 3, Theme, 1, Slide Titles, 58
Titles Of Parts                 : Arial, Calibri, Calibri Light, Office Theme, Forensics is fun, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation, PowerPoint Presentation
Links Up To Date                : No
Shared Doc                      : No
Hyperlinks Changed              : No
App Version                     : 16.0000
```

Here there are two noteworthy finds: an embedded preview image and one hidden slide.

Let's extract the preview image and view it with `eog`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ exiftool -b -PreviewImage Forensics_is_fun.pptm > preview.jpg 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ eog preview.jpg &
```

The preview image doesn't contain any flag though, just the text "Forensics is fun".

### Hunt for the hidden slide

The .pptm [file format](https://en.wikipedia.org/wiki/Office_Open_XML) is essentially a zip file that can be unpacked with `unzip`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ unzip Forensics_is_fun.pptm 
Archive:  Forensics_is_fun.pptm
  inflating: [Content_Types].xml     
  inflating: _rels/.rels             
  inflating: ppt/presentation.xml
  inflating: ppt/slides/_rels/slide46.xml.rels  
  inflating: ppt/slides/slide1.xml   
  inflating: ppt/slides/slide2.xml   
  inflating: ppt/slides/slide3.xml   
  inflating: ppt/slides/slide4.xml   
  inflating: ppt/slides/slide5.xml   
  inflating: ppt/slides/slide6.xml    
<---snip---> 
  inflating: ppt/slideLayouts/_rels/slideLayout11.xml.rels  
  inflating: ppt/theme/theme1.xml    
 extracting: docProps/thumbnail.jpeg  
  inflating: ppt/vbaProject.bin      
  inflating: ppt/presProps.xml       
  inflating: ppt/viewProps.xml       
  inflating: ppt/tableStyles.xml     
  inflating: docProps/core.xml       
  inflating: docProps/app.xml        
  inflating: ppt/slideMasters/hidden  
```

Ah, the very last file called `hidden` looks very interesting.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ file ppt/slideMasters/hidden 
ppt/slideMasters/hidden: ASCII text, with no line terminators

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ cat ppt/slideMasters/hidden
Z m x h Z z o g c G l j b 0 N U R n t E M W R f d V 9 r b j B 3 X 3 B w d H N f c l 9 6 M X A 1 f Q  
```

### Decode the flag

Hhm, apart from the spaces it almost looks like [Base64](https://en.wikipedia.org/wiki/Base64).

Let's try that

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/MacroHard_WeakEdge]
└─$ cat ppt/slideMasters/hidden | tr -d " " | base64 -d
flag: picoCTF{<REDACTED>}base64: invalid input
```

Probably some missing padding but we have the flag.

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [eog - Manual page](https://manpages.debian.org/bullseye/eog/eog.1.en.html)
- [Exif - Wikipedia](https://en.wikipedia.org/wiki/Exif)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [Macro (computer science) - Wikipedia](https://en.wikipedia.org/wiki/Macro_(computer_science))
- [Office Open XML - Wikipedia](https://en.wikipedia.org/wiki/Office_Open_XML)
- [oletools - GitHub](https://github.com/decalage2/oletools)
- [oletools - PyPI](https://pypi.org/project/oletools/)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
