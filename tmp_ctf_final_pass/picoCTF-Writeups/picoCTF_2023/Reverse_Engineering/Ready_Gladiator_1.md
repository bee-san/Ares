# Ready Gladiator 1

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Reverse Engineering, CoreWars
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES
 
Description:
Can you make a CoreWars warrior that wins?
Your opponent is the Imp. The source is available here. 

If you wanted to pit the Imp against himself, you could download the Imp and connect to the CoreWars server like this:
nc saturn.picoctf.net 62741 < imp.red

To get the flag, you must beat the Imp at least once out of the many rounds.

Hints:
 1. You may be able to find a viable warrior in beginner docs
```

Challenge link: [https://play.picoctf.org/practice/challenge/369](https://play.picoctf.org/practice/challenge/369)

## Solution

This is a continuation of the [previous challenge](Ready_Gladiator_0.md) and I started off by re-reading the [beginner's docs](https://corewars.org/docs/guide.html) in hope of finding a CoreWars warrior as the hint suggested. I wasn't very keen on coding my own warrior.

### Enter the dwarf warrior

I found a warrior called 'The dwarf' and decided to try it out as an opponent to 'The Imp'

```text
;redcode
;name The Dwarf
;assert 1
add #4, 3
mov 2, @2
jmp -2
dat #0, #0
end
```

### Get the flag

Then I sent the dwarf into battle

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Reverse_Engineering/Ready_Gladiator_1]
└─$ nc saturn.picoctf.net 62741 < the_dwarf.red 
;redcode
;name The Dwarf
;assert 1
add #4, 3
mov 2, @2
jmp -2
dat #0, #0
end
Submit your warrior: (enter 'end' when done)

Warrior1:
;redcode
;name The Dwarf
;assert 1
add #4, 3
mov 2, @2
jmp -2
dat #0, #0
end

Rounds: 100
Warrior 1 wins: 26
Warrior 2 wins: 0
Ties: 74
You did it!
picoCTF{<REDACTED>}
```

It won 26 times so I got the flag.

For additional information, please see the references below.

## References

- [CoreWars](https://corewars.org/)
- [Beginner's guide to Redcode](https://corewars.org/docs/guide.html)
