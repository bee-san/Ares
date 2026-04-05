# Collaborative Development

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
My team has been working very hard on new features for our flag printing program! 
I wonder how they'll work together?

You can download the challenge files here:
challenge.zip

Hints:
1. git branch -a will let you see available branches
2. How can file 'diffs' be brought to the main branch? Don't forget to git config!
3. Merge conflicts can be tricky! Try a text editor like nano, emacs, or vim.
```

Challenge link: [https://play.picoctf.org/practice/challenge/410](https://play.picoctf.org/practice/challenge/410)

## Solution

### Analyse the git repo

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Collaborative_Development]
└─$ unzip challenge.zip 
Archive:  challenge.zip
   creating: drop-in/
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
<---snip--->
   creating: drop-in/.git/logs/
  inflating: drop-in/.git/logs/HEAD  
   creating: drop-in/.git/logs/refs/
   creating: drop-in/.git/logs/refs/heads/
  inflating: drop-in/.git/logs/refs/heads/main  
   creating: drop-in/.git/logs/refs/heads/feature/
  inflating: drop-in/.git/logs/refs/heads/feature/part-1  
  inflating: drop-in/.git/logs/refs/heads/feature/part-2  
  inflating: drop-in/.git/logs/refs/heads/feature/part-3  
  inflating: drop-in/flag.py 
```

Ah, there is a `flag.py` script in the repo. That's interesting!

Next, we check for changes with `git log` and list branches with `git branch -a`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Collaborative_Development]
└─$ cd drop-in  

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git log                                                                                                                      
commit 2258a0f267d57e8b6025e2a020b77fac7a553c92 (HEAD -> main)
Author: picoCTF <ops@picoctf.com>
Date:   Tue Mar 12 00:07:54 2024 +0000

    init flag printer

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git branch -a
  feature/part-1
  feature/part-2
  feature/part-3
* main

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ 
```

There a three additional branches in addition to the `main` branch.

### Analyse the branches

Let's check the contents of the `flag.py` file in the `main` branch

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ ls                  
flag.py
 
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ cat flag.py 
print("Printing the flag...")
```

Then we checkout another branch and check the contents of `flag.py` again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git checkout feature/part-1 
error: Your local changes to the following files would be overwritten by checkout:
        flag.py
Please commit your changes or stash them before you switch branches.
Aborting

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git checkout -f feature/part-1
Switched to branch 'feature/part-1'

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ cat flag.py 
print("Printing the flag...")
print("picoCTF{<REDACTED>_", end='')  
```

Ah, each branch will probably add another part of the script/flag.  

On to the second branch. Here we will use another way to view the file with `git show`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git show feature/part-2:flag.py
print("Printing the flag...")

print("<REDACTED>_", end='')
```

We can get the third part in a similar way.

### Get the flag

Finally, to get the flag we display all the parts

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Collaborative_Development/drop-in]
└─$ git show feature/part-{1..3}:flag.py
print("Printing the flag...")
print("picoCTF{<REDACTED>_", end='')print("Printing the flag...")

print("<REDACTED>", end='')print("Printing the flag...")

print("<REDACTED>")
```

and manually construct it with cut-and-paste.

For additional information, please see the references below.

## References

- [Git & Version Control - The CTF Primer](https://primer.picoctf.org/#_git_version_control)
- [git - Linux manual page](https://man7.org/linux/man-pages/man1/git.1.html)
- [git-branch - Linux manual page](https://man7.org/linux/man-pages/man1/git-branch.1.html)
- [git-checkout - Linux manual page](https://man7.org/linux/man-pages/man1/git-checkout.1.html)
- [git-log - Linux manual page](https://man7.org/linux/man-pages/man1/git-log.1.html)
- [git-show - Linux manual page](https://man7.org/linux/man-pages/man1/git-show.1.html)
