# ARMssembly 2

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
What integer does this program print with argument 4189673334? 

File: chall_2.S 

Flag format: picoCTF{XXXXXXXX} -> (hex, lowercase, no 0x, and 32 bits. ex. 5614267 would be picoCTF{0055aabb})

Hints:
1. Loops
```

Challenge link: [https://play.picoctf.org/practice/challenge/150](https://play.picoctf.org/practice/challenge/150)

## Solutions

### Solution #1 - Compile, emulate the program and brute force

One way to solve this challenge is to compile the assembly code and then emulate the program to find out what the answer is.  
This doesn't require any knowledge of ARM assembly at all. So let's start with that.

First we need to install a cross compiler to compile on a non-ARM machine such as Intel x64. We do that with `sudo apt install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu`.

Then we assemble and link

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_2]
└─$ aarch64-linux-gnu-as -o chall_2.o chall_2.S

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_2]
└─$ aarch64-linux-gnu-gcc -static -o chall_2 chall_2.o

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_2]
└─$ file chall_2
chall_2: ELF 64-bit LSB executable, ARM aarch64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=2257f29c20690e75868b2112f674b5f49a65e78f, for GNU/Linux 3.7.0, not stripped
```

Next, we need [QEMU to emulate the execution environment](https://azeria-labs.com/arm-on-x86-qemu-user/). We install it with `sudo apt install qemu-user qemu-user-static`.

Then we can just run the program. In one of the [previous challenges](ARMssembly_0.md) I had to reboot my machine before the emulation worked.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_2]
└─$ ./chall_2 4189673334
Result: 3979085410
```

The result takes a few seconds to calculate.

### Solution #2 - Manual analysis

In case you need a little ARM refresher, see the [first ARMssembly challenge](ARMssembly_0.md).

Let's start with analysing the `main` function

```text
.LC0:
    .string "Result: %ld\n"
    .text
    .align  2
    .global main
    .type   main, %function
main:
    stp x29, x30, [sp, -48]!
    add x29, sp, 0
    str w0, [x29, 28]
    str x1, [x29, 16]
    ldr x0, [x29, 16]
    add x0, x0, 8               # Point to arg1 on stack
    ldr x0, [x0]                # x0 = arg1
    bl  atoi                    # Call atoi (convert arg1 to int)
    bl  func1                   # Call func1
    str w0, [x29, 44]           # Store the result on the stack
    adrp x0, .LC0
    add x0, x0, :lo12:.LC0
    ldr w1, [x29, 44]           # Get the result from func1 from the stack
    bl  printf                  # Print the result
    nop
    ldp x29, x30, [sp], 48      # Restore stack pointer
    ret
    .size   main, .-main
    .ident  "GCC: (Ubuntu/Linaro 7.5.0-3ubuntu1~18.04) 7.5.0"
    .section    .note.GNU-stack,"",@progbits
```

The logic all happens in the `func1` function

```text
func1:
    sub sp, sp, #32             # Allocate space on the stack
    str w0, [sp, 12]            # Store arg1 on stack at *(sp+12)
    str wzr, [sp, 24]           # Store 0 on stack at *(sp+24)
    str wzr, [sp, 28]           # Store 0 on stack at *(sp+28)
    b   .L2                     # Branch to .L2
.L3:
    ldr w0, [sp, 24]            # w0 = *(sp+24)
    add w0, w0, 3               # w0 += 3, that is increase w0 with 3
    str w0, [sp, 24]            # Store w0 on stack at *(sp+24)
    ldr w0, [sp, 28]            # w0 = *(sp+28)
    add w0, w0, 1               # w0 += 1, that is increase w0 with 1
    str w0, [sp, 28]            # Store w0 on stack at *(sp+28)
.L2:
    ldr w1, [sp, 28]            # w1 = *(sp+28)
    ldr w0, [sp, 12]            # w0 = *(sp+12), that is arg1 
    cmp w1, w0                  # Compare w1 and w0
    bcc .L3                     # Branch to .L3 if w1 < w0
    ldr w0, [sp, 24]            # w0 = *(sp+24)
    add sp, sp, 32              # Restore stack pointer
    ret                         # Return
    .size   func1, .-func1
    .section  .rodata
    .align  3
```

Compared to the previous challenges we have:

- A new register [`WZR`](https://developer.arm.com/documentation/102374/0101/Registers-in-AArch64---other-registers) that always reads as 0
- A new instruction BCC [Branch on Carry Clear](https://community.arm.com/support-forums/f/architectures-and-processors-forum/5941/could-you-explain-bcc-command-to-me)

The position sp+24 acts as an accumulator increasing the result by 3 for each round. The position sp+28 is a counter.  
The end result is arg1 * 3.

Remember to convert the flag to hex before submitting the flag.

For additional information, please see the references below.

## References

- [A64 - Base Instructions - ARM](https://developer.arm.com/documentation/ddi0602/2023-06/Base-Instructions?lang=en)
- [ARM Instruction Reference - ARM](https://developer.arm.com/documentation/dui0231/b/arm-instruction-reference)
- [Condition Flags and Codes - ARM](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes)
- [Intro to ARM Assembly - Azeria Labs](https://azeria-labs.com/writing-arm-assembly-part-1/)
- [Running Arm Binaries on X86 with QEMU-User - Azeria Labs](https://azeria-labs.com/arm-on-x86-qemu-user/)
- [QEMU - Wikipedia](https://en.wikipedia.org/wiki/QEMU)
