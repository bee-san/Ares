# m00nwalk2

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JOON

Description:
Revisit the last transmission. We think this transmission contains a hidden message. 
There are also some clues clue 1, clue 2, clue 3.

Hints:
1. Use the clues to extract the another flag from the .wav file
```

Challenge link: [https://play.picoctf.org/practice/challenge/28](https://play.picoctf.org/practice/challenge/28)

## Solution

The setup is the same as in the [previous moonwalk challenge](m00nwalk.md).

### Decode the clues

First we decode the clue wav-files

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk2]
└─$ sstv -d clue1.wav -o clue1_result.png
[sstv] Searching for calibration header... Found!    
[sstv] Detected SSTV mode Martin 1
[sstv] Decoding image...                              [####################################################################################################] 100%
[sstv] Drawing image data...
[sstv] ...Done!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk2]
└─$ sstv -d clue2.wav -o clue2_result.png
[sstv] Searching for calibration header... Found!    
[sstv] Detected SSTV mode Scottie 2
[sstv] Decoding image...                              [####################################################################################################] 100%
[sstv] Drawing image data...
[sstv] ...Done!

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk2]
└─$ sstv -d clue3.wav -o clue3_result.png
[sstv] Searching for calibration header... Found!    
[sstv] Detected SSTV mode Martin 2
[sstv] Decoding image...                              [####################################################################################################] 100%
[sstv] Drawing image data...
[sstv] ...Done!
```

The resulting pictures is a bit hard to read but they contain:

- Clue1: Password hidden_stegosaurus
- Clue2: The quieter you are the more you can HEAR
- Clue3: Alan Eliasen the Future Boy

Googling the last clue points to this [Steganographic Decoder](https://futureboy.us/stegano/decinput.html) which uses the [Steghide](https://steghide.sourceforge.net/) tool.

### Get the flag

Lets run `steghide` with the password from clue #1

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk2]
└─$ steghide extract -sf message.wav -p hidden_stegosaurus

wrote extracted data to "steganopayload12154.txt".

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk2]
└─$ cat steganopayload12154.txt                                         
picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Slow-scan television - Wikipedia](https://en.wikipedia.org/wiki/Slow-scan_television)
- [SSTV Decoder - GitHub](https://github.com/colaclanth/sstv)
- [Steganographic Decoder - futureboy.us](https://futureboy.us/stegano/decinput.html)
- [Steganography - Wikipedia](https://en.wikipedia.org/wiki/Steganography)
- [steghide - Homepage](https://steghide.sourceforge.net/)
- [steghide - Kali Tools](https://www.kali.org/tools/steghide/)
