# basic-mod1

This is the write-up for the challenge "basic-mod1" challenge in PicoCTF

# The challenge

## Description
We found this weird message being passed around on the servers, we think we have a working decryption scheme.
Download the message here.
Take each number mod 37 and map it to the following character set: 0-25 is the alphabet (uppercase), 26-35 are the decimal digits, and 36 is an underscore.
Wrap your decrypted message in the picoCTF flag format (i.e. picoCTF{decrypted_message})

![](img/succes.png)

## Hints
1. Do you know what mod 37 means?
2. mod 37 means modulo 37. It gives the remainder of a number after being divided by 37.

## Initial look
file named "message.txt" contain encrypted text
# How to solve it
First I followed the description and looked at the file - "message.txt" which contained the following:

```bash
54 211 168 309 262 110 272 73 54 137 131 383 188 332 39 396 370 182 328 327 366 70 
```

I went back with the clues they brought us and said the numbers mod 37 the result:
```bash
17, 26, 20, 13, 3, 36, 13, 36, 17, 26, 20, 13, 3, 36, 1, 32, 1, 28, 31, 31, 29, 27
```

In the description they gave us how to map the letters and numbers, therefore the mapping according to the description will be:
```bash
A: 0
B: 1
C: 2
D: 3
E: 4
F: 5
G: 6
H: 7
I: 8
J: 9
K: 10
L: 11
M: 12
N: 13
O: 14
P: 15
Q: 16
R: 17
S: 18
T: 19
U: 20
V: 21
W: 22
X: 23
Y: 24
Z: 25
0: 26
1: 27
2: 28
3: 29
4: 30
5: 31
6: 32
7: 33
8: 34
9: 35
_: 36
```

With the assignment of the numerical values to their corresponding characters, the resulting sequence was as follows:
```bash
[17, 26, 20, 13, 3, 36, 13, 36, 17, 26, 20, 13, 3, 36, 1, 32, 1, 28, 31, 31, 29, 27]
→ [R, 0, U, N, D, _, N, _, R, 0, U, N, D, _, B, 6, B, 2, 5, 5, 3, 1]
```

Voila!!! 😎

the message is:
```bash
R0UND_N_R0UND_B6B25531
```

The flag is `picoCTF{R0UND_N_R0UND_B6B25531}`

