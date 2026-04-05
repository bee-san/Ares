# useless

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, General Skills, man
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LOIC SHEMA

Description:
There's an interesting script in the user's home directory

The work computer is running SSH. We've been given a script which performs some basic calculations, 
explore the script and find a flag.
 
Hostname: saturn.picoctf.net
Port:     55661
Username: picoplayer
Password: password
 
Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/384](https://play.picoctf.org/practice/challenge/384)

## Solution

Start by connecting to the server with SSH

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/General_Skills/useless]
└─$ ssh -p 55661 picoplayer@saturn.picoctf.net
The authenticity of host '[saturn.picoctf.net]:55661 ([13.59.203.175]:55661)' can't be established.
ED25519 key fingerprint is SHA256:ves7M6DhshpiJSsScBWo3n34oOFTUXvLZqPyqLWeTHk.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[saturn.picoctf.net]:55661' (ED25519) to the list of known hosts.
picoplayer@saturn.picoctf.net's password: 
Welcome to Ubuntu 20.04.6 LTS (GNU/Linux 5.19.0-1024-aws x86_64)

 * Documentation:  https://help.ubuntu.com
 * Management:     https://landscape.canonical.com
 * Support:        https://ubuntu.com/advantage

The programs included with the Ubuntu system are free software;
the exact distribution terms for each program are described in the
individual files in /usr/share/doc/*/copyright.

Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by
applicable law.

picoplayer@challenge:~$ 
```

The script should be located in our home directory so let's look for it

```bash
picoplayer@challenge:~$ ls -la
total 16
drwxr-xr-x 1 picoplayer picoplayer   20 Jul 28 14:39 .
drwxr-xr-x 1 root       root         24 Mar 16 02:30 ..
-rw-r--r-- 1 picoplayer picoplayer  220 Feb 25  2020 .bash_logout
-rw-r--r-- 1 picoplayer picoplayer 3771 Feb 25  2020 .bashrc
drwx------ 2 picoplayer picoplayer   34 Jul 28 14:39 .cache
-rw-r--r-- 1 picoplayer picoplayer  807 Feb 25  2020 .profile
-rwxr-xr-x 1 root       root        517 Mar 16 01:30 useless

picoplayer@challenge:~$ file useless
useless: Bourne-Again shell script, ASCII text executable

picoplayer@challenge:~$ cat useless
#!/bin/bash
# Basic mathematical operations via command-line arguments

if [ $# != 3 ]
then
  echo "Read the code first"
else
        if [[ "$1" == "add" ]]
        then 
          sum=$(( $2 + $3 ))
          echo "The Sum is: $sum"  

        elif [[ "$1" == "sub" ]]
        then 
          sub=$(( $2 - $3 ))
          echo "The Substract is: $sub" 

        elif [[ "$1" == "div" ]]
        then 
          div=$(( $2 / $3 ))
          echo "The quotient is: $div" 

        elif [[ "$1" == "mul" ]]
        then
          mul=$(( $2 * $3 ))
          echo "The product is: $mul" 

        else
          echo "Read the manual"
         
        fi
fi
```

Hhm, no flag there. But there is an instruction to read the manual and the challenge is also tagged with `man` so let's try that

```bash
picoplayer@challenge:~$ man useless

useless
     useless, — This is a simple calculator script

SYNOPSIS
     useless, [add sub mul div] number1 number2

DESCRIPTION
     Use the useless, macro to make simple calulations like addition,subtraction, multiplication and division.

Examples
     ./useless add 1 2
       This will add 1 and 2 and return 3

     ./useless mul 2 3
       This will return 6 as a product of 2 and 3

     ./useless div 6 3
       This will return 2 as a quotient of 6 and 3

     ./useless sub 6 5
       This will return 1 as a remainder of substraction of 5 from 6

Authors
     This script was designed and developed by Cylab Africa

     picoCTF{<REDACTED>}

```

Ah, the flag is included at the bottom of the man page.

For additional information, please see the references below.

## References

- [man page - Wikipedia](https://en.wikipedia.org/wiki/Man_page)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [Shell script - Wikipedia](https://en.wikipedia.org/wiki/Shell_script)
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
