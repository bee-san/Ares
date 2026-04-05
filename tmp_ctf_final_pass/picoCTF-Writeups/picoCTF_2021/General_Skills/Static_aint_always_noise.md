# Static ain't always noise

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL
  
Description:
Can you look at the data in this binary: static? 
This BASH script might help!
 
Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/163](https://play.picoctf.org/practice/challenge/163)

## Solution

### Analyse the files

Let's start by looking at what we have got.

A bash script called `ltdis.sh` that looks like this (with some empty lines removed)

```bash
#!/bin/bash

echo "Attempting disassembly of $1 ..."

#This usage of "objdump" disassembles all (-D) of the first file given by 
#invoker, but only prints out the ".text" section (-j .text) (only section
#that matters in almost any compiled program...

objdump -Dj .text $1 > $1.ltdis.x86_64.txt

#Check that $1.ltdis.x86_64.txt is non-empty
#Continue if it is, otherwise print error and eject

if [ -s "$1.ltdis.x86_64.txt" ]
then
   echo "Disassembly successful! Available at: $1.ltdis.x86_64.txt"

   echo "Ripping strings from binary with file offsets..."
   strings -a -t x $1 > $1.ltdis.strings.txt
   echo "Any strings found in $1 have been written to $1.ltdis.strings.txt with file offset"
else
   echo "Disassembly failed!"
   echo "Usage: ltdis.sh <program-file>"
   echo "Bye!"
fi
```

The script's comments give a good picture of what it does, essentially runs `objdump` and `strings` on the first supplied file (`$1`).

Let's check out the binary also

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ file static                                   
static: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=17ad46e6c58b7c40148a89923e314662595d101b, not stripped
```

OK, a 64-bit ELF binary. Why not run it and see what happens?

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ ./static                                    
Oh hai! Wait what? A flag? Yes, it's around here somewhere!
```

### Run the script and analyse the results

Let's follow the instructions and run the script on the binary

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ ./ltdis.sh static 
Attempting disassembly of static ...
Disassembly successful! Available at: static.ltdis.x86_64.txt
Ripping strings from binary with file offsets...
Any strings found in static have been written to static.ltdis.strings.txt with file offset
```

Let's briefly look at the dissasembly

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ head -n20 static.ltdis.x86_64.txt

static:     file format elf64-x86-64


Disassembly of section .text:

0000000000000530 <_start>:
 530:   31 ed                   xor    %ebp,%ebp
 532:   49 89 d1                mov    %rdx,%r9
 535:   5e                      pop    %rsi
 536:   48 89 e2                mov    %rsp,%rdx
 539:   48 83 e4 f0             and    $0xfffffffffffffff0,%rsp
 53d:   50                      push   %rax
 53e:   54                      push   %rsp
 53f:   4c 8d 05 8a 01 00 00    lea    0x18a(%rip),%r8        # 6d0 <__libc_csu_fini>
 546:   48 8d 0d 13 01 00 00    lea    0x113(%rip),%rcx        # 660 <__libc_csu_init>
 54d:   48 8d 3d e6 00 00 00    lea    0xe6(%rip),%rdi        # 63a <main>
 554:   ff 15 86 0a 20 00       call   *0x200a86(%rip)        # 200fe0 <__libc_start_main@GLIBC_2.2.5>
 55a:   f4                      hlt
 55b:   0f 1f 44 00 00          nopl   0x0(%rax,%rax,1)
```

Well, assembly code. Let's wait with diving into the details there...

Let's check the strings instead

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ head -n20 static.ltdis.strings.txt 
    238 /lib64/ld-linux-x86-64.so.2
    290 >1FbY]
    361 libc.so.6
    36b puts
    370 __cxa_finalize
    37f __libc_start_main
    391 GLIBC_2.2.5
    39d _ITM_deregisterTMCloneTable
    3b9 __gmon_start__
    3c8 _ITM_registerTMCloneTable
    660 AWAVI
    667 AUATL
    6ba []A\A]A^A_
    6e8 Oh hai! Wait what? A flag? Yes, it's around here somewhere!
    7c7 ;*3$"
   1020 picoCTF{<REDACTED>}
   1040 GCC: (Ubuntu 7.5.0-3ubuntu1~18.04) 7.5.0
   1671 crtstuff.c
   167c deregister_tm_clones
   1691 __do_global_dtors_aux
```

And there, at offset 1020, we have the flag.

If we didn't want to manually go through the file we could `grep` for the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Static_ain't_always_noise]
└─$ grep picoCTF static.ltdis.strings.txt 
   1020 picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Adding arguments and options to your Bash scripts](https://www.redhat.com/sysadmin/arguments-options-bash-scripts)
- [Disassembler - Wikipedia](https://en.wikipedia.org/wiki/Disassembler)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [head - Linux manual page](https://man7.org/linux/man-pages/man1/head.1.html)
- [objdump - Linux manual page](https://man7.org/linux/man-pages/man1/objdump.1.html)
- [Shell script - Wikipedia](https://en.wikipedia.org/wiki/Shell_script)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
- [strings (Unix) - Wikipedia](https://en.wikipedia.org/wiki/Strings_(Unix))
