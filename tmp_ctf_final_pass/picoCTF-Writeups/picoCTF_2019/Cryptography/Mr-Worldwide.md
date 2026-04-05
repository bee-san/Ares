# Mr-Worldwide

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY

Description:
A musician left us a message. What's it mean?

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/40](https://play.picoctf.org/practice/challenge/40)

## Solution

Let's start by checking the contents of the message

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Mr_Worldwide]
└─$ cat message.txt             
picoCTF{(35.028309, 135.753082)(46.469391, 30.740883)(39.758949, -84.191605)(41.015137, 28.979530)(24.466667, 54.366669)(3.140853, 101.693207)_(9.005401, 38.763611)(-3.989038, -79.203560)(52.377956, 4.897070)(41.085651, -73.858467)(57.790001, -152.407227)(31.205753, 29.924526)} 
```

Ah, this could be [longitude](https://en.wikipedia.org/wiki/Longitude) and [latitude](https://en.wikipedia.org/wiki/Latitude) coordinates for places around the world.

Lets use [Google Maps](https://www.google.com/maps/) to find out what place each coordinate corresponds to.

|Coordinate|City|
|----|----|
|(35.028309, 135.753082)|Kyoto|
|(46.469391, 30.740883)|Odesa|
|(39.758949, -84.191605)|Dayton|
|(41.015137, 28.979530)|Istanbul|
|(24.466667, 54.366669)|Abu Dhabi|
|(3.140853, 101.693207)|Kuala Lumpur|
|etc.|etc.|

For each city, take the first letter and you have the flag (which should be in CAPITALS).

For additional information, please see the references below.

## References

- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [Google Maps - Homepage](https://www.google.com/maps/)
- [Latitude - Wikipedia](https://en.wikipedia.org/wiki/Latitude)
- [Longitude - Wikipedia](https://en.wikipedia.org/wiki/Longitude)
