# Investigative Reversing 0

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2019, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: DANNY TUNITIS

Description:
We have recovered a binary and an image. See what you can make of it. 
There should be a flag somewhere.

Hints:
1. Try using some forensics skills on the image
2. This problem requires both forensics and reversing skills
3. A hex editor may be helpful
```

Challenge link: [https://play.picoctf.org/practice/challenge/70](https://play.picoctf.org/practice/challenge/70)

## Solution

### Analysing the files

Let's start by checking the image file `mystery.png`.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_0]
└─$ file mystery.png 
mystery.png: PNG image data, 1411 x 648, 8-bit/color RGB, non-interlaced

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_0]
└─$ exiftool mystery.png 
ExifTool Version Number         : 12.52
File Name                       : mystery-newest.png
Directory                       : .
File Size                       : 125 kB
File Modification Date/Time     : 2023:09:03 11:04:11-04:00
File Access Date/Time           : 2023:09:03 11:13:22-04:00
File Inode Change Date/Time     : 2023:09:03 11:04:11-04:00
File Permissions                : -rwxrwxrwx
File Type                       : PNG
File Type Extension             : png
MIME Type                       : image/png
Image Width                     : 1411
Image Height                    : 648
Bit Depth                       : 8
Color Type                      : RGB
Compression                     : Deflate/Inflate
Filter                          : Adaptive
Interlace                       : Noninterlaced
SRGB Rendering                  : Perceptual
Gamma                           : 2.2
Pixels Per Unit X               : 5669
Pixels Per Unit Y               : 5669
Pixel Units                     : meters
Warning                         : [minor] Trailer data after PNG IEND chunk
Image Size                      : 1411x648
Megapixels                      : 0.914
                                
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_0]
└─$ xxd mystery.png | tail -n 3
0001e860: ed5a 9d38 d01f 5600 0000 0049 454e 44ae  .Z.8..V....IEND.
0001e870: 4260 8270 6963 6f43 544b 806b 357a 7369  B`.picoCTK.k5zsi
0001e880: 6436 715f 6662 3531 6338 3231 7d         d6q_fb51c821}
```

There are some flag data appended at the end of the PNG image. Be aware that `xxd` shows all non 7-bit ascii-data as `.` (dots).  
So `0x80` in the flag data is shown as `.` rather than `€`.

Next, let's check the binary

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_0]
└─$ file mystery    
mystery-newest: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=34b772a4f30594e2f30ac431c72667c3e10fa3e9, not stripped
```

Then we decompile the file in [Ghidra](https://ghidra-sre.org/) and study the code. Import the file in Ghidra and analyze it with the default settings.  
Double-click on the `main` function to show the decompiled version of it.

```c
void main(void)

{
  FILE *__stream;
  FILE *__stream_00;
  size_t sVar1;
  long in_FS_OFFSET;
  int local_54;
  int local_50;
  char local_38 [4];
  char local_34;
  char local_33;
  char local_29;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __stream = fopen("flag.txt","r");
  __stream_00 = fopen("mystery.png","a");
  if (__stream == (FILE *)0x0) {
    puts("No flag found, please make sure this is run on the server");
  }
  if (__stream_00 == (FILE *)0x0) {
    puts("mystery.png is missing, please run this on the server");
  }
  sVar1 = fread(local_38,0x1a,1,__stream);
  if ((int)sVar1 < 1) {
                    /* WARNING: Subroutine does not return */
    exit(0);
  }
  puts("at insert");
  fputc((int)local_38[0],__stream_00);
  fputc((int)local_38[1],__stream_00);
  fputc((int)local_38[2],__stream_00);
  fputc((int)local_38[3],__stream_00);
  fputc((int)local_34,__stream_00);
  fputc((int)local_33,__stream_00);
  for (local_54 = 6; local_54 < 0xf; local_54 = local_54 + 1) {
    fputc((int)(char)(local_38[local_54] + '\x05'),__stream_00);
  }
  fputc((int)(char)(local_29 + -3),__stream_00);
  for (local_50 = 0x10; local_50 < 0x1a; local_50 = local_50 + 1) {
    fputc((int)local_38[local_50],__stream_00);
  }
  fclose(__stream_00);
  fclose(__stream);
  if (local_10 != *(long *)(in_FS_OFFSET + 0x28)) {
                    /* WARNING: Subroutine does not return */
    __stack_chk_fail();
  }
  return;
}
```

We see that the program opens the flag file and puts an encoded version of it at the end of the image file.  
The encoding consists of these lines of code

```c
  for (local_54 = 6; local_54 < 0xf; local_54 = local_54 + 1) {
    fputc((int)(char)(local_38[local_54] + '\x05'),__stream_00);
  }
  fputc((int)(char)(local_29 + -3),__stream_00);
  }
```

The rest of the bytes are copied 'as is'.

### Write a Python decoder

We can write a python script to reverse these operations

```python
#!/usr/bin/python

data_len = 0x1a
with open("mystery.png", 'rb') as f:
    enc_flag = bytearray(f.read()[-data_len:])

flag = ''
for i in range(0, 6):
    flag += chr(enc_flag[i])

# local_54 = 6;
# while (local_54 < 0xf) {
#    fputc((int)(char)(local_38[local_54] + '\x05'),__stream_00);
#    local_54 = local_54 + 1;
# }
for local_54 in range(6, 0xf):
    flag += chr(enc_flag[local_54] - 0x5)

# fputc((int)(char)(local_29 + -3),__stream_00);    
flag += chr(enc_flag[15] + 3)

# for (local_50 = 0x10; local_50 < 0x1a; local_50 = local_50 + 1) {
#    fputc((int)local_38[local_50],__stream_00);
#  }
for i in range(0x10, data_len):
    flag += chr(enc_flag[i])

print(flag)
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_0]
└─$ ./decode.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ExifTool - Homepage](https://exiftool.org/)
- [exiftool - Linux manual page](https://linux.die.net/man/1/exiftool)
- [ExifTool - Wikipedia](https://en.wikipedia.org/wiki/ExifTool)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [Ghidra - Homepage](https://ghidra-sre.org/)
- [PNG - Wikipedia](https://en.wikipedia.org/wiki/PNG)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [tail - Linux manual page](https://man7.org/linux/man-pages/man1/tail.1.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
