# WhitePages

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JOHN HAMMOND
 
Description:
I stopped using YellowPages and moved onto WhitePages... but the page they gave me is all blank!

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/51](https://play.picoctf.org/practice/challenge/51)

## Solution

### Analyse the setup

Let's start by checking the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/WhitePages]
└─$ file whitepages.txt           
whitepages.txt: Unicode text, UTF-8 text, with very long lines (1376), with no line terminators

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/WhitePages]
└─$ xxd -g 1 -l 128 whitepages.txt
00000000: e2 80 83 e2 80 83 e2 80 83 e2 80 83 20 e2 80 83  ............ ...
00000010: 20 e2 80 83 e2 80 83 e2 80 83 e2 80 83 e2 80 83   ...............
00000020: 20 e2 80 83 e2 80 83 20 e2 80 83 e2 80 83 e2 80   ...... ........
00000030: 83 e2 80 83 20 e2 80 83 e2 80 83 20 e2 80 83 20  .... ...... ... 
00000040: 20 20 e2 80 83 e2 80 83 e2 80 83 e2 80 83 e2 80    ..............
00000050: 83 20 20 e2 80 83 20 e2 80 83 e2 80 83 20 e2 80  .  ... ...... ..
00000060: 83 20 20 e2 80 83 e2 80 83 e2 80 83 20 20 e2 80  .  .........  ..
00000070: 83 20 20 e2 80 83 20 20 20 20 e2 80 83 20 e2 80  .  ...    ... ..
```

It is somewhat hard to see but the file consists of two types of Unicode [whitespace characters](https://en.wikipedia.org/wiki/Whitespace_character):

- Normal SPACE (U+0020, hex `20`)
- EM SPACE (U+2003, hex `e2 80 83`)

Converting hex values to Unicode code points can be done with

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/WhitePages]
└─$ echo -ne '\xe2\x80\x83' | iconv -f 'utf-8' -t 'utf-16be' | xxd -p
2003                                        ...
```

And converting code point to hex values can be done with

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/WhitePages]
└─$ echo -n $'\u2003' | xxd -g 1
00000000: e2 80 83                                         ...
```

### Write a Python decoder

Lets write a Python script that assumes these spaces form a binary string of ascii characters

```python
#!/usr/bin/python

# Convert to binary string
with open("whitepages.txt", mode="r", encoding="utf8") as f:
    result = ""
    text = f.read(1)
    while text:
        if text == u'\u2003':   # EM SPACE
            result += '0'
        elif text == u'\u0020': # SPACE
            result += '1'
        text = f.read(1)

# Divide the binary string into array of 8-bit binary chunks
n = 8
split_result = [result[i:i+n] for i in range(0, len(result), n)]

# Convert to ascii text
flag = ""
for item in split_result:
    flag += chr(int(str(item), 2))
print(flag)
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/WhitePages]
└─$ ./decode.py

                picoCTF

                SEE PUBLIC RECORDS & BACKGROUND REPORT
                5000 Forbes Ave, Pittsburgh, PA 15213
                picoCTF{<REDACTED>}

                                                                                             
```

For additional information, please see the references below.

## References

- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [iconv - Linux manual page](https://man7.org/linux/man-pages/man1/iconv.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Unicode - Wikipedia](https://en.wikipedia.org/wiki/Unicode)
- [Unicode Character Search](https://www.fileformat.info/info/unicode/char/search.htm)
- [Whitespace character - Wikipedia](https://en.wikipedia.org/wiki/Whitespace_character)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
