# Blame Game

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
Someone's commits seems to be preventing the program from working. Who is it?

You can download the challenge files here:
challenge.zip

Hints:
1. In collaborative projects, many users can make many changes. How can you see the 
   changes within one file?
2. Read the chapter on Git from the picoPrimer here
3. You can use python3 <file>.py to try running the code, though you won't need 
   to for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/405](https://play.picoctf.org/practice/challenge/405)

## Solution

### Analyse the git repo

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Blame_Game]
└─$ unzip challenge.zip
Archive:  challenge.zip
   creating: drop-in/
 extracting: drop-in/message.py      
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
  inflating: drop-in/.git/hooks/pre-push.sample  
  inflating: drop-in/.git/hooks/pre-rebase.sample  
  inflating: drop-in/.git/hooks/pre-receive.sample  
  inflating: drop-in/.git/hooks/prepare-commit-msg.sample  
  inflating: drop-in/.git/hooks/update.sample  
   creating: drop-in/.git/info/
<---snip--->
```

### Get the flag

Next, we check for included files and the changes made on them with `git log`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Blame_Game]
└─$ cd drop-in   

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Blame_Game/drop-in]
└─$ ls -la       
total 5
drwxrwxrwx 1 root root    0 Mar 12 01:07 .
drwxrwxrwx 1 root root    0 Jun 10 10:20 ..
drwxrwxrwx 1 root root 4096 Mar 12 01:07 .git
-rwxrwxrwx 1 root root   22 Mar 12 01:07 message.py

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Blame_Game/drop-in]
└─$ git log message.py 
commit 23e9d4ce78b3cea725992a0ce6f5eea0bf0bcdd4
Author: picoCTF{<REDACTED>} <ops@picoctf.com>
Date:   Tue Mar 12 00:07:15 2024 +0000

    optimize file size of prod code

commit 3ce5c692e2f9682a866c59ac1aeae38d35d19771
Author: picoCTF <ops@picoctf.com>
Date:   Tue Mar 12 00:07:15 2024 +0000

    create top secret project

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Blame_Game/drop-in]
└─$ 
```

And there we have the flag in the second commit.

For additional information, please see the references below.

## References

- [Git & Version Control - The CTF Primer](https://primer.picoctf.org/#_git_version_control)
- [git - Linux manual page](https://man7.org/linux/man-pages/man1/git.1.html)
- [git-log - Linux manual page](https://man7.org/linux/man-pages/man1/git-log.1.html)
- [git-show - Linux manual page](https://man7.org/linux/man-pages/man1/git-show.1.html)
