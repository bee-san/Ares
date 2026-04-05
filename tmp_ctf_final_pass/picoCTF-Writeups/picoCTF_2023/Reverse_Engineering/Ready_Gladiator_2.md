# Ready Gladiator 2

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
Can you make a CoreWars warrior that wins every single round?
Your opponent is the Imp. The source is available here. 

If you wanted to pit the Imp against himself, you could download the Imp and connect to the CoreWars server like this:
nc saturn.picoctf.net 54966 < imp.red

To get the flag, you must beat the Imp all 100 rounds.

Hints:
 1. If your warrior is close, try again, it may work on subsequent tries... why is that?
```

Challenge link: [https://play.picoctf.org/practice/challenge/370](https://play.picoctf.org/practice/challenge/370)

## Solution

I started researching warriors that were good at defeating 'The Imp' and [learned](http://moscova.inria.fr/~doligez/corewar/by-types/idx.htm) that there are different types/classification of warriors and that `gates try to prevent imps from overrunning them by constantly decrementing a core location before themselves`.

So I started to try some `gates` from [other people](http://moscova.inria.fr/~doligez/corewar/by-types/Xgate.htm).

### Try #1 - S/D Clear

I first tried the [S/D Clear](http://moscova.inria.fr/~doligez/corewar/rc/SDClear.txt) warrior

```text
;redcode-94
;name S/D Clear
;author David Moore
;assert 1

org start

gate  spl   #-1,  100
s     dat    -1,    7
start mov    *s, >gate
      djn.a  -1,  gate
      djn.a  -2, }s

end
```

But there were only `ties` with 'The Imp'

### Try #2 - stargate

Next, I tried [stargate](http://moscova.inria.fr/~doligez/corewar/rc/stargate.txt)

```text
;name stargate
;assert 1

org stun

gate    dat     stun,kill+10
for 10
        dat     0,0
rof

stun    spl     #kill-gate,kill-gate+10
        mov     *gate,>gate
        djn.f   -1,{gate-kill-15
kill    dat     kill-gate,kill-gate+10
end
```

Stargate worked much better and won 90-99 times against 'The Imp'.  
But unfortunately, I never got it to win 100 times.

### Try #3 - Gnat

Finally, I found [Gnat](http://moscova.inria.fr/~doligez/corewar/rc/Gnat.txt)

```text
;redcode verbose
;author J.Cisek
;name Gnat
;date 4/16/92
;assert 1

gnat    mov -1, <-2
        jmp gnat, <-3
end
```

It was slightly modified - the `end` and `assert` lines were added.

And the Gnat won 100 times

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2023/Reverse_Engineering/Ready_Gladiator_2]
└─$ nc saturn.picoctf.net 54966 < gnat.red
;redcode verbose
;author J.Cisek
;name Gnat
;date 4/16/92
;assert 1

gnat    mov -1, <-2
        jmp gnat, <-3
end
Submit your warrior: (enter 'end' when done)

Warrior1:
;redcode verbose
;author J.Cisek
;name Gnat
;date 4/16/92
;assert 1

gnat    mov -1, <-2
        jmp gnat, <-3
end

Rounds: 100
Warrior 1 wins: 100
Warrior 2 wins: 0
Ties: 0
You did it!
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [CoreWars](https://corewars.org/)
- [Beginner's guide to Redcode](https://corewars.org/docs/guide.html)
- [CoreWars gate warriors](http://moscova.inria.fr/~doligez/corewar/by-types/Xgate.htm)
