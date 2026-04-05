# Play Nice

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2021, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MADSTACKS

Description:
Not all ancient ciphers were so bad... 
The flag is not in standard format. 

nc mercury.picoctf.net 6057 
playfair.py

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/114](https://play.picoctf.org/practice/challenge/114)

## Solution

The name of the source code file reveals that this is the [Playfair cipher](https://en.wikipedia.org/wiki/Playfair_cipher).  

### Analyze the source code

Let's start by looking at the python source (with some empty lines removed).

```python
#!/usr/bin/python3 -u
import signal

SQUARE_SIZE = 6

def generate_square(alphabet):
    assert len(alphabet) == pow(SQUARE_SIZE, 2)
    matrix = []
    for i, letter in enumerate(alphabet):
        if i % SQUARE_SIZE == 0:
            row = []
        row.append(letter)
        if i % SQUARE_SIZE == (SQUARE_SIZE - 1):
            matrix.append(row)
    return matrix

def get_index(letter, matrix):
    for row in range(SQUARE_SIZE):
        for col in range(SQUARE_SIZE):
            if matrix[row][col] == letter:
                return (row, col)
    print("letter not found in matrix.")
    exit()

def encrypt_pair(pair, matrix):
    p1 = get_index(pair[0], matrix)
    p2 = get_index(pair[1], matrix)

    if p1[0] == p2[0]:
        return matrix[p1[0]][(p1[1] + 1)  % SQUARE_SIZE] + matrix[p2[0]][(p2[1] + 1)  % SQUARE_SIZE]
    elif p1[1] == p2[1]:
        return matrix[(p1[0] + 1)  % SQUARE_SIZE][p1[1]] + matrix[(p2[0] + 1)  % SQUARE_SIZE][p2[1]]
    else:
        return matrix[p1[0]][p2[1]] + matrix[p2[0]][p1[1]]

def encrypt_string(s, matrix):
    result = ""
    if len(s) % 2 == 0:
        plain = s
    else:
        plain = s + "meiktp6yh4wxruavj9no13fb8d027c5glzsq"[0]
    for i in range(0, len(plain), 2):
        result += encrypt_pair(plain[i:i + 2], matrix)
    return result

alphabet = open("key").read().rstrip()
m = generate_square(alphabet)
msg = open("msg").read().rstrip()
enc_msg = encrypt_string(msg, m)
print("Here is the alphabet: {}\nHere is the encrypted message: {}".format(alphabet, enc_msg))
signal.alarm(18)
resp = input("What is the plaintext message? ").rstrip()
if resp and resp == msg:
    print("Congratulations! Here's the flag: {}".format(open("flag").read()))

# https://en.wikipedia.org/wiki/Playfair_cipher
```

We can see that:

- This is a non-standard playfair cipher with a 6x6-matrix, rather than the 5x5 standard one.
- The most interesting function is the `encrypt_pair` function which implements the three encoding cases "same row", "same column" and "rectangle case" as described in the Wikipedia article.

The `-u` parameter in the [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)) forces stdin, stdout and stderr to be totally unbuffered.

### Connect to the site

Next we connect to the site to get the alphabet and encrypted message

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Play_Nice]
└─$ nc mercury.picoctf.net 6057
Here is the alphabet: meiktp6yh4wxruavj9no13fb8d027c5glzsq
Here is the encrypted message: y7bcvefqecwfste224508y1ufb21ld
What is the plaintext message? 
```

If we don't give an answer within 18 seconds the connection ends. This is due to the alarm (`signal.alarm(18)`) going off.

### Online solution

We can solve this online at [dCode.fr](https://www.dcode.fr/playfair-cipher).

1. Set `y7bcvefqecwfste224508y1ufb21ld` as `PlayFair ciphertext`
2. Set grid size as 6x6 and press `RESIZE`
3. Set `meiktp6yh4wxruavj9no13fb8d027c5glzsq` as alphabet under the matrix
4. Click `DECRYPT PLAYFAIR`

The result is uppercase letters and the site expects lowercase letters so we need to convert them is an interactive python session or something similar

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Play_Nice]
└─$ python                   
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 'WD9BUKBSPDTJ7SKD3KL8D6OA3F03G0'.lower()
'wd9bukbspdtj7skd3kl8d6oa3f03g0'
>>> exit()
```

Then we can connect to the site again to get the flag. Both the alphabet and encrypted message seems to be static (don't change).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Play_Nice]
└─$ nc mercury.picoctf.net 6057
Here is the alphabet: meiktp6yh4wxruavj9no13fb8d027c5glzsq
Here is the encrypted message: y7bcvefqecwfste224508y1ufb21ld
What is the plaintext message? wd9bukbspdtj7skd3kl8d6oa3f03g0
Congratulations! Here's the flag: <REDACTED>
```

### Write python decoder

Alternatively, we can write a Python script to decode the playfair cipher and use [pwntools](https://docs.pwntools.com/en/stable/index.html) to automate the connection to the site and send/receive input/output.

The total script looks like this

```python
#!/usr/bin/python3

from pwn import *

SQUARE_SIZE = 6

SERVER = 'mercury.picoctf.net'
PORT = 6057

def generate_square(alphabet):
    assert len(alphabet) == pow(SQUARE_SIZE, 2)
    matrix = []
    for i, letter in enumerate(alphabet):
        if i % SQUARE_SIZE == 0:
            row = []
        row.append(letter)
        if i % SQUARE_SIZE == (SQUARE_SIZE - 1):
            matrix.append(row)
    return matrix

def get_index(letter, matrix):
    for row in range(SQUARE_SIZE):
        for col in range(SQUARE_SIZE):
            if matrix[row][col] == letter:
                return (row, col)
    print("letter not found in matrix.")
    exit()

def decrypt_pair(pair, matrix):
    p1 = get_index(pair[0], matrix)
    p2 = get_index(pair[1], matrix)

    if p1[0] == p2[0]:
        return matrix[p1[0]][(p1[1] - 1)  % SQUARE_SIZE] + matrix[p2[0]][(p2[1] - 1)  % SQUARE_SIZE]
    elif p1[1] == p2[1]:
        return matrix[(p1[0] - 1)  % SQUARE_SIZE][p1[1]] + matrix[(p2[0] - 1)  % SQUARE_SIZE][p2[1]]
    else:
        return matrix[p1[0]][p2[1]] + matrix[p2[0]][p1[1]]

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "info"

io = remote(SERVER, PORT)

alphabet = io.recvlineS(keepends = False).split(':')[1].strip()
log.info(f"Alphabet: {alphabet}")
matrix = generate_square(alphabet)

enc_msg = io.recvlineS(keepends = False).split(':')[1].strip()
log.info(f"Encrypted msg: {enc_msg}")

plain = ""
for i in range(0, len(enc_msg), 2):
    plain += decrypt_pair(enc_msg[i:i + 2] ,matrix)
io.sendlineafter(b'What is the plaintext message? ', bytearray(plain, 'utf8'))

flag = io.recvlineS(keepends = False).split(':')[1].strip()
print(f"Flag: {flag}")
```

The functions `generate_square` and `get_index` are re-used unchanged.  
The `decrypt_pair` is almost exactly as the `encrypt_pair` but with subtraction instead of addition.

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Play_Nice]
└─$ ~/python_venvs/pwntools/bin/python solve.py
[+] Opening connection to mercury.picoctf.net on port 6057: Done
[*] Alphabet: meiktp6yh4wxruavj9no13fb8d027c5glzsq
[*] Encrypted msg: y7bcvefqecwfste224508y1ufb21ld
Flag: <REDACTED>
[*] Closed connection to mercury.picoctf.net port 6057
```

For additional information, please see the references below.

## References

- [dCode.fr - Homepage](https://www.dcode.fr/en)
- [dCode.fr - PlayFair Cipher](https://www.dcode.fr/playfair-cipher)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Playfair cipher - Wikipedia](https://en.wikipedia.org/wiki/Playfair_cipher)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Shebang (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Shebang_(Unix))
