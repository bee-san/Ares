# hashcrack

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Cryptography, picoCTF 2025, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: Nana Ama Atombo-Sackey
 
Description:
A company stored a secret message on a server which got breached due to the admin using 
weakly hashed passwords. Can you gain access to the secret stored within the server?
Access the server using nc verbal-sleep.picoctf.net 51759

Hints:
1. Understanding hashes is very crucial. Read more here.
2. Can you identify the hash algorithm? Look carefully at the length and structure of each hash identified.
3. Tried using any hash cracking tools?
```

Challenge link: [https://play.picoctf.org/practice/challenge/475](https://play.picoctf.org/practice/challenge/475)

## Solution

## Connect to the server

We start by connecting to the server with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ nc verbal-sleep.picoctf.net 51759
Welcome!! Looking For the Secret?

We have identified a hash: 482c811da5d5b4bc6d497ffa98491e38
Enter the password for identified hash: 
```

Before starting to crack the hash let's see if it is already known by [CrackStation](https://crackstation.net/).  
And it is - the corresponding password for the MD5 hash is `password123`.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ nc verbal-sleep.picoctf.net 51759
Welcome!! Looking For the Secret?

We have identified a hash: 482c811da5d5b4bc6d497ffa98491e38
Enter the password for identified hash: password123
Correct! You've cracked the MD5 hash with no secret found!

Flag is yet to be revealed!! Crack this hash: b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3
Enter the password for the identified hash: 
```

This SHA1-hash is also known by Crackstation and the corresponding password is `letmein`.  
On to the next hash

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ nc verbal-sleep.picoctf.net 51759
Welcome!! Looking For the Secret?

We have identified a hash: 482c811da5d5b4bc6d497ffa98491e38
Enter the password for identified hash: password123
Correct! You've cracked the MD5 hash with no secret found!

Flag is yet to be revealed!! Crack this hash: b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3
Enter the password for the identified hash: letmein
Correct! You've cracked the SHA-1 hash with no secret found!

Almost there!! Crack this hash: 916e8c4f79b25028c9e467f1eb8eee6d6bbdff965f9928310ad30a8d88697745
Enter the password for the identified hash: 
```

This sha256-hash is also known by Crackstation and the corresponding password is `qwerty098`.  

## Get the flag

After the third correct password we get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ nc verbal-sleep.picoctf.net 51759
Welcome!! Looking For the Secret?

We have identified a hash: 482c811da5d5b4bc6d497ffa98491e38
Enter the password for identified hash: password123
Correct! You've cracked the MD5 hash with no secret found!

Flag is yet to be revealed!! Crack this hash: b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3
Enter the password for the identified hash: letmein
Correct! You've cracked the SHA-1 hash with no secret found!

Almost there!! Crack this hash: 916e8c4f79b25028c9e467f1eb8eee6d6bbdff965f9928310ad30a8d88697745
Enter the password for the identified hash: qwerty098
Correct! You've cracked the SHA-256 hash with a secret found. 
The flag is: picoCTF{<REDACTED>}
```

## Manually cracking the hashes

If we for some reason want to crack the hashes ourselves we can use [Hashcat](https://hashcat.net/hashcat/).  
But that requires use to know, or guess, the type of hash.

Hashcat helps with that if we don't know the type and output suggestions

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ hashcat 482c811da5d5b4bc6d497ffa98491e38                                                   
hashcat (v6.2.6) starting in autodetect mode

OpenCL API (OpenCL 3.0 PoCL 6.0+debian  Linux, None+Asserts, RELOC, LLVM 18.1.8, SLEEF, DISTRO, POCL_DEBUG) - Platform #1 [The pocl project]
============================================================================================================================================
* Device #1: cpu-sandybridge-Intel(R) Core(TM) i7-4790 CPU @ 3.60GHz, 2913/5890 MB (1024 MB allocatable), 8MCU

The following 11 hash-modes match the structure of your input hash:

      # | Name                                                       | Category
  ======+============================================================+======================================
    900 | MD4                                                        | Raw Hash
      0 | MD5                                                        | Raw Hash
     70 | md5(utf16le($pass))                                        | Raw Hash
   2600 | md5(md5($pass))                                            | Raw Hash salted and/or iterated
   3500 | md5(md5(md5($pass)))                                       | Raw Hash salted and/or iterated
   4400 | md5(sha1($pass))                                           | Raw Hash salted and/or iterated
  20900 | md5(sha1($pass).md5($pass).sha1($pass))                    | Raw Hash salted and/or iterated
   4300 | md5(strtoupper(md5($pass)))                                | Raw Hash salted and/or iterated
   1000 | NTLM                                                       | Operating System
   9900 | Radmin2                                                    | Operating System
   8600 | Lotus Notes/Domino 5                                       | Enterprise Application Software (EAS)

Please specify the hash-mode with -m [hash-mode].

Started: Sat Apr 12 14:40:33 2025
Stopped: Sat Apr 12 14:40:48 2025
```

So this hash could be either `MD4`, `MD5`, `NTLM`, etc.

Since `MD5` is the most common hash format let's start with that (parameter `-m 0`).  
We will use the most basic attack called `Straight` (parameter `-a 0`) and the most common wordlist (`rockyou.txt`).

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Cryptography/hashcrack]
└─$ hashcat -m 0 -a 0 482c811da5d5b4bc6d497ffa98491e38 /usr/share/wordlists/rockyou.txt        
hashcat (v6.2.6) starting

OpenCL API (OpenCL 3.0 PoCL 6.0+debian  Linux, None+Asserts, RELOC, LLVM 18.1.8, SLEEF, DISTRO, POCL_DEBUG) - Platform #1 [The pocl project]
============================================================================================================================================
* Device #1: cpu-sandybridge-Intel(R) Core(TM) i7-4790 CPU @ 3.60GHz, 2913/5890 MB (1024 MB allocatable), 8MCU

Minimum password length supported by kernel: 0
Maximum password length supported by kernel: 256

Hashes: 1 digests; 1 unique digests, 1 unique salts
Bitmaps: 16 bits, 65536 entries, 0x0000ffff mask, 262144 bytes, 5/13 rotates
Rules: 1

Optimizers applied:
* Zero-Byte
* Early-Skip
* Not-Salted
* Not-Iterated
* Single-Hash
* Single-Salt
* Raw-Hash

ATTENTION! Pure (unoptimized) backend kernels selected.
Pure kernels can crack longer passwords, but drastically reduce performance.
If you want to switch to optimized kernels, append -O to your commandline.
See the above message to find out about the exact limits.

Watchdog: Temperature abort trigger set to 90c

Host memory required for this attack: 2 MB

Dictionary cache hit:
* Filename..: /usr/share/wordlists/rockyou.txt
* Passwords.: 14344385
* Bytes.....: 139921507
* Keyspace..: 14344385

482c811da5d5b4bc6d497ffa98491e38:password123              
                                                          
Session..........: hashcat
Status...........: Cracked
Hash.Mode........: 0 (MD5)
Hash.Target......: 482c811da5d5b4bc6d497ffa98491e38
Time.Started.....: Sat Apr 12 14:48:06 2025 (1 sec)
Time.Estimated...: Sat Apr 12 14:48:07 2025 (0 secs)
Kernel.Feature...: Pure Kernel
Guess.Base.......: File (/usr/share/wordlists/rockyou.txt)
Guess.Queue......: 1/1 (100.00%)
Speed.#1.........:    61714 H/s (0.49ms) @ Accel:512 Loops:1 Thr:1 Vec:8
Recovered........: 1/1 (100.00%) Digests (total), 1/1 (100.00%) Digests (new)
Progress.........: 4096/14344385 (0.03%)
Rejected.........: 0/4096 (0.00%)
Restore.Point....: 0/14344385 (0.00%)
Restore.Sub.#1...: Salt:0 Amplifier:0-1 Iteration:0-1
Candidate.Engine.: Device Generator
Candidates.#1....: 123456 -> oooooo
Hardware.Mon.#1..: Util:  5%

Started: Sat Apr 12 14:47:39 2025
Stopped: Sat Apr 12 14:48:09 2025
```

We see the cracked password on this row `482c811da5d5b4bc6d497ffa98491e38:password123`.
The other hashes are cracked in similar fashion, but with other values for the `-m` parameter.

For additional information, please see the references below.

## References

- [CrackStation - Free Password Hash Cracker](https://crackstation.net/)
- [Cryptographic hash function - Wikipedia](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
- [Hashcat - Homepage](https://hashcat.net/hashcat/)
- [Hashcat - Kali Tools](https://www.kali.org/tools/hashcat/)
- [Hashing - The CTF Primer](https://primer.picoctf.org/#_hashing)
- [MD4 - Wikipedia](https://en.wikipedia.org/wiki/MD4)
- [MD5 - Wikipedia](https://en.wikipedia.org/wiki/MD5)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [SHA-1 - Wikipedia](https://en.wikipedia.org/wiki/SHA-1)
- [SHA-2 - Wikipedia](https://en.wikipedia.org/wiki/SHA-2)
