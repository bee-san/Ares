# XtraORdinary

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoMini by redpwn, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: BOOLEAN

Description:
Check out my new, never-before-seen method of encryption! I totally invented it myself. 
I added so many for loops that I don't even know what it does. It's extraordinarily secure!

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/208](https://play.picoctf.org/practice/challenge/208)

## Solution

### Analyze the given files

Let's start by looking at the given files. First the python script

```python
#!/usr/bin/env python3

from random import randint

with open('flag.txt', 'rb') as f:
    flag = f.read()

with open('secret-key.txt', 'rb') as f:
    key = f.read()

def encrypt(ptxt, key):
    ctxt = b''
    for i in range(len(ptxt)):
        a = ptxt[i]
        b = key[i % len(key)]
        ctxt += bytes([a ^ b])
    return ctxt

ctxt = encrypt(flag, key)

random_strs = [
    b'my encryption method',
    b'is absolutely impenetrable',
    b'and you will never',
    b'ever',
    b'ever',
    b'ever',
    b'ever',
    b'ever',
    b'ever',
    b'break it'
]

for random_str in random_strs:
    for i in range(randint(0, pow(2, 8))):
        for j in range(randint(0, pow(2, 6))):
            for k in range(randint(0, pow(2, 4))):
                for l in range(randint(0, pow(2, 2))):
                    for m in range(randint(0, pow(2, 0))):
                        ctxt = encrypt(ctxt, random_str)

with open('output.txt', 'w') as f:
    f.write(ctxt.hex())
```

The script XORs the flag with an unknown key and the result are then XORed again with fixed messages a random number of times.  

And then let's check the encrypted hex-encoded `output.txt` file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/XtraORdinary]
└─$ cat output.txt 
57657535570c1e1c612b3468106a18492140662d2f5967442a2960684d28017931617b1f3637
```

No conclusions from that.

### Break the cipher

If some data is XORed an even number of times with a message then the XORing cancels each other out since `X ^ X = 0`.  
Also, if we have the flag (F) XORed with the key (K) we can XOR the result again with the key to get the flag back since `(F ^ K) ^ K = F`.  
Lastly, we don't need to consider the same fixed messages more than once because of the first fact above.

Let's write a brute forcer script to find the right combination of extra XORing by trying all combinations of the fixed messages, that is if they are included or not in the XORing.

```python
#!/usr/bin/env python3

import itertools

def encrypt(ptxt, key):
    ctxt = b''
    for i in range(len(ptxt)):
        a = ptxt[i]
        b = key[i % len(key)]
        ctxt += bytes([a ^ b])
    return ctxt
    
def multi_encrypt(ctxt, included_or_not):
    for idx, to_xor in enumerate(included_or_not):
        if to_xor:
            ctxt = encrypt(ctxt, random_strs[idx])
    return ctxt

with open("output.txt") as f:
    enc_flag = bytes.fromhex(f.read())

flag_prefix = b"picoCTF{"

random_strs = [
    b'my encryption method',
    b'is absolutely impenetrable',
    b'and you will never',
    b'ever',
    b'break it'
]

included_or_not = [False, True]
included_or_not_table = itertools.product(included_or_not, repeat=len(random_strs))

for case in included_or_not_table:
    ctxt = multi_encrypt(enc_flag, case)
    print(f"{case}:\t\t{encrypt(ctxt[:len(flag_prefix)], flag_prefix)}")
```

Then we run the brute force script

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/XtraORdinary]
└─$ ./brute_force.py
(False, False, False, False, False):            b"'\x0c\x16Z\x14XXg"
(False, False, False, False, True):             b'E~s;\x7fx1\x13'
(False, False, False, True, False):             b'Bzs(q.=\x15'
(False, False, False, True, True):              b' \x08\x16I\x1a\x0eTa'
(False, False, True, False, False):             b'Fbrzm7-G'
(False, False, True, False, True):              b'$\x10\x17\x1b\x06\x17D3'
(False, False, True, True, False):              b'#\x14\x17\x08\x08AH5'
(False, False, True, True, True):               b'Africa!A'                    <-------- Here!
(False, True, False, False, False):             b'N\x7f6;v+7\x0b'
(False, True, False, False, True):              b',\rSZ\x1d\x0b^\x7f'
(False, True, False, True, False):              b'+\tSI\x13]Ry'
(False, True, False, True, True):               b'I{6(x};\r'
(False, True, True, False, False):              b'/\x11R\x1b\x0fDB+'
(False, True, True, False, True):               b'Mc7zdd+_'
(False, True, True, True, False):               b"Jg7ij2'Y"
(False, True, True, True, True):                b'(\x15R\x08\x01\x12N-'
(True, False, False, False, False):             b'Ju6?z;*\x1e'
(True, False, False, False, True):              b'(\x07S^\x11\x1bCj'
(True, False, False, True, False):              b'/\x03SM\x1fMOl'
(True, False, False, True, True):               b'Mq6,tm&\x18'
(True, False, True, False, False):              b'+\x1bR\x1f\x03T_>'
(True, False, True, False, True):               b'Ii7~ht6J'
(True, False, True, True, False):               b'Nm7mf":L'
(True, False, True, True, True):                b',\x1fR\x0c\r\x02S8'
(True, True, False, False, False):              b'#\x06\x16^\x18HEr'
(True, True, False, False, True):               b'Ats?sh,\x06'
(True, True, False, True, False):               b'Fps,}> \x00'
(True, True, False, True, True):                b'$\x02\x16M\x16\x1eIt'
(True, True, True, False, False):               b"Bhr~a'0R"
(True, True, True, False, True):                b' \x1a\x17\x1f\n\x07Y&'
(True, True, True, True, False):                b"'\x1e\x17\x0c\x04QU "
(True, True, True, True, True):                 b'Elrmoq<T'
```

Most of the output is gibberish but on one of the lines we can see the word `Africa!`.  
This is the `secret-key.txt`.

### Get the flag

Now, with this information let's write another script that gives us the flag. The code is mostly the same as above.

```python
#!/usr/bin/env python3

def encrypt(ptxt, key):
    ctxt = b''
    for i in range(len(ptxt)):
        a = ptxt[i]
        b = key[i % len(key)]
        ctxt += bytes([a ^ b])
    return ctxt
    
def multi_encrypt(ctxt, included_or_not):
    for idx, to_xor in enumerate(included_or_not):
        if to_xor:
            ctxt = encrypt(ctxt, random_strs[idx])
    return ctxt

with open("output.txt") as f:
    enc_flag = bytes.fromhex(f.read())

random_strs = [
    b'my encryption method',
    b'is absolutely impenetrable',
    b'and you will never',
    b'ever',
    b'break it'
]

# From the brute force output
correct_included_or_not = (False, False, True, True, True)
secret = b'Africa!'

ctxt = multi_encrypt(enc_flag, correct_included_or_not)
flag = encrypt(ctxt, secret)
print(flag.decode())
```

Finally, time to get the plain text flag by running the `get_flag.py` script

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/XtraORdinary]
└─$ ./get_flag.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [itertools module - Python](https://docs.python.org/3/library/itertools.html)
- [itertools.product - Python](https://docs.python.org/3/library/itertools.html#itertools.product)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [XOR cipher - Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher)
