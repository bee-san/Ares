# Wave a flag

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2021, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL

Description:
Can you invoke help flags for a tool or binary? 

This program has extraordinarily helpful information...

Hints:
1. This program will only work in the webshell or another Linux computer.
2. To get the file accessible in your shell, enter the following in the Terminal prompt: 
   $ wget https://mercury.picoctf.net/static/a00f554b16385d9970dae424f66ee1ab/warm
3. Run this program by entering the following in the Terminal prompt: 
   $ ./warm, but you'll first have to make it executable with $ chmod +x warm
4. -h and --help are the most common arguments to give to programs to get more information from them!
5. Not every program implements help features like -h and --help.
```

Challenge link: [https://play.picoctf.org/practice/challenge/170](https://play.picoctf.org/practice/challenge/170)

## Solution

Let's make sure the program is executable and run it

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Wave_a_flag]
└─$ chmod +x warm       

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Wave_a_flag]
└─$ ./warm            
Hello user! Pass me a -h to learn what I can do!
```

Ah, as both the description and the hints suggests we should ask for help with the `-h` parameter.

Ask for help

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/General_Skills/Wave_a_flag]
└─$ ./warm -h
Oh, help? I actually don't do much, but I do have this flag here: picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [8 Ways To Get Help On The Linux Shell](https://vitux.com/get-help-on-linux-shell/)
- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
