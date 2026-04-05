# File types

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: GEOFFREY NJOGU
 
Description:
This file was found among some files marked confidential but my pdf reader cannot read it, 
maybe yours can.

You can download the file from here.
 
Hints:
1. Remember that some file types can contain and nest other files
```

Challenge link: [https://play.picoctf.org/practice/challenge/268](https://play.picoctf.org/practice/challenge/268)

## Solution

### The so called PDF-file

Let's see what the `file` command have to say about what file it is

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file Flag.pdf 
Flag.pdf: shell archive text
```

Hhm, shell archive text. Let's see the first lines of it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ head Flag.pdf     
#!/bin/sh
# This is a shell archive (produced by GNU sharutils 4.15.2).
# To extract the files from this archive, save it to some FILE, remove
# everything before the '#!/bin/sh' line above, then type 'sh FILE'.
#
lock_dir=_sh00046
# Made on 2022-03-15 06:50 UTC by <root@2751d08abaab>.
# Source directory was '/app'.
#
# Existing files will *not* be overwritten, unless '-c' is specified.
```

Why not make it executable and then run it?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ chmod +x Flag.pdf      
                          
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ ./Flag.pdf  
x - created lock directory _sh00046.
x - extracting flag (text)
./Flag.pdf: 119: uudecode: not found
restore of flag failed
flag: MD5 check failed
x - removed lock directory _sh00046.
```

Oh, `uuencode` not found. We need to install sharutils.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ sudo apt-get install sharutils
[sudo] password for kali: 
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following packages were automatically installed and are no longer required:
  freeglut3 libatk1.0-data libev4 libexporter-tiny-perl libflac8 libfmt8 libgdal31 libgeos3.11.0 libgssdp-1.2-0 libgupnp-1.2-1 libhttp-server-simple-perl libilmbase25 liblerc3
  liblist-moreutils-perl liblist-moreutils-xs-perl libopenexr25 libopenh264-6 libperl5.34 libplacebo192 libpoppler118 libpython3.9-minimal libpython3.9-stdlib libsvtav1enc0 libwebsockets16
  libwireshark15 libwiretap12 libwsutil13 openjdk-11-jre perl-modules-5.34 python-pastedeploy-tpl python3-dataclasses-json python3-limiter python3-marshmallow-enum python3-mypy-extensions
  python3-ntlm-auth python3-requests-ntlm python3-responses python3-spyse python3-token-bucket python3-typing-inspect python3.9 python3.9-minimal
Use 'sudo apt autoremove' to remove them.
Suggested packages:
  bsd-mailx | mailx sharutils-doc
The following NEW packages will be installed:
  sharutils
0 upgraded, 1 newly installed, 0 to remove and 1532 not upgraded.
Need to get 262 kB of archives.
After this operation, 1,449 kB of additional disk space will be used.
Get:1 http://ftp.acc.umu.se/mirror/kali.org/kali kali-rolling/main amd64 sharutils amd64 1:4.15.2-9 [262 kB]
Fetched 262 kB in 1s (475 kB/s)   
Selecting previously unselected package sharutils.
(Reading database ... 390528 files and directories currently installed.)
Preparing to unpack .../sharutils_1%3a4.15.2-9_amd64.deb ...
Unpacking sharutils (1:4.15.2-9) ...
Setting up sharutils (1:4.15.2-9) ...
Processing triggers for man-db (2.11.1-1) ...
Processing triggers for kali-menu (2022.4.1) ...
```

Now let's try running the script again

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ ./Flag.pdf                    
x - created lock directory _sh00046.
x - extracting flag (text)
x - removed lock directory _sh00046.
```

A file called `flag` was extracted. What type of file is it?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag      
flag: current ar archive
```

### The ar archive file

I wonder if there is an `ar` command to unpack it?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ ar -h
Usage: ar [emulation options] [-]{dmpqrstx}[abcDfilMNoOPsSTuvV] [--plugin <name>] [member-name] [count] archive-file file...
       ar -M [<mri-script]
 commands:
  d            - delete file(s) from the archive
  m[ab]        - move file(s) in the archive
  p            - print file(s) found in the archive
  q[f]         - quick append file(s) to the archive
  r[ab][f][u]  - replace existing or insert new file(s) into the archive
  s            - act as ranlib
  t[O][v]      - display contents of the archive
  x[o]         - extract file(s) from the archive
 command specific modifiers:
  [a]          - put file(s) after [member-name]
  [b]          - put file(s) before [member-name] (same as [i])
  [D]          - use zero for timestamps and uids/gids (default)
  [U]          - use actual timestamps and uids/gids
  [N]          - use instance [count] of name
  [f]          - truncate inserted file names
  [P]          - use full path names when matching
  [o]          - preserve original dates
  [O]          - display offsets of files in the archive
  [u]          - only replace files that are newer than current archive contents
 generic modifiers:
  [c]          - do not warn if the library had to be created
  [s]          - create an archive index (cf. ranlib)
  [l <text> ]  - specify the dependencies of this library
  [S]          - do not build a symbol table
  [T]          - deprecated, use --thin instead
  [v]          - be verbose
  [V]          - display the version number
  @<file>      - read options from <file>
  --target=BFDNAME - specify the target object format as BFDNAME
  --output=DIRNAME - specify the output directory for extraction operations
  --record-libdeps=<text> - specify the dependencies of this library
  --thin       - make a thin archive
 optional:
  --plugin <p> - load the specified plugin
 emulation options: 
  No emulation specific options
ar: supported targets: elf64-x86-64 elf32-i386 elf32-iamcu elf32-x86-64 pei-i386 pe-x86-64 pei-x86-64 elf64-little elf64-big elf32-little elf32-big pe-bigobj-x86-64 pe-i386 pdb srec symbolsrec verilog tekhex binary ihex plugin
Report bugs to <https://sourceware.org/bugzilla/>
```

We then extract the contents with `ar x`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ ar x flag
```

What kind of file might 'flag' be?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag
flag: cpio archive 
```

### The cpio archive file

Like before, I wonder if there is an `cpio` command to unpack it?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cpio -h          
cpio: invalid option -- 'h'
Try 'cpio --help' or 'cpio --usage' for more information.
                                                     
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cpio --help
Usage: cpio [OPTION...] [destination-directory]
GNU `cpio' copies files to and from archives

Examples:
  # Copy files named in name-list to the archive
  cpio -o < name-list [> archive]
  # Extract files from the archive
  cpio -i [< archive]
  # Copy files named in name-list to destination-directory
  cpio -p destination-directory < name-list

 Main operation mode:
  -i, --extract              Extract files from an archive (run in copy-in
                             mode)
  -o, --create               Create the archive (run in copy-out mode)
  -p, --pass-through         Run in copy-pass mode
  -t, --list                 Print a table of contents of the input
<---snip--->
```

So we can extract files with `cpio -i`. Lets try that.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cpio -i flag

^C
```

Hhm, the command "hanged" almost like it expected something more and I had to CTRL-C out of it.

We could try redirecting the file instead

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cpio -i < flag
cpio: flag not created: newer or same age version exists
2 blocks
```

Better, but we get a new error instead. `cpio` doesn't like that there is a file called flag already present.
Let's rename our file and try again

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ mv flag flag.cpio    
                                                             
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cpio -i < flag.cpio
2 blocks
```

Excellent. Now we check what type of file we have got this time

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag 
flag: bzip2 compressed data, block size = 900k 
```

### The bzip2 file

Again check if there is a `bzip2` command and what parameters we need to unpack

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ bzip2 -h
bzip2, a block-sorting file compressor.  Version 1.0.8, 13-Jul-2019.

   usage: bzip2 [flags and input files in any order]

   -h --help           print this message
   -d --decompress     force decompression
   -z --compress       force compression
   -k --keep           keep (don't delete) input files
   -f --force          overwrite existing output files
   -t --test           test compressed file integrity
   -c --stdout         output to standard out
   -q --quiet          suppress noncritical error messages
   -v --verbose        be verbose (a 2nd -v gives more)
   -L --license        display software version & license
   -V --version        display software version & license
   -s --small          use less memory (at most 2500k)
   -1 .. -9            set block size to 100k .. 900k
   --fast              alias for -1
   --best              alias for -9

   If invoked as `bzip2', default action is to compress.
              as `bunzip2',  default action is to decompress.
              as `bzcat', default action is to decompress to stdout.

   If no file names are given, bzip2 compresses or decompresses
   from standard input to standard output.  You can combine
   short flags, so `-v -4' means the same as -v4 or -4v, &c. 
```

Unpack with `bzip2 -d`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ bzip2 -d flag
bzip2: Can't guess original name for flag -- using flag.out 
```

What kind of file is 'flag.out' then?

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag.out
flag.out: gzip compressed data, was "flag", last modified: Tue Mar 15 06:50:39 2022, from Unix, original size modulo 2^32 328
```

### The gzip file

Check for a `gzip` command and what parameters we need to unpack

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ gzip -h                                                                                      
Usage: gzip [OPTION]... [FILE]...
Compress or uncompress FILEs (by default, compress FILES in-place).

Mandatory arguments to long options are mandatory for short options too.

  -c, --stdout      write on standard output, keep original files unchanged
  -d, --decompress  decompress
  -f, --force       force overwrite of output file and compress links
  -h, --help        give this help
  -k, --keep        keep (don't delete) input files
  -l, --list        list compressed file contents
  -L, --license     display software license
  -n, --no-name     do not save or restore the original name and timestamp
  -N, --name        save or restore the original name and timestamp
  -q, --quiet       suppress all warnings
  -r, --recursive   operate recursively on directories
      --rsyncable   make rsync-friendly archive
  -S, --suffix=SUF  use suffix SUF on compressed files
      --synchronous synchronous output (safer if system crashes, but slower)
  -t, --test        test compressed file integrity
  -v, --verbose     verbose mode
  -V, --version     display version number
  -1, --fast        compress faster
  -9, --best        compress better

With no FILE, or when FILE is -, read standard input.

Report bugs to <bug-gzip@gnu.org>.
```

Unpack with `gzip -d`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ gzip -d flag.out 
gzip: flag.out: unknown suffix -- ignored  
```

Ah, that's right. Gzip is one of those programs that expect a certain file extension.  
Rename the file and try again

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ mv flag.out flag.gz

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ gzip -d flag.gz    
gzip: flag: Value too large for defined data type
```

We got some kind of warning but there is a new flag file

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag    
flag: lzip compressed data, version: 1
```

### The lzip file

You know the drill, check for a `lzip` command and what parameters we need to unpack

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzip -h
Command 'lzip' not found, did you mean:
  command 'rzip' from deb rzip
  command 'mzip' from deb mtools
  command 'zip' from deb zip
  command 'clzip' from deb clzip
  command 'plzip' from deb plzip
  command 'lzmp' from deb lzma
  command 'gzip' from deb gzip
  command 'lzop' from deb lzop
  command 'wzip' from deb wzip
  command 'lrzip' from deb lrzip
Try: sudo apt install <deb name>
```

Nope, no `lzip` command present. We need to install it.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ sudo apt-get install lzip
[sudo] password for kali: 
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following packages were automatically installed and are no longer required:
  freeglut3 libatk1.0-data libev4 libexporter-tiny-perl libflac8 libfmt8 libgdal31 libgeos3.11.0 libgssdp-1.2-0 libgupnp-1.2-1 libhttp-server-simple-perl libilmbase25 liblerc3
  liblist-moreutils-perl liblist-moreutils-xs-perl libopenexr25 libopenh264-6 libperl5.34 libplacebo192 libpoppler118 libpython3.9-minimal libpython3.9-stdlib libsvtav1enc0 libwebsockets16
  libwireshark15 libwiretap12 libwsutil13 openjdk-11-jre perl-modules-5.34 python-pastedeploy-tpl python3-dataclasses-json python3-limiter python3-marshmallow-enum python3-mypy-extensions
  python3-ntlm-auth python3-requests-ntlm python3-responses python3-spyse python3-token-bucket python3-typing-inspect python3.9 python3.9-minimal
Use 'sudo apt autoremove' to remove them.
The following NEW packages will be installed:
  lzip
0 upgraded, 1 newly installed, 0 to remove and 1532 not upgraded.
Need to get 86.9 kB of archives.
After this operation, 179 kB of additional disk space will be used.
Get:1 http://ftp.acc.umu.se/mirror/kali.org/kali kali-rolling/main amd64 lzip amd64 1.23-6 [86.9 kB]
Fetched 86.9 kB in 1s (168 kB/s)
Selecting previously unselected package lzip.
(Reading database ... 390574 files and directories currently installed.)
Preparing to unpack .../archives/lzip_1.23-6_amd64.deb ...
Unpacking lzip (1.23-6) ...
Setting up lzip (1.23-6) ...
update-alternatives: using /usr/bin/lzip.lzip to provide /usr/bin/lzip (lzip) in auto mode
update-alternatives: using /usr/bin/lzip.lzip to provide /usr/bin/lzip-compressor (lzip-compressor) in auto mode
update-alternatives: using /usr/bin/lzip.lzip to provide /usr/bin/lzip-decompressor (lzip-decompressor) in auto mode
Processing triggers for man-db (2.11.1-1) ...
Processing triggers for kali-menu (2022.4.1) ... 
```

Try asking for help again

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzip -h
Lzip is a lossless data compressor with a user interface similar to the one
of gzip or bzip2. Lzip uses a simplified form of the 'Lempel-Ziv-Markov
chain-Algorithm' (LZMA) stream format and provides a 3 factor integrity
checking to maximize interoperability and optimize safety. Lzip can compress
about as fast as gzip (lzip -0) or compress most files more than bzip2
(lzip -9). Decompression speed is intermediate between gzip and bzip2.
Lzip is better than gzip and bzip2 from a data recovery perspective. Lzip
has been designed, written, and tested with great care to replace gzip and
bzip2 as the standard general-purpose compressed format for unix-like
systems.

Usage: lzip [options] [files]

Options:
  -h, --help                     display this help and exit
  -V, --version                  output version information and exit
  -a, --trailing-error           exit with error status if trailing data
  -b, --member-size=<bytes>      set member size limit in bytes
  -c, --stdout                   write to standard output, keep input files
  -d, --decompress               decompress
  -f, --force                    overwrite existing output files
  -F, --recompress               force re-compression of compressed files
  -k, --keep                     keep (don't delete) input files
  -l, --list                     print (un)compressed file sizes
<---snip--->
```

Unpack with `lzip -d`

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzip -d flag    
```

Now there was some slight confusion about what output file it created but it was 'flag.out'

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file flag.out
flag.out: LZ4 compressed data (v1.4+) 
```

### The LZ4 file

Check for a `lz4` command

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lz4 -h      
Command 'lz4' not found, but can be installed with:
sudo apt install lz4
Do you want to install it? (N/y)y
sudo apt install lz4
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following packages were automatically installed and are no longer required:
  freeglut3 libatk1.0-data libev4 libexporter-tiny-perl libflac8 libfmt8 libgdal31 libgeos3.11.0 libgssdp-1.2-0 libgupnp-1.2-1 libhttp-server-simple-perl libilmbase25 liblerc3
  liblist-moreutils-perl liblist-moreutils-xs-perl libopenexr25 libopenh264-6 libperl5.34 libplacebo192 libpoppler118 libpython3.9-minimal libpython3.9-stdlib libsvtav1enc0 libwebsockets16
  libwireshark15 libwiretap12 libwsutil13 openjdk-11-jre perl-modules-5.34 python-pastedeploy-tpl python3-dataclasses-json python3-limiter python3-marshmallow-enum python3-mypy-extensions
  python3-ntlm-auth python3-requests-ntlm python3-responses python3-spyse python3-token-bucket python3-typing-inspect python3.9 python3.9-minimal
Use 'sudo apt autoremove' to remove them.
The following NEW packages will be installed:
  lz4
0 upgraded, 1 newly installed, 0 to remove and 1532 not upgraded.
Need to get 92.7 kB of archives.
After this operation, 248 kB of additional disk space will be used.
Get:1 http://ftp.acc.umu.se/mirror/kali.org/kali kali-rolling/main amd64 lz4 amd64 1.9.4-1 [92.7 kB]
Fetched 92.7 kB in 1s (174 kB/s)
Selecting previously unselected package lz4.
(Reading database ... 390584 files and directories currently installed.)
Preparing to unpack .../archives/lz4_1.9.4-1_amd64.deb ...
Unpacking lz4 (1.9.4-1) ...
Setting up lz4 (1.9.4-1) ...
Processing triggers for man-db (2.11.1-1) ...
Processing triggers for kali-menu (2022.4.1) ...
```

Check unpack parameters

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lz4 -h
*** LZ4 command line interface 64-bits v1.9.4, by Yann Collet ***
Usage : 
      lz4 [arg] [input] [output] 

input   : a filename 
          with no FILE, or when FILE is - or stdin, read standard input
Arguments : 
 -1     : Fast compression (default) 
 -9     : High compression 
 -d     : decompression (default for .lz4 extension)
 -z     : force compression 
 -D FILE: use FILE as dictionary 
 -f     : overwrite output without prompting 
<---snip---> 
```

Unpack with a new specifed output file called 'newflag'

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lz4 -d flag.out newflag
flag.out             : decoded 266 bytes                                       

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file newflag 
newflag: LZMA compressed data, non-streamed, size 254
```

### The LZMA file

This is really starting to get tedious now, but like before let's check for a `lzma` command

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzma -h                
Usage: lzma [OPTION]... [FILE]...
Compress or decompress FILEs in the .xz format.

  -z, --compress      force compression
  -d, --decompress    force decompression
  -t, --test          test compressed file integrity
  -l, --list          list information about .xz files
  -k, --keep          keep (don't delete) input files
  -f, --force         force overwrite of output file and (de)compress links
  -c, --stdout        write to standard output and don't delete input files
  -0 ... -9           compression preset; default is 6; take compressor *and*
                      decompressor memory usage into account before using 7-9!
  -e, --extreme       try to improve compression ratio by using more CPU time;
                      does not affect decompressor memory requirements
  -T, --threads=NUM   use at most NUM threads; the default is 1; set to 0
                      to use as many threads as there are processor cores
  -q, --quiet         suppress warnings; specify twice to suppress errors too
  -v, --verbose       be verbose; specify twice for even more verbose
  -h, --help          display this short help and exit
  -H, --long-help     display the long help (lists also the advanced options)
  -V, --version       display the version number and exit

With no FILE, or when FILE is -, read standard input.

Report bugs to <xz@tukaani.org> (in English or Finnish).
XZ Utils home page: <https://tukaani.org/xz/>
```

Unpack with `lzma -d` etc.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzma -d newflag 
lzma: newflag: Filename has an unknown suffix, skipping

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ mv newflag newflag.lzma

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzma -d newflag.lzma   
lzma: newflag: Cannot set the file permissions: Value too large for defined data type

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file newflag
newflag: lzop compressed data - version 1.040, LZO1X-1, os: Unix
```

### The lzop file

More of the same so I'm limiting my comments now.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzop -h             
Command 'lzop' not found, but can be installed with:
sudo apt install lzop
Do you want to install it? (N/y)y
sudo apt install lzop
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following packages were automatically installed and are no longer required:
  freeglut3 libatk1.0-data libev4 libexporter-tiny-perl libflac8 libfmt8 libgdal31 libgeos3.11.0 libgssdp-1.2-0 libgupnp-1.2-1 libhttp-server-simple-perl libilmbase25 liblerc3
  liblist-moreutils-perl liblist-moreutils-xs-perl libopenexr25 libopenh264-6 libperl5.34 libplacebo192 libpoppler118 libpython3.9-minimal libpython3.9-stdlib libsvtav1enc0 libwebsockets16
  libwireshark15 libwiretap12 libwsutil13 openjdk-11-jre perl-modules-5.34 python-pastedeploy-tpl python3-dataclasses-json python3-limiter python3-marshmallow-enum python3-mypy-extensions
  python3-ntlm-auth python3-requests-ntlm python3-responses python3-spyse python3-token-bucket python3-typing-inspect python3.9 python3.9-minimal
Use 'sudo apt autoremove' to remove them.
The following NEW packages will be installed:
  lzop
0 upgraded, 1 newly installed, 0 to remove and 1532 not upgraded.
Need to get 84.2 kB of archives.
After this operation, 168 kB of additional disk space will be used.
Get:1 http://ftp.acc.umu.se/mirror/kali.org/kali kali-rolling/main amd64 lzop amd64 1.04-2 [84.2 kB]
Fetched 84.2 kB in 0s (176 kB/s)
Selecting previously unselected package lzop.
(Reading database ... 390595 files and directories currently installed.)
Preparing to unpack .../archives/lzop_1.04-2_amd64.deb ...
Unpacking lzop (1.04-2) ...
Setting up lzop (1.04-2) ...
Processing triggers for man-db (2.11.1-1) ...
Processing triggers for kali-menu (2022.4.1) ...

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzop -d newflag
lzop: newflag: unknown suffix -- ignored
skipping newflag [newflag.raw]
    
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ mv newflag newflag.lzop

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzop -d newflag.lzop   
                                                                
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file newflag
newflag: lzip compressed data, version: 1
```

### The other lzip file

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ lzip -d newflag

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file newflag.out 
newflag.out: XZ compressed data, checksum CRC64
```

### The xz file

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ xz -h     
Usage: xz [OPTION]... [FILE]...
Compress or decompress FILEs in the .xz format.

  -z, --compress      force compression
  -d, --decompress    force decompression
  -t, --test          test compressed file integrity
  -l, --list          list information about .xz files
  -k, --keep          keep (don't delete) input files
  -f, --force         force overwrite of output file and (de)compress links
  -c, --stdout        write to standard output and don't delete input files
  -0 ... -9           compression preset; default is 6; take compressor *and*
                      decompressor memory usage into account before using 7-9!
  -e, --extreme       try to improve compression ratio by using more CPU time;
                      does not affect decompressor memory requirements
  -T, --threads=NUM   use at most NUM threads; the default is 1; set to 0
                      to use as many threads as there are processor cores
  -q, --quiet         suppress warnings; specify twice to suppress errors too
  -v, --verbose       be verbose; specify twice for even more verbose
  -h, --help          display this short help and exit
  -H, --long-help     display the long help (lists also the advanced options)
  -V, --version       display the version number and exit

With no FILE, or when FILE is -, read standard input.

Report bugs to <xz@tukaani.org> (in English or Finnish).
XZ Utils home page: <https://tukaani.org/xz/>

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ xz -d newflag.out 
xz: newflag.out: Filename has an unknown suffix, skipping

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ mv newflag.out newflag.xz

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ xz -d newflag.xz         
xz: newflag: Cannot set the file permissions: Value too large for defined data type

┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ file newflag    
newflag: ASCII text
```

### The encoded flag

And finally we have something that is close to the flag.

Let's see it

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ cat newflag
7069636f4354467b66316c656e406d335f6d406e3170756c407431306e5f
6630725f3062326375723137795f37396230316332367d0a
```

It looks like hex-encoded data. We can use Python to decode it.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Forensics/File_Types]
└─$ python                                                 
Python 3.10.9 (main, Dec  7 2022, 13:47:07) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> enc_flag = "7069636f4354467b66316c656e406d335f6d406e3170756c407431306e5f6630725f3062326375723137795f37396230316332367d0a"
>>> bytes.fromhex(enc_flag).decode()
'picoCTF{<REDACTED>}\n'
```

For additional information, please see the references below.

## References

- [ar - Linux manual page](https://man7.org/linux/man-pages/man1/ar.1.html)
- [bzip2 - Linux manual page](https://linux.die.net/man/1/bzip2)
- [cpio - Linux manual page](https://linux.die.net/man/1/cpio)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [gzip - Linux manual page](https://linux.die.net/man/1/gzip)
- [head - Linux manual page](https://man7.org/linux/man-pages/man1/head.1.html)
- [lzip - Linux manual page](https://linux.die.net/man/1/lzip)
- [lzma - Linux manual page](https://linux.die.net/man/1/lzma)
- [lzop - Linux manual page](https://linux.die.net/man/1/lzop)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [xz - Linux manual page](https://linux.die.net/man/1/xz)
