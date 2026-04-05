# Mob psycho

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2024, Forensics, browser_webshell_solvable, apk
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: NGIRIMANA SCHADRACK

Description:
Can you handle APKs?

Download the android apk here.

Hints:
1. Did you know you can unzip APK files?
2. Now you have the whole host of shell tools for searching these files.
```

Challenge link: [https://play.picoctf.org/practice/challenge/420](https://play.picoctf.org/practice/challenge/420)

## Solution

### Unpacking and basic analysis

We start by unpacking the [apk-file](https://en.wikipedia.org/wiki/Apk_(file_format)) with `unzip`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Mob_psycho]
└─$ unzip mobpsycho.apk 
Archive:  mobpsycho.apk
   creating: res/
   creating: res/anim/
  inflating: res/anim/abc_fade_in.xml  
  inflating: res/anim/abc_fade_out.xml  
  inflating: res/anim/abc_grow_fade_in_from_bottom.xml  
  inflating: res/anim/abc_popup_enter.xml  
  inflating: res/anim/abc_popup_exit.xml  
<---snip--->
```

Let's be optimistic and `grep` for the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Mob_psycho]
└─$ grep -iR picoCTF *  

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Mob_psycho]
└─$ strings mobpsycho.apk | grep flag   
res/color/flag.txtUT
res/color/flag.txtUT

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Mob_psycho]
└─$ cat res/color/flag.txt                                     
7069636f4354467b6178386d433052553676655f4e5838356c346178386d436c5f61336562356163327d
```

This looks like hex-encoding.

### Get the flag

Decode with `xxd` like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Mob_psycho]
└─$ cat res/color/flag.txt | xxd -r -p
picoCTF{<REDACTED>}   
```

For additional information, please see the references below.

## References

- [apk (file format) - Wikipedia](https://en.wikipedia.org/wiki/Apk_(file_format))
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
