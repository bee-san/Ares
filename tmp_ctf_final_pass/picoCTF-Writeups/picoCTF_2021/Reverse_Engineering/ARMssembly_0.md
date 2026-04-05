# ARMssembly 0

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
What integer does this program print with arguments 182476535 and 3742084308? 

File: chall.S 
Flag format: picoCTF{XXXXXXXX} -> (hex, lowercase, no 0x, and 32 bits. ex. 5614267 would be picoCTF{0055aabb})

Hints:
1. Simple compare
```

Challenge link: [https://play.picoctf.org/practice/challenge/160](https://play.picoctf.org/practice/challenge/160)

## Solutions

### Solution #1 - Compile and emulate the program

To some extent, the easiest way to solve this challenge is to compile the code and then emulate the program to find out what the result is. This doesn't require any knowledge of ARM assembly at all. So let's start with that.

First we need to install a cross compiler to compile on a non-ARM machine such as Intel x64. We do that with `sudo apt install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu`.

Then we assemble and link

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_0]
└─$ aarch64-linux-gnu-as -o chall.o chall.S

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_0]
└─$ aarch64-linux-gnu-gcc -static -o chall chall.o

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_0]
└─$ file chall            
chall: ELF 64-bit LSB executable, ARM aarch64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=18151f0592a38d1c12e3567b2c6f8183e0de1d8c, for GNU/Linux 3.7.0, not stripped
```

Next, we need [QEMU to emulate the execution environment](https://azeria-labs.com/arm-on-x86-qemu-user/). We install it with `sudo apt install qemu-user qemu-user-static`.

Then we can just run the program with the two numbers. I had to reboot my machine before the emulation worked though.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_0]
└─$ ./chall 182476535 3742084308 
Result: 3742084308
```

The larger value was returned.

### Solution #2 - Manual analysis

Before analysing the actual assembler code we might need a little ARM refresher.

**Registers**  
In ARMv8 there are 31 [general purpose registers](https://developer.arm.com/documentation/102374/0101/Registers-in-AArch64---general-purpose-registers). These can be used as 64-bit X-registers (called X0 through X30) or as 32-bit W-registers (called W0 through W30). This is just two separate ways of looking at the same registers.

The [SP-register](https://developer.arm.com/documentation/102374/0101/Registers-in-AArch64---other-registers) is the stack pointer. It is used as the base address for loads and stores.

**Instructions**  
If we simplify somewhat, the [instruction format](https://azeria-labs.com/arm-instruction-set-part-3/) looks like this: `MNEMONIC <Reg>, <Oper1>, <Oper2>`.

For example `ADD X0, 11, 22` will add 11 and 12 and store the result in the X0-register. The register is usually the "destination" where the result is stored. The result is stored "to the left" in the instruction.

However, this is not the case in the store instruction `STR` where `STR W0, [X29, 16]` will store the value in the W0-register in the memory address pointed to by the X29-register + 16 bytes. The result is stored "to the right" in the instruction.

Some brief descriptions of [common instructions](https://developer.arm.com/documentation/dui0231/b/arm-instruction-reference) used in the program:

- ADD - Add
- BL - Branch with link
- CMP - Compare
- LDR - Load register
- MOV - Move
- STR - Store register

Now, let's dive into the assembler listing starting with `main`

```text
<---snip--->
main:
    stp x29, x30, [sp, -48]!
    add x29, sp, 0
    str x19, [sp, 16]
    str w0, [x29, 44]                   
    str x1, [x29, 32]                   
    ldr x0, [x29, 32]
    add x0, x0, 8                       # Point to arg1 on stack
    ldr x0, [x0]                        # x0 = arg1
    bl  atoi                            # Call atoi (convert arg1 to int)
    mov w19, w0                         # Store result of function call (arg1) in w19
    ldr x0, [x29, 32]                   
    add x0, x0, 16                      # Point to arg2 on stack
    ldr x0, [x0]                        # x0 = arg2
    bl  atoi                            # Call atoi (convert arg2 to int)
    mov w1, w0                          # Store result of function call (arg2) in w1
    mov w0, w19                         # w0 = w19 (i.e. arg1)
    bl  func1
    mov w1, w0                          # Store result of function call in w1
    adrp    x0, .LC0
    add x0, x0, :lo12:.LC0
    bl  printf                          # Print the result
    mov w0, 0
    ldr x19, [sp, 16]
    ldp x29, x30, [sp], 48
    ret
    .size   main, .-main
    .ident  "GCC: (Ubuntu/Linaro 7.5.0-3ubuntu1~18.04) 7.5.0"
    .section    .note.GNU-stack,"",@progbits
```

To get a high-level overview it is usually good to start with focusing only on what functions are called, i.e. the `bl` instructions. We have two calls to `atoi` (converts strings to integers), one call to `func1` and one call to `printf`.

Very broadly, we read the strings (the arguments supplied to the program), convert them to integers, then calls the `func1` (the compare function most likely) and finally prints the result. The return values of the functions are passed in the X0/W0-register. The instructions in the beginning (function prologue) and end (function epilogue) essentially sets up and restore the stack.

There are some additional comments with focus on the program arguments. arg1 is the first number passed (`182476535` in this case) and arg2 is the second number passed (`3742084308` in this case).

```text
func1:
    sub sp, sp, #16                     # Allocate space on stack (16 bytes)
    str w0, [sp, 12]                    # (sp+12) = arg1 
    str w1, [sp, 8]                     # (sp+8) = arg2 
    ldr w1, [sp, 12]                    # w1 = (sp+12) i.e. arg1 (182476535)
    ldr w0, [sp, 8]                     # w0 = (sp+8) i.e. arg2 (3742084308)
    cmp w1, w0                          # Compare (w1 <= w0)
    bls .L2                             # If w1 <= w0 jump to .L2, yes we jump
    ldr w0, [sp, 12]                    
    b   .L3
.L2:
    ldr w0, [sp, 8]                     # w0 = (sp+8) i.e. arg2
.L3:
    add sp, sp, 16                      # Restore stack
    ret                                 # Return to main
```

Again, comments are added and the most important thing is the the `CMP` instruction and the following `BLS` (Branch if lower or same). In our case arg1 is lower or same than arg2 so we jump to `.L2` where arg2 is returned in `w0`.

Remember to convert the flag to hex before submitting the flag.

For additional information, please see the references below.

## References

- [A64 - Base Instructions - ARM](https://developer.arm.com/documentation/ddi0602/2023-06/Base-Instructions?lang=en)
- [ARM Instruction Reference - ARM](https://developer.arm.com/documentation/dui0231/b/arm-instruction-reference)
- [Condition Flags and Codes - ARM](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes)
- [Intro to ARM Assembly - Azeria Labs](https://azeria-labs.com/writing-arm-assembly-part-1/)
- [Registers in AArch64 - general-purpose registers - ARM](https://developer.arm.com/documentation/102374/0103/Registers-in-AArch64---general-purpose-registers)
- [Running Arm Binaries on X86 with QEMU-User - Azeria Labs](https://azeria-labs.com/arm-on-x86-qemu-user/)
- [QEMU - Wikipedia](https://en.wikipedia.org/wiki/QEMU)
