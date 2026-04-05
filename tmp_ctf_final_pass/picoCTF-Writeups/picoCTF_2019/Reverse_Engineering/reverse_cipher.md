# reverse_cipher

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY TUNITIS
 
Description:
We have recovered a binary and a text file. Can you reverse the flag.

Hints:
1. objdump and Gihdra are some tools that could assist with this
```

Challenge link: [https://play.picoctf.org/practice/challenge/79](https://play.picoctf.org/practice/challenge/79)

## Solution

### Analyse the setup

Let's start by checking what we were given

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Reverse_cipher]
└─$ file rev 
rev: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=523d51973c11197605c76f84d4afb0fe9e59338c, not stripped

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Reverse_cipher]
└─$ file rev_this 
rev_this: ASCII text, with no line terminators

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Reverse_cipher]
└─$ cat rev_this 
picoCTF{w1{1wq817/gbf/g}   
```

Ah, we've got an 64-bit ELF binary and a text-file with a scrambled flag.

Next, let's decompile the file in [Ghidra](https://ghidra-sre.org/) and study the code.
Import the file in Ghidra and analyze it with the default settings.  
Double-click on the `main` function to show the decompiled version of it.

```C
void main(void)

{
  size_t sVar1;
  char local_58 [23];
  char local_41;
  int local_2c;
  FILE *local_28;
  FILE *local_20;
  uint local_14;
  int local_10;
  char local_9;
  
  local_20 = fopen("flag.txt","r");
  local_28 = fopen("rev_this","a");
  if (local_20 == (FILE *)0x0) {
    puts("No flag found, please make sure this is run on the server");
  }
  if (local_28 == (FILE *)0x0) {
    puts("please run this on the server");
  }
  sVar1 = fread(local_58,0x18,1,local_20);
  local_2c = (int)sVar1;
  if ((int)sVar1 < 1) {
                    /* WARNING: Subroutine does not return */
    exit(0);
  }
  for (local_10 = 0; local_10 < 8; local_10 = local_10 + 1) {
    local_9 = local_58[local_10];
    fputc((int)local_9,local_28);
  }
  for (local_14 = 8; (int)local_14 < 0x17; local_14 = local_14 + 1) {
    if ((local_14 & 1) == 0) {
      local_9 = local_58[(int)local_14] + '\x05';
    }
    else {
      local_9 = local_58[(int)local_14] + -2;
    }
    fputc((int)local_9,local_28);
  }
  local_9 = local_41;
  fputc((int)local_41,local_28);
  fclose(local_28);
  fclose(local_20);
  return;
}
```

We can see that the code loops through the flag data and makes some minor modifications:

|Offset|Modification|
|----|----|
|0 - 7|None|
|8 - 0x16|Adds 5 for even offsets|
||Subtracts 2 for odd offset|
|0x17|None|

### Write a Python decoder

Let's write a small python script to re-create the flag

```python
#!/usr/bin/python

with open("rev_this", 'rb') as f:
    encoded_flag = bytearray(f.read())

flag = ''
# for (local_10 = 0; local_10 < 8; local_10 = local_10 + 1) {
#    local_9 = local_58[local_10];
#    fputc((int)local_9,local_28);
# }
for local_10 in range(0,8):
    flag += chr(encoded_flag[local_10])

# for (local_14 = 8; (int)local_14 < 0x17; local_14 = local_14 + 1) {
#    if ((local_14 & 1) == 0) {
#       local_9 = local_58[(int)local_14] + '\x05';
#    }
#    else {
#       local_9 = local_58[(int)local_14] + -2;
#    }
#    fputc((int)local_9,local_28);
#  }
for local_14 in range(8, 0x17):
    if local_14 % 2 == 0:
        flag += chr(encoded_flag[local_14] - 5)
    else:
        flag += chr(encoded_flag[local_14] + 2)

# local_9 = local_41;
# fputc((int)local_41,local_28);
flag += chr(encoded_flag[0x17])
print(flag)
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Reverse_cipher]
└─$ ./decode.py 
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [Ghidra - Homepage](https://ghidra-sre.org/)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
