# la cifra de

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: ALEX FULTON/DANIEL TUNITIS
 
Description:
I found this cipher in an old book. Can you figure out what it says? 

Connect with nc jupiter.challenges.picoctf.org 32411.

Hints:
1. There are tools that make this easy.
2. Perhaps looking at history will help
```

Challenge link: [https://play.picoctf.org/practice/challenge/3](https://play.picoctf.org/practice/challenge/3)

## Solution

### Connect to the server to get the cipher text

Let's start by connecting to the server with netcat and see what we get

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Cryptography/La_cifra_de]
└─$ nc jupiter.challenges.picoctf.org 32411
Encrypted message:
Ne iy nytkwpsznyg nth it mtsztcy vjzprj zfzjy rkhpibj nrkitt ltc tnnygy ysee itd tte cxjltk

Ifrosr tnj noawde uk siyyzre, yse Bnretèwp Cousex mls hjpn xjtnbjytki xatd eisjd

Iz bls lfwskqj azycihzeej yz Brftsk ip Volpnèxj ls oy hay tcimnyarqj dkxnrogpd os 1553 my Mnzvgs Mazytszf Merqlsu ny hox moup Wa inqrg ipl. Ynr. Gotgat Gltzndtg Gplrfdo 

Ltc tnj tmvqpmkseaznzn uk ehox nivmpr g ylbrj ts ltcmki my yqtdosr tnj wocjc hgqq ol fy oxitngwj arusahje fuw ln guaaxjytrd catizm tzxbkw zf vqlckx hizm ceyupcz yz tnj fpvjc hgqqpohzCZK{m311a50_0x_a1rn3x3_h1ah3x7g996649}

Ehk ktryy herq-ooizxetypd jjdcxnatoty ol f aordllvmlbkytc inahkw socjgex, bls sfoe gwzuti 1467 my Rjzn Hfetoxea Gqmexyt.

Tnj Gimjyèrk Htpnjc iy ysexjqoxj dosjeisjd cgqwej yse Gqmexyt Doxn ox Fwbkwei Inahkw.

Tn 1508, Ptsatsps Zwttnjxiax tnbjytki ehk xz-cgqwej ylbaql rkhea (g rltxni ol xsilypd gqahggpty) ysaz bzuri wazjc bk f nroytcgq nosuznkse ol yse Bnretèwp Cousex.

Gplrfdo’y xpcuso butvlky lpvjlrki tn 1555 gx l cuseitzltoty ol yse lncsz. Yse rthex mllbjd ol yse gqahggpty fce tth snnqtki cemzwaxqj, bay ehk fwpnfmezx lnj yse osoed qptzjcs gwp mocpd hd xegsd ol f xnkrznoh vee usrgxp, wnnnh ify bk itfljcety hizm paim noxwpsvtydkse.
```

Ah, it looks like a [substitution cipher](https://en.wikipedia.org/wiki/Substitution_cipher) and based on the amount of text displayed it is probably a [Vigenère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher). At fairly long text is usually needed to do [frequency analysis](https://en.wikipedia.org/wiki/Frequency_analysis) of the characters.

### Use an online service

We can use an online service such as [Guballa](https://www.guballa.de/vigenere-solver) to solve this.  
Copy the text to the `Cipher Text` text box and leave the settings as default.  
Then press the `Break Cipher` button.

You need to scroll down a bit in the decoded text to find the flag.

For additional information, please see the references below.

## References

- [Frequency analysis - Wikipedia](https://en.wikipedia.org/wiki/Frequency_analysis)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
- [Vigenère cipher - Wikipedia](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)
- [Vigenère Solver - Guballa](https://www.guballa.de/vigenere-solver)
