# No Padding, No Problem

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SARA

Description:
Oracles can be your best friend, they will decrypt anything, except the flag's ciphertext. 
How will you break it? 

Connect with nc mercury.picoctf.net 30048.

Hints:
1. What can you do with a different pair of ciphertext and plaintext? What if it is not so different after all...
```

Challenge link: [https://play.picoctf.org/practice/challenge/154](https://play.picoctf.org/practice/challenge/154)

## Solution

### Analyse the setup

Let's connect to the site with netcat and see what happens

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/No_Padding_No_Problem]
└─$ nc mercury.picoctf.net 30048
Welcome to the Padding Oracle Challenge
This oracle will take anything you give it and decrypt using RSA. It will not accept the ciphertext with the secret message... Good Luck!

n: 149339794643219776077507050405585800845650154445400997027282558261918295181128401226665909551062604507468313330240617458499797395756032257155674839996486166516574545027201628500180024314440747340226186308811844919358885944726556185137705076462329277448848067069859899665734103028988264186236265809461347046493
e: 65537
ciphertext: 143460509784335709807803862663710668614379682004103390639942551254527693255198262983067608505864006052197653296007286997320606631898297980440703747924065571216199654925519056776308507470427426932198403030157804868066287076368897153026433903487170719574584574181941166414723596175704184610511174191375169426606

Give me ciphertext to decrypt: ^C
```

We get:

- the modulus number `n`
- the public key exponent `e` and
- the cipher text `ciphertext`.

Remember that unpadded RSA is [Homomorphic](https://en.wikipedia.org/wiki/Homomorphic_encryption). This means that for two messages `m1` and `m2`:  

`encrypt(m1) * encrypt(m2) = ((m1**e) * (m2**e)) mod n = (m1 * m2)**e mod n = encrypt(m1 * m2)`

Now if we select `m2` to be just the message `2` and call `encrypt(m1)` by the name `c`:

`c * encrypt(2) = encrypt(m1 * 2)`

We already have `c` and can calculate `encrypt(2)`. If we ask for `c * encrypt(2)` to be decrypted we can divide the result by 2 to get `m1` which is the flag.

### Solve with pwntools

Let's write a script with the help of [pwntools](https://docs.pwntools.com/en/stable/index.html)

```python
#!/usr/bin/python

from pwn import *

SERVER = 'mercury.picoctf.net'
PORT = 30048

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "warning"

io = remote(SERVER, PORT)
io.recvuntil(b"Good Luck!\n\n\n")

# Get the crypto values
n = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"n: {n}")
e = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"e: {e}")
c = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"c: {c}")

# Create msg to send
M2 = 2
ENC_M2 = pow(M2, e, n)
Msg_to_send = str(c*ENC_M2).encode('ascii')
log.info(f"Msg to send: {Msg_to_send}")
io.sendlineafter(b"Give me ciphertext to decrypt: ", bytes(Msg_to_send))

# Get the flag
m1 = int(io.recvlineS(keepends = False).split(':')[1].strip()) // M2
log.info(f"m1: {m1}")
print(bytearray.fromhex(format(m1, 'x')).decode())

io.close()
```

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Cryptography/No_Padding_No_Problem]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
picoCTF{m4yb3_<REDACTED>}
```

For additional information, please see the references below.

## References

- [Homomorphic encryption - Wikipedia](https://en.wikipedia.org/wiki/Homomorphic_encryption)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [RSA (cryptosystem) - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
- [The RSA Cryptosystem - Concepts](https://cryptobook.nakov.com/asymmetric-key-ciphers/the-rsa-cryptosystem-concepts)
