# Bit-O-Asm-3

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
Can you figure out what is in the eax register? 

Put your answer in the picoCTF flag format: picoCTF{n} where n is the contents of the eax register in the decimal number base.  
If the answer was 0x11 your flag would be picoCTF{17}.

Hints:
1. Not everything in this disassembly listing is optimal.
```

Challenge link: [https://play.picoctf.org/practice/challenge/393](https://play.picoctf.org/practice/challenge/393)

## Solution

Study the assembler listing to figure out what happens. The interesting lines are prefixed with <+15> through <+36>.  
The RBP register points to the current stack frame.

```text
<+0>:     endbr64 
<+4>:     push   rbp
<+5>:     mov    rbp,rsp
<+8>:     mov    DWORD PTR [rbp-0x14],edi
<+11>:    mov    QWORD PTR [rbp-0x20],rsi
<+15>:    mov    DWORD PTR [rbp-0xc],0x9fe1a
<+22>:    mov    DWORD PTR [rbp-0x8],0x4
<+29>:    mov    eax,DWORD PTR [rbp-0xc]
<+32>:    imul   eax,DWORD PTR [rbp-0x8]
<+36>:    add    eax,0x1f5
<+41>:    mov    DWORD PTR [rbp-0x4],eax
<+44>:    mov    eax,DWORD PTR [rbp-0x4]
<+47>:    pop    rbp
<+48>:    ret
```

In more detail the following happens:

- The stack at position rbp-0xc is set to `0x9fe1a`
- The stack at position rbp-0x8 is set to `0x4`
- EAX is set to the value at position rbp-0xc (i.e. `0x9fe1a`)
- EAX is multiplied with the value at position rbp-0x8 (i.e. `0x4`)
- `0x1f5` is added to EAX

For more information on the x64 instruction set, see references below.

The flag should be in decimal format so convert it in Python:

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Bit-O-Asm-3]
└─$ python                                                             
Python 3.10.9 (main, Dec  7 2022, 13:47:07) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 0x9fe1a*4 + 0x1f5
2619997
```

Finally, create the flag like this `picoCTF{<Your_number>}`.

## References

- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)

Intel 64 and IA-32 Architectures Developer's Manuals in PDF-format

- [Volume 2A: Instruction Set Reference, A-M](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [Volume 2B: Instruction Set Reference, M-U](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2b-manual.pdf)
- [Volume 2C: Instruction Set Reference, V-Z](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2c-manual.pdf)
