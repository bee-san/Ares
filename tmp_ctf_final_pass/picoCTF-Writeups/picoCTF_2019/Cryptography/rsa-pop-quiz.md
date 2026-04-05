# rsa-pop-quiz

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WPARKS/NMONTIERTH

Description:
Class, take your seats! It's PRIME-time for a quiz... 

nc jupiter.challenges.picoctf.org 58617
 
Hints:
1. RSA info
```

Challenge link: [https://play.picoctf.org/practice/challenge/61](https://play.picoctf.org/practice/challenge/61)

## Solution

I wrote a Python script that uses [pwntools](https://docs.pwntools.com/en/stable/index.html) to automate this

```python
#!/usr/bin/python

from pwn import *

SERVER = 'jupiter.challenges.picoctf.org'
PORT = 58617

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "warning"

io = remote(SERVER, PORT)

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce n from p and q
log.info("New problem - Produce n from p and q")
q = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"q: {q}")
p = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"p: {p}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# n = p*q
n = str(p * q)
log.info(f"Sending n: {n}")
io.sendlineafter(b'n:', n.encode('ascii'))

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce q from p and n
log.info("New problem - Produce q from p and n")
p = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"p: {p}")
n = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"n: {n}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# q = n/p
q = str(n // p)
log.info(f"Sending q: {q}")
io.sendlineafter(b'q:', q.encode('ascii'))

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce q and p from e and n
log.info("New problem - Produce q and p from e and n")
# Feasible? No, not generally
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'N')

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce totient(n) from p and q
log.info("New problem - Produce totient(n) from p and q")
q = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"q: {q}")
p = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"p: {p}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# tot(n) = (p-1) * (q-1)
tot = str((p-1) * (q-1))
log.info(f"Sending tot: {tot}")
io.sendlineafter(b'totient(n):', tot.encode('ascii'))

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce ciphertext from plaintext, e and n
log.info("New problem - Produce ciphertext from plaintext, e and n")
plain = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"plain: {plain}")
e = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"e: {e}")
n = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"n: {n}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# cipher = plain ** e mod n
cipher = str(pow(plain, e, n))
log.info(f"Sending cipher: {cipher}")
io.sendlineafter(b'ciphertext:', cipher.encode('ascii'))

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce plaintext from ciphertext, e and n
log.info("New problem - Produce plaintext from ciphertext, e and n")
# Feasible? No, not generally
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'N')

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce d from q, p and e
log.info("New problem - Produce d from q, p and e")
q = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"q: {q}")
p = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"p: {p}")
e = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"e: {e}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# d = mod_inv(e, tot)
tot = (p-1)*(q-1)
d = str(pow(e, -1, tot))
log.info(f"Sending d: {d}")
io.sendlineafter(b'd:', d.encode('ascii'))

io.recvuntil(b"#### NEW PROBLEM ####\n")
# Produce plain from p, cipher, e and n
log.info("New problem - Produce plain from p, cipher, e and n")
p = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"p: {p}")
cipher = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"cipher: {cipher}")
e = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"e: {e}")
n = int(io.recvlineS(keepends = False).split(':')[1].strip())
log.info(f"n: {n}")
# Feasible? Yes
io.sendlineafter(b'IS THIS POSSIBLE and FEASIBLE? (Y/N):', b'Y')
# plain = cipher ** d mod n, where d = mod_inv(e, tot) as before
q = n // p
tot = (p-1)*(q-1)
d = pow(e, -1, tot)
plain = pow(cipher, d, n)
log.info(f"Sending plain: {plain}")
io.sendlineafter(b'plaintext:', str(plain).encode('ascii'))

# Get the flag
flag = bytearray.fromhex(format(plain, 'x')).decode()
print(flag)

io.close()
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Rsa-pop-quiz]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [RSA (cryptosystem) - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
- [The RSA Cryptosystem - Concepts](https://cryptobook.nakov.com/asymmetric-key-ciphers/the-rsa-cryptosystem-concepts)
