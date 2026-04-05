# Blast from the past

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: Forensics, picoCTF 2024, browser_webshell_solvable, metadata
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL

Description:
The judge for these pictures is a real fan of antiques. Can you age this photo to the specifications?

Set the timestamps on this picture to 1970:01:01 00:00:00.001+00:00 with as much precision as possible 
for each timestamp. 

In this example, +00:00 is a timezone adjustment. Any timezone is acceptable as long as the time is 
equivalent. As an example, this timestamp is acceptable as well: 
1969:12:31 19:00:00.001-05:00. 

For timestamps without a timezone adjustment, put them in GMT time (+00:00). The checker program provides 
the timestamp needed for each.

Use this picture.

Submit your modified picture here:
nc -w 2 mimas.picoctf.net 57184 < original_modified.jpg

Check your modified picture here:
nc mimas.picoctf.net 62826

Hints:
1. Exiftool is really good at reading metadata, but you might want to use something else to modify it.
```

Challenge link: [https://play.picoctf.org/practice/challenge/432](https://play.picoctf.org/practice/challenge/432)

## Solution

### Basic analysis of the picture

We start by checking the metadata of the picture with `exiftool`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool original.jpg   
ExifTool Version Number         : 12.52
File Name                       : original.jpg
Directory                       : .
File Size                       : 2.9 MB
File Modification Date/Time     : 2024:07:14 15:23:35+02:00
File Access Date/Time           : 2024:07:14 15:24:37+02:00
File Inode Change Date/Time     : 2024:07:14 15:23:35+02:00
File Permissions                : -rwxrwxrwx
File Type                       : JPEG
File Type Extension             : jpg
MIME Type                       : image/jpeg
Exif Byte Order                 : Little-endian (Intel, II)
Image Description               : 
Make                            : samsung
Camera Model Name               : SM-A326U
Orientation                     : Rotate 90 CW
X Resolution                    : 72
Y Resolution                    : 72
Resolution Unit                 : inches
Software                        : MediaTek Camera Application
Modify Date                     : 2023:11:20 15:46:23
Y Cb Cr Positioning             : Co-sited
Exposure Time                   : 1/24
F Number                        : 1.8
Exposure Program                : Program AE
ISO                             : 500
Sensitivity Type                : Unknown
Recommended Exposure Index      : 0
Exif Version                    : 0220
Date/Time Original              : 2023:11:20 15:46:23
Create Date                     : 2023:11:20 15:46:23
Components Configuration        : Y, Cb, Cr, -
Shutter Speed Value             : 1/24
Aperture Value                  : 1.9
Brightness Value                : 3
Exposure Compensation           : 0
Max Aperture Value              : 1.8
Metering Mode                   : Center-weighted average
Light Source                    : Other
Flash                           : On, Fired
Focal Length                    : 4.6 mm
Sub Sec Time                    : 703
Sub Sec Time Original           : 703
Sub Sec Time Digitized          : 703
Flashpix Version                : 0100
Color Space                     : sRGB
Exif Image Width                : 4000
Exif Image Height               : 3000
Interoperability Index          : R98 - DCF basic file (sRGB)
Interoperability Version        : 0100
Exposure Mode                   : Auto
White Balance                   : Auto
Digital Zoom Ratio              : 1
Focal Length In 35mm Format     : 25 mm
Scene Capture Type              : Standard
Compression                     : JPEG (old-style)
Thumbnail Offset                : 1408
Thumbnail Length                : 64000
Image Width                     : 4000
Image Height                    : 3000
Encoding Process                : Baseline DCT, Huffman coding
Bits Per Sample                 : 8
Color Components                : 3
Y Cb Cr Sub Sampling            : YCbCr4:2:0 (2 2)
Time Stamp                      : 2023:11:20 21:46:21.420+01:00
MCC Data                        : United States / Guam
Aperture                        : 1.8
Image Size                      : 4000x3000
Megapixels                      : 12.0
Scale Factor To 35 mm Equivalent: 5.4
Shutter Speed                   : 1/24
Create Date                     : 2023:11:20 15:46:23.703
Date/Time Original              : 2023:11:20 15:46:23.703
Modify Date                     : 2023:11:20 15:46:23.703
Thumbnail Image                 : (Binary data 64000 bytes, use -b option to extract)
Circle Of Confusion             : 0.006 mm
Field Of View                   : 71.5 deg
Focal Length                    : 4.6 mm (35 mm equivalent: 25.0 mm)
Hyperfocal Distance             : 2.13 m
Light Value                     : 4.0
```

We ought to have 7 timestamps to change. The first 3 timestamps are file system timestamps only.

First we make a copy of the file to work with

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ cp original.jpg original_nulled.jpg

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ ls
original.jpg  original_nulled.jpg
```

### Change most timestamps with exiftool

There is a `-AllDates` parameter in `exiftool` that sets most (all?) timestamps.  
Let's start with that

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool -AllDates="1970:01:01 00:00:00.001+00:00" original_nulled.jpg 
    1 image files updated

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool original_nulled.jpg
ExifTool Version Number         : 12.52
File Name                       : original_nulled.jpg
Directory                       : .
File Size                       : 2.9 MB
File Modification Date/Time     : 2024:07:14 15:57:01+02:00
File Access Date/Time           : 2024:07:14 15:57:01+02:00
File Inode Change Date/Time     : 2024:07:14 15:57:01+02:00
File Permissions                : -rwxrwxrwx
File Type                       : JPEG
File Type Extension             : jpg
MIME Type                       : image/jpeg
Exif Byte Order                 : Little-endian (Intel, II)
Image Description               : 
Make                            : samsung
Camera Model Name               : SM-A326U
Orientation                     : Rotate 90 CW
X Resolution                    : 72
Y Resolution                    : 72
Resolution Unit                 : inches
Software                        : MediaTek Camera Application
Modify Date                     : 1970:01:01 00:00:00
Y Cb Cr Positioning             : Co-sited
Exposure Time                   : 1/24
F Number                        : 1.8
Exposure Program                : Program AE
ISO                             : 500
Sensitivity Type                : Unknown
Recommended Exposure Index      : 0
Exif Version                    : 0220
Date/Time Original              : 1970:01:01 00:00:00
Create Date                     : 1970:01:01 00:00:00
Components Configuration        : Y, Cb, Cr, -
Shutter Speed Value             : 1/24
Aperture Value                  : 1.9
Brightness Value                : 3
Exposure Compensation           : 0
Max Aperture Value              : 1.8
Metering Mode                   : Center-weighted average
Light Source                    : Other
Flash                           : On, Fired
Focal Length                    : 4.6 mm
Sub Sec Time                    : 703
Sub Sec Time Original           : 703
Sub Sec Time Digitized          : 703
Flashpix Version                : 0100
Color Space                     : sRGB
Exif Image Width                : 4000
Exif Image Height               : 3000
Interoperability Index          : R98 - DCF basic file (sRGB)
Interoperability Version        : 0100
Exposure Mode                   : Auto
White Balance                   : Auto
Digital Zoom Ratio              : 1
Focal Length In 35mm Format     : 25 mm
Scene Capture Type              : Standard
Compression                     : JPEG (old-style)
Thumbnail Offset                : 1124
Thumbnail Length                : 64000
Image Width                     : 4000
Image Height                    : 3000
Encoding Process                : Baseline DCT, Huffman coding
Bits Per Sample                 : 8
Color Components                : 3
Y Cb Cr Sub Sampling            : YCbCr4:2:0 (2 2)
Time Stamp                      : 2023:11:20 21:46:21.420+01:00
MCC Data                        : United States / Guam
Aperture                        : 1.8
Image Size                      : 4000x3000
Megapixels                      : 12.0
Scale Factor To 35 mm Equivalent: 5.4
Shutter Speed                   : 1/24
Create Date                     : 1970:01:01 00:00:00.703
Date/Time Original              : 1970:01:01 00:00:00.703
Modify Date                     : 1970:01:01 00:00:00.703
Thumbnail Image                 : (Binary data 64000 bytes, use -b option to extract)
Circle Of Confusion             : 0.006 mm
Field Of View                   : 71.5 deg
Focal Length                    : 4.6 mm (35 mm equivalent: 25.0 mm)
Hyperfocal Distance             : 2.13 m
Light Value                     : 4.0
```

Only one timestamp called `Time stamp` seems to remain (apart from the file system timestamps).

Next we submit and check how close we are

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc -w 2 mimas.picoctf.net 57531 < original_nulled.jpg

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc mimas.picoctf.net 54870                                            
MD5 of your picture:
e7537a20fd614f08232eaa16d4f6587a  test.out

Checking tag 1/7
Looking at IFD0: ModifyDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 2/7
Looking at ExifIFD: DateTimeOriginal
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 3/7
Looking at ExifIFD: CreateDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 4/7
Looking at Composite: SubSecCreateDate
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.703
Oops! That tag isn't right. Please try again.
```

No, the milliseconds field isn't correct.  
Thankfully, the checking scripts tells us the field name.

Alternatively, we can list tags in `exiftool` and `grep` for the relevant ones

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool -list -EXIF:All original.jpg | grep -i -e time -e date
  CompositeImageCount CompositeImageExposureTimes CompressedBitsPerPixel
  Compression ConsecutiveBadFaxLines Contrast Converter Copyright CreateDate
  DNGLensInfo DNGPrivateData DNGVersion DataType DateTimeOriginal Decode
  ExposureIndex ExposureMode ExposureProgram ExposureTime ExtraSamples FNumber
  FaxProfile FaxRecvParams FaxRecvTime FaxSubAddress FedexEDR FileSource
  GPSAltitudeRef GPSAreaInformation GPSDOP GPSDateStamp GPSDestBearing
  GPSProcessingMethod GPSSatellites GPSSpeed GPSSpeedRef GPSStatus GPSTimeStamp
  MDColorTable MDFileTag MDFileUnits MDLabName MDPrepDate MDPrepTime
  ModelTiePoint ModelTransform ModifyDate MoireFilter MultiProfiles Multishot
  OceImageLogic OceScanjobDesc OffsetSchema OffsetTime OffsetTimeDigitized
  OffsetTimeOriginal OldSubfileType OpcodeList1 OpcodeList2 OpcodeList3
  PreviewApplicationVersion PreviewColorSpace PreviewDateTime PreviewImage
  SceneType SecurityClassification SelfTimerMode SemanticInstanceID
  StoNits StripByteCounts StripOffsets StripRowCounts SubSecTime
  SubSecTimeDigitized SubSecTimeOriginal SubTileBlockSize SubfileType
  TimeCodes TimeZoneOffset TransferFunction TransferRange Transformation
File Modification Date/Time     : 2024:07:14 15:34:44+02:00
File Access Date/Time           : 2024:07:14 15:34:44+02:00
File Inode Change Date/Time     : 2024:07:14 15:34:44+02:00
Modify Date                     : 2023:11:20 15:46:23
Exposure Time                   : 1/24
Date/Time Original              : 2023:11:20 15:46:23
Create Date                     : 2023:11:20 15:46:23
Sub Sec Time                    : 703
Sub Sec Time Original           : 703
Sub Sec Time Digitized          : 703
Time Stamp                      : 2023:11:20 21:46:21.420+01:00
Create Date                     : 2023:11:20 15:46:23.703
Date/Time Original              : 2023:11:20 15:46:23.703
Modify Date                     : 2023:11:20 15:46:23.703
```

### Adjust timestamps in exiftool

After some trial-and-error, we have further adjusted 3 timestamps and now we resubmit and check again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool -AllDates="1970:01:01 00:00:00.001" -SubSecCreateDate="1970:01:01 00:00:00.001" -SubSecDateTimeOriginal="1970:01:01 00:00:00.001" -SubSecModifyDate="1970:01:01 00:00:00.001" original_nulled.jpg
    1 image files updated

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc -w 2 mimas.picoctf.net 57531 < original_nulled.jpg

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc mimas.picoctf.net 54870
MD5 of your picture:
eb5ae92ce9f801b9d1aa8e4c800e9705  test.out

Checking tag 1/7
Looking at IFD0: ModifyDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 2/7
Looking at ExifIFD: DateTimeOriginal
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 3/7
Looking at ExifIFD: CreateDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 4/7
Looking at Composite: SubSecCreateDate
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 5/7
Looking at Composite: SubSecDateTimeOriginal
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 6/7
Looking at Composite: SubSecModifyDate
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 7/7
Timezones do not have to match, as long as it's the equivalent time.
Looking at Samsung: TimeStamp
Looking for '1970:01:01 00:00:00.001+00:00'
Found: 2023:11:20 20:46:21.420+00:00
Oops! That tag isn't right. Please try again.
```

Trying to set this timestamp in the same manner gives us a warning

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool -AllDates="1970:01:01 00:00:00.001" -SubSecCreateDate="1970:01:01 00:00:00.001" -SubSecDateTimeOriginal="1970:01:01 00:00:00.001" -SubSecModifyDate="1970:01:01 00:00:00.001" -TimeStamp="1970:01:01 00:00:00.001" original_nulled.jpg
Warning: Not an integer for XMP-apple-fi:TimeStamp
    1 image files updated
```

and the modification fails!

Using an integer value here for an [epoch timestamp](https://en.wikipedia.org/wiki/Epoch_(computing)) doesn't return any warning message, but still fails to update the timestamp.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ exiftool -AllDates="1970:01:01 00:00:00.001" -SubSecCreateDate="1970:01:01 00:00:00.001" -SubSecDateTimeOriginal="1970:01:01 00:00:00.001" -SubSecModifyDate="1970:01:01 00:00:00.001" -TimeStamp="1" original_nulled.jpg
    1 image files updated
```

We need to change this value manually.

### Manually modifying the the Samsung timestamp

The Samsung time stamp can [be modified manually](https://stackoverflow.com/questions/78185037/how-to-edit-the-samsung-trailer-tag-timestamp) with any hexeditor.

We find the timestamp at the end of the picture

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ xxd original_nulled.jpg | tail -20 
002b8d20: 6fbc ff00 f5ee 6802 a32b ed02 465d c54f  o.....h..+..F].O
002b8d30: ef48 e09f febd 5657 0b21 7779 019a 3e10  .H....VW.!wy..>.
002b8d40: b640 3d39 5f5e a6af 5d7f c7b4 5ff0 1ac7  .@=9_^..]..._...
002b8d50: b8ff 005e bff5 f03f 91a0 074f 2bb5 b336  ...^...?...O+..6
002b8d60: f552 f206 ea71 8fe9 ebf5 aabe 7394 8dde  .R...q......s...
002b8d70: 50ee e0b4 d83e 87f9 9a74 bff1 e2ff 00ee  P....>...t......
002b8d80: aff3 aa0b fead be8b 4d6e 606c bbf3 1ac9  ........Mn`l....
002b8d90: e4a1 8fe6 f3cf 7c8e 83ea 2877 3b11 72d9  ......|...(w;.r.
002b8da0: 90ef 049c 823d 47b6 474a a97b f763 fa2f  .....=G.GJ.{.c./
002b8db0: fe82 2acb ff00 abd3 bfeb c5bf f433 5d50  ..*..........3]P
002b8dc0: dcc6 6ddc ffd9 0000 010a 0e00 0000 496d  ..m...........Im
002b8dd0: 6167 655f 5554 435f 4461 7461 3137 3030  age_UTC_Data1700
002b8de0: 3531 3331 3831 3432 3000 00a1 0a08 0000  513181420.......
002b8df0: 004d 4343 5f44 6174 6133 3130 0000 610c  .MCC_Data310..a.
002b8e00: 1800 0000 4361 6d65 7261 5f43 6170 7475  ....Camera_Captu
002b8e10: 7265 5f4d 6f64 655f 496e 666f 3153 4546  re_Mode_Info1SEF
002b8e20: 486b 0000 0003 0000 0000 0001 0a57 0000  Hk...........W..
002b8e30: 0023 0000 0000 00a1 0a34 0000 0013 0000  .#.......4......
002b8e40: 0000 0061 0c21 0000 0021 0000 0030 0000  ...a.!...!...0..
002b8e50: 0053 4546 54                             .SEFT
```

After manual modification the timestamp is set to `0000000000001`.  
The file now looks like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ xxd original_nulled.jpg | tail -20 
002b8d20: 6fbc ff00 f5ee 6802 a32b ed02 465d c54f  o.....h..+..F].O
002b8d30: ef48 e09f febd 5657 0b21 7779 019a 3e10  .H....VW.!wy..>.
002b8d40: b640 3d39 5f5e a6af 5d7f c7b4 5ff0 1ac7  .@=9_^..]..._...
002b8d50: b8ff 005e bff5 f03f 91a0 074f 2bb5 b336  ...^...?...O+..6
002b8d60: f552 f206 ea71 8fe9 ebf5 aabe 7394 8dde  .R...q......s...
002b8d70: 50ee e0b4 d83e 87f9 9a74 bff1 e2ff 00ee  P....>...t......
002b8d80: aff3 aa0b fead be8b 4d6e 606c bbf3 1ac9  ........Mn`l....
002b8d90: e4a1 8fe6 f3cf 7c8e 83ea 2877 3b11 72d9  ......|...(w;.r.
002b8da0: 90ef 049c 823d 47b6 474a a97b f763 fa2f  .....=G.GJ.{.c./
002b8db0: fe82 2acb ff00 abd3 bfeb c5bf f433 5d50  ..*..........3]P
002b8dc0: dcc6 6ddc ffd9 0000 010a 0e00 0000 496d  ..m...........Im
002b8dd0: 6167 655f 5554 435f 4461 7461 3030 3030  age_UTC_Data0000
002b8de0: 3030 3030 3030 3030 3100 00a1 0a08 0000  000000001.......
002b8df0: 004d 4343 5f44 6174 6133 3130 0000 610c  .MCC_Data310..a.
002b8e00: 1800 0000 4361 6d65 7261 5f43 6170 7475  ....Camera_Captu
002b8e10: 7265 5f4d 6f64 655f 496e 666f 3153 4546  re_Mode_Info1SEF
002b8e20: 486b 0000 0003 0000 0000 0001 0a57 0000  Hk...........W..
002b8e30: 0023 0000 0000 00a1 0a34 0000 0013 0000  .#.......4......
002b8e40: 0000 0061 0c21 0000 0021 0000 0030 0000  ...a.!...!...0..
002b8e50: 0053 4546 54                             .SEFT
```

### Get the flag

Finally, we submit and check the file again to get our flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc -w 2 mimas.picoctf.net 57531 < original_nulled.jpg

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Blast_from_the_past]
└─$ nc mimas.picoctf.net 54870                           
MD5 of your picture:
f157518a7ee116a69d5013bb8f1f869d  test.out

Checking tag 1/7
Looking at IFD0: ModifyDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 2/7
Looking at ExifIFD: DateTimeOriginal
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 3/7
Looking at ExifIFD: CreateDate
Looking for '1970:01:01 00:00:00'
Found: 1970:01:01 00:00:00
Great job, you got that one!

Checking tag 4/7
Looking at Composite: SubSecCreateDate
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 5/7
Looking at Composite: SubSecDateTimeOriginal
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 6/7
Looking at Composite: SubSecModifyDate
Looking for '1970:01:01 00:00:00.001'
Found: 1970:01:01 00:00:00.001
Great job, you got that one!

Checking tag 7/7
Timezones do not have to match, as long as it's the equivalent time.
Looking at Samsung: TimeStamp
Looking for '1970:01:01 00:00:00.001+00:00'
Found: 1970:01:01 00:00:00.001+00:00
Great job, you got that one!

You did it!
picoCTF{<REDACTED>} 
```

For additional information, please see the references below.

## References

- [Epoch (computing) - Wikipedia](https://en.wikipedia.org/wiki/Epoch_(computing))
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [JPEG - Wikipedia](https://en.wikipedia.org/wiki/JPEG)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [tail - Linux manual page](https://man7.org/linux/man-pages/man1/tail.1.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
