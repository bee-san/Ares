# Bit-O-Asm-4

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
1. Don't tell anyone I told you this, but you can solve this problem without understanding the compare/jump relationship.
2. Of course, if you're really good, you'll only need one attempt to solve this problem.
```

Challenge link: [https://play.picoctf.org/practice/challenge/394](https://play.picoctf.org/practice/challenge/394)

## Solution

Study the assembler listing to figure out what happens. The interesting lines are prefixed with <+15> through <+41>.  
The RBP register points to the current stack frame.

```text
<+0>:     endbr64 
<+4>:     push   rbp
<+5>:     mov    rbp,rsp
<+8>:     mov    DWORD PTR [rbp-0x14],edi
<+11>:    mov    QWORD PTR [rbp-0x20],rsi
<+15>:    mov    DWORD PTR [rbp-0x4],0x9fe1a
<+22>:    cmp    DWORD PTR [rbp-0x4],0x2710
<+29>:    jle    0x55555555514e <main+37>
<+31>:    sub    DWORD PTR [rbp-0x4],0x65
<+35>:    jmp    0x555555555152 <main+41>
<+37>:    add    DWORD PTR [rbp-0x4],0x65
<+41>:    mov    eax,DWORD PTR [rbp-0x4]
<+44>:    pop    rbp
<+45>:    ret
```

In more detail the following happens:

- The stack at position rbp-0x4 is set to `0x9fe1a`
- Then the value at position rbp-0x4 is compared with the value `0x2710`
- If the value at position rbp-0x4 is less than or equal to the value `0x2710` (which it isn't), jump to `<main+37>`
- The value at position rbp-0x4 is subtracted by `0x65`
- Then jump to `<main+41>` where the value is copied to EAX

For more information on the x64 instruction set, see references below.

The flag should be in decimal so convert it in Python:

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Bit-O-Asm-4]
└─$ python                                                             
Python 3.10.9 (main, Dec  7 2022, 13:47:07) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 0x9fe1a - 0x65
654773
```

Finally, create the flag like this `picoCTF{<Your_number>}`.

## References

- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)
- [x86 calling conventions - Wikipedia](https://en.wikipedia.org/wiki/X86_calling_conventions)

Intel 64 and IA-32 Architectures Developer's Manuals in PDF-format

- [Volume 2A: Instruction Set Reference, A-M](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2a-manual.pdf)
- [Volume 2B: Instruction Set Reference, M-U](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2b-manual.pdf)
- [Volume 2C: Instruction Set Reference, V-Z](https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-2c-manual.pdf)
