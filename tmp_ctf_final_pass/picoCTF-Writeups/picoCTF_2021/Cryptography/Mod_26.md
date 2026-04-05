# Mod 26

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: PANDU

Description:
Cryptography can be easy, do you know what ROT13 is? 

cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_GYpXOHqX}

Hints:
1. This can be solved online if you don't want to do it by hand!
```

Challenge link: [https://play.picoctf.org/practice/challenge/144](https://play.picoctf.org/practice/challenge/144)

## Solution

There are several ways to solve this challenge and here are some of them.

### CyberChef solution

As the hint suggested you can use an online site such as [CyberChef](https://gchq.github.io/CyberChef/) and use the 'ROT13' recipe.

Enter 'rot13' in the `Operations` search bar, then drag and drop it to the `Recipe`.  
Copy the scrambled flag to the `Input` pane and press `BAKE`.

### Use a rot13 commandline tool in Linux

There are at least two sets of packages that contains prepacked `rot13` tools:

- [hxtools](https://manpages.debian.org/testing/hxtools/hxtools.7.en.html)
- [bsdgames](https://wiki.linuxquestions.org/wiki/BSD_games)

Install them with either `sudo apt install hxtools` or `sudo apt install bsdgames`.

The tool from `hxtools` installs as `/usr/bin/rot13` and is a script that invokes the `tr` command more or less as described below.

The tool from `bsdgames` installs as `/usr/games/rot13` and calls the `caesar` tool (which is also included in the package) but with a rotation of 13.

After one of these tools have been installed you can run

```bash
┌──(kali㉿kali)-[~]
└─$ echo "cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_GYpXOHqX}" | rot13
picoCTF{next_time_<REDACTED>}
```

### Use the tr tool in Linux

Alternatively, you can use the `tr` tool to "manually" do the decoding

```bash
┌──(kali㉿kali)-[~]
└─$ echo "cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_GYpXOHqX}" | tr 'A-Za-z' 'N-ZA-Mn-za-m'
picoCTF{next_time_<REDACTED>}
```

For additional information, please see the references below.

## References

- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Modulo - Wikipedia](https://en.wikipedia.org/wiki/Modulo)
- [ROT13 - Wikipedia](https://en.wikipedia.org/wiki/ROT13)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
