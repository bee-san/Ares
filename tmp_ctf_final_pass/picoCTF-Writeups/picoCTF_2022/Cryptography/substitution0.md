# substitution0

- [Challenge information](#challenge-information)
- [Quipqiup solution](#quipqiup-solution)
- [Python solution](#python-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography, Substitution
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG

Description:
A message has come in but it seems to be all scrambled. Luckily it seems to have the key at the beginning. 
Can you crack this substitution cipher?

Download the message here.

Hints:
1. Try a frequency attack. An online tool might help.
```

Challenge link: [https://play.picoctf.org/practice/challenge/307](https://play.picoctf.org/practice/challenge/307)

The message we were given looks like this

```text
QWITJSYHXCNDFERMUKGOPVALBZ 

Hjkjpmre Djykqet qkrgj, axoh q ykqvj qet goqojdb qxk, qet wkrpyho fj ohj wjjodj
skrf q ydqgg iqgj xe ahxih xo aqg jeidrgjt. Xo aqg q wjqpoxspd giqkqwqjpg, qet, qo
ohqo oxfj, penerae or eqopkqdxgog—rs irpkgj q ykjqo mkxzj xe q gixjeoxsxi mrxeo
rs vxja. Ohjkj ajkj oar krpet wdqin gmrog ejqk rej jlokjfxob rs ohj wqin, qet q
drey rej ejqk ohj rohjk. Ohj giqdjg ajkj jlijjtxeydb hqkt qet ydrggb, axoh qdd ohj
qmmjqkqeij rs wpkexghjt yrdt. Ohj ajxyho rs ohj xegjio aqg vjkb kjfqknqwdj, qet,
oqnxey qdd ohxeyg xeor iregxtjkqoxre, X irpdt hqktdb wdqfj Cpmxojk srk hxg rmxexre
kjgmjioxey xo.

Ohj sdqy xg: mxirIOS{5PW5717P710E_3V0DP710E_03055505}
```

The description conveniently tells as that the first line is a decryption key.  
So 'Q' decrypts to 'A', 'W' to 'B', etc.

Then we have a body of text, large enough for us to use letter frequency analysis should we need to.

And at the last line we have what looks like our encrypted flag.

There probably are more ways to solve this challenge, but here are two solutions.

## Quipqiup solution

As suggested in the hint we can use an online tool such as [quipqiup](https://quipqiup.com/) to solve this.

Input the entire message in the `Puzzle` text field and press `Solve` (with the default setting).

After a short while, you have the flag at the top of the possible solutions.

## Python solution

Alternatively, we can write a small Python script to solve this.  
There is no need to use frequency analysis or brute force since we have the key.

Let's create a script called `solve.py`

```python
#!/usr/bin/python
# -*- coding: latin-1 -*-

import string

encrypted_msg = """Hjkjpmre Djykqet qkrgj, axoh q ykqvj qet goqojdb qxk, qet wkrpyho fj ohj wjjodj
skrf q ydqgg iqgj xe ahxih xo aqg jeidrgjt. Xo aqg q wjqpoxspd giqkqwqjpg, qet, qo
ohqo oxfj, penerae or eqopkqdxgog—rs irpkgj q ykjqo mkxzj xe q gixjeoxsxi mrxeo
rs vxja. Ohjkj ajkj oar krpet wdqin gmrog ejqk rej jlokjfxob rs ohj wqin, qet q
drey rej ejqk ohj rohjk. Ohj giqdjg ajkj jlijjtxeydb hqkt qet ydrggb, axoh qdd ohj
qmmjqkqeij rs wpkexghjt yrdt. Ohj ajxyho rs ohj xegjio aqg vjkb kjfqknqwdj, qet,
oqnxey qdd ohxeyg xeor iregxtjkqoxre, X irpdt hqktdb wdqfj Cpmxojk srk hxg rmxexre
kjgmjioxey xo.

Ohj sdqy xg: mxirIOS{5PW5717P710E_3V0DP710E_03055505}"""

key = "QWITJSYHXCNDFERMUKGOPVALBZ"
alphabet = string.ascii_uppercase

decrypted_msg = ""
for c in encrypted_msg:
    if c.isupper():
        decrypted_msg += alphabet[key.index(c)]
    elif c.islower():
        decrypted_msg += alphabet[key.index(c.upper())].lower()
    else:
        decrypted_msg += c

print(decrypted_msg)
```

Then make the script executable and run it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Substitution0]
└─$ chmod +x solve.py

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Cryptography/Substitution0]
└─$ ./solve.py
Hereupon Legrand arose, with a grave and stately air, and brought me the beetle
from a glass case in which it was enclosed. It was a beautiful scarabaeus, and, at
that time, unknown to naturalistsof course a great prize in a scientific point
of view. There were two round black spots near one extremity of the back, and a
long one near the other. The scales were exceedingly hard and glossy, with all the
appearance of burnished gold. The weight of the insect was very remarkable, and,
taking all things into consideration, I could hardly blame Jupiter for his opinion
respecting it.

The flag is: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Frequency analysis - Wikipedia](https://en.wikipedia.org/wiki/Frequency_analysis)
- [Letter frequency - Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Quipqiup - A fast and automated cryptogram solver](https://quipqiup.com/)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
