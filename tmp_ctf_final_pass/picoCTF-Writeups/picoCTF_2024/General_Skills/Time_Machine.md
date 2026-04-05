# Time Machine

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, General Skills, browser_webshell_solvable, git
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JEFFERY JOHN

Description:
What was I last working on? I remember writing a note to help me remember...

You can download the challenge files here:
challenge.zip

Hints:
1. The cat command will let you read a file, but that won't help you here!
2. Read the chapter on Git from the picoPrimer here
3. When committing a file with git, a message can (and should) be included.
```

Challenge link: [https://play.picoctf.org/practice/challenge/425](https://play.picoctf.org/practice/challenge/425)

## Solution

### Analyse the git repo

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Time_Machine]
└─$ unzip challenge.zip
Archive:  challenge.zip
   creating: drop-in/
  inflating: drop-in/message.txt     
   creating: drop-in/.git/
   creating: drop-in/.git/branches/
  inflating: drop-in/.git/description  
   creating: drop-in/.git/hooks/
  inflating: drop-in/.git/hooks/applypatch-msg.sample  
  inflating: drop-in/.git/hooks/commit-msg.sample  
  inflating: drop-in/.git/hooks/fsmonitor-watchman.sample  
  inflating: drop-in/.git/hooks/post-update.sample  
  inflating: drop-in/.git/hooks/pre-applypatch.sample  
  inflating: drop-in/.git/hooks/pre-commit.sample  
  inflating: drop-in/.git/hooks/pre-merge-commit.sample  
<---snip--->
```

### Get the flag

Next, we check for changes with `git log` in the repository

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Time_Machine]
└─$ cd drop-in   

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Time_Machine/drop-in]
└─$ git log                                          
fatal: detected dubious ownership in repository at '/mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Time_Machine/drop-in'
To add an exception for this directory, call:

        git config --global --add safe.directory /mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Time_Machine/drop-in

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Time_Machine/drop-in]
└─$ git config --global --add safe.directory /mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Time_Machine/drop-in

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Time_Machine/drop-in]
└─$ git log                                                                                                         
commit 712314f105348e295f8cadd7d7dc4e9fa871e9a2 (HEAD -> master)
Author: picoCTF <ops@picoctf.com>
Date:   Tue Mar 12 00:07:26 2024 +0000

    picoCTF{<REDACTED>}

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Time_Machine/drop-in]
└─$ 
```

And there we have the flag as a note to the commit.

For additional information, please see the references below.

## References

- [Git & Version Control - The CTF Primer](https://primer.picoctf.org/#_git_version_control)
- [git - Linux manual page](https://man7.org/linux/man-pages/man1/git.1.html)
- [git-log - Linux manual page](https://man7.org/linux/man-pages/man1/git-log.1.html)
- [git-show - Linux manual page](https://man7.org/linux/man-pages/man1/git-show.1.html)
