# Bases

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SANJAY C/DANNY T

Description:
What does this bDNhcm5fdGgzX3IwcDM1 mean? 

I think it has something to do with bases.

Hints:
1. Submit your answer in our flag format. For example, if your answer was 'hello', 
   you would submit 'picoCTF{hello}' as the flag.
```

Challenge link: [https://play.picoctf.org/practice/challenge/67](https://play.picoctf.org/practice/challenge/67)

## Solution

This is [Base64 encoding](https://en.wikipedia.org/wiki/Base64) and there are several ways to decode it.

### CyberChef solution

We can use [CyberChef](https://gchq.github.io/CyberChef/) and the `Base64` recipe to decode it.  
Type 'base64' in the `Operations` search bar, then drag and drop `From Base64` to the `Recipe` pane.  
Then copy and paste `bDNhcm5fdGgzX3IwcDM1` to the `Input` pane.  
Finally, press `BAKE` if you don't have `Auto Bake` selected already.
The result is shown in the `Output` pane.

To get the full flag you need to add the 'picoCTF{' and '}' parts as instructed in the hint.

### Use the base64 commandline tool

Alternatively, you can use the `base64` tool like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Bases]
└─$ echo 'bDNhcm5fdGgzX3IwcDM1' | base64 -d     
l3arn_<REDACTED>
```

Again, you need to add the 'picoCTF{' and '}' parts to get the full flag.

### Write a Python script

Of course, you can always write a Python script to decode it

```python
#!/usr/bin/python

from base64 import b64decode

enc = 'bDNhcm5fdGgzX3IwcDM1'

decoded = b64decode(enc).decode()
print(f"picoCTF{{{decoded}}}")
```

Then we make sure the script is executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Bases]
└─$ chmod +x decode.py     

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Bases]
└─$ ./decode.py       
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [base64 - Linux manual page](https://man7.org/linux/man-pages/man1/base64.1.html)
- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
