# Super SSH

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, General Skills, shell, ssh, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JEFFERY JOHN

Description:
Using a Secure Shell (SSH) is going to be pretty important.
Can you ssh as ctf-player to titan.picoctf.net at port 64614 to get the flag?

You'll also need the password 1ad5be0d. If asked, accept the fingerprint with yes.

If your device doesn't have a shell, you can use: https://webshell.picoctf.org
If you're not sure what a shell is, check out our Primer: https://primer.picoctf.com/#_the_shell

Hints:
1. https://linux.die.net/man/1/ssh
2. You can try logging in 'as' someone with <user>@titan.picoctf.net
3. How could you specify the port?
4. Remember, passwords are hidden when typed into the shell
```

Challenge link: [https://play.picoctf.org/practice/challenge/424](https://play.picoctf.org/practice/challenge/424)

## Solution

We connect with SSH like this:

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Super_SSH]
└─$ ssh -p 50400 ctf-player@titan.picoctf.net
The authenticity of host '[titan.picoctf.net]:50400 ([3.139.174.234]:50400)' can't be established.
ED25519 key fingerprint is SHA256:4S9EbTSSRZm32I+cdM5TyzthpQryv5kudRP9PIKT7XQ.
This host key is known by the following other names/addresses:
    ~/.ssh/known_hosts:94: [hashed name]
    ~/.ssh/known_hosts:95: [hashed name]
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[titan.picoctf.net]:50400' (ED25519) to the list of known hosts.
ctf-player@titan.picoctf.net's password: 
Welcome ctf-player, here's your flag: picoCTF{<REDACTED>}
Connection to titan.picoctf.net closed.
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
