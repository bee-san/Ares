# chrono

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, General Skills, linux
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
How to automate tasks to run at intervals on linux servers?

Use ssh to connect to this server:
Server: saturn.picoctf.net
Port: 57689
Username: picoplayer 
Password: KkPyI5bkmn

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/347](https://play.picoctf.org/practice/challenge/347)

## Solution

Start by connecting to the server with SSH

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/General_Skills/chrono]
└─$ ssh -p 57689 picoplayer@saturn.picoctf.net
The authenticity of host '[saturn.picoctf.net]:57689 ([13.59.203.175]:57689)' can't be established.
ED25519 key fingerprint is SHA256:p/PvzCEZdcZTX+VPBLVApO7dmZmo7L7qwjpiIdTTHao.
This host key is known by the following other names/addresses:
    ~/.ssh/known_hosts:1: [hashed name]
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[saturn.picoctf.net]:57689' (ED25519) to the list of known hosts.
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

I know since previously that scheduled jobs in linux is specified in the `crontab`.  
Otherwise this might require some googling to find out...

You can list your cronjobs with `crontab -l` so let's try that

```bash
picoplayer@challenge:~$ crontab -l
no crontab for picoplayer
```

Hm, no joy there. The configuration files for cron are, like most other configurations, stored in the `/etc` directory so let's go there

```bash
picoplayer@challenge:~$ cd /etc
picoplayer@challenge:/etc$ ls -la cron*
-rw-r--r-- 1 root root 43 Mar 16 02:00 crontab

cron.d:
total 8
drwxr-xr-x 1 root root  26 Mar 16 02:00 .
drwxr-xr-x 1 root root  66 Jul 25 17:20 ..
-rw-r--r-- 1 root root 102 Feb 13  2020 .placeholder
-rw-r--r-- 1 root root 201 Feb 14  2020 e2scrub_all

cron.daily:
total 12
drwxr-xr-x 1 root root   26 Mar 16 02:00 .
drwxr-xr-x 1 root root   66 Jul 25 17:20 ..
-rw-r--r-- 1 root root  102 Feb 13  2020 .placeholder
-rwxr-xr-x 1 root root 1478 Apr  9  2020 apt-compat
-rwxr-xr-x 1 root root 1187 Sep  5  2019 dpkg

cron.hourly:
total 4
drwxr-xr-x 2 root root  26 Mar 16 02:00 .
drwxr-xr-x 1 root root  66 Jul 25 17:20 ..
-rw-r--r-- 1 root root 102 Feb 13  2020 .placeholder

cron.monthly:
total 4
drwxr-xr-x 2 root root  26 Mar 16 02:00 .
drwxr-xr-x 1 root root  66 Jul 25 17:20 ..
-rw-r--r-- 1 root root 102 Feb 13  2020 .placeholder

cron.weekly:
total 4
drwxr-xr-x 2 root root  26 Mar 16 02:00 .
drwxr-xr-x 1 root root  66 Jul 25 17:20 ..
-rw-r--r-- 1 root root 102 Feb 13  2020 .placeholder
```

Let's start by checking the `/etc/crontab` file which also contains the flag.

```bash
picoplayer@challenge:/etc$ cat crontab 
# picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [cron - Wikipedia](https://en.wikipedia.org/wiki/Cron)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
