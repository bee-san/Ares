# Pitter, Patter, Platters

- [Challenge information](#challenge-information)
- [Sleuth Kit solution](#sleuth-kit-solution)
- [FTK Imager solution](#ftk-imager-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2020 Mini-Competition, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL

Description:
'Suspicious' is written all over this disk image. 
Download suspicious.dd.sda1

Hints:
1. It may help to analyze this image in multiple ways: 
   as a blob, and as an actual mounted disk.
2. Have you heard of slack space? There is a certain set of tools 
   that now come with Ubuntu that I'd recommend for examining that 
   disk space phenomenon...
```

Challenge link: [https://play.picoctf.org/practice/challenge/87](https://play.picoctf.org/practice/challenge/87)

## Sleuth Kit Solution

### Basic analysis of the file (system)

We start with some basic analysis of the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ file suspicious.dd.sda1                         
suspicious.dd.sda1: Linux rev 1.0 ext3 filesystem data, UUID=fc168af0-183b-4e53-bdf3-9c1055413b40 (needs journal recovery)
```

We can use the `fsstat` tool from [The Sleuth Kit (TSK)](https://wiki.sleuthkit.org/index.php?title=TSK_Tool_Overview) to get more information on the file system

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ fsstat suspicious.dd.sda1                     
FILE SYSTEM INFORMATION
--------------------------------------------
File System Type: Ext3
Volume Name: 
Volume ID: 403b4155109cf3bd534e3b18f08a16fc

Last Written at: 2020-09-30 11:26:26 (CEST)
Last Checked at: 2015-11-11 20:55:28 (CET)

Last Mounted at: 2020-09-30 11:26:26 (CEST)
Unmounted properly
Last mounted on: /mnt/sda1

Source OS: Linux
Dynamic Structure
Compat Features: Journal, Ext Attributes, Resize Inode, Dir Index
InCompat Features: Filetype, Needs Recovery, 
Read Only Compat Features: Sparse Super, 

Journal ID: 00
Journal Inode: 8

METADATA INFORMATION
--------------------------------------------
Inode Range: 1 - 8033
Root Directory: 2
Free Inodes: 7962

<---snip--->

Group: 3:
  Inode Range: 6025 - 8032
  Block Range: 24577 - 32095
  Layout:
    Super Block: 24577 - 24577
    Group Descriptor Table: 24578 - 24578
    Data bitmap: 24704 - 24704
    Inode bitmap: 24705 - 24705
    Inode Table: 24706 - 24956
    Data Blocks: 24957 - 32095
  Free Inodes: 2008 (100%)
  Free Blocks: 5123 (68%)
  Total Directories: 0
```

But that doesn't give us much useful information, other than the type of file system (`Ext3`).

### List the files on the file system

Next, we use the `fls` tool from TSK to recursively list the files on the file system

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ fls -F -r suspicious.dd.sda1               
r/r 2013:       boot/grub/e2fs_stage1_5
r/r 2014:       boot/grub/fat_stage1_5
r/r 2015:       boot/grub/ffs_stage1_5
r/r 2016:       boot/grub/iso9660_stage1_5
r/r 2017:       boot/grub/jfs_stage1_5
<---snip--->
r/r 4052:       tce/optional/glib2.tcz.dep
r/r 4053:       tce/optional/libffi.tcz.md5.txt
r/r 4054:       tce/optional/libffi.tcz
r/r 4055:       tce/optional/glib2.tcz.md5.txt
r/r 4056:       tce/optional/glib2.tcz
r/r 4021:       tce/onboot.lst
r/r 12: suspicious-file.txt
```

Ah, the final file called `suspicious-file.txt` looks suspicious... ;-)

Let's get the contents of the file with `icat`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ icat suspicious.dd.sda1 12       
Nothing to see here! But you may want to look here -->
```

Hm, no flag there. But one of the hints mentioned slask space so we need to check that as well.

### Check the slack space

With the tool `istat` we can get more information on the [inode](https://en.wikipedia.org/wiki/Inode) in question

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ istat suspicious.dd.sda1 12
inode: 12
Allocated
Group: 0
Generation Id: 3646402035
uid / gid: 0 / 0
mode: rrw-r--r--
size: 55
num of links: 1

Inode Times:
Accessed:       2020-09-30 15:15:59 (CEST)
File Modified:  2020-09-30 07:17:23 (CEST)
Inode Modified: 2020-09-30 15:15:54 (CEST)

Direct Blocks:
2049
```

The inode number 12 points to block number 2049. We need to check that next.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ blkcat suspicious.dd.sda1 2049
Nothing to see here! But you may want to look here -->
}<REDACTED>{FTCocip
```

Ah, there we have the flag - but in reverse.

### Get the flag

Finally, we use `rev` to get the flag in a more readable format

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2020_Mini-Comp/Forensics/Pitter,_Patter,_Platters]
└─$ blkcat suspicious.dd.sda1 2049 | rev               
>-- ereh kool ot tnaw yam uoy tuB !ereh ees ot gnihtoN
picoCTF{<REDACTED>} 
```

## FTK Imager solution

Alternatively, we can open the file in [FTK Imager](https://www.exterro.com/ftk-imager):

1. In the `File`-menu, select `Add Evidence Item...`
2. Select the `Image File` option in the popup window
3. Browse to the `suspicious.dd.sda1` file

In the `Evidence Tree` to the left expand `suspicious.dd.sda1`, `NONAME [Ext3]` and `[root]`.  
Then select the `[root]` node.

Note that there are both a `suspicious-file.txt` file and a `suspicious-file.txt.FileSlack` file slack space in the `File List` to the right.

Next, click on `suspicious-file.txt.FileSlack` to get the flag in reverse.

Finally, use [CyberChef](https://gchq.github.io/CyberChef/) or the `rev` command to get the flag.

For additional information, please see the references below.

## References

- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Ext3 - Wikipedia](https://en.wikipedia.org/wiki/Ext3)
- [File system fragmentation - Wikipedia](https://en.wikipedia.org/wiki/File_system_fragmentation)
- [FTK Imager - Homepage](https://www.exterro.com/ftk-imager)
- [inode - Wikipedia](https://en.wikipedia.org/wiki/Inode)
- [rev - Linux manual page](https://man7.org/linux/man-pages/man1/rev.1.html)
- [Sleuthkit - Homepage](https://www.sleuthkit.org/sleuthkit/)
- [Sleuthkit - Kali Tools](https://www.kali.org/tools/sleuthkit/)
- [Sleuthkit - Tool Overview](https://wiki.sleuthkit.org/index.php?title=TSK_Tool_Overview)
