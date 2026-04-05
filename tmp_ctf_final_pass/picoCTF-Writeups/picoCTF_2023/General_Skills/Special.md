# Special

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, General Skills, bash, ssh
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Don't power users get tired of making spelling mistakes in the shell? Not anymore! 
Enter Special, the Spell Checked Interface for Affecting Linux. 
Now, every word is properly spelled and capitalized... automatically and behind-the-scenes! 

Be the first to test Special in beta, and feel free to tell us all about how Special 
streamlines every development process that you face. When your co-workers see your 
amazing shell interface, just tell them: That's Special (TM)

Start your instance to see connection details.
ssh -p 63243 ctf-player@saturn.picoctf.net
The password is 3f39b042
 
Hints:
1. Experiment with different shell syntax
```

Challenge link: [https://play.picoctf.org/practice/challenge/377](https://play.picoctf.org/practice/challenge/377)

## Solution

### Connect to the server

Start by connecting to the server with SSH

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/General_Skills/Special]
└─$ ssh -p 63243 ctf-player@saturn.picoctf.net
The authenticity of host '[saturn.picoctf.net]:63243 ([13.59.203.175]:63243)' can't be established.
ED25519 key fingerprint is SHA256:tJ0wuU5yBvNO/FrkHmR9iY36VJClMhKV+Hq2sxqKFmg.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[saturn.picoctf.net]:63243' (ED25519) to the list of known hosts.
ctf-player@saturn.picoctf.net's password: 
Welcome to Ubuntu 20.04.3 LTS (GNU/Linux 5.19.0-1024-aws x86_64)

 * Documentation:  https://help.ubuntu.com
 * Management:     https://landscape.canonical.com
 * Support:        https://ubuntu.com/advantage

This system has been minimized by removing packages and content that are
not required on a system that users do not log into.

To restore this content, you can run the 'unminimize' command.

The programs included with the Ubuntu system are free software;
the exact distribution terms for each program are described in the
individual files in /usr/share/doc/*/copyright.

Ubuntu comes with ABSOLUTELY NO WARRANTY, to the extent permitted by
applicable law.

Special$ 
```

### Try different commands

Let's try some sample commands

```bash
Special$ ls
Is 
sh: 1: Is: not found
Special$ pwd
Pod 
sh: 1: Pod: not found
Special$ id
Id 
sh: 1: Id: not found
```

As said in the challenge description, the commands are spell-checked and Capitalized.

### Try to launch a real shell

Then I tried to launch a new real shell

```bash
Special$ bash
Why go back to an inferior shell?
Special$ sh
Why go back to an inferior shell?
Special$ csh
Why go back to an inferior shell?
Special$ zsh
Why go back to an inferior shell?
Special$ $0
I 
sh: 1: I: not found
```

Nope, not allowed or didn't work.

### Experiment with different shell syntax

Next, I followed the hint to `experiment with different shell syntax`.

I tried to `quote` commands with `'` and `"`

```bash
Special$ 'ls'
Also 
sh: 1: Also: not found
Special$ "ls"
Also 
sh: 1: Also: not found
```

This worked for some commands like `pwd` and `grep`

```bash
Special$ "pwd"
"pwd" 
/home/ctf-player

Special$ "grep"
"grep" 
Usage: grep [OPTION]... PATTERNS [FILE]...
Try 'grep --help' for more information.
```

I also tried [command substitution](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html). This also worked some what on commands like `ls`

```bash
Special$ $(ls)  
$(ls) 
sh: 1: blargh: not found
```

So there is a file or directory called `blargh` in the current directory (which is `/home/ctf-player`).

### Get the flag

Now we ought to be able to get the flag by grepping for anything (`.`) recursively

```bash
Special$ "grep" -r . *
"grep" or . * 
grep: .: Is a directory
grep: blargh: Is a directory

Special$ "grep" -"r" . *
"grep" -"r" . * 
blargh/flag.txt:picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Command substitution - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [Shell (computing) - Wikipedia](https://en.wikipedia.org/wiki/Shell_(computing))
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
