# Tapping

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
Theres tapping coming in from the wires. What's it saying 

nc jupiter.challenges.picoctf.org 21610.

Hints:
1. What kind of encoding uses dashes and dots?
2. The flag is in the format PICOCTF{}
```

Challenge link: [https://play.picoctf.org/practice/challenge/21](https://play.picoctf.org/practice/challenge/21)

## Solution

Tapping, dashes and dots - that ought to mean [morse code](https://en.wikipedia.org/wiki/Morse_code).

Let's connect to the server and find out

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Tapping]
└─$ nc jupiter.challenges.picoctf.org 21610
.--. .. -.-. --- -.-. - ..-. { -- ----- .-. ... ...-- -.-. ----- -.. ...-- .---- ... ..-. ..- -. ...-- ----. ----- ..--- ----- .---- ----. ..... .---- ----. } 
```

Yes, that looks like morse code (apart from the curly braces).

To decode it we can use an online service such as the one from [OnlineConversion.com](https://www.onlineconversion.com/morse_code.htm).

Copy and paste the output above to the lower part of the web site under `Convert morse code back into English`.  
Then press `Translate!` and the flag will be shown.

For additional information, please see the references below.

## References

- [Morse code - Wikipedia](https://en.wikipedia.org/wiki/Morse_code)
- [Morse Code Conversion - OnlineConversion.com](https://www.onlineconversion.com/morse_code.htm)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
