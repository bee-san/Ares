# asm1

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SANJAY C

Description:
What does asm1(0x345) return? 

Submit the flag as a hexadecimal value (starting with '0x'). 

NOTE: Your submission for this question will NOT be in the normal flag format. 

Source

Hints:
1. assembly conditions
```

Challenge link: [https://play.picoctf.org/practice/challenge/20](https://play.picoctf.org/practice/challenge/20)

## Solutions

### Manual analysis of the assembly source

Before analysing the actual source code we might need a little x86 assembly refresher:

#### Registers

There are eight 32-bit general-purpose registers:

- EAX — Accumulator for operands and results data
- EBX — Pointer to data in the DS segment
- ECX — Counter for string and loop operations
- EDX — I/O pointer
- ESI — Pointer to data in the segment pointed to by the DS register, source pointer for string operations
- EDI — Pointer to data (or destination) in the segment pointed to by the ES register, destination pointer for string operations.
- ESP — Stack pointer (in the SS segment).
- EBP — Pointer to data on the stack (in the SS segment).

#### Instructions

Assembly listings can be in two different syntaxes: [AT&T Syntax versus Intel Syntax](https://www.cs.mcgill.ca/~cs573/winter2001/AttLinux_syntax.htm).  
The code in these challenges uses Intel syntax where the general instruction format is `INSTR <Dest> <Src>`.  
For example `sub eax,0x12` will subtract 12 from the EAX-register. That is, `EAX = EAX - 12`.

Common instructions:

- add - Integer Addition
- cmp - Compare
- je - Jump if equal
- jne - Jump if not equal
- jg - Jump if greater than
- jge - Jump if greater than or equal to
- jl - Jump if less than
- jle - Jump if less than or equal to
- mov - Move
- pop - Pop from stack
- push - Push onto stack
- ret - Return from subroutine/function
- sub - Integer Subtraction

Now, lets look at the assembly source of the `asm1` function

```text
asm1:
    <+0>:    push   ebp                          # Store base pointer on stack
    <+1>:    mov    ebp,esp                      # Set stack pointer
    <+3>:    cmp    DWORD PTR [ebp+0x8],0x37a    # Compare arg1 (that is 0x345) with 0x37a
    <+10>:   jg     0x512 <asm1+37>              # Jump if 0x345 > 0x37a, this is not the case so we don't jump
    <+12>:   cmp    DWORD PTR [ebp+0x8],0x345    # Compare arg1 (that is 0x345) with 0x345
    <+19>:   jne    0x50a <asm1+29>              # Jump if 0x345 != 0x345, this is not the case so we don't jump
    <+21>:   mov    eax,DWORD PTR [ebp+0x8]      # Set EAX to 0x345
    <+24>:   add    eax,0x3                      # Add 3 to EAX, that is EAX is now 0x348
    <+27>:   jmp    0x529 <asm1+60>              # Jump to <asm1+60>
    <+29>:   mov    eax,DWORD PTR [ebp+0x8]
    <+32>:   sub    eax,0x3
    <+35>:   jmp    0x529 <asm1+60>
    <+37>:   cmp    DWORD PTR [ebp+0x8],0x5ff
    <+44>:   jne    0x523 <asm1+54>
    <+46>:   mov    eax,DWORD PTR [ebp+0x8]
    <+49>:   sub    eax,0x3
    <+52>:   jmp    0x529 <asm1+60>
    <+54>:   mov    eax,DWORD PTR [ebp+0x8]
    <+57>:   add    eax,0x3
    <+60>:   pop    ebp                          # Restore base pointer
    <+61>:   ret                                 # Return, Return value will be EAX (that is 0x348)
```

So after the function call, EAX will be `0x348`.

### Build, run and debug the code

An alternative solution is to build, run and debug the code. This will mainly come in handy in the more advanced asm challenges later on.

However, we cannot assemble the code above as is. It needs to be changed in a number of ways.  
We need to:

- Add labels (`Case_xx:`) for each `<asm1+xx>`
- Remove the hex offset and refer to the label instead
- Add % before the registers, eax => %eax
- Add instructions to the assembler about the output format, for exampel `.code32` for 32-bit code
- Add a `_start` label where we call the `asm1` function

The result looks like this

```text
.text 
    .code32
    .intel_syntax
    .globl _start
    .type Asm1, @function

    Asm1:
        push   %ebp
        mov    %ebp, %esp
        cmp    DWORD PTR [%ebp+0x8], 0x37a
        jg     Case_37
        cmp    DWORD PTR [%ebp+0x8], 0x345
        jne    Case_29
        mov    %eax, DWORD PTR [%ebp+0x8]
        add    %eax, 0x3
        jmp    Case_60
        
    Case_29:
        mov    %eax, DWORD PTR [%ebp+0x8]
        sub    %eax, 0x3
        jmp    Case_60

    Case_37:    
        cmp    DWORD PTR [%ebp+0x8], 0x5ff
        jne    Case_54
        mov    %eax, DWORD PTR [%ebp+0x8]
        sub    %eax, 0x3
        jmp    Case_60
        
    Case_54:    
        mov    %eax, DWORD PTR [%ebp+0x8]
        add    %eax, 0x3
        
    Case_60:
        pop    %ebp
        ret    

    _start:
        push  0x345
        call Asm1
        nop
```

The final `nop` (No operation) instruction is just a dummy instruction where we will set our breakpoint later.

Next, we assemble the file (`test_wrapper.s`) with `as` as 32-bit and with debug information

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm1]
└─$ as -g --gstabs --32 -o test.o test_wrapper.s
```

Then we link with `ld` as 32-bit ELF

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm1]
└─$ ld -m elf_i386 -o test test.o 
```

Now we start debugging with `gdb` and set a breakpoint at the `nop` instruction

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Asm1]
└─$ gdb -q ./test
GEF for linux ready, type `gef' to start, `gef config' to configure
88 commands loaded and 5 functions added for GDB 13.2 in 0.00ms using Python engine 3.11
Reading symbols from ./test...

gef➤  disas _start
Dump of assembler code for function _start:
   0x0804903e <+0>:     push   0x345
   0x08049043 <+5>:     call   0x8049000 <Asm1>
   0x08049048 <+10>:    nop
End of assembler dump.

gef➤  break *0x08049048
Breakpoint 1 at 0x8049048: file test_wrapper.s, line 42.
```

In case you are wondering about the prompt, I have [GEF (GDB Enhanced Features)](https://github.com/hugsy/gef) installed.
I have also added additional empty line for improved readability.

Time to execute the program

```bash
gef➤  run
```

GEF will automatically show us the status of the registers after the breakpoint is hit:

```text
[ Legend: Modified register | Code | Heap | Stack | String ]
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── registers ────
$eax   : 0x348     
$ebx   : 0x0       
$ecx   : 0x0       
$edx   : 0x0       
$esp   : 0xffffcffc  →  0x00000345
$ebp   : 0x0       
$esi   : 0x0       
$edi   : 0x0       
$eip   : 0x08049048  →  <_start+10> nop 
$eflags: [zero carry PARITY adjust sign trap INTERRUPT direction overflow resume virtualx86 identification]
$cs: 0x23 $ss: 0x2b $ds: 0x2b $es: 0x2b $fs: 0x00 $gs: 0x00 
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── stack ────
0xffffcffc│+0x0000: 0x00000345   ← $esp
0xffffd000│+0x0004: 0x00000001
0xffffd004│+0x0008: 0xffffd1cc  →  "/mnt/hgfs/CTFs/picoCTF/picoCTF_2019/Reverse_Engine[...]"
0xffffd008│+0x000c: 0x00000000
0xffffd00c│+0x0010: 0xffffd20e  →  "COLORFGBG=15;0"
0xffffd010│+0x0014: 0xffffd21d  →  "COLORTERM=truecolor"
0xffffd014│+0x0018: 0xffffd231  →  "COMMAND_NOT_FOUND_INSTALL_PROMPT=1"
0xffffd018│+0x001c: 0xffffd254  →  "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/[...]"
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── code:x86:32 ────
<---snip--->
```

Again we can see that EAX is `0x348`.

For additional information, please see the references below.

## References

- [as - Linux manual page](https://man7.org/linux/man-pages/man1/as.1.html)
- [Assembly - Conditions](https://www.tutorialspoint.com/assembly_programming/assembly_conditions.htm)
- [AT&T Syntax versus Intel Syntax](https://www.cs.mcgill.ca/~cs573/winter2001/AttLinux_syntax.htm)
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
