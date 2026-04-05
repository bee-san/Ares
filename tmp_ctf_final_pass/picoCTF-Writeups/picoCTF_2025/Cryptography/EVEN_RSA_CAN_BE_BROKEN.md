# EVEN RSA CAN BE BROKEN???

- [Challenge information](#challenge-information)
- [Python Solution](#python-solution)
- [Dcode Solution](#dcode-solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Cryptography, picoCTF 2025, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: Michael Crotty
 
Description:
This service provides you an encrypted flag. Can you decrypt it with just N & e?

Connect to the program with netcat:
$ nc verbal-sleep.picoctf.net 52407

The program's source code can be downloaded here.

Hints:
1. How much do we trust randomness?
2. Notice anything interesting about N?
3. Try comparing N across multiple requests
```

Challenge link: [https://play.picoctf.org/practice/challenge/470](https://play.picoctf.org/practice/challenge/470)

## Python Solution

### Connect to the server

We start by connecting to the server with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/EVEN_RSA_CAN_BE_BROKEN]
└─$ nc verbal-sleep.picoctf.net 52407
N: 15748023202970263626821453336665100444962696631820156062726891050814861859869762569254066360501336828060264464106081187321407123252010969709193831266158174
e: 65537
cyphertext: 9221304625019142707603920008007283199496617149370164612508101702543155427328813011858400828662718006116704496653188095450636020526784062108085707184488773
```

We get:

- The modulus `N`
- The public exponent `e`
- The cyphertext

Running the program multiple times reveals that `N` is always even so `p` must be 2.  
A major mistake in the implementation of prime number selection!

### Analyse the Python source code

Next we analyse the source code

```python
from sys import exit
from Crypto.Util.number import bytes_to_long, inverse
from setup import get_primes

e = 65537

def gen_key(k):
    """
    Generates RSA key with k bits
    """
    p,q = get_primes(k//2)
    N = p*q
    d = inverse(e, (p-1)*(q-1))

    return ((N,e), d)

def encrypt(pubkey, m):
    N,e = pubkey
    return pow(bytes_to_long(m.encode('utf-8')), e, N)

def main(flag):
    pubkey, _privkey = gen_key(1024)
    encrypted = encrypt(pubkey, flag) 
    return (pubkey[0], encrypted)

if __name__ == "__main__":
    flag = open('flag.txt', 'r').read()
    flag = flag.strip()
    N, cypher  = main(flag)
    print("N:", N)
    print("e:", e)
    print("cyphertext:", cypher)
    exit()
```

### Write a solve script

Since `N` is easily factorized we have everything we need to write a solve script in Python with the help of:

- [pwntools](https://docs.pwntools.com/en/stable/index.html) and
- [gmpy2](https://pypi.org/project/gmpy2/)

```python
#!/usr/bin/env python

from pwn import *
from gmpy2 import invert

SERVER = 'verbal-sleep.picoctf.net'
PORT = 52407

# Set output level (critical, error, warning, info, debug)
context.log_level = "warning"

def decrypt(c, p, q, e):
     ph = (p-1) * (q-1)
     d = invert(e, ph)
     n = p * q
     return pow(c, d, n)

# Connect to server and read N, e and the cyphertext
io = remote(SERVER, PORT)
n = int(io.recvlineS().split(':')[1].strip())
log.info(f"N: {n}")
e = int(io.recvlineS().split(':')[1].strip())
log.info(f"e: {e}")
ct = int(io.recvlineS().split(':')[1].strip())
log.info(f"Cyphertext: {ct}")
io.close()

# "Factorize" N into p and q
p = 2
q = n // p
log.info(f"q: {q}")

# Decrypt the flag
flag = decrypt(ct, p, q, e)
print(bytes.fromhex(format(flag, 'x')).decode())
```

### Get the flag

Finally, we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/EVEN_RSA_CAN_BE_BROKEN]
└─$ source ~/Python_venvs/PwnTools/bin/activate

┌──(PwnTools)─(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/EVEN_RSA_CAN_BE_BROKEN]
└─$ ./get_flag.py
picoCTF{<REDACTED>}
```

## Dcode Solution

Alternatively, we can solve the challenge with [dcode.fr's RSA Cipher decoder](https://www.dcode.fr/rsa-cipher).

Just enter the information we get from the program (N, e and the cyphertext), leave the rest of the settings as-is and press `Calculate/Decrypt` to get the flag.

For additional information, please see the references below.

## References

- [gmpy2 - PyPI Module](https://pypi.org/project/gmpy2/)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Prime number - Wikipedia](https://en.wikipedia.org/wiki/Prime_number)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [RSA (cryptosystem) - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
- [The RSA Cryptosystem - Concepts](https://cryptobook.nakov.com/asymmetric-key-ciphers/the-rsa-cryptosystem-concepts)
