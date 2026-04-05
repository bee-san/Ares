# Specialer

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, General Skills, bash, ssh
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES, ET AL.

Description:
Reception of Special has been cool to say the least. That's why we made an exclusive version 
of Special, called Secure Comprehensive Interface for Affecting Linux Empirically Rad, 
or just 'Specialer'. 

With Specialer, we really tried to remove the distractions from using a shell. 
Yes, we took out spell checker because of everybody's complaining. 

But we think you will be excited about our new, reduced feature set for keeping you focused 
on what needs it the most. Please start an instance to test your very own copy of Specialer.

ssh -p 62169 ctf-player@saturn.picoctf.net. The password is d137d16e
 
Hints:
1. What programs do you have access to?
```

Challenge link: [https://play.picoctf.org/practice/challenge/378](https://play.picoctf.org/practice/challenge/378)

## Solution

This is a continuation of the [previous challange](Special.md).

### Connect to the server

Start by connecting to the server with SSH

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/General_Skills/Specialer]
└─$ ssh -p 62169 ctf-player@saturn.picoctf.net
The authenticity of host '[saturn.picoctf.net]:62169 ([13.59.203.175]:62169)' can't be established.
ED25519 key fingerprint is SHA256:lMXKIC17ONzyUJx7ZYBY5VSwoxCz20uq5/Nm+IhXKew.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[saturn.picoctf.net]:62169' (ED25519) to the list of known hosts.
ctf-player@saturn.picoctf.net's password: 
Specialer$ 
```

### Get a list of available commands

We can use [command-line completion](https://en.wikipedia.org/wiki/Command-line_completion) to find out what commmands are available by pressing `TAB` twice

```bash
Specialer$ 
!          ]]         break      command    coproc     done       esac       false      function   if         local      pushd      return     source     times      ulimit     wait
./         alias      builtin    compgen    declare    echo       eval       fc         getopts    in         logout     pwd        select     suspend    trap       umask      while
:          bash       caller     complete   dirs       elif       exec       fg         hash       jobs       mapfile    read       set        test       true       unalias    {
[          bg         case       compopt    disown     else       exit       fi         help       kill       popd       readarray  shift      then       type       unset      }
[[         bind       cd         continue   do         enable     export     for        history    let        printf     readonly   shopt      time       typeset    until   
```

No `grep` so we can't use the trick we used in the [previous challange](Special.md).  
And no `cat` or `ls`. But there is `bash`.

### Try to launch a real shell

Let's try to launch `bash`

```bash
Specialer$ bash
Specialer$ 
!          ]]         break      command    coproc     done       esac       false      function   if         local      pushd      return     source     times      ulimit     wait
./         alias      builtin    compgen    declare    echo       eval       fc         getopts    in         logout     pwd        select     suspend    trap       umask      while
:          bash       caller     complete   dirs       elif       exec       fg         hash       jobs       mapfile    read       set        test       true       unalias    {
[          bg         case       compopt    disown     else       exit       fi         help       kill       popd       readarray  shift      then       type       unset      }
[[         bind       cd         continue   do         enable     export     for        history    let        printf     readonly   shopt      time       typeset    until      
Specialer$ exit
exit
Specialer$ 
```

It worked but it was also limited so I logged out of that session.

### Map out files and directories

Next, I mapped out files and directories with any command (`pushd` used below) by pressing `TAB` twice as before

```bash
Specialer$ pushd 
.bash_history  .hushlogin     .profile       abra/          ala/           sim/   
Specialer$ pushd abra/cada
cadabra.txt   cadaniel.txt  
Specialer$ pushd ala/
kazam.txt  mode.txt  
Specialer$ pushd sim/
city.txt     salabim.txt  
```

### Get the flag

Since `echo` is available to us we can use [command substitution](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html) and [input redirection](https://www.gnu.org/software/bash/manual/html_node/Redirections.html) to read files

```bash
Specialer$ echo $(<abra/cadabra.txt)
Nothing up my sleeve!
Specialer$ echo $(<abra/cadaniel.txt)
Yes, I did it! I really did it! I'm a true wizard!
Specialer$ echo $(<ala/kazam.txt)
return 0 picoCTF{<REDACTED>}
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Command-line completion - Wikipedia](https://en.wikipedia.org/wiki/Command-line_completion)
- [Command substitution - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Command-Substitution.html)
- [Redirections - Bash Manual](https://www.gnu.org/software/bash/manual/html_node/Redirections.html)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [Shell (computing) - Wikipedia](https://en.wikipedia.org/wiki/Shell_(computing))
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
