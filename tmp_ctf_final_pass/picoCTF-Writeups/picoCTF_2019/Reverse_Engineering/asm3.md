# asm3

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
What does asm3(0xc264bd5c, 0xb5a06caa, 0xad761175) return? 

Submit the flag as a hexadecimal value (starting with '0x'). 

NOTE: Your submission for this question will NOT be in the normal flag format.

Source

Hints:
1. more(?) registers
```

Challenge link: [https://play.picoctf.org/practice/challenge/72](https://play.picoctf.org/practice/challenge/72)

## Solutions

In case you need a little x86 assembly refresher, see the [asm1 challenge](asm1.md).  
Also, the general stack layout after a function is called is described in the [asm2 challenge](asm2.md).  

Finally, in this challenge we need to remember that there is also 16-bit and 8-versions of the general purpose-registers.  
For example, EAX is a 32-bit register. The lower half of EAX is AX, a 16-bit register.  
AX is divided into two 8-bit registers, AH and AL (a-high and a-low).

The situation is mostly the same for the other general purpose-registers. In total there are:

- Eight 32-bit registers: eax, ebx, ecx, edx, esi, edi, ebp, esp.
- Eight 16-bit registers: ax, bx, cx, dx, si, di, bp, sp.
- Eight 8-bit registers: ah, al, bh, bl, ch, cl, dh, dl.

Now, lets look at the assembly source of the `asm3` function

```text
asm3:
    <+0>:    push   ebp
    <+1>:    mov    ebp,esp
    <+3>:    xor    eax,eax                      
    <+5>:    mov    ah,BYTE PTR [ebp+0x9]
    <+8>:    shl    ax,0x10
    <+12>:   sub    al,BYTE PTR [ebp+0xd]
    <+15>:   add    ah,BYTE PTR [ebp+0xf]
    <+18>:   xor    ax,WORD PTR [ebp+0x10]
    <+22>:   nop
    <+23>:   pop    ebp
    <+24>:   ret    
```

We certainly could analyse this code manually but that would be too tedious.  
Lets build, run and debug the code as described in the [asm1 challenge](asm1.md) instead.

Two things to note though:

1. `xor eax, eax` will zero-out (clear) the EAX-register. Anything XORed with itself is zero.
2. `SHL` is a new instruction - Shift Left - which is essentially a multiplication by 2 the specified number of times.

The re-worked assembly code looks like this

```text
.text 

    .code32
    .intel_syntax
    .globl _start
    .type Asm3, @function

    Asm3:
        push   %ebp
        mov    %ebp, %esp
        xor    %eax, %eax
        mov    %ah, BYTE PTR [%ebp+0x9]
        shl    %ax, 0x10
        sub    %al, BYTE PTR [%ebp+0xd]
        add    %ah, BYTE PTR [%ebp+0xf]
        xor    %ax, WORD PTR [%ebp+0x10]
        nop
        pop    %ebp
        ret    

    _start:
        push   0xad761175
        push   0xb5a06caa
        push   0xc264bd5c
        call   Asm3
        nop
```

Next, we assemble the file with `as`, link it with `ld` and verify the result with `file`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm3]
└─$ as -g --gstabs --32 -o asm3.o asm3.s       

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm3]
└─$ ld -m elf_i386 -o asm3 asm3.o 

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm3]
└─$ file asm3
asm3: ELF 32-bit LSB executable, Intel 80386, version 1 (SYSV), statically linked, not stripped
```

Now we start debugging with `gdb` and set a breakpoint at the `nop` instruction

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm3]
└─$ gdb -q ./asm3                                   
GEF for linux ready, type `gef' to start, `gef config' to configure
88 commands loaded and 5 functions added for GDB 13.2 in 0.00ms using Python engine 3.11
Reading symbols from ./asm3...
gef➤  disas _start
Dump of assembler code for function _start:
   0x08049019 <+0>:     push   0xad761175
   0x0804901e <+5>:     push   0xb5a06caa
   0x08049023 <+10>:    push   0xc264bd5c
   0x08049028 <+15>:    call   0x8049000 <Asm3>
   0x0804902d <+20>:    nop
End of assembler dump.
gef➤  break *0x0804902d
Breakpoint 1 at 0x804902d: file asm3.s, line 26.
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
$eax   : 0xa4e1    
$ebx   : 0x0       
$ecx   : 0x0       
$edx   : 0x0       
$esp   : 0xffffcff4  →  0xc264bd5c
$ebp   : 0x0       
$esi   : 0x0       
$edi   : 0x0       
$eip   : 0x0804902d  →  <_start+20> nop 
$eflags: [zero carry PARITY adjust SIGN trap INTERRUPT direction overflow resume virtualx86 identification]
$cs: 0x23 $ss: 0x2b $ds: 0x2b $es: 0x2b $fs: 0x00 $gs: 0x00 
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── stack ────
0xffffcff4│+0x0000: 0xc264bd5c   ← $esp
0xffffcff8│+0x0004: 0xb5a06caa
0xffffcffc│+0x0008: 0xad761175
0xffffd000│+0x000c: 0x00000001
0xffffd004│+0x0010: 0xffffd1cc  →  "/mnt/hgfs/CTFs/picoCTF/picoCTF_2019/Reverse_Engine[...]"
0xffffd008│+0x0014: 0x00000000
0xffffd00c│+0x0018: 0xffffd20e  →  "COLORFGBG=15;0"
0xffffd010│+0x001c: 0xffffd21d  →  "COLORTERM=truecolor"
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── code:x86:32 ────
<---snip--->
```

We can see that EAX is `0xa4e1`.

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
- [Registers - SkullSecurity](https://wiki.skullsecurity.org/index.php?title=Registers)
- [x86 - Wikipedia](https://en.wikipedia.org/wiki/X86)
- [x86 Assembly Guide](https://www.cs.virginia.edu/~evans/cs216/guides/x86.html)
- [x86 assembly language - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language)
- [x86 instruction listings - Wikipedia](https://en.wikipedia.org/wiki/X86_instruction_listings)
