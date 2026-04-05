# Reverse

- [Challenge information](#challenge-information)
- [Searching for strings solution](#searching-for-strings-solution)
- [Decompiling with Ghidra solution](#decompiling-with-ghidra-solution)
- [Debugging in GDB solution](#debugging-in-gdb-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
Try reversing this file? Can ya?

I forgot the password to this file. Please find it for me?

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/372](https://play.picoctf.org/practice/challenge/372)

There are several ways to solve this challenge. Here are three solutions presented in increasing difficulty.

## Searching for strings solution

On easy challenges it's always recommended to search for the flag in plain text with `strings`.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Reverse]
└─$ strings -a -n 8 ret    
/lib64/ld-linux-x86-64.so.2
libc.so.6
__isoc99_scanf
<---snip--->
[]A\A]A^A_
Enter the password to unlock this file: 
You entered: %s
Password correct, please see flag: picoCTF{<REDACTED>}             <----- Here
Access denied
GCC: (Ubuntu 9.4.0-1ubuntu1~20.04.1) 9.4.0
crtstuff.c
deregister_tm_clones
<---snip--->
```

And indeed, the flag is visable among the strings.

## Decompiling with Ghidra solution

A more sofisticated solution is to decompile the file in [Ghidra](https://ghidra-sre.org/) and study the code.

Import the file in Ghidra and analyze it with the default settings. Double-click on the `main` function to show the decompiled version of it.

```C
undefined8 main(void)

{
  int iVar1;
  long in_FS_OFFSET;
  char local_68 [48];
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  local_38 = 0x7b4654436f636970;
  local_30 = 0x337633725f666c33;
  local_28 = 0x75735f676e693572;
  local_20 = 0x6c75663535656363;
  local_18 = 0x346434316237645f;
  printf("Enter the password to unlock this file: ");
  __isoc99_scanf(&DAT_00102031,local_68);
  printf("You entered: %s\n",local_68);
  iVar1 = strcmp(local_68,(char *)&local_38);
  if (iVar1 == 0) {
    puts("Password correct, please see flag: picoCTF{<REDACTED>}");                <----- Here
    puts((char *)&local_38);
  }
  else {
    puts("Access denied");
  }
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return 0;
}
```

And there the flag is again.

## Debugging in GDB solution

A more advanced solution is to debug the file in GDB and examine the strings that are compared.

Start GDB in quite mode and then set the disassembly format to intel, which I prefer.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Reverse]
└─$ gdb -q ret                                                    
Reading symbols from ret...
(No debugging symbols found in ret)
(gdb) set disassembly-flavor intel
```

Disassemble the `main` function and look for the place where the string comparison is done

```text
(gdb) disass main
Dump of assembler code for function main:
   0x00000000000011c9 <+0>:     endbr64 
   0x00000000000011cd <+4>:     push   rbp
   0x00000000000011ce <+5>:     mov    rbp,rsp
   0x00000000000011d1 <+8>:     sub    rsp,0x60
   0x00000000000011d5 <+12>:    mov    rax,QWORD PTR fs:0x28
   0x00000000000011de <+21>:    mov    QWORD PTR [rbp-0x8],rax
   0x00000000000011e2 <+25>:    xor    eax,eax
   0x00000000000011e4 <+27>:    movabs rax,0x7b4654436f636970
   0x00000000000011ee <+37>:    movabs rdx,0x337633725f666c33
   0x00000000000011f8 <+47>:    mov    QWORD PTR [rbp-0x30],rax
   0x00000000000011fc <+51>:    mov    QWORD PTR [rbp-0x28],rdx
   0x0000000000001200 <+55>:    movabs rax,0x75735f676e693572
   0x000000000000120a <+65>:    movabs rdx,0x6c75663535656363
   0x0000000000001214 <+75>:    mov    QWORD PTR [rbp-0x20],rax
   0x0000000000001218 <+79>:    mov    QWORD PTR [rbp-0x18],rdx
   0x000000000000121c <+83>:    movabs rax,0x346434316237645f
   0x0000000000001226 <+93>:    mov    QWORD PTR [rbp-0x10],rax
   0x000000000000122a <+97>:    lea    rdi,[rip+0xdd7]        # 0x2008
   0x0000000000001231 <+104>:   mov    eax,0x0
   0x0000000000001236 <+109>:   call   0x10b0 <printf@plt>
   0x000000000000123b <+114>:   lea    rax,[rbp-0x60]
   0x000000000000123f <+118>:   mov    rsi,rax
   0x0000000000001242 <+121>:   lea    rdi,[rip+0xde8]        # 0x2031
   0x0000000000001249 <+128>:   mov    eax,0x0
   0x000000000000124e <+133>:   call   0x10d0 <__isoc99_scanf@plt>
   0x0000000000001253 <+138>:   lea    rax,[rbp-0x60]
   0x0000000000001257 <+142>:   mov    rsi,rax
   0x000000000000125a <+145>:   lea    rdi,[rip+0xdd3]        # 0x2034
   0x0000000000001261 <+152>:   mov    eax,0x0
   0x0000000000001266 <+157>:   call   0x10b0 <printf@plt>
   0x000000000000126b <+162>:   lea    rdx,[rbp-0x30]
   0x000000000000126f <+166>:   lea    rax,[rbp-0x60]
   0x0000000000001273 <+170>:   mov    rsi,rdx
   0x0000000000001276 <+173>:   mov    rdi,rax
   0x0000000000001279 <+176>:   call   0x10c0 <strcmp@plt>              <------ Here
   0x000000000000127e <+181>:   test   eax,eax
   0x0000000000001280 <+183>:   jne    0x129c <main+211>
<---snip--->
```

Note that the comparision in done at `main+176` and that the registers RSI and RDI is setup just before the call to `strcmp`.

Set a breakpoint there and run the program. When prompted enter your password guess, 'test' in this case.

```text
(gdb) break *main+176
Breakpoint 1 at 0x1279
(gdb) r
Starting program: /CTFs/picoCTF/picoCTF_2023/Reverse_Engineering/Reverse/ret 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
Enter the password to unlock this file: test
You entered: test

Breakpoint 1, 0x0000555555555279 in main ()
```

Examine the RSI and RDI registers and you have the main part of the flag again

```text
Breakpoint 1, 0x0000555555555279 in main ()
(gdb) x/s $rsi
0x7fffffffdd30: "picoCTF{<PARTIAL FLAG>"
(gdb) x/s $rdi
0x7fffffffdd00: "test"
```

Note however, that you don't get the full flag here as only a portion of the flag is compared.
To get the full flag you need to run the program again and input your partial flag. Then the full flag in printed.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Reverse]
└─$ ./ret                                                     
Enter the password to unlock this file: picoCTF{<PARTIAL FLAG>
You entered: picoCTF{<PARTIAL FLAG>
Password correct, please see flag: picoCTF{<FULL FLAG>}
```

For additional information, please see the references below.

### References

- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [Ghidra - Homepage](https://ghidra-sre.org/)
- [Intel Assembly Syntax - Wikipedia](https://en.wikipedia.org/wiki/X86_assembly_language#Syntax)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
