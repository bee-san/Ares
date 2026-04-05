# Transformation

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MADSTACKS

Description:
I wonder what this really is... enc 

''.join([chr((ord(flag[i]) << 8) + ord(flag[i + 1])) for i in range(0, len(flag), 2)])

Hints:
1. You may find some decoders online
```

Challenge link: [https://play.picoctf.org/practice/challenge/104](https://play.picoctf.org/practice/challenge/104)

## Solutions

### Analyze the given information

Let's start by looking at what we have.

We have an encoded file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Transformation]
└─$ cat enc                              
灩捯䍔䙻ㄶ形楴獟楮獴㌴摟潦弸弲㘶㠴挲ぽ  
```

that looks like Chinese when viewed with UTF-8 encoding.

We also have the Python code snippet that encoded the data. It uses list comprehension, the `<<` (left bitwise shift) operator and the `ord` and `chr` functions to encode the flag as a 16-bit string.  
`''.join([chr((ord(flag[i]) << 8) + ord(flag[i + 1])) for i in range(0, len(flag), 2)])`

### CyberChef solution

As the hint suggested you can use an online site such as [CyberChef](https://gchq.github.io/CyberChef/) and the 'Encode text' recipe to get the flag.

Enter 'text' in the `Operations` search bar, then drag and drop `Encode text` to the `Recipe`.  
Change the Encoding to `UTF-16BE (1201)`, copy the scrambled flag to the `Input` pane and press `BAKE`.

The flag will be shown in the `Output` pane.

### Python reverse decoder

Alternatively, we can put together a Python script that reverses what was done. Something like this

```python
#!/usr/bin/python
# -*- coding: utf-8 -*-

enc_flag = '灩捯䍔䙻ㄶ形楴獟楮獴㌴摟潦弸弲㘶㠴挲ぽ'    

flag = ''
for i in range(0, len(enc_flag)):
    flag += chr(ord(enc_flag[i]) >> 8)
    flag += chr(ord(enc_flag[i]) - ((ord(enc_flag[i])>>8)<<8))
print(flag)
```

Then, make sure the script is executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Transformation]
└─$ chmod +x solve.py 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Transformation]
└─$ ./solve.py
picoCTF{<REDACTED>}
```

And we have that flag again.

### Python encoding brute forcer

Finally, we can assume that a standing encoding scheme was used and just brute force all combinations

```python
#!/usr/bin/python

import string

# From https://stackoverflow.com/questions/1728376/get-a-list-of-all-the-encodings-python-can-encode-to, Python 3.8 list
encodings = ['ascii', 'big5', 'big5hkscs', 'cp037', 'cp273', 'cp424', 'cp437', 'cp500', 'cp720', 'cp737', 'cp775', 'cp850',
    'cp852', 'cp855', 'cp856', 'cp857', 'cp858', 'cp860', 'cp861', 'cp862', 'cp863', 'cp864', 'cp865', 'cp866', 'cp869',
    'cp874', 'cp875', 'cp932', 'cp949', 'cp950', 'cp1006', 'cp1026', 'cp1125', 'cp1140', 'cp1250', 'cp1251', 'cp1252',
    'cp1253', 'cp1254', 'cp1255', 'cp1256', 'cp1257', 'cp1258', 'euc_jp', 'euc_jis_2004', 'euc_jisx0213', 'euc_kr', 'gb2312',
    'gbk', 'gb18030', 'hz', 'iso2022_jp', 'iso2022_jp_1', 'iso2022_jp_2', 'iso2022_jp_2004', 'iso2022_jp_3', 'iso2022_jp_ext',
    'iso2022_kr', 'latin_1', 'iso8859_2', 'iso8859_3', 'iso8859_4', 'iso8859_5', 'iso8859_6', 'iso8859_7', 'iso8859_8',
    'iso8859_9', 'iso8859_10', 'iso8859_11', 'iso8859_13', 'iso8859_14', 'iso8859_15', 'iso8859_16', 'johab', 'koi8_r',
    'koi8_t', 'koi8_u', 'kz1048', 'mac_cyrillic', 'mac_greek', 'mac_iceland', 'mac_latin2', 'mac_roman', 'mac_turkish',
    'ptcp154', 'shift_jis', 'shift_jis_2004', 'shift_jisx0213', 'utf_32', 'utf_32_be', 'utf_32_le', 'utf_16', 'utf_16_be',
    'utf_16_le', 'utf_7', 'utf_8', 'utf_8_sig']

flag_format = 'picoCTF'

# Read the encoded flag
with open("enc", 'r') as fh:
    enc_flag = fh.read().strip()

for enc in encodings:
    try:
        plain = enc_flag.encode(enc).decode()
        if (flag_format in plain):
            print(f"Flag found with encoding {enc}: {plain}")
    except:
        pass
```

Then we run the script and hope for the best

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Transformation]
└─$ ./bf_encoding.py
Flag found with encoding utf_16_be: picoCTF{<REDACTED>}
```

And there we have the flag in yet another way.

## References

- [Bitwise Operators - Python](https://wiki.python.org/moin/BitwiseOperators)
- [Character encoding - Wikipedia](https://en.wikipedia.org/wiki/Character_encoding)
- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python List Comprehension - Programiz](https://www.programiz.com/python-programming/list-comprehension)
- [Python ord(), chr() functions - Digital Ocean](https://www.digitalocean.com/community/tutorials/python-ord-chr)
- [UTF-8 - Wikipedia](https://en.wikipedia.org/wiki/UTF-8)
- [UTF-16 - Wikipedia](https://en.wikipedia.org/wiki/UTF-16)
