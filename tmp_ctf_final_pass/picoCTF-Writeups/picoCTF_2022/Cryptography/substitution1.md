# substitution1

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Cryptography, Substitution_cipher
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG
 
Description:
A second message has come in the mail, and it seems almost identical to the first one. 
Maybe the same thing will work again.

Download the message here.

Hints:
1. Try a frequency attack
2. Do the punctuation and the individual words help you make any substitutions?
```

Challenge link: [https://play.picoctf.org/practice/challenge/308](https://play.picoctf.org/practice/challenge/308)

## Solution

The message we were given looks like this (with line breaks added)

```text
WYHg (gzray hra wimybas yzs hvij) ias i yums rh wrombysa gswbakyu wromsykykrl. Wrlysgyilyg ias 
masgslysn dkyz i gsy rh wzivvsljsg dzkwz ysgy yzska wasiykxkyu, yswzlkwiv (iln jrrjvklj) gckvvg, 
iln marqvso-grvxklj iqkvkyu. Wzivvsljsg bgbivvu wrxsa i lboqsa rh wiysjraksg, iln dzsl grvxsn, 
siwz uksvng i gyaklj (wivvsn i hvij) dzkwz kg gbqokyysn yr il rlvkls gwraklj gsaxkws. WYHg ias 
i jasiy diu yr vsial i dkns iaaiu rh wrombysa gswbakyu gckvvg kl i gihs, vsjiv slxkarlosly, iln 
ias zrgysn iln mviusn qu oilu gswbakyu jarbmg iarbln yzs dravn hra hbl iln maiwykws. 
Hra yzkg marqvso, yzs hvij kg: mkwrWYH{HA3FB3LWU_4774WC5_4A3_W001_7II384QW}
```

Compared to the [previous challenge](substitution0.md) there is no key this time.

Let's use [quipqiup](https://quipqiup.com/) to solve this as before.

Input the entire message in the `Puzzle` text field and press `Solve` (with the default setting).

After a short while, you have the flag at the top of the possible solutions.

For additional information, please see the references below.

## References

- [Frequency analysis - Wikipedia](https://en.wikipedia.org/wiki/Frequency_analysis)
- [Letter frequency - Wikipedia](https://en.wikipedia.org/wiki/Letter_frequency)
- [Quipqiup - A fast and automated cryptogram solver](https://quipqiup.com/)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
