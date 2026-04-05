# Easy Peasy

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MADSTACKS

Description:
A one-time pad is unbreakable, but can you manage to recover the flag? 
(Wrap with picoCTF{}) 

nc mercury.picoctf.net 20266 otp.py

Hints:
1. Maybe there's a way to make this a 2x pad.
```

Challenge link: [https://play.picoctf.org/practice/challenge/125](https://play.picoctf.org/practice/challenge/125)

## Solution

### Analysing the Python script

Let's start with analysing the Python script

```python
#!/usr/bin/python3 -u
import os.path

KEY_FILE = "key"
KEY_LEN = 50000
FLAG_FILE = "flag"


def startup(key_location):
    flag = open(FLAG_FILE).read()
    kf = open(KEY_FILE, "rb").read()

    start = key_location
    stop = key_location + len(flag)

    key = kf[start:stop]
    key_location = stop

    result = list(map(lambda p, k: "{:02x}".format(ord(p) ^ k), flag, key))
    print("This is the encrypted flag!\n{}\n".format("".join(result)))

    return key_location

def encrypt(key_location):
    ui = input("What data would you like to encrypt? ").rstrip()
    if len(ui) == 0 or len(ui) > KEY_LEN:
        return -1

    start = key_location
    stop = key_location + len(ui)

    kf = open(KEY_FILE, "rb").read()

    if stop >= KEY_LEN:
        stop = stop % KEY_LEN
        key = kf[start:] + kf[:stop]
    else:
        key = kf[start:stop]
    key_location = stop

    result = list(map(lambda p, k: "{:02x}".format(ord(p) ^ k), ui, key))

    print("Here ya go!\n{}\n".format("".join(result)))

    return key_location


print("******************Welcome to our OTP implementation!******************")
c = startup(0)
while c >= 0:
    c = encrypt(c)
```

After printing a welcome message the script prints the encrypted flag for us. Then it asks for data, encrypts it and prints it for us.

The one-time pad/key is used until it gets to `KEY_LEN` (that is 50 000) and then it wraps around.  
This means that the same key will be re-used.

So we have the encrypted flag which is `Flag ^ Key` (^ is short for XOR). If we use the encrypted flag as input we will get the plain text flag since `(Flag ^ Key) ^ Key = Flag`.
Or more generally, `(M ^ K) ^ K = M` since `K ^ K = 0`. If you XOR something with itself, the operations cancel each other out.

### Create an exploit script

Let's write a script with the help of [pwntools](https://docs.pwntools.com/en/stable/index.html)

```python
#!/usr/bin/python

from pwn import *

SERVER = 'mercury.picoctf.net'
PORT = 20266

KEY_LEN = 50000
CHUNK_SIZE = 2500

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "warning"

# Get the encrypted flag
io = remote(SERVER, PORT)
io.recvuntil(b"This is the encrypted flag!\n")
enc_flag = io.recvlineS(keepends = False)
log.info(f"Encrypted flag: {enc_flag}")
bin_flag = unhex(enc_flag)

# Cause wrap around
data_left = KEY_LEN - len(bin_flag)
while data_left > 0:
    log.debug(f"{data_left} bytes left to send")
    chunk_size = min(data_left, CHUNK_SIZE)
    io.sendlineafter(b"What data would you like to encrypt? ", b"Q" * chunk_size)
    data_left -= chunk_size

# Send encrypted flag
io.sendlineafter(b"What data would you like to encrypt? ", bin_flag)
here_you_go = io.recvlineS()
flag = unhex(io.recvlineS()).decode()

print(f"Plain text flag: picoCTF{{{flag}}}")
io.close()
```

First, I get the encrypted flag. Then I cause a wrap aroung by keep sending data until `KEY_LEN` bytes are sent.  
Finally I send the encrypted flag to receive the plain text one.

With `log_level` I change the verbosity of the script. When finished I set it to `warning` to minimize the output.

Running the script from my pwntools virtual environment finally looks like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/Easy_Peasy]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
Plain text flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Exclusive or - Wikipedia](https://en.wikipedia.org/wiki/Exclusive_or)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Stream cipher attacks - Reused key attack - Wikipedia](https://en.wikipedia.org/wiki/Stream_cipher_attacks#Reused_key_attack)
- [Stream Cipher Reuse: A Graphic Example](https://cryptosmith.com/2008/05/31/stream-reuse/)
- [Taking advantage of one-time pad key reuse?](https://crypto.stackexchange.com/questions/59/taking-advantage-of-one-time-pad-key-reuse)
- [XOR cipher - Wikipedia](https://en.wikipedia.org/wiki/XOR_cipher)
