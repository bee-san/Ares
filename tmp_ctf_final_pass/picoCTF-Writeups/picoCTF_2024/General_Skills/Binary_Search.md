# Binary Search

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, General Skills, shell, browser_webshell_solvable, ls
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: JEFFERY JOHN

Description:
Want to play a game? As you use more of the shell, you might be interested in how they work! 

Binary search is a classic algorithm used to quickly find an item in a sorted list. 
Can you find the flag? You'll have 1000 possibilities and only 10 guesses.

Cyber security often has a huge amount of data to look through - from logs, vulnerability reports, 
and forensics. Practicing the fundamentals manually might help you in the future when you have to 
write your own tools!

You can download the challenge files here:
challenge.zip

ssh -p 55705 ctf-player@atlas.picoctf.net
Using the password 83dcefb7. Accept the fingerprint with yes, and ls once connected to begin. 
Remember, in a shell, passwords are hidden!

Hints:
1. Have you ever played hot or cold? Binary search is a bit like that.
2. You have a very limited number of guesses. Try larger jumps between numbers!
3. The program will randomly choose a new number each time you connect. You can always try again, 
   but you should start your binary search over from the beginning - try around 500. 
   Can you think of why?
```

Challenge link: [https://play.picoctf.org/practice/challenge/442](https://play.picoctf.org/practice/challenge/442)

## Solution

The challenge name and description reveals that you should solve this challenge with a [binary search](https://en.wikipedia.org/wiki/Binary_search).

### Manual solution

We connect with SSH and start with an initial guess of `500`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/Binary_Search]
└─$ ssh -p 55705 ctf-player@atlas.picoctf.net
The authenticity of host '[atlas.picoctf.net]:55705 ([18.217.83.136]:55705)' can't be established.
ED25519 key fingerprint is SHA256:M8hXanE8l/Yzfs8iuxNsuFL4vCzCKEIlM/3hpO13tfQ.
This key is not known by any other names.
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '[atlas.picoctf.net]:55705' (ED25519) to the list of known hosts.
ctf-player@atlas.picoctf.net's password: 
Welcome to the Binary Search Game!
I'm thinking of a number between 1 and 1000.
Enter your guess: 500
```

Then we take half of the previous guess and add it to the lower or upper start value depending on if the random number is `Lower` or `Higher`

```bash
Enter your guess: 500
Lower! Try again.
Enter your guess: 250
Higher! Try again.
Enter your guess: 375
Lower! Try again.
Enter your guess: 312
Lower! Try again.
Enter your guess: 281
Lower! Try again.
Enter your guess: 265
Lower! Try again.
Enter your guess: 257
Higher! Try again.
Enter your guess: 261
Higher! Try again.
Enter your guess: 263
Higher! Try again.
Enter your guess: 264
Congratulations! You guessed the correct number: 264
Here's your flag: picoCTF{<REDACTED>}
Connection to atlas.picoctf.net closed.
```

And there we have the flag.

For additional information, please see the references below.

## References

- [Binary search - Wikipedia](https://en.wikipedia.org/wiki/Binary_search)
- [Secure Shell - Wikipedia](https://en.wikipedia.org/wiki/Secure_Shell)
- [ssh - Linux manual page](https://man7.org/linux/man-pages/man1/ssh.1.html)
