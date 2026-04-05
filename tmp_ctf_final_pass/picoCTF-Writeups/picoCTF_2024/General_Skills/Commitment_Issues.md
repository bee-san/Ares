# Commitment Issues

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
I accidentally wrote the flag down. Good thing I deleted it!

You download the challenge files here:
challenge.zip

Hints:
1. Version control can help you recover files if you change or lose them!
2. Read the chapter on Git from the picoPrimer here
3. You can 'checkout' commits to see the files inside them
```

Challenge link: [https://play.picoctf.org/practice/challenge/411](https://play.picoctf.org/practice/challenge/411)

## Solution

### Analyse the git repo

We start by unpacking the zip-file

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Commitment_Issues]
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
```

Next, we check for changes with `git log` in the repository

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Commitment_Issues]
└─$ cd drop-in  

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ git log                                            
fatal: detected dubious ownership in repository at '/mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Commitment_Issues/drop-in'
To add an exception for this directory, call:

        git config --global --add safe.directory /mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Commitment_Issues/drop-in

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ git config --global --add safe.directory /mnt/hgfs/CTFs/picoCTF/picoCTF_2024/General_Skills/Commitment_Issues/drop-in

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ git log                                                                                                              
commit a6dca68e4310585eac3b5c9caf0f75967dfe972c (HEAD -> master)
Author: picoCTF <ops@picoctf.com>
Date:   Sat Mar 9 21:10:06 2024 +0000

    remove sensitive info

commit e720dc26a1a55405fbdf4d338d465335c439fb3e
Author: picoCTF <ops@picoctf.com>
Date:   Sat Mar 9 21:10:06 2024 +0000

    create flag

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ 
```

The sensitve data, a.k.a the flag, is in the commit `e720dc26a1a55405fbdf4d338d465335c439fb3e`.

### Get the flag

Finally, to get the flag we display the commit with `git show`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ git show e720dc26a1a55405fbdf4d338d465335c439fb3e
commit e720dc26a1a55405fbdf4d338d465335c439fb3e
Author: picoCTF <ops@picoctf.com>
Date:   Sat Mar 9 21:10:06 2024 +0000

    create flag

diff --git a/message.txt b/message.txt
new file mode 100644
index 0000000..d263841
--- /dev/null
+++ b/message.txt
@@ -0,0 +1 @@
+picoCTF{<REDACTED>}

┌──(kali㉿kali)-[/mnt/…/picoCTF_2024/General_Skills/Commitment_Issues/drop-in]
└─$ 
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Git & Version Control - The CTF Primer](https://primer.picoctf.org/#_git_version_control)
- [git - Linux manual page](https://man7.org/linux/man-pages/man1/git.1.html)
- [git-log - Linux manual page](https://man7.org/linux/man-pages/man1/git-log.1.html)
- [git-show - Linux manual page](https://man7.org/linux/man-pages/man1/git-show.1.html)
