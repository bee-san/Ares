# crackme-py

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL
  
Description:

crackme.py

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/175](https://play.picoctf.org/practice/challenge/175)

## Solutions

### Analyze the Python script

Let's start by looking at the `decode_secret` function of the script

```python
def decode_secret(secret):
    """ROT47 decode

    NOTE: encode and decode are the same operation in the ROT cipher family.
    """

    # Encryption key
    rotate_const = 47

    # Storage for decoded secret
    decoded = ""

    # decode loop
    for c in secret:
        index = alphabet.find(c)
        original_index = (index + rotate_const) % len(alphabet)
        decoded = decoded + alphabet[original_index]

    print(decoded)
```

This is actually the entire code to decode the flag. We can just reuse most of the script.

### Write a decoder script

Let's copy the original script, remove the `choose_greatest` function, remove some comments, add a shebang and add a call to the `decode_secret` function

```python
#!/usr/bin/python

bezos_cc_secret = "A:4@r%uL`M-^M0c0AbcM-MFE0cdhb52g2N"

# Reference alphabet
alphabet = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ"+ \
            "[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"

def decode_secret(secret):
    rotate_const = 47

    decoded = ""
    for c in secret:
        index = alphabet.find(c)
        original_index = (index + rotate_const) % len(alphabet)
        decoded = decoded + alphabet[original_index]
    print(decoded)

decode_secret(bezos_cc_secret)
```

### Get the flag

Then, make sure the script is executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/crackme-py]
└─$ chmod +x get_flag.py 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/crackme-py]
└─$ ./get_flag.py       
picoCTF{<REDACTED>}
```

## References

- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [ROT13 - Wikipedia](https://en.wikipedia.org/wiki/ROT13)
- [Shebang (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Shebang_(Unix))
