# GDB baby step 1

- [Challenge information](#challenge-information)
- [GDB Solution](#gdb-solution)
- [Objdump Solution](#objdump-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoGym Exclusive, Reverse Engineering, X86_64
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Can you figure out what is in the eax register at the end of the main function?

Put your answer in the picoCTF flag format: picoCTF{n} where n is the contents of the eax register in the decimal number base.  
If the answer was 0x11 your flag would be picoCTF{17}.

Hints:
1. gdb is a very good debugger to use for this problem and many others!
2. main is actually a recognized symbol that can be used with gdb commands.
```

Challenge link: [https://play.picoctf.org/practice/challenge/395](https://play.picoctf.org/practice/challenge/395)

## GDB Solution

Start by checking the file type with `file`.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_1]
└─$ file debugger0_a 
debugger0_a: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=15a10290db2cd2ec0c123cf80b88ed7d7f5cf9ff, for GNU/Linux 3.2.0, not stripped
```

The file isn't stripped of debug information which makes it easier to debug.

Start GDB in quiet mode and then set the disassembly format to Intel, which I prefer.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_1]
└─$ gdb -q debugger0_a 
Reading symbols from debugger0_a...
(No debugging symbols found in debugger0_a)
(gdb) set disassembly-flavor intel
```

Then disassemble the `main` function

```bash
(gdb) disass main
Dump of assembler code for function main:
   0x0000000000001129 <+0>:     endbr64 
   0x000000000000112d <+4>:     push   rbp
   0x000000000000112e <+5>:     mov    rbp,rsp
   0x0000000000001131 <+8>:     mov    DWORD PTR [rbp-0x4],edi
   0x0000000000001134 <+11>:    mov    QWORD PTR [rbp-0x10],rsi
   0x0000000000001138 <+15>:    mov    eax,0x86342
   0x000000000000113d <+20>:    pop    rbp
   0x000000000000113e <+21>:    ret    
End of assembler dump.
```

EAX is set to `0x86342`.

The flag should be in decimal so convert it in Python:

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Bit-O-Asm-4]
└─$ python                                                             
Python 3.10.9 (main, Dec  7 2022, 13:47:07) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 0x86342
549698
```

Finally, create the flag like this `picoCTF{<Your_number>}`.

## Objdump Solution

An alternative solution is to disassemble with `objdump` instead.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_1]
└─$ objdump -d debugger0_a --disassemble=main -M intel

debugger0_a:     file format elf64-x86-64


Disassembly of section .init:

Disassembly of section .plt:

Disassembly of section .plt.got:

Disassembly of section .text:

0000000000001129 <main>:
    1129:       f3 0f 1e fa             endbr64
    112d:       55                      push   rbp
    112e:       48 89 e5                mov    rbp,rsp
    1131:       89 7d fc                mov    DWORD PTR [rbp-0x4],edi
    1134:       48 89 75 f0             mov    QWORD PTR [rbp-0x10],rsi
    1138:       b8 42 63 08 00          mov    eax,0x86342
    113d:       5d                      pop    rbp
    113e:       c3                      ret

Disassembly of section .fini:

```

## References

- [Debugger - Wikipedia](https://en.wikipedia.org/wiki/Debugger)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [GDB (The GNU Project Debugger) - Documentation](https://sourceware.org/gdb/documentation/)
- [GDB (The GNU Project Debugger) - Homepage](https://sourceware.org/gdb/)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [objdump - Linux manual page](https://man7.org/linux/man-pages/man1/objdump.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)

Intel 64 and IA-32 Architectures Developer's Manuals in PDF-format

- [Volume 2A: Instruction Set Reference, A-M](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [Volume 2B: Instruction Set Reference, M-U](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2b-manual.pdf)
- [Volume 2C: Instruction Set Reference, V-Z](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2c-manual.pdf)
