# m00nwalk

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JOON

Description:
Decode this message from the moon.

Hints:
1. How did pictures from the moon landing get sent back to Earth?
2. What is the CMU mascot?, that might help select a RX option
```

Challenge link: [https://play.picoctf.org/practice/challenge/26](https://play.picoctf.org/practice/challenge/26)

## Solution

After some googling I understood that this is [SSTV (Slow-scan television)](https://en.wikipedia.org/wiki/Slow-scan_television) and there is a [SSTV Deocoder](https://github.com/colaclanth/sstv) available.

After installing the decoder we run it like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk]
└─$ sstv -d message.wav -o moonwalk_result.png
[sstv] Searching for calibration header... Found!    
[sstv] Detected SSTV mode Scottie 1
[sstv] Decoding image...                              [####################################################################################################] 100%
[sstv] Drawing image data...
[sstv] ...Done!
```

The program automatically detects the SSTV mode as `Scottie 1` for us.

Then we just view the resulting image

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/M00nwalk]
└─$ eog moonwalk_result.png &
```

Rotate the picture twice and then you can see the flag more easily.

For additional information, please see the references below.

## References

- [Slow-scan television - Wikipedia](https://en.wikipedia.org/wiki/Slow-scan_television)
- [SSTV Decoder - GitHub](https://github.com/colaclanth/sstv)
