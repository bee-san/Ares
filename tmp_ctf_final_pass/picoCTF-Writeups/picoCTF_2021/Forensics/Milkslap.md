# Milkslap

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JAMES LYNCH

Description:
🥛
 
Hints:
1. Look at the problem category
```

Challenge link: [https://play.picoctf.org/practice/challenge/139](https://play.picoctf.org/practice/challenge/139)

## Solution

### Analyse the web site

The emoji is a link that redirects you to `http://mercury.picoctf.net:16940/`.

On the site, right-click and select 'View page source' (or press `CTRL + U`) to get

```html
<!doctype html>

<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=400" />
  <title>🥛</title>
  <link rel="stylesheet" href="style.css" />

</head>
<body>
  <div id="image" class="center"></div>
  <div id="foot" class="center">
    <h1>MilkSlap!</h1>
    Inspired by <a href="http://eelslap.com">http://eelslap.com</a> <br>
    Credit to: <a href="https://github.com/boxmein">boxmein</a> for code inspiration.
  </div>
  <script src="script.js">


</script>
</body>
</html>
```

Next, check the `style.css` file

```css
/* source: milkslap-milkslap.scss */
body {
  margin: 0;
  padding: 0;
  overflow: hidden; }

a {
  color: inherit; }

.center {
  width: 1080px;
  height: 720px;
  margin: 0 auto; }

#image {
  height: 720px;
  margin-top: 5%;
  margin-bottom: 20px;
  background-image: url(concat_v.png);
  background-position: 0 0; }

#foot {
  margin-bottom: 5px;
  color: #999999; }
  #foot h1 {
    font-family: serif;
    font-weight: normal;
    font-size: 1rem;
    text-align: center; }
```

We can see that the background is a file called `concat_v.png`.

### Analyse the picture file

Let's retreive the picture file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Milkslap]
└─$ wget http://mercury.picoctf.net:16940/concat_v.png
--2023-09-13 11:31:35--  http://mercury.picoctf.net:16940/concat_v.png
Resolving mercury.picoctf.net (mercury.picoctf.net)... 18.189.209.142
Connecting to mercury.picoctf.net (mercury.picoctf.net)|18.189.209.142|:16940... connected.
HTTP request sent, awaiting response... 200 OK
Length: 18095920 (17M) [image/png]
Saving to: ‘concat_v.png’

concat_v.png                                            100%[===============================================================================================================================>]  17.26M  5.14MB/s    in 3.4s    

2023-09-13 11:31:38 (5.14 MB/s) - ‘concat_v.png’ saved [18095920/18095920]
```

Next, let's do some basic checks on it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Milkslap]
└─$ file concat_v.png    
concat_v.png: PNG image data, 1280 x 47520, 8-bit/color RGB, non-interlaced

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Milkslap]
└─$ exiftool concat_v.png
ExifTool Version Number         : 12.52
File Name                       : concat_v.png
Directory                       : .
File Size                       : 18 MB
File Modification Date/Time     : 2021:03:15 14:24:47-04:00
File Access Date/Time           : 2023:09:13 11:40:02-04:00
File Inode Change Date/Time     : 2021:03:15 14:24:47-04:00
File Permissions                : -rwxrwxrwx
File Type                       : PNG
File Type Extension             : png
MIME Type                       : image/png
Image Width                     : 1280
Image Height                    : 47520
Bit Depth                       : 8
Color Type                      : RGB
Compression                     : Deflate/Inflate
Filter                          : Adaptive
Interlace                       : Noninterlaced
Image Size                      : 1280x47520
Megapixels                      : 60.8

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Milkslap]
└─$ binwalk concat_v.png 

DECIMAL       HEXADECIMAL     DESCRIPTION
--------------------------------------------------------------------------------
41            0x29            Zlib compressed data, default compression
3210141       0x30FB9D        MySQL ISAM compressed data file Version 2
```

Nothing that stands out.

### Get the flag with zsteg

Then, I checked the file with `zsteg`. For unknown reasons my Linux installation of zsteg couldn't handle the file so I used my Windows version instead

```bash
Z:\CTFs\picoCTF\picoCTF_2021\Forensics\Milkslap>zsteg -a concat_v.png
imagedata           .. text: "\n\n\n\n\n\n\t\t"
b1,b,lsb,xy         .. text: "picoCTF{<REDACTED>}\n"
b1,bgr,lsb,xy       .. <wbStego size=9706075, data="\xB6\xAD\xB6}\xDB\xB2lR\x7F\xDF\x86\xB7c\xFC\xFF\xBF\x02Zr\x8E\xE2Z\x12\xD8q\xE5&MJ-X:\xB5\xBF\xF7\x7F\xDB\xDFI\bm\xDB\xDB\x80m\x00\x00\x00\xB6m\xDB\xDB\xB6\x00\x00\x00\xB6\xB6\x00m\xDB\x12\x12m\xDB\xDB\x00\x00\x00\x00\x00\xB6m\xDB\x00\xB6\x00\x00\x00\xDB\xB6mm\xDB\xB6\xB6\x00\x00\x00\x00\x00m\xDB", even=true, mix=true, controlbyte="[">
b2,r,lsb,xy         .. text: ["U" repeated 8 times]
b2,r,msb,xy         .. text: ["U" repeated 8 times]
b2,g,lsb,xy         .. text: "lUUUUUUi@"
b2,g,msb,xy         .. text: ["U" repeated 8 times]
b2,b,msb,xy         .. text: "UfUUUU@UUU"
b4,r,lsb,xy         .. text: "\"\"\"\"\"#4D"
b4,r,msb,xy         .. text: "wwww3333"
b4,g,lsb,xy         .. text: "wewwwwvUS"
b4,g,msb,xy         .. text: "\"\"\"\"DDDD"
b4,b,lsb,xy         .. text: "vdUeVwweDFw"
b4,b,msb,xy         .. text: "UUYYUUUUUUUU"
b5,bgr,msb,xy       .. text: "Nlgnp?yX\"n"
b6,r,lsb,xy         .. text: "UUUUUUQD"
b8,r,msb,xy         .. text: ["K" repeated 11 times]
b8,g,msb,xy         .. text: "I\t\t\t\t\t\t\t\tUm"
<---snip--->
```

And there, on the second line of the result, is the flag.

For additional information, please see the references below.

## References

- [Binwalk - GitHub](https://github.com/ReFirmLabs/binwalk)
- [Binwalk - Kali Tools](https://www.kali.org/tools/binwalk/)
- [binwalk - Linux manual page](https://manpages.debian.org/testing/binwalk/binwalk.1.en.html)
- [CSS - Wikipedia](https://en.wikipedia.org/wiki/CSS)
- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [HTML - Wikipedia](https://en.wikipedia.org/wiki/HTML)
- [wget - Linux manual page](https://man7.org/linux/man-pages/man1/wget.1.html)
- [zsteg - Github](https://github.com/zed-0xff/zsteg)
