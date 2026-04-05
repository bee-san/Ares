# Ready Gladiator 0

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
Can you make a CoreWars warrior that always loses, no ties?
 
Your opponent is the Imp. The source is available here. 

If you wanted to pit the Imp against himself, you could download the Imp and connect to the CoreWars server like this:
nc saturn.picoctf.net 54485 < imp.red

Hints:
 1. CoreWars is a well-established game with a lot of docs and strategy
 2. Experiment with input to the CoreWars handler or create a self-defeating bot
```

Challenge link: [https://play.picoctf.org/practice/challenge/368](https://play.picoctf.org/practice/challenge/368)

## Solution

### Read up on CoreWars

Since I hadn't played around with [CoreWars](https://corewars.org/) much I started out with reading tutorials such as [this one](https://corewars.org/docs/guide.html).

### Checkout the Imp

I then proceded with looking at the source code for the Imp

```text
;redcode
;name Imp Ex
;assert 1
mov 0, 1
end
```

And then I did a testrun with the Imp playing against itself as suggested in the description

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Ready_Gladiator_0]
└─$ nc saturn.picoctf.net 54485 < imp.red
;redcode
;name Imp Ex
;assert 1
mov 0, 1
end
Submit your warrior: (enter 'end' when done)

Warrior1:
;redcode
;name Imp Ex
;assert 1
mov 0, 1
end

Rounds: 100
Warrior 1 wins: 0
Warrior 2 wins: 0
Ties: 100
Try again. Your warrior (warrior 1) must lose all rounds, no ties.
```

### Create my own CoreWars warrior

Then I started to write to my own warrior, a self-destructive one the kills itself with the `dat` instruction.

After some trial and error with the syntax this became the result which I named `looser.red`

```text
;redcode
;name Looser
;assert 1
dat #0, #0
end
```

### Get the flag

Finally, I sent my warrior into battle

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Ready_Gladiator_0]
└─$ nc saturn.picoctf.net 54485 < looser.red
;redcode
;name Looser
;assert 1
dat #0, #0
end
Submit your warrior: (enter 'end' when done)

Warrior1:
;redcode
;name Looser
;assert 1
dat #0, #0
end

Rounds: 100
Warrior 1 wins: 0
Warrior 2 wins: 100
Ties: 0
You did it!
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [CoreWars](https://corewars.org/)
- [Beginner's guide to Redcode](https://corewars.org/docs/guide.html)
