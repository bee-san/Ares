# GDB baby step 2

- [Challenge information](#challenge-information)
- [Solution](#solution)
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
1. You could calculate eax yourself, or you could set a breakpoint for after the calculcation and inspect eax to let the program do the heavy-lifting for you.
```

Challenge link: [https://play.picoctf.org/practice/challenge/396](https://play.picoctf.org/practice/challenge/396)

## Solution

Start by checking the file type with `file`.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_2]
└─$ file debugger0_b 
debugger0_b: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=95b0203be2982e75dbc01d1cc25b1309f7aec5f7, for GNU/Linux 3.2.0, not stripped
```

The file isn't stripped of debug information which makes it easier to debug.

Start GDB in quiet mode and then set the disassembly format to Intel, which I prefer.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_2]
└─$ gdb -q debugger0_b 
Reading symbols from debugger0_b...
(No debugging symbols found in debugger0_b)
(gdb) set disassembly-flavor intel
```

Then disassemble the `main` function

```bash
(gdb) disass main
Dump of assembler code for function main:
   0x0000000000401106 <+0>:     endbr64 
   0x000000000040110a <+4>:     push   rbp
   0x000000000040110b <+5>:     mov    rbp,rsp
   0x000000000040110e <+8>:     mov    DWORD PTR [rbp-0x14],edi
   0x0000000000401111 <+11>:    mov    QWORD PTR [rbp-0x20],rsi
   0x0000000000401115 <+15>:    mov    DWORD PTR [rbp-0x4],0x1e0da
   0x000000000040111c <+22>:    mov    DWORD PTR [rbp-0xc],0x25f
   0x0000000000401123 <+29>:    mov    DWORD PTR [rbp-0x8],0x0
   0x000000000040112a <+36>:    jmp    0x401136 <main+48>
   0x000000000040112c <+38>:    mov    eax,DWORD PTR [rbp-0x8]
   0x000000000040112f <+41>:    add    DWORD PTR [rbp-0x4],eax
   0x0000000000401132 <+44>:    add    DWORD PTR [rbp-0x8],0x1
   0x0000000000401136 <+48>:    mov    eax,DWORD PTR [rbp-0x8]
   0x0000000000401139 <+51>:    cmp    eax,DWORD PTR [rbp-0xc]
   0x000000000040113c <+54>:    jl     0x40112c <main+38>
   0x000000000040113e <+56>:    mov    eax,DWORD PTR [rbp-0x4]
   0x0000000000401141 <+59>:    pop    rbp
   0x0000000000401142 <+60>:    ret    
End of assembler dump.
```

Set a breakpoint at <main+59> and then run the program.

```bash
(gdb) break *main+59
Breakpoint 1 at 0x401141
(gdb) r
Starting program: /CTFs/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_2/debugger0_b 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, 0x0000000000401141 in main ()
```

Then print the value of EAX.

```bash
(gdb) print $eax
$1 = 307019
```

## References

- [Debugger - Wikipedia](https://en.wikipedia.org/wiki/Debugger)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [GDB (The GNU Project Debugger) - Documentation](https://sourceware.org/gdb/documentation/)
- [GDB (The GNU Project Debugger) - Homepage](https://sourceware.org/gdb/)
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)

Intel 64 and IA-32 Architectures Developer's Manuals in PDF-format

- [Volume 2A: Instruction Set Reference, A-M](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [Volume 2B: Instruction Set Reference, M-U](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2b-manual.pdf)
- [Volume 2C: Instruction Set Reference, V-Z](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2c-manual.pdf)
