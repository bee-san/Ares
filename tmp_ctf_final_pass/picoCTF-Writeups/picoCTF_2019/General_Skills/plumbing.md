# plumbing

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX FULTON/DANNY TUNITIS

Description:
Sometimes you need to handle process data outside of a file.

Can you find a way to keep the output from this program and search for the flag? 

Connect to jupiter.challenges.picoctf.org 7480.

Hints:
1. Remember the flag format is picoCTF{XXXX}
2. What's a pipe? No not that kind of pipe... This kind
```

Challenge link: [https://play.picoctf.org/practice/challenge/48](https://play.picoctf.org/practice/challenge/48)

## Solution

Let's connect to the server with `nc` and see what we get

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Plumbing]
└─$ nc jupiter.challenges.picoctf.org 7480 
Not a flag either
Again, I really don't think this is a flag
I don't think this is a flag either
This is defintely not a flag
Again, I really don't think this is a flag
Not a flag either
I don't think this is a flag either
Again, I really don't think this is a flag
Again, I really don't think this is a flag
This is defintely not a flag
This is defintely not a flag
This is defintely not a flag
This is defintely not a flag
Not a flag either
Again, I really don't think this is a flag
Not a flag either
<---snip--->
```

OK, too much output. Lets `grep` for the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Plumbing]
└─$ nc jupiter.challenges.picoctf.org 7480 | grep picoCTF
picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [grep - Linux man page](https://linux.die.net/man/1/grep)
- [nc - Linux man page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Pipes: A Brief Introduction](http://www.linfo.org/pipes.html)
