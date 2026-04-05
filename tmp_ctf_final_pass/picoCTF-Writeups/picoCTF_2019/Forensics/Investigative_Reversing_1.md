# Investigative Reversing 1

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
We have recovered a binary and a few images: image, image2, image3. 
See what you can make of it. There should be a flag somewhere.

Hints:
1. Try using some forensics skills on the image
2. This problem requires both forensics and reversing skills
3. A hex editor may be helpful
```

Challenge link: [https://play.picoctf.org/practice/challenge/27](https://play.picoctf.org/practice/challenge/27)

## Solution

### Analysing the files

Let's start by checking the image files

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ exiftool mystery*  
======== mystery2.png
ExifTool Version Number         : 12.52
File Name                       : mystery2.png
Directory                       : .
File Size                       : 125 kB
File Modification Date/Time     : 2023:12:01 08:35:51-05:00
File Access Date/Time           : 2023:12:01 08:35:51-05:00
File Inode Change Date/Time     : 2023:12:01 08:35:51-05:00
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
======== mystery3.png
ExifTool Version Number         : 12.52
File Name                       : mystery3.png
Directory                       : .
File Size                       : 125 kB
File Modification Date/Time     : 2023:12:01 08:35:53-05:00
File Access Date/Time           : 2023:12:01 08:35:53-05:00
File Inode Change Date/Time     : 2023:12:01 08:35:53-05:00
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
======== mystery.png
ExifTool Version Number         : 12.52
File Name                       : mystery.png
Directory                       : .
File Size                       : 125 kB
File Modification Date/Time     : 2023:12:01 08:35:50-05:00
File Access Date/Time           : 2023:12:01 08:36:01-05:00
File Inode Change Date/Time     : 2023:12:01 08:35:50-05:00
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
    3 image files read
```

As in the [previous challenge](Investigative_Reversing_0.md), there are flag data appended at the end of the PNG images.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ xxd mystery.png | tail -n 3
0001e860: ed5a 9d38 d01f 5600 0000 0049 454e 44ae  .Z.8..V....IEND.
0001e870: 4260 8243 467b 416e 315f 3961 3437 3134  B`.CF{An1_9a4714
0001e880: 317d 60                                  1}`

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ xxd mystery2.png | tail -n 3
0001e850: 8220 0882 2008 8220 6417 ffef fffd 7f5e  . .. .. d......^
0001e860: ed5a 9d38 d01f 5600 0000 0049 454e 44ae  .Z.8..V....IEND.
0001e870: 4260 8285 73                             B`..s

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ xxd mystery3.png | tail -n 3
0001e850: 8220 0882 2008 8220 6417 ffef fffd 7f5e  . .. .. d......^
0001e860: ed5a 9d38 d01f 5600 0000 0049 454e 44ae  .Z.8..V....IEND.
0001e870: 4260 8269 6354 3074 6861 5f              B`.icT0tha_
```

Next, let's check the binary

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ file mystery                                                                                                                      
mystery: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=1b08f7a782a77a6eeb80d7c1d621b4f16f76200a, not stripped
```

Then we decompile the file in [Ghidra](https://ghidra-sre.org/) and study the code. Import the file in Ghidra and analyze it with the default settings.  
Double-click on the `main` function to show the decompiled version of it.

```c
void main(void)

{
  FILE *__stream;
  FILE *__stream_00;
  FILE *__stream_01;
  FILE *__stream_02;
  long in_FS_OFFSET;
  char local_6b;
  int local_68;
  int local_64;
  int local_60;
  char local_38 [4];
  char local_34;
  char local_33;
  long local_10;
  
  local_10 = *(long *)(in_FS_OFFSET + 0x28);
  __stream = fopen("flag.txt","r");
  __stream_00 = fopen("mystery.png","a");
  __stream_01 = fopen("mystery2.png","a");
  __stream_02 = fopen("mystery3.png","a");
  if (__stream == (FILE *)0x0) {
    puts("No flag found, please make sure this is run on the server");
  }
  if (__stream_00 == (FILE *)0x0) {
    puts("mystery.png is missing, please run this on the server");
  }
  fread(local_38,0x1a,1,__stream);
  fputc((int)local_38[1],__stream_02);
  fputc((int)(char)(local_38[0] + '\x15'),__stream_01);
  fputc((int)local_38[2],__stream_02);
  local_6b = local_38[3];
  fputc((int)local_33,__stream_02);
  fputc((int)local_34,__stream_00);
  for (local_68 = 6; local_68 < 10; local_68 = local_68 + 1) {
    local_6b = local_6b + '\x01';
    fputc((int)local_38[local_68],__stream_00);
  }
  fputc((int)local_6b,__stream_01);
  for (local_64 = 10; local_64 < 0xf; local_64 = local_64 + 1) {
    fputc((int)local_38[local_64],__stream_02);
  }
  for (local_60 = 0xf; local_60 < 0x1a; local_60 = local_60 + 1) {
    fputc((int)local_38[local_60],__stream_00);
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

We can see that the program opens the flag file and appends an encoded version of it at the end of the image files.  

### Write a Python decoder

We can write a python script to reverse these operations

```python
#!/usr/bin/python

class File_Object(object):

    def __init__(self, file_name):
        self.file = open(file_name, 'rb')
        self.file_contents = self.file.read()
        # Add 8 bytes for length of IEND tag + CRC tag
        self.data = self.file_contents[self.file_contents.find(b'IEND') + 8:]
        self.offset = 0
        
    def read_byte(self):
        b = self.data[self.offset]
        self.offset += 1
        return b
        
    def __del__(self):
        self.file.close()

stream_00 = File_Object("mystery.png")
stream_01 = File_Object("mystery2.png")
stream_02 = File_Object("mystery3.png")

flag = bytearray(0x1a)

# fputc((int)local_38[1],__stream_02);
flag[1] = stream_02.read_byte()

# fputc((int)(char)(local_38[0] + '\x15'),__stream_01);
flag[0] = stream_01.read_byte() - 0x15

# fputc((int)local_38[2],__stream_02);
flag[2] = stream_02.read_byte()
  
# fputc((int)local_33,__stream_02);
flag[5] = stream_02.read_byte()

# fputc((int)local_34,__stream_00);
flag[4] = stream_00.read_byte()

# for (local_68 = 6; local_68 < 10; local_68 = local_68 + 1) {
#    local_6b = local_6b + '\x01';
#    fputc((int)local_38[local_68],__stream_00);
# }
for i in range(6, 10):
    flag[i] = stream_00.read_byte()

# local_6b = local_38[3];   and
# fputc((int)local_6b,__stream_01);
flag[3] = stream_01.read_byte() - (10 - 6)

# for (local_64 = 10; local_64 < 0xf; local_64 = local_64 + 1) {
#    fputc((int)local_38[local_64],__stream_02);
# }
for i in range(10, 0xf):
    flag[i] = stream_02.read_byte()

#  for (local_60 = 0xf; local_60 < 0x1a; local_60 = local_60 + 1) {
#    fputc((int)local_38[local_60],__stream_00);
#  }
for i in range(0xf, 0x1a):
    flag[i] = stream_00.read_byte()

print(flag.decode())
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Forensics/Investigating_Reversing_1]
└─$ ./decode.py
picoCTF{<REDACTED>}`
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
