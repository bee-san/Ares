# The Numbers

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: PANDU

Description:
The numbers... what do they mean?

Hints:
1. The flag is in the format PICOCTF{}
```

Challenge link: [https://play.picoctf.org/practice/challenge/68](https://play.picoctf.org/practice/challenge/68)

## Solution

There are several ways to solve this challenge and here are some of them.

### CyberChef solution

We can use [CyberChef](https://gchq.github.io/CyberChef/) and the `Magic` recipe to solve this.

Write the numbers before the '{' character (that is `16 9 3 15 3 20 6`) with spaces in between in the `Input` pane of CyberChef. Don't end with a space!

Click the 'Magic Wand' icon at the `Output` pane.
If you don't get an icon, you need to type 'magic' in the `Operations` search bar, then drag and drop it to the `Recipe` and press `BAKE`.

Cyberchef recognizes the encoding as `A1Z26`.  
Unfortunately, CyberChef can't handle the '{' and '}' characters so you need to leave them out.

Then add the rest of the number in the `Input` pane (`20 8 5 14 21 13 2 5 18 19 13 1 19 15 14`).

Finally, add the `To Upper case` recipe as instructed in the hint.

You need to manually add the '{' and '}' characters before submitting the flag.

### Use an online A1Z26 decoder service

You can also use an online A1Z26 decoder service such as [Boxentriq](https://www.boxentriq.com/code-breaking/a1z26).

Add the numbers in the `Numbers` text field and you get the result in the `Letters` text field.

Again, you need to add the '{' and '}' characters before submitting the flag.

### Write a Python decoder

Alternatively, you can write a Python script to do the decoding

```python
#!/usr/bin/python

import string

ALPHABET = string.ascii_uppercase

enc_flag = "16 9 3 15 3 20 6 { 20 8 5 14 21 13 2 5 18 19 13 1 19 15 14 }".split()

flag = ''
for num in enc_flag:
    if num.isnumeric():
        flag += ALPHABET[int(num)-1]
    else:
        flag += num
print(flag)
```

Then we make sure the script is executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/The_Numbers]
└─$ chmod +x decode.py 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/The_Numbers]
└─$ ./decode.py
PICOCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [A1Z26 Cipher (What it is and How to Teach Your Kids)](https://dadstuffsite.com/a1z26-cipher-what-it-is-and-how-to-teach-your-kids/)
- [A1Z26 Cipher Decoder - Boxentriq](https://www.boxentriq.com/code-breaking/a1z26)
- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
