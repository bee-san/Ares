# ARMssembly 3

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DYLAN MCGUIRE

Description:
What integer does this program print with argument 2541039191? 

File: chall_3.S

Flag format: picoCTF{XXXXXXXX} -> (hex, lowercase, no 0x, and 32 bits. ex. 5614267 would be picoCTF{0055aabb})

Hints:
1. beep boop beep boop...
```

Challenge link: [https://play.picoctf.org/practice/challenge/106](https://play.picoctf.org/practice/challenge/106)

## Solutions

As in the previous challenges, we compile the assembly code and then emulate the program to find out what the answer is.

First we need to install a cross compiler to compile on a non-ARM machine such as Intel x64. We do that with `sudo apt install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu`.

Then we assemble and link

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_3]
└─$ aarch64-linux-gnu-as -o chall_3.o chall_3.S 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_3]
└─$ aarch64-linux-gnu-gcc -static -o chall_3 chall_3.o 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_3]
└─$ file chall_3     
chall_3: ELF 64-bit LSB executable, ARM aarch64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=c5d10c1814cd2f51e391be70fdefc0aedda780b6, for GNU/Linux 3.7.0, not stripped
```

Next, we need [QEMU to emulate the execution environment](https://azeria-labs.com/arm-on-x86-qemu-user/). We install it with `sudo apt install qemu-user qemu-user-static`.

Then we can just run the program. In one of the [previous challenges](ARMssembly_0.md) I had to reboot my machine before the emulation worked.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_3]
└─$ ./chall_3 2541039191
Result: 57
```

To convert the result to hexadecimal we can use interactive python

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_3]
└─$ python          
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> hex(57)
'0x39'
>>> exit()
```

Then all we need is to create the flag according to the instructions.

For additional information, please see the references below.

## References

- [A64 - Base Instructions - ARM](https://developer.arm.com/documentation/ddi0602/2023-06/Base-Instructions?lang=en)
- [ARM Instruction Reference - ARM](https://developer.arm.com/documentation/dui0231/b/arm-instruction-reference)
- [Condition Flags and Codes - ARM](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes)
- [Intro to ARM Assembly - Azeria Labs](https://azeria-labs.com/writing-arm-assembly-part-1/)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Running Arm Binaries on X86 with QEMU-User - Azeria Labs](https://azeria-labs.com/arm-on-x86-qemu-user/)
- [QEMU - Wikipedia](https://en.wikipedia.org/wiki/QEMU)
