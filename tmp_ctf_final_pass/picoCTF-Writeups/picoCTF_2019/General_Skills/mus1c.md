# mus1c

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY

Description:
I wrote you a song. Put it in the picoCTF{} flag format.

Hints:
1. Do you think you can master rockstar?
```

Challenge link: [https://play.picoctf.org/practice/challenge/15](https://play.picoctf.org/practice/challenge/15)

## Solution

Let's start by checking the contents of the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Mus1c]
└─$ cat lyrics.txt              
Pico's a CTFFFFFFF
my mind is waitin
It's waitin

Put my mind of Pico into This
my flag is not found
put This into my flag
put my flag into Pico


shout Pico
shout Pico
shout Pico

My song's something
put Pico into This

Knock This down, down, down
put This into CTF

shout CTF
my lyric is nothing
Put This without my song into my lyric
Knock my lyric down, down, down

shout my lyric

Put my lyric into This
Put my song with This into my lyric
Knock my lyric down

shout my lyric

Build my lyric up, up ,up

shout my lyric
shout Pico
shout It

Pico CTF is fun
security is important
Fun is fun
Put security with fun into Pico CTF
Build Fun up
shout fun times Pico CTF
put fun times Pico CTF into my song

build it up

shout it
shout it

build it up, up
shout it
shout Pico
```

After some googling I understood that this is the programming language [Rockstar](https://esolangs.org/wiki/Rockstar) with an [online emulator](https://codewithrockstar.com/online) available.

Copy and paste the source code above in the emulator and click the `Rock!`-button.  
You get the following in the output pane:

```text
114
114
114
111
99
107
110
114
110
48
49
49
51
114
Program completed in 168 ms
```

This looks like [ASCII-values](https://en.wikipedia.org/wiki/ASCII) and a short Python-script can compile the flag for us

```python
#!/usr/bin/python

ascii = [114, 114, 114, 111, 99, 107, 110, 114, 110, 48, 49, 49, 51, 114]

print(f"picoCTF{{{''.join(map(chr, ascii))}}}")
```

Finally we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/General_Skills/Mus1c]
└─$ ./decode.py 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Rockstar - Esolang](https://esolangs.org/wiki/Rockstar)
- [Rockstar Online Emulator](https://codewithrockstar.com/online)
