# FactCheck

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2024, Reverse Engineering, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JUNIAS BONOU
 
Description:
This binary is putting together some important piece of information... 
Can you uncover that information?

Examine this file. Do you understand its inner workings?

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/416](https://play.picoctf.org/practice/challenge/416)

## Solution

### Basic file analysis

We start with some basic analysis of the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/FactCheck]
└─$ file bin                    
bin: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=d134239fc06b6e50d2b04696cac10504a052fcfd, for GNU/Linux 3.2.0, not stripped
```

The file is a 64-bit [ELF-binary](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

Next, we check for interesting strings

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/FactCheck]
└─$ strings -n 6 bin
/lib64/ld-linux-x86-64.so.2
libstdc++.so.6
<---snip--->
GLIBC_2.2.5
[]A\A]A^A_
picoCTF{<REDACTED>
GCC: (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0
crtstuff.c
<---snip--->
```

Ah, we have a likely beginning of the flag.

### Static analysis in Ghidra

We continue with decompiling the file in [Ghidra](https://ghidra-sre.org/) and study the code.  
Import the file in Ghidra and analyze it with the default settings.  
Double-click on the `main` function to show the decompiled version of it

```c
undefined8 main(void)

{
  char cVar1;
  char *pcVar2;
  long in_FS_OFFSET;
  allocator<char> local_249;
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_248 [32];
  basic_string local_228 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_208 [32];
  basic_string local_1e8 [32];
  basic_string local_1c8 [32];
  basic_string local_1a8 [32];
  basic_string local_188 [32];
  basic_string local_168 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_148 [32];
  basic_string local_128 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_108 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_e8 [32];
  basic_string local_c8 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_a8 [32];
  basic_string local_88 [32];
  basic_string local_68 [32];
  basic_string<char,std::char_traits<char>,std::allocator<char>> local_48 [40];
  long local_20;
  
  local_20 = *(long *)(in_FS_OFFSET + 0x28);
  std::allocator<char>::allocator();
                    /* try { // try from 001012cf to 001012d3 has its CatchHandler @ 00101975 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_248,(allocator *)"picoCTF{<REDACTED>");
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010130a to 0010130e has its CatchHandler @ 00101996 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_228,(allocator *)&DAT_0010201d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101345 to 00101349 has its CatchHandler @ 001019b1 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_208,(allocator *)&DAT_0010201f);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101380 to 00101384 has its CatchHandler @ 001019cc */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_1e8,(allocator *)&DAT_00102021);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001013bb to 001013bf has its CatchHandler @ 001019e7 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_1c8,(allocator *)&DAT_0010201d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001013f6 to 001013fa has its CatchHandler @ 00101a02 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_1a8,(allocator *)&DAT_00102023);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101431 to 00101435 has its CatchHandler @ 00101a1d */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_188,(allocator *)&DAT_00102025);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010146c to 00101470 has its CatchHandler @ 00101a38 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_168,(allocator *)&DAT_00102027);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001014a7 to 001014ab has its CatchHandler @ 00101a53 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_148,(allocator *)&DAT_00102029);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001014e2 to 001014e6 has its CatchHandler @ 00101a6e */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_128,(allocator *)&DAT_0010202b);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010151d to 00101521 has its CatchHandler @ 00101a89 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_108,(allocator *)&DAT_0010202d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101558 to 0010155c has its CatchHandler @ 00101aa4 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_e8,(allocator *)&DAT_00102025);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101593 to 00101597 has its CatchHandler @ 00101abf */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_c8,(allocator *)&DAT_0010202f);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001015ce to 001015d2 has its CatchHandler @ 00101ada */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_a8,(allocator *)&DAT_00102031);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101606 to 0010160a has its CatchHandler @ 00101af5 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_88,(allocator *)&DAT_00102033);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 0010163e to 00101642 has its CatchHandler @ 00101b0d */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_68,(allocator *)&DAT_0010201d);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101676 to 0010167a has its CatchHandler @ 00101b25 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_48,(allocator *)&DAT_00102033);
  std::allocator<char>::~allocator(&local_249);
                    /* try { // try from 00101699 to 0010185f has its CatchHandler @ 00101b3d */
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)local_208);
  if (*pcVar2 < 'B') {
    std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
              (local_248,local_c8);
  }
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)local_a8);
  if (*pcVar2 != 'A') {
    std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
              (local_248,local_68);
  }
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)local_1c8);
  cVar1 = *pcVar2;
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)local_148);
  if ((int)cVar1 - (int)*pcVar2 == 3) {
    std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
              (local_248,local_1c8);
  }
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_1e8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_188);
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)local_168);
  if (*pcVar2 == 'G') {
    std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
              (local_248,local_168);
  }
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_1a8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_88);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_228);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,local_128);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,'}');
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_48);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_68);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_88);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_a8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_c8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_e8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_108);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_128);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_148);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_168);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_188);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_1a8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_1c8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_1e8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_208);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            ((basic_string<char,std::char_traits<char>,std::allocator<char>> *)local_228);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::~basic_string
            (local_248);
  if (local_20 == *(long *)(in_FS_OFFSET + 0x28)) {
    return 0;
  }
                    /* WARNING: Subroutine does not return */
  __stack_chk_fail();
}
```

At the beginning of the code, we again see the first part of the flag.

It's rather hard to see but each line of the form

```text
std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_228,(allocator *)&DAT_0010201d);
```

corresponds to a character. You can double-click on the memory reference (`DAT_0010201d`) to find out what character it corresponds to. Then rename it (`Rename Global`) to something like `char_X`. The result will look like this

```text
                    /* try { // try from 0010130a to 0010130e has its CatchHandler @ 00101996 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_228,(allocator *)&char_3);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101345 to 00101349 has its CatchHandler @ 001019b1 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_208,(allocator *)&char_5);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101380 to 00101384 has its CatchHandler @ 001019cc */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)local_1e8,(allocator *)&char_9);
  std::allocator<char>::~allocator(&local_249);
```

Then we rename the corresponding local_variable to the same name. E.g. `local_208` shold be renamed (`Rename Variable`) to `char_3`. If you get `duplicate name` errors, shorten one of the names to `chr_X` instead. We now have

```text
                    /* try { // try from 0010130a to 0010130e has its CatchHandler @ 00101996 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)char_3,(allocator *)&::char_3);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101345 to 00101349 has its CatchHandler @ 001019b1 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)char_5,(allocator *)&::char_5);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 00101380 to 00101384 has its CatchHandler @ 001019cc */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)char_9,(allocator *)&::char_9);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
                    /* try { // try from 001013bb to 001013bf has its CatchHandler @ 001019e7 */
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::basic_string
            ((char *)chr_3,(allocator *)&::char_3);
  std::allocator<char>::~allocator(&local_249);
  std::allocator<char>::allocator();
```

It's still a bit hard to read though.

Further down we see that the flag is created by characters appended (`operator+`). Sometimes if some condition is meet.

```text
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,char_b);
  pcVar2 = (char *)std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::
                   operator[]((ulong)char_a);
  if (*pcVar2 == 'G') {
    std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
              (local_248,char_a);
  }
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,char_4);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,char_8);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,char_3);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,char_f);
  std::__cxx11::basic_string<char,std::char_traits<char>,std::allocator<char>>::operator+=
            (local_248,'}');
```

We see that the last character appended is the `}` character.

This is getting too tedious. Let's switch to a dynamic approach instead and debug the binary in `gdb`

### Dynamic analysis in GDB

We start gdb with the [GEF extension](https://hugsy.github.io/gef/)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Reverse_Engineering/FactCheck]
└─$ gdb-gef -q bin         
Reading symbols from bin...
(No debugging symbols found in bin)
Error while writing index for `/mnt/hgfs/CTFs/picoCTF/picoCTF_2024/Reverse_Engineering/FactCheck/bin': No debugging symbols
GEF for linux ready, type `gef' to start, `gef config' to configure
88 commands loaded and 5 functions added for GDB 13.2 in 0.01ms using Python engine 3.11
gef➤  
```

We want to break on the first assembly instruction after the appending of the `}` character, which is a relative address ending with `0x01860`.

First we figure out the relative offset from start of `main`

```text
gef➤  disass main
Dump of assembler code for function main:
   0x0000000000001289 <+0>:     endbr64
   0x000000000000128d <+4>:     push   rbp
   0x000000000000128e <+5>:     mov    rbp,rsp
   0x0000000000001291 <+8>:     push   rbx
   0x0000000000001292 <+9>:     sub    rsp,0x248
   0x0000000000001299 <+16>:    mov    rax,QWORD PTR fs:0x28
   0x00000000000012a2 <+25>:    mov    QWORD PTR [rbp-0x18],rax
<---snip--->
   0x0000000000001858 <+1487>:  mov    rdi,rax
   0x000000000000185b <+1490>:  call   0x1100 <_ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEpLEc@plt>
   0x0000000000001860 <+1495>:  mov    ebx,0x0
   0x0000000000001865 <+1500>:  lea    rax,[rbp-0x40]
   0x0000000000001869 <+1504>:  mov    rdi,rax
<---snip--->
```

Ah, the offset is `main+1495`. We set a break point there and run the program

```text
gef➤  break *main+1495
Breakpoint 1 at 0x1860
gef➤  run
```

### Get the flag

When the program stops we have the complete flag pointed to by both the `RAX` and `RDI` registers, as well as on the stack.

```text
Breakpoint 1, 0x0000555555555860 in main ()
[ Legend: Modified register | Code | Heap | Stack | String ]
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── registers ────
$rax   : 0x00007fffffffdae0  →  0x000055555556b2d0  →  "picoCTF{<REDACTED>}"
$rbx   : 0xffffffce        
$rcx   : 0x21              
$rdx   : 0x2e              
$rsp   : 0x00007fffffffdad0  →  0x00007ffff7e5b370  →  0x00007ffff7d3e020  →  <std::basic_ostream<wchar_t,+0> endbr64 
$rbp   : 0x00007fffffffdd20  →  0x0000000000000001
$rsi   : 0x7d              
$rdi   : 0x00007fffffffdae0  →  0x000055555556b2d0  →  "picoCTF{<REDACTED>}"
$rip   : 0x0000555555555860  →  <main+1495> mov ebx, 0x0
$r8    : 0x1               
$r9    : 0x0               
$r10   : 0x7               
$r11   : 0x000055555556b2b0  →  0x000000055555556b
$r12   : 0x0               
$r13   : 0x00007fffffffde48  →  0x00007fffffffe1f4  →  0x5245545f5353454c ("LESS_TER"?)
$r14   : 0x00007ffff7ffd000  →  0x00007ffff7ffe2c0  →  0x0000555555554000  →   jg 0x555555554047
$r15   : 0x0               
$eflags: [zero carry parity adjust sign trap INTERRUPT direction overflow resume virtualx86 identification]
$cs: 0x33 $ss: 0x2b $ds: 0x00 $es: 0x00 $fs: 0x00 $gs: 0x00 
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── stack ────
0x00007fffffffdad0│+0x0000: 0x00007ffff7e5b370  →  0x00007ffff7d3e020  →  <std::basic_ostream<wchar_t,+0> endbr64        ← $rsp
0x00007fffffffdad8│+0x0008: 0x00007ffff7e5b398  →  0x00007ffff7d3e060  →  <virtual+0> endbr64 
0x00007fffffffdae0│+0x0010: 0x000055555556b2d0  →  "picoCTF{<REDACTED>}"    ← $rax, $rdi
0x00007fffffffdae8│+0x0018: 0x0000000000000020 (" "?)
0x00007fffffffdaf0│+0x0020: 0x000000000000002e ("."?)
0x00007fffffffdaf8│+0x0028: 0x2f6b636568437463
0x00007fffffffdb00│+0x0030: 0x00007fffffffdb10  →  0x0000000000000033 ("3"?)
0x00007fffffffdb08│+0x0038: 0x0000000000000001
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── code:x86:64 ────
   0x555555555853 <main+1482>      mov    esi, 0x7d
   0x555555555858 <main+1487>      mov    rdi, rax
   0x55555555585b <main+1490>      call   0x555555555100 <_ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEpLEc@plt>
 → 0x555555555860 <main+1495>      mov    ebx, 0x0
   0x555555555865 <main+1500>      lea    rax, [rbp-0x40]
   0x555555555869 <main+1504>      mov    rdi, rax
   0x55555555586c <main+1507>      call   0x5555555550f0 <_ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEED1Ev@plt>
   0x555555555871 <main+1512>      lea    rax, [rbp-0x60]
   0x555555555875 <main+1516>      mov    rdi, rax
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── threads ────
[#0] Id 1, Name: "bin", stopped 0x555555555860 in main (), reason: BREAKPOINT
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────── trace ────
[#0] 0x555555555860 → main()
─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
gef➤  
```

For additional information, please see the references below.

## References

- [Executable and Linkable Format - Wikipedia](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gdb - Linux manual page](https://man7.org/linux/man-pages/man1/gdb.1.html)
- [GEF (GDB Enhanced Features) - Github](https://github.com/hugsy/gef)
- [GEF (GDB Enhanced Features) - Homepage](https://hugsy.github.io/gef/)
- [Ghidra - Homepage](https://ghidra-sre.org/)
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
