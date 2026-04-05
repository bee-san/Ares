# miniRSA

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SPEEEDAY/DANNY

Description:
Let's decrypt this: ciphertext? Something seems a bit small.

Hints:
1. RSA tutorial
2. How could having too small an e affect the security of this 2048 bit key?
3. Make sure you don't lose precision, the numbers are pretty big (besides the e value)
```

Challenge link: [https://play.picoctf.org/practice/challenge/73](https://play.picoctf.org/practice/challenge/73)

## Solution

### Analyse the setup

Let's start by analysing what we have

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/miniRSA]
└─$ cat ciphertext.txt                                                 

N: 29331922499794985782735976045591164936683059380558950386560160105740343201513369939006307531165922708949619162698623675349030430859547825708994708321803705309459438099340427770580064400911431856656901982789948285309956111848686906152664473350940486507451771223435835260168971210087470894448460745593956840586530527915802541450092946574694809584880896601317519794442862977471129319781313161842056501715040555964011899589002863730868679527184420789010551475067862907739054966183120621407246398518098981106431219207697870293412176440482900183550467375190239898455201170831410460483829448603477361305838743852756938687673
e: 3

ciphertext (c): 2205316413931134031074603746928247799030155221252519872649613686408884798530321139183194114380675760980675288213509494488928149890378350358245536745970253162283534968545300178396900226131454240625540026296473434895830304509610598192929125 
```

So we have a modulus number `N`,  the public key exponent `e`, and the cipher text `c`.

Remember that in RSA `M**3 mod n = c`. We can rewrite this as `M**3 = i*n + c` for some value of `i`.  
This means that `M = iroot(i*n+c, 3)` for some `i`. We just need to find the correct `i` value.

We will use the `iroot` function from [gmpy2 module](https://pypi.org/project/gmpy2/).  
From the [manpage](https://manpages.ubuntu.com/manpages/trusty/man3/gmpy2.3.html)

```text
       iroot(...)
              iroot(x,n) returns a 2-element tuple (y, b) such that y is the integer n-th root of
              x and b is True if the root is exact. x must be >= 0 and n must be > 0.
```

### Write a solve script

This Python script will search for the correct value of `i`

```python
#!/usr/bin/python

from gmpy2 import iroot

# Given in the challenge
N = 29331922499794985782735976045591164936683059380558950386560160105740343201513369939006307531165922708949619162698623675349030430859547825708994708321803705309459438099340427770580064400911431856656901982789948285309956111848686906152664473350940486507451771223435835260168971210087470894448460745593956840586530527915802541450092946574694809584880896601317519794442862977471129319781313161842056501715040555964011899589002863730868679527184420789010551475067862907739054966183120621407246398518098981106431219207697870293412176440482900183550467375190239898455201170831410460483829448603477361305838743852756938687673
e = 3
c = 2205316413931134031074603746928247799030155221252519872649613686408884798530321139183194114380675760980675288213509494488928149890378350358245536745970253162283534968545300178396900226131454240625540026296473434895830304509610598192929125

for i in range(5000):
    m, exact_root = iroot(i*N + c, e)
    if exact_root:
        print(f"Found i: {i}")
        msg = bytes.fromhex(format(m, 'x')).decode()
        print(f"Found Msg: {msg}")
        break
```

### Get the flag

The we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/miniRSA]
└─$ ~/python_venvs/gmpy2/bin/python get_flag.py 
Found i: 0
Found Msg: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [gmpy2 - GitHub](https://github.com/gmpy2/gmpy2)
- [gmpy2 - PyPI](https://pypi.org/project/gmpy2/)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [RSA (cryptosystem) - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
- [The RSA Cryptosystem - Concepts](https://cryptobook.nakov.com/asymmetric-key-ciphers/the-rsa-cryptosystem-concepts)
