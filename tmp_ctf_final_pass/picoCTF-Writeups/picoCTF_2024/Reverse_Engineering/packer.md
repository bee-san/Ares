# packer

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Reverse Engineering, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL
 
Description:
Reverse this linux executable?
binary
 
Hints:
1. What can we do to reduce the size of a binary after compiling it.
```

Challenge link: [https://play.picoctf.org/practice/challenge/421](https://play.picoctf.org/practice/challenge/421)

## Solution

The challenge name and the hint about file sizes tells us that a [packer](https://en.wikipedia.org/wiki/Executable_compression) have been used on the binary.

### Basic file analysis

We start with some basic analysis of the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ file out            
out: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, no section header

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ strings -n 8 out
AWAVAUATUS
7069636fH
<---snip--->
Enter the password
o unlock,is file: 
lagX7069636f4354467b5539585f
<---snip--->
HLP''''TX\c''''jr|
PROT_EXEC|PROT_WRITE failed.
$Info: This file is packed with the UPX executable packer http://upx.sf.net $
$Id: UPX 3.95 Copyright (C) 1996-2018 the UPX Team. All Rights Reserved. $
/proc/self/exe
<---snip--->
l(0BhKCo
p$mkqui#
Z/-id%ABI-
```

Ah, UPX (Ultimate Packer for eXecutables) have been used and we are looking for a password.

### Unpack the file

UPX-packed files can be unpacked/decompressed with `upx -d`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ upx -d -o unpacked out   
                       Ultimate Packer for eXecutables
                          Copyright (C) 1996 - 2024
UPX 4.2.2       Markus Oberhumer, Laszlo Molnar & John Reiser    Jan 3rd 2024

        File size         Ratio      Format      Name
   --------------------   ------   -----------   -----------
[WARNING] bad b_info at 0x4b718

[WARNING] ... recovery at 0x4b714

    877724 <-    336520   38.34%   linux/amd64   unpacked

Unpacked 1 file.

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ file unpacked       
unpacked: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=2e06e54daad34a6d4b0c7ef71b3e1ce17ffbf6db, for GNU/Linux 3.2.0, not stripped

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ ls -l 
total 1181
-rwxrwxrwx 1 root root 336520 Jun 10 15:30 out
-rwxrwxrwx 1 root root 872088 Jun 10 15:30 unpacked
```

Now we can check for strings again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ strings -n 8 unpacked | more     
AWAVAUATUSH
[]A\A]A^A_
7069636fH
4354467bH
<---snip--->
AWAVAUATUSH
[]A\A]A^A_
Enter the password to unlock this file: 
You entered: %s
Password correct, please see flag: 7069636f4354467b5539585f556e5034636b314e365f42316e34526933535f65313930633366337d
Access denied
xeon_phi          <--- possible password
../csu/libc-start.c
FATAL: kernel too old
<---snip--->
```

We have a possible password (`xeon_phi`) and what is likely a hex-encoded flag. It starts with `7069636f` which is ASCII for `pico`.

### Get the flag

The password turned out to be incorrect but we can get the flag with one-liners in `python` or `xxd`.  
The python way

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ python -c "print(bytes.fromhex('7069636f4354467b5539585f556e5034636b314e365f42316e34526933535f65313930633366337d'))"
b'picoCTF{<REDACTED>}'
```

and the xxd way

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/packer]
└─$ echo '7069636f4354467b5539585f556e5034636b314e365f42316e34526933535f65313930633366337d' | xxd -r -p
picoCTF{<REDACTED>}   
```

For additional information, please see the references below.

## References

- [Executable compression - Wikipedia](https://en.wikipedia.org/wiki/Executable_compression)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [UPX - Github](https://github.com/upx/upx)
- [UPX - Homepage](https://upx.github.io/)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
