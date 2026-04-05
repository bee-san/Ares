# FANTASY CTF

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: General Skills, picoCTF 2025, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL

Description:
Play this short game to get familiar with terminal applications and some of the most 
important rules in scope for picoCTF.

Connect to the program with netcat:
$ nc verbal-sleep.picoctf.net 55716

Hints:
1. When a choice is presented like [a/b/c], choose one, for example: 
   c and then press Enter.
```

Challenge link: [https://play.picoctf.org/practice/challenge/471](https://play.picoctf.org/practice/challenge/471)

## Solution

We start by connecting with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/FANTASY_CTF]
└─$ nc verbal-sleep.picoctf.net 55716
FANTASY CTF SIMULATION

The simulation begins in the private room of Eibhilin, a bright, young student.
The room is dimly lit, with the glow of her multiple monitors casting an
electric blue hue on the walls. Around the room are posters of vintage movies
from the MCU — ancient guardians from another age staring down like digital
sentinels.

---
(Press Enter to continue...)
---
```

### Get the flag

You get the flag by playing the game, i.e. choosing `A) *Play the game*`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/General_Skills/FANTASY_CTF]
└─$ nc verbal-sleep.picoctf.net 55716
FANTASY CTF SIMULATION

The simulation begins in the private room of Eibhilin, a bright, young student.
The room is dimly lit, with the glow of her multiple monitors casting an
electric blue hue on the walls. Around the room are posters of vintage movies
from the MCU — ancient guardians from another age staring down like digital
sentinels.

<---snip--->

---
(Press Enter to continue...)
---

Options:
A) *Play the game*
B) *Search the Ether for the flag*
[a/b] > a

"Good choice, Ei," Nyx says, "You never want to share flags or artifact
downloads."

---
(Press Enter to continue...)
---

 Playing the Game
Playing the Game: 100%|██████████████████████████████████████ [time left: 00:00]
Playing the Game completed successfully!

---
(Press Enter to continue...)
---

"That was fun!" Eibhilin exclaims, "I found the flag!"

---
(Press Enter to continue...)
---

Nyx says, "Great job, Ei! I've read that a lot of players create writeups of
interesting challenges they solve during the competition. Just be sure to wait
to publish them until after the winners have been announced. We can work on
that together if you'd like."

---
(Press Enter to continue...)
---

"Thanks, Nyx! Here's the flag I found: picoCTF{<REDACTED>}"

---
(Press Enter to continue...)
---

"Great, you just got 10 points!" Nyx exclaims.

---
(Press Enter to continue...)
---

Eibhilin smiles, "I'm off to a good start!"

---
(Press Enter to continue...)
---

Nyx says, "Let's keep going!"

---
(Press Enter to continue...)
---

END OF FANTASY CTF SIMULATION
Thank you for playing! To reemphasize some rules for this year:
1. Register only one account.
2. Do not share accounts, flags or artifact downloads.
3. Wait to publish writeups publicly until after the organizers announce the
winners.
4. picoCTF{<REDACTED>} is a real flag! Submit it for some
points in picoCTF 2025!

---
(Press Enter to continue...)
---
```

For additional information, please see the references below.

## References

- [Computer terminal - Wikipedia](https://en.wikipedia.org/wiki/Computer_terminal)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Shell (computing) - Wikipedia](https://en.wikipedia.org/wiki/Shell_(computing))
