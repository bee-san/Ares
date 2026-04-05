# Verify

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, Forensics, grep, browser_webshell_solvable, checksum
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JEFFERY JOHN

Description:
People keep trying to trick my players with imitation flags. I want to make sure they get 
the real thing! I'm going to provide the SHA-256 hash and a decrypt script to help you 
know that my flags are legitimate.

You can download the challenge files here:
challenge.zip

The same files are accessible via SSH here:
ssh -p 52722 ctf-player@rhea.picoctf.net
Using the password 83dcefb7. Accept the fingerprint with yes, and ls once connected to begin. 
Remember, in a shell, passwords are hidden!

Checksum: 467a10447deb3d4e17634cacc2a68ba6c2bb62a6637dad9145ea673bf0be5e02
To decrypt the file once you've verified the hash, run ./decrypt.sh files/<file>.

Hints:
1. Checksums let you tell if a file is complete and from the original distributor. 
   If the hash doesn't match, it's a different file.
2. You can create a SHA checksum of a file with sha256sum <file> or all files in a 
   directory with sha256sum <directory>/*.
3. Remember you can pipe the output of one command to another with |. 
   Try practicing with the 'First Grep' challenge if you're stuck!
```

Challenge link: [https://play.picoctf.org/practice/challenge/450](https://play.picoctf.org/practice/challenge/450)

## Solution

### Unpacking and basic analysis

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Verify]
└─$ unzip challenge.zip 
Archive:  challenge.zip
   creating: home/ctf-player/drop-in/
   creating: home/ctf-player/drop-in/files/
 extracting: home/ctf-player/drop-in/files/LmicJDs8  
 extracting: home/ctf-player/drop-in/files/c6c8b911  
 extracting: home/ctf-player/drop-in/files/3eJU0bPR  
 extracting: home/ctf-player/drop-in/files/EfRHiDLP  
 extracting: home/ctf-player/drop-in/files/DBQbeL0I  
 extracting: home/ctf-player/drop-in/files/sOhwN7cV  
 extracting: home/ctf-player/drop-in/files/iKj2d6J4  
 extracting: home/ctf-player/drop-in/files/EC1I5QwZ  
 extracting: home/ctf-player/drop-in/files/047MJYW7  
 extracting: home/ctf-player/drop-in/files/x1wlAOTr  
 extracting: home/ctf-player/drop-in/files/rK99ez1a  
 <---snip--->
```

We have a lot of files. Let's search for the given [hash](https://en.wikipedia.org/wiki/Cryptographic_hash_function).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Verify]
└─$ sha256sum home/ctf-player/drop-in/files/* | grep 467a10447deb3d4e17634cacc2a68ba6c2bb62a6637dad9145ea673bf0be5e02
467a10447deb3d4e17634cacc2a68ba6c2bb62a6637dad9145ea673bf0be5e02  home/ctf-player/drop-in/files/c6c8b911

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Verify]
└─$ cat home/ctf-player/drop-in/files/c6c8b911
Salted__���05�.Q�+�P��&pE�?B�{M:��e�Wm�a4Wua��l�5�yU!����NA�

```

The file is encrypted.

### Run the decryption script

Next, we try to decrypt the file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/Forensics/Verify]
└─$ cd home/ctf-player/drop-in/              

┌──(kali㉿kali)-[/mnt/…/Verify/home/ctf-player/drop-in]
└─$ cat decrypt.sh                            

        #!/bin/bash

        # Check if the user provided a file name as an argument
        if [ $# -eq 0 ]; then
            echo "Expected usage: decrypt.sh <filename>"
            exit 1
        fi

        # Store the provided filename in a variable
        file_name="$1"

        # Check if the provided argument is a file and not a folder
        if [ ! -f "/home/ctf-player/drop-in/$file_name" ]; then
            echo "Error: '$file_name' is not a valid file. Look inside the 'files' folder with 'ls -R'!"
            exit 1
        fi

        # If there's an error reading the file, print an error message
        if ! openssl enc -d -aes-256-cbc -pbkdf2 -iter 100000 -salt -in "/home/ctf-player/drop-in/$file_name" -k picoCTF; then
            echo "Error: Failed to decrypt '$file_name'. This flag is fake! Keep looking!"
        fi

┌──(kali㉿kali)-[/mnt/…/Verify/home/ctf-player/drop-in]
└─$ ./decrypt.sh c6c8b911
Error: 'c6c8b911' is not a valid file. Look inside the 'files' folder with 'ls -R'!

┌──(kali㉿kali)-[/mnt/…/Verify/home/ctf-player/drop-in]
└─$ ./decrypt.sh files/c6c8b911
Error: 'files/c6c8b911' is not a valid file. Look inside the 'files' folder with 'ls -R'!

┌──(kali㉿kali)-[/mnt/…/Verify/home/ctf-player/drop-in]
└─$ 
```

Hhm, the script is looking for the file in the absolute path of `/home/ctf-player/drop-in/`.  
That won't work in my case. Let's do the decryption without the script.

### Get the flag

Decrypt the flag with `openssl` like this

```bash
┌──(kali㉿kali)-[/mnt/…/Verify/home/ctf-player/drop-in]
└─$ openssl enc -d -aes-256-cbc -pbkdf2 -iter 100000 -salt -in files/c6c8b911 -k picoCTF
picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Cryptographic hash function - Wikipedia](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [openssl - Linux manual page](https://linux.die.net/man/1/openssl)
- [SHA-2 - Wikipedia](https://en.wikipedia.org/wiki/SHA-2)
- [sha256sum - Linux manual page](https://man7.org/linux/man-pages/man1/sha256sum.1.html)
