# GDB baby step 3

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
Now for something a little different. 0x2262c96b is loaded into memory in the main function. 
Examine byte-wise the memory that the constant is loaded in by using the GDB command x/4xb addr. 

The flag is the four bytes as they are stored in memory. 
If you find the bytes 0x11 0x22 0x33 0x44 in the memory location, your flag would be: picoCTF{0x11223344}.

Hints:
 1. You'll need to breakpoint the instruction after the memory load.
 2. Use the gdb command x/4xb addr with the memory location as the address addr to examine.
 3. Any registers in addr should be prepended with $ like $rbp.
 4. Don't use square brackets for addr
 5. What is endianness?
```

Challenge link: [https://play.picoctf.org/practice/challenge/397](https://play.picoctf.org/practice/challenge/397)

## Solution

Start by checking the file type with `file`.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_3]
└─$ file debugger0_c 
debugger0_c: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=a10a8fa896351748020d158a4e18bb4be15cd3aa, for GNU/Linux 3.2.0, not stripped
```

The file isn't stripped of debug information which makes it easier to debug.

Start GDB in quiet mode and then set the disassembly format to Intel, which I prefer.

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_3]
└─$ gdb -q debugger0_c 
Reading symbols from debugger0_c...
(No debugging symbols found in debugger0_c)
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
   0x0000000000401115 <+15>:    mov    DWORD PTR [rbp-0x4],0x2262c96b
   0x000000000040111c <+22>:    mov    eax,DWORD PTR [rbp-0x4]
   0x000000000040111f <+25>:    pop    rbp
   0x0000000000401120 <+26>:    ret    
End of assembler dump.
```

Set a breakpoint at `<main+25>` and then run the program.

```bash
(gdb) break *main+25
Breakpoint 1 at 0x40111f
(gdb) r
Starting program: /CTFs/picoCTF/picoGym/Reverse_Engineering/GDB_baby_step_3/debugger0_c 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, 0x000000000040111f in main ()
```

Then examine the 4 bytes of memory starting at position RBP-0x4.

```bash
(gdb) x/4xb $rbp-4
0x7fffffffdd2c: 0x6b    0xc9    0x62    0x22
```

Finally, create the flag with the hex values in the order above.

## References

- [Debugger - Wikipedia](https://en.wikipedia.org/wiki/Debugger)
- [Endianness - Wikipedia](https://en.wikipedia.org/wiki/Endianness)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [GDB (The GNU Project Debugger) - Documentation](https://sourceware.org/gdb/documentation/)
- [GDB (The GNU Project Debugger) - Homepage](https://sourceware.org/gdb/)'
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)

Intel 64 and IA-32 Architectures Developer's Manuals in PDF-format

- [Volume 2A: Instruction Set Reference, A-M](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [Volume 2B: Instruction Set Reference, M-U](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2b-manual.pdf)
- [Volume 2C: Instruction Set Reference, V-Z](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2c-manual.pdf)
