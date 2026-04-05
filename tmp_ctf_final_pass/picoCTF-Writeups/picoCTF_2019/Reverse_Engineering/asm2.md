# asm2

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SANJAY C

Description:
What does asm2(0xc,0x15) return? 

Submit the flag as a hexadecimal value (starting with '0x'). 

NOTE: Your submission for this question will NOT be in the normal flag format.

Source

Hints:
1. assembly conditions
```

Challenge link: [https://play.picoctf.org/practice/challenge/16](https://play.picoctf.org/practice/challenge/16)

## Solutions

### Manual analysis of the assembly source

In case you need a little x86 assembly refresher, see the [previous challenge](asm1.md).

In this challenge we need to remember the general stack layout in 32-bit mode after a function is called:

|Offset|Content|
|----|----|
|ebp-0xc|Local variable 3|
|ebp-0x8|Local variable 2|
|ebp-0x4|Local variable 1|
|ebp|Old ebp value|
|ebp+0x4|Saved EIP (Return address)|
|ebp+0x8|Parameter 1|
|ebp+0xc|Parameter 2|
|ebp+0x10|Parameter 3|

Now, lets look at the assembly source of the `asm2` function

```text
asm2:
    <+0>:    push   ebp
    <+1>:    mov    ebp,esp
    <+3>:    sub    esp,0x10                     # Allocate space on stack
    <+6>:    mov    eax,DWORD PTR [ebp+0xc]      # EAX = Param 2, that is 0x15
    <+9>:    mov    DWORD PTR [ebp-0x4],eax      # Local Var 1 = EAX, that is 0x15
    <+12>:   mov    eax,DWORD PTR [ebp+0x8]      # EAX = Param 1, that is 0xC
    <+15>:   mov    DWORD PTR [ebp-0x8],eax      # Local Var 2 = EAX, that is 0xC
    <+18>:   jmp    0x50c <asm2+31>              # Jump to <asm2+31>
    <+20>:   add    DWORD PTR [ebp-0x4],0x1      # Local Var 1 += 1
    <+24>:   add    DWORD PTR [ebp-0x8],0xaf     # Local Var 2 += 0xAF
    <+31>:   cmp    DWORD PTR [ebp-0x8],0xa3d3   # Compare Local Var 2 and 0xa3d3
    <+38>:   jle    0x501 <asm2+20>              # If Local Var 2 <= 0xa3d3 then jump to <asm2+20>
    <+40>:   mov    eax,DWORD PTR [ebp-0x4]      # EAX = Local Var 1
    <+43>:   leave  
    <+44>:   ret   
```

It it not obvious what the value of `EAX` it when the function returns.

Lets create a small Python script that does the calculation for us

```python
#!/usr/bin/python

LV1 = 0x15
LV2 = 0xC

while (LV2 <= 0xA3D3):
    LV1 += 1
    LV2 += 0xAF
    
print(hex(LV1))
```

When we run it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm2]
└─$ ./asm2.py  
0x105
```

we see that the value is `0x105`.

### Build, run and debug the code

An alternative solution is to build, run and debug the code as in the [previous challenge](asm1.md).

The re-worked assembly code looks like this

```text
.text 
    .code32
    .intel_syntax
    .globl _start
    .type Asm2, @function

    Asm2:
        push   %ebp
        mov    %ebp, %esp
        sub    %esp, 0x10
        mov    %eax, DWORD PTR [%ebp+0xc]
        mov    DWORD PTR [%ebp-0x4], %eax
        mov    %eax, DWORD PTR [%ebp+0x8]
        mov    DWORD PTR [%ebp-0x8], %eax
        jmp    case_31
        
    case_20:
        add    DWORD PTR [%ebp-0x4], 0x1
        add    DWORD PTR [%ebp-0x8], 0xaf
        
    case_31:
        cmp    DWORD PTR [%ebp-0x8], 0xa3d3
        jle    case_20
        mov    %eax, DWORD PTR [%ebp-0x4]
        leave  
        ret    

    _start:
        push   0x15
        push   0xC
        call   Asm2
        nop
```

Next, we assemble the file with `as`, link it with `ld` and verify the result with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm2]
└─$ as -g --gstabs --32 -o asm2.o test_wrapper.s

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm2]
└─$ ld -m elf_i386 -o asm2 asm2.o 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm2]
└─$ file asm2                  
asm2: ELF 32-bit LSB executable, Intel 80386, version 1 (SYSV), statically linked, not stripped
```

Now we start debugging with `gdb` and set a breakpoint at the `nop` instruction

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm2]
└─$ gdb -q ./asm2
GEF for linux ready, type `gef' to start, `gef config' to configure
88 commands loaded and 5 functions added for GDB 13.2 in 0.00ms using Python engine 3.11
Reading symbols from ./asm2...
gef➤  disas _start
Dump of assembler code for function _start:
   0x0804902d <+0>:     push   0x15
   0x0804902f <+2>:     push   0xc
   0x08049031 <+4>:     call   0x8049000 <Asm2>
   0x08049036 <+9>:     nop
End of assembler dump.
gef➤  break *0x08049036
Breakpoint 1 at 0x8049036: file test_wrapper.s, line 32.
```

In case you are wondering about the prompt, I have [GEF (GDB Enhanced Features)](https://github.com/hugsy/gef) installed.

Time to execute the program

```bash
gef➤  run
```

GEF will automatically show us the status of the registers after the breakpoint is hit:

```text
[ Legend: Modified register | Code | Heap | Stack | String ]
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── registers ────
$eax   : 0x105     
$ebx   : 0x0       
$ecx   : 0x0       
$edx   : 0x0       
$esp   : 0xffffcff8  →  0x0000000c ("
                                     "?)
$ebp   : 0x0       
$esi   : 0x0       
$edi   : 0x0       
$eip   : 0x08049036  →  <_start+9> nop 
$eflags: [zero carry parity adjust sign trap INTERRUPT direction overflow resume virtualx86 identification]
$cs: 0x23 $ss: 0x2b $ds: 0x2b $es: 0x2b $fs: 0x00 $gs: 0x00 
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── stack ────
0xffffcff8│+0x0000: 0x0000000c ("
                                 "?)     ← $esp
0xffffcffc│+0x0004: 0x00000015
0xffffd000│+0x0008: 0x00000001
0xffffd004│+0x000c: 0xffffd1cc  →  "/mnt/hgfs/CTFs/picoCTF/picoCTF_2019/Reverse_Engine[...]"
0xffffd008│+0x0010: 0x00000000
0xffffd00c│+0x0014: 0xffffd20e  →  "COLORFGBG=15;0"
0xffffd010│+0x0018: 0xffffd21d  →  "COLORTERM=truecolor"
0xffffd014│+0x001c: 0xffffd231  →  "COMMAND_NOT_FOUND_INSTALL_PROMPT=1"
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── code:x86:32 ────
<---snip--->
```

Again we can see that EAX is `0x105`.

For additional information, please see the references below.

## References

- [as - Linux manual page](https://man7.org/linux/man-pages/man1/as.1.html)
- [Assembly - Conditions](https://www.tutorialspoint.com/assembly_programming/assembly_conditions.htm)
- [AT&T Syntax versus Intel Syntax](https://www.cs.mcgill.ca/~cs573/winter2001/AttLinux_syntax.htm)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [GDB (The GNU Project Debugger) - Documentation](https://sourceware.org/gdb/documentation/)
- [GDB (The GNU Project Debugger) - Homepage](https://sourceware.org/gdb/)
- [GEF (GDB Enhanced Features) - Documentation](https://hugsy.github.io/gef/)
- [GEF (GDB Enhanced Features) - GitHub](https://github.com/hugsy/gef)
- [Intel 64 and IA-32 Architectures Software Developer Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [ld - Linux manual page](https://man7.org/linux/man-pages/man1/ld.1.html)
- [x86 - Wikipedia](https://en.wikipedia.org/wiki/X86)
- [x86 Assembly Guide](https://www.cs.virginia.edu/~evans/cs216/guides/x86.html)
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)
- [x86 instruction listings - Wikipedia](https://en.wikipedia.org/wiki/X86_instruction_listings)
