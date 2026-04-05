# caesar

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SANJAY C/DANIEL TUNITIS

Description:
Decrypt this message.

Hints:
1. caesar cipher tutorial
```

Challenge link: [https://play.picoctf.org/practice/challenge/64](https://play.picoctf.org/practice/challenge/64)

## Solution

If we look at the ciphertext we can see that it is only partially encrypted

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Caesar]
└─$ cat ciphertext
picoCTF{tifjjzexkyvilsztfehnahooda}
```

Only the parts within '{' and '}' are encrypted.

There are several ways to solve this challenge and here are two of them.

### CyberChef solution

We can use [CyberChef](https://gchq.github.io/CyberChef/) and the `ROT13` recipe to solve this.  
Type 'rot13' in the `Operations` search bar, then drag and drop it to the `Recipe` pane.  
Then copy and paste the text within the curly braces (that is `tifjjzexkyvilsztfehnahooda`) to the `Input` pane.  
Neither the standard amount of `13` for `ROT13` or `3`, which is the default `caesar` rotation, makes sense though.  
Select the `Auto Bake` check box and start changing the amount until you can recognize some English words.  
With an `Amount` of `9` you get the correct flag part.

To get the complete flag you need to add 'picoCTF{' and '}' to the decrypted data.

### Use a caesar commandline tool in Linux

The [bsdgames](https://wiki.linuxquestions.org/wiki/BSD_games) package contains a `caesar` commandline tool we can use to solve this. Install the package with `sudo apt install bsdgames`.

Then we can brute force the solution like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/Caesar]
└─$ for i in $(seq 1 25); do echo -n "$i: "; echo 'tifjjzexkyvilsztfehnahooda' | caesar $i; done
1: ujgkkafylzwjmtaugfiobippeb
2: vkhllbgzmaxknubvhgjpcjqqfc
3: wlimmchanbylovcwihkqdkrrgd
4: xmjnndiboczmpwdxjilrelsshe
5: ynkooejcpdanqxeykjmsfmttif
6: zolppfkdqeboryfzlkntgnuujg
7: apmqqglerfcpszgamlouhovvkh
8: bqnrrhmfsgdqtahbnmpvipwwli
9: crossingtherubiconqwjqxxmj                         <--- Correct rotation
10: dspttjohuifsvcjdporxkryynk
11: etquukpivjgtwdkeqpsylszzol
12: furvvlqjwkhuxelfrqtzmtaapm
13: gvswwmrkxlivyfmgsruanubbqn
14: hwtxxnslymjwzgnhtsvbovccro
15: ixuyyotmznkxahoiutwcpwddsp
16: jyvzzpunaolybipjvuxdqxeetq
17: kzwaaqvobpmzcjqkwvyeryffur
18: laxbbrwpcqnadkrlxwzfszggvs
19: mbyccsxqdrobelsmyxagtahhwt
20: nczddtyrespcfmtnzybhubiixu
21: odaeeuzsftqdgnuoazcivcjjyv
22: pebffvatgurehovpbadjwdkkzw
23: qfcggwbuhvsfipwqcbekxellax
24: rgdhhxcviwtgjqxrdcflyfmmby
25: sheiiydwjxuhkrysedgmzgnncz
```

Again, to get the complete flag you need to add 'picoCTF{' and '}' to the decrypted data.

For additional information, please see the references below.

## References

- [caesar - Linux manual page](https://manpages.debian.org/testing/bsdgames/caesar.6.en.html)
- [Caesar cipher - Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Modulo - Wikipedia](https://en.wikipedia.org/wiki/Modulo)
