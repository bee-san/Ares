# ASCII FTW

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoGym Exclusive, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
This program has constructed the flag using hex ascii values. Identify the flag text by disassembling the program.

Hints:
1. The combined range of hex-ascii for English alphabets and numerical digits is from 30 to 7A.
2. Online hex-ascii converters can be helpful.
```

Challenge link: [https://play.picoctf.org/practice/challenge/389](https://play.picoctf.org/practice/challenge/389)

## Solution

Import the file in [Ghidra](https://ghidra-sre.org/) and analyze it with the default settings.  
Double-click on the `main` function to show the decompiled version of it.

```C
void main(void)

{
  long lVar1;
  long in_FS_OFFSET;
  
  lVar1 = *(long *)(in_FS_OFFSET + 0x28);
  printf("The flag starts with %x\n",0x70);
  if (lVar1 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}
```

The flag data is stored at memory position `in_FS_OFFSET`. Double-click to navigate to it in the Listing window.

```text
        0010117e 48 89 45 f8     MOV        qword ptr [RBP + local_10],RAX
        00101182 31 c0           XOR        EAX,EAX
        00101184 c6 45 d0 70     MOV        byte ptr [RBP + local_38],0x70
        00101188 c6 45 d1 69     MOV        byte ptr [RBP + local_37],0x69
        0010118c c6 45 d2 63     MOV        byte ptr [RBP + local_36],0x63
        00101190 c6 45 d3 6f     MOV        byte ptr [RBP + local_35],0x6f
        00101194 c6 45 d4 43     MOV        byte ptr [RBP + local_34],0x43
        00101198 c6 45 d5 54     MOV        byte ptr [RBP + local_33],0x54
        0010119c c6 45 d6 46     MOV        byte ptr [RBP + local_32],0x46
< --- snip --- >
```

The flag is stored as ASCII-values byte by byte. I.e. the values 0x70, 0x69, etc. 0x70 corresponds to 'p', 0x69 to 'i', etc.  The lookup can be done manually with an online [ASCII table](https://www.ascii-code.com/) or within Ghidra by right-clicking on each value and selecting `Convert -> Char` in the menu.

The result then looks like this:

```text
        0010117e 48 89 45 f8     MOV        qword ptr [RBP + local_10],RAX
        00101182 31 c0           XOR        EAX,EAX
        00101184 c6 45 d0 70     MOV        byte ptr [RBP + local_38],'p'
        00101188 c6 45 d1 69     MOV        byte ptr [RBP + local_37],'i'
        0010118c c6 45 d2 63     MOV        byte ptr [RBP + local_36],'c'
        00101190 c6 45 d3 6f     MOV        byte ptr [RBP + local_35],'o'
        00101194 c6 45 d4 43     MOV        byte ptr [RBP + local_34],'C'
        00101198 c6 45 d5 54     MOV        byte ptr [RBP + local_33],'T'
        0010119c c6 45 d6 46     MOV        byte ptr [RBP + local_32],'F'
        001011a0 c6 45 d7 7b     MOV        byte ptr [RBP + local_31],'{'
        001011a4 c6 45 d8 41     MOV        byte ptr [RBP + local_30],'A'
<---snip--->
        001011fc c6 45 ee 7d     MOV        byte ptr [RBP + local_1a],'}'
        00101200 0f b6 45 d0     MOVZX      EAX,byte ptr [RBP + local_38]
```

Finally, manually create the flag by going down the listing line by line.

For additional information, please see the references below.

## References

- [ASCII Table](https://www.asciitable.com/)
- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Ghidra - Homepage](https://ghidra-sre.org/)
