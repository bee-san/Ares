# file-run1

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILL HONG

Description:
A program has been provided to you, what happens if you try to run it on the command line?

Download the program here.

Hints:
1. To run the program at all, you must make it executable (i.e. $ chmod +x run)
2. Try running it by adding a '.' in front of the path to the file (i.e. $ ./run)
```

Challenge link: [https://play.picoctf.org/practice/challenge/266](https://play.picoctf.org/practice/challenge/266)

## Solution

This challenge is really simple and the hints give it all away

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Reverse_Engineering/File_Run1]
└─$ chmod +x run                    
                                                                                   
┌──(kali㉿kali)-[/picoCTF/picoCTF_2022/Reverse_Engineering/File_Run1]
└─$ ./run  
The flag is: picoCTF{<REDACTED>}     
```

If you need more information, please see the references below.

## References

- [Linux path environment variable](https://linuxconfig.org/linux-path-environment-variable)
- [Linux file permissions explained](https://www.redhat.com/sysadmin/linux-file-permissions-explained)
