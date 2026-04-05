# Flag Hunters

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Reverse Engineering, picoCTF 2025, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SYREAL
 
Description:
Lyrics jump from verses to the refrain kind of like a subroutine call. There's a hidden 
refrain this program doesn't print by default. Can you get it to print it? 
There might be something in it for you.

The program's source code can be downloaded here.
Connect to the program with netcat:
$ nc verbal-sleep.picoctf.net 61138
 
Hints:
1. This program can easily get into undefined states. Don't be shy about Ctrl-C.
2. Unsanitized user input is always good, right?
3. Is there any syntax that is ripe for subversion?
```

Challenge link: [https://play.picoctf.org/practice/challenge/472](https://play.picoctf.org/practice/challenge/472)

## Solution

### Do a test run

We start with doing a test run of the program

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Reverse_Engineering/Flag_Hunters]
└─$ nc verbal-sleep.picoctf.net 61138
Command line wizards, we’re starting it right,
Spawning shells in the terminal, hacking all night.
Scripts and searches, grep through the void,
Every keystroke, we're a cypher's envoy.
Brute force the lock or craft that regex,
Flag on the horizon, what challenge is next?

We’re flag hunters in the ether, lighting up the grid,
No puzzle too dark, no challenge too hid.
With every exploit we trigger, every byte we decrypt,
We’re chasing that victory, and we’ll never quit.
Crowd: 
```

The program stop and wait for our input. Let's check the source for more details.

### Analyse the Python source code

Next, we analyse of the source code

```python
import re
import time

# Read in flag from file
flag = open('flag.txt', 'r').read()

secret_intro = \
'''Pico warriors rising, puzzles laid bare,
Solving each challenge with precision and flair.
With unity and skill, flags we deliver,
The ether’s ours to conquer, '''\
+ flag + '\n'

song_flag_hunters = secret_intro +\
'''

[REFRAIN]
We’re flag hunters in the ether, lighting up the grid,
No puzzle too dark, no challenge too hid.
<---snip--->
Control the instruction, flags call my name.

REFRAIN;

END;
'''

MAX_LINES = 100

def reader(song, startLabel):
  lip = 0
  start = 0
  refrain = 0
  refrain_return = 0
  finished = False

  # Get list of lyric lines
  song_lines = song.splitlines()
  
  # Find startLabel, refrain and refrain return
  for i in range(0, len(song_lines)):
    if song_lines[i] == startLabel:
      start = i + 1
    elif song_lines[i] == '[REFRAIN]':
      refrain = i + 1
    elif song_lines[i] == 'RETURN':
      refrain_return = i

  # Print lyrics
  line_count = 0
  lip = start
  while not finished and line_count < MAX_LINES:
    line_count += 1
    for line in song_lines[lip].split(';'):
      if line == '' and song_lines[lip] != '':
        continue
      if line == 'REFRAIN':
        song_lines[refrain_return] = 'RETURN ' + str(lip + 1)
        lip = refrain
      elif re.match(r"CROWD.*", line):
        crowd = input('Crowd: ')
        song_lines[lip] = 'Crowd: ' + crowd
        lip += 1
      elif re.match(r"RETURN [0-9]+", line):
        lip = int(line.split()[1])
      elif line == 'END':
        finished = True
      else:
        print(line, flush=True)
        time.sleep(0.5)
        lip += 1

reader(song_flag_hunters, '[VERSE1]')
```

When the program reaches this line `CROWD (Singalong here!);` the following if statement waits for our input

```python
<---snip--->
      elif re.match(r"CROWD.*", line):
        crowd = input('Crowd: ')
        song_lines[lip] = 'Crowd: ' + crowd
        lip += 1
<---snip--->
```

We want to reach the prepended secret intro with the flag

```python
<---snip--->
# Read in flag from file
flag = open('flag.txt', 'r').read()

secret_intro = \
'''Pico warriors rising, puzzles laid bare,
Solving each challenge with precision and flair.
With unity and skill, flags we deliver,
The ether’s ours to conquer, '''\
+ flag + '\n'

song_flag_hunters = secret_intro +\
<---snip--->
```

This can be done with this section of the code

```python
<---snip--->
      elif re.match(r"RETURN [0-9]+", line):
        lip = int(line.split()[1])
<---snip--->
```

were we can read any line in the lyrics.  

The flag is on the fourth line in the lyrics so our payload should be `RETURN 3` due to indexes starting at 0.

There is one gotcha though. Since the reader splits on semi-colons (`for line in song_lines[lip].split(';'):`) we need to prepend our payload with a semi-colon.

### Get the flag

To get the flag we connect to the program and input `;RETURN 3`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2025/Reverse_Engineering/Flag_Hunters]
└─$ nc verbal-sleep.picoctf.net 61138
Command line wizards, we’re starting it right,
Spawning shells in the terminal, hacking all night.
Scripts and searches, grep through the void,
Every keystroke, we're a cypher's envoy.
Brute force the lock or craft that regex,
Flag on the horizon, what challenge is next?

We’re flag hunters in the ether, lighting up the grid,
No puzzle too dark, no challenge too hid.
With every exploit we trigger, every byte we decrypt,
We’re chasing that victory, and we’ll never quit.
Crowd: ;RETURN 3

Echoes in memory, packets in trace,
Digging through the remnants to uncover with haste.
Hex and headers, carving out clues,
Resurrect the hidden, it's forensics we choose.
Disk dumps and packet dumps, follow the trail,
Buried deep in the noise, but we will prevail.

We’re flag hunters in the ether, lighting up the grid,
No puzzle too dark, no challenge too hid.
With every exploit we trigger, every byte we decrypt,
We’re chasing that victory, and we’ll never quit.
Crowd: 
The ether’s ours to conquer, picoCTF{<REDACTED>}


[REFRAIN]
We’re flag hunters in the ether, lighting up the grid,
No puzzle too dark, no challenge too hid.
With every exploit we trigger, every byte we decrypt,
We’re chasing that victory, and we’ll never quit.
Crowd: 
The ether’s ours to conquer, picoCTF{<REDACTED>}

^C
```

And there we have the flag.

For additional information, please see the references below.

## References

- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Regular expression - Wikipedia](https://en.wikipedia.org/wiki/Regular_expression)
