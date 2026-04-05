# Sleuthkit Intro

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Forensics, sleuthkit
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Download the disk image and use mmls on it to find the size of the Linux partition. 

Connect to the remote checker service to check your answer and get the flag.

Note: if you are using the webshell, download and extract the disk image into /tmp not your home directory.
Download disk image

Access checker program: nc saturn.picoctf.net 52472

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/301](https://play.picoctf.org/practice/challenge/301)

## Solution

### Unpacking and file identification

Let's start by unpacking the disk image with `gzip -d`. Add `-k` if you want to keep the original input file.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ gzip -d -k disk.img.gz
gzip: disk.img: Value too large for defined data type
```

Then we can use `file` to identify the type of image

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ file disk.img   
disk.img: DOS/MBR boot sector; partition 1 : ID=0x83, active, start-CHS (0x0,32,33), end-CHS (0xc,190,50), startsector 2048, 202752 sectors
```

### Using mmls to find the partition siza

I don't know the specifics of `mmls` by heart so I need to consult the help

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ mmls -h
mmls: invalid option -- 'h'
Unknown argument
mmls [-i imgtype] [-b dev_sector_size] [-o imgoffset] [-BrvV] [-aAmM] [-t vstype] image [images]
        -t vstype: The type of volume system (use '-t list' for list of supported types)
        -i imgtype: The format of the image file (use '-i list' for list supported types)
        -b dev_sector_size: The size (in bytes) of the device sectors
        -o imgoffset: Offset to the start of the volume that contains the partition system (in sectors)
        -B: print the rounded length in bytes
        -r: recurse and look for other partition tables in partitions (DOS Only)
        -v: verbose output
        -V: print the version
Unless any of these are specified, all volume types are shown
        -a: Show allocated volumes
        -A: Show unallocated volumes
        -m: Show metadata volumes
        -M: Hide metadata volumes

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ mmls -i list
Supported image format types:
        raw (Single or split raw file (dd))
        aff (Advanced Forensic Format)
        afd (AFF Multiple File)
        afm (AFF with external metadata)
        afflib (All AFFLIB image formats (including beta ones))
        ewf (Expert Witness Format (EnCase))
        vmdk (Virtual Machine Disk (VmWare, Virtual Box))
        vhd (Virtual Hard Drive (Microsoft))
```

I'm not sure if you need to specify the image type or not but let's try without it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ mmls disk.img
DOS Partition Table
Offset Sector: 0
Units are in 512-byte sectors

      Slot      Start        End          Length       Description
000:  Meta      0000000000   0000000000   0000000001   Primary Table (#0)
001:  -------   0000000000   0000002047   0000002048   Unallocated
002:  000:000   0000002048   0000204799   0000202752   Linux (0x83)
```

Seemed to work fine and the size of the Linux partition is `202752` sectors.

### Connecting to the server

Now we can connect to the server and get our flag

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/Sleuthkit_Intro]
└─$ nc saturn.picoctf.net 52472
What is the size of the Linux partition in the given disk image?
Length in sectors: 202752
202752
Great work!
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gzip - Linux manual page](https://linux.die.net/man/1/gzip)
- [The Sleuth Kit commands](https://wiki.sleuthkit.org/index.php?title=The_Sleuth_Kit_commands)
