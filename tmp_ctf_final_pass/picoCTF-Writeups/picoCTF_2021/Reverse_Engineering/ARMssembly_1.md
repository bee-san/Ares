# ARMssembly 1

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: PRANAY GARG

Description:
For what argument does this program print `win` with variables 79, 7 and 3? 

File: chall_1.S 

Flag format: picoCTF{XXXXXXXX} -> (hex, lowercase, no 0x, and 32 bits. ex. 5614267 would be picoCTF{0055aabb})

Hints:
1. Shifts
```

Challenge link: [https://play.picoctf.org/practice/challenge/111](https://play.picoctf.org/practice/challenge/111)

## Solutions

### Solution #1 - Compile, emulate the program and brute force

To some extent, the easiest way to solve this challenge is to compile the code and then emulate the program to find out what the answer is by brute force. This doesn't require any knowledge of ARM assembly at all. So let's start with that.

First we need to install a cross compiler to compile on a non-ARM machine such as Intel x64. We do that with `sudo apt install binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu`.

Then we assemble and link

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ aarch64-linux-gnu-as -o chall_1.o chall_1.S

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ aarch64-linux-gnu-gcc -static -o chall_1 chall_1.o

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ file chall_1                
chall_1: ELF 64-bit LSB executable, ARM aarch64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=f83ed15a5dc86e4eee97dd9789a8f660009dae4d, for GNU/Linux 3.7.0, not stripped
```

Next, we need [QEMU to emulate the execution environment](https://azeria-labs.com/arm-on-x86-qemu-user/). We install it with `sudo apt install qemu-user qemu-user-static`.

Then we can just run the program. In [previous challenge](ARMssembly_0.md) I had to reboot my machine before the emulation worked.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ ./chall_1 1
You Lose :(

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ ./chall_1 2
You Lose :(
```

Next we brute force the answer

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/ARMssembly_1]
└─$ for i in {0..10000}; do echo -n "$i "; ./chall_1 $i; done | grep win
3370 You win!
```

I first tried only the first 1000 numbers but that wasn't enought so I increased it to 10000.

### Solution #2 - Manual analysis

In case you need a little ARM refresher, see the [previous challenge](ARMssembly_0.md).

Let's start with the `main` function

```text
.LC0:
    .string "You win!"
    .align  3
.LC1:
    .string "You Lose :("
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
    str w0, [x29, 44]           # Store the result on the stack
    ldr w0, [x29, 44]           # Load it back (prepare for function call)
    bl  func                    # Call func
    cmp w0, 0                   # Check result of function call
    bne .L4                     # Non-zero result?, branch to .L4
    adrp    x0, .LC0
    add x0, x0, :lo12:.LC0
    bl  puts                    # Print the winning result
    b   .L6
.L4:
    adrp    x0, .LC1
    add x0, x0, :lo12:.LC1
    bl  puts                    # Print the losing result
.L6:
    nop
    ldp x29, x30, [sp], 48      # Restore stack pointer
    ret                         # Return/exit
    .size   main, .-main
    .ident  "GCC: (Ubuntu/Linaro 7.5.0-3ubuntu1~18.04) 7.5.0"
    .section    .note.GNU-stack,"",@progbits
```

To get a winning result the return value of the `func` function needs to be zero.

The logic all happens in the `func` function

```text
func:
    sub sp, sp, #32             # Allocate space on the stack
    str w0, [sp, 12]            # arg1
    mov w0, 79                  # w0 = 79
    str w0, [sp, 16]            # *(sp+16) = w0, that is 79
    mov w0, 7                   # w0 = 7
    str w0, [sp, 20]            # *(sp+20) = w0, that is 7
    mov w0, 3                   # w0 = 3
    str w0, [sp, 24]            # *(sp+24) = w0, that is 3
    ldr w0, [sp, 20]            # w0 = *(sp+20), that is 7
    ldr w1, [sp, 16]            # w1 = *(sp+16), that is 79
    lsl w0, w1, w0              # w0 = w1 << 7, that is 79 * 2**7 = 10112
    str w0, [sp, 28]            # *(sp+28) = w0, that is 10112
    ldr w1, [sp, 28]            # w1 = *(sp+28), that is 10112
    ldr w0, [sp, 24]            # w0 = *(sp+24), that is 3
    sdiv w0, w1, w0             # w0 = 10112 / 3, that is 3370
    str w0, [sp, 28]            # *(sp+28) = w0, that is 3370
    ldr w1, [sp, 28]            # w1 = *(sp+28), that is 3370
    ldr w0, [sp, 12]            # w0 = arg1
    sub w0, w1, w0              # w0 = 3370 - arg1
    str w0, [sp, 28]            # *(sp+28) = w0
    ldr w0, [sp, 28]            # w0 = *(sp+28)
    add sp, sp, 32              # Restore stack pointer
```

Compared to the previous challenge we have two new instructions:

- LSL - [Logical Shift Left](https://developer.arm.com/documentation/100076/0200/a32-t32-instruction-set-reference/a32-and-t32-instructions/lsl)
- SDIV - [Signed Divide](https://developer.arm.com/documentation/100076/0200/a32-t32-instruction-set-reference/a32-and-t32-instructions/sdiv)

In order for `w0` to be `0`, `3370 - arg1` needs to be zero and `arg1` needs to be 3370.

Remember to convert the flag to hex before submitting the flag.

For additional information, please see the references below.

## References

- [A64 - Base Instructions - ARM](https://developer.arm.com/documentation/ddi0602/2023-06/Base-Instructions?lang=en)
- [ARM Instruction Reference - ARM](https://developer.arm.com/documentation/dui0231/b/arm-instruction-reference)
- [Condition Flags and Codes - ARM](https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/condition-codes-1-condition-flags-and-codes)
- [Intro to ARM Assembly - Azeria Labs](https://azeria-labs.com/writing-arm-assembly-part-1/)
- [Running Arm Binaries on X86 with QEMU-User - Azeria Labs](https://azeria-labs.com/arm-on-x86-qemu-user/)
- [QEMU - Wikipedia](https://en.wikipedia.org/wiki/QEMU)
