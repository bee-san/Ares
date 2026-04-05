# Chronohack #
 
## Overview ##

200 points

Category: [Reverse Engineering](../)

Tags: `#reverseengineering #PRNG #seed #bruteforce`

## Description ##

Can you guess the exact token and unlock the hidden flag? Our school relies on tokens to authenticate students. 
Unfortunately, someone leaked an important file for token generation. Guess the token to get the flag.

## Approach ##

Analysing the provided `token_generator.py` challenge source file, at line `#6` we see that the python Pseudo Random Number Generator (PRNG) is being seeded with the current time as a basis.

    random.seed(int(time.time() * 1000))  # seeding with current time 

Python's `time.time()` function is documented as:

    "Return the time in seconds since the epoch as a floating-point number."

Multiplied by 1000 yields a time in milliseconds that is then truncated from a floating point number to an integer to form the seed.

So the general plan here is to run the token generator locally to generate a token that is submitted to the challenge server with as little time difference as possible, such that if we use the same time the local and remote sides should arrive at the same seed and hence the same series of random numbers from the PRNG.

However in reality we have to allow for slight variations between the local and remote system times and any latencies in connecting to the challenge server and it spawning and instance of the token generator. Therefore we can brute force this variation by running the process in a loop, adjusting the timestamp used as a seed incrementally on the local machine to hopefully come across the correct adjustment for differences in times.

To implement this I created a [pwntools](https://docs.pwntools.com/en/stable/) script that used a modified version of the `get_random()` function from the original `token_generator.py` source file. It was modified to take an additional `seed` parameter to seed the PRNG with instead of generating the seed itself using `time.time()`.

Now `token_generator.py` only allows for 50 guesses before it terminates. Therefore locally we can make 50 adjustments before we must invoke another instance of the token generator.

The [pwntools](https://docs.pwntools.com/en/stable/) script stores the current local time (`start_time`) immediately prior to spawning the challenge server process, to ensure they are as close as possible.

A loop then submits the generated local token starting from the `start_time`, incrementing by 1 millisecond each guess, stopping if the key is dumped, or we exhaust our 50 guesses.

Then outside of this we are running another loop to increase the total adjustment beyond the 50 milliseconds range, to cover a range of -50 to 1000 milliseconds.

Note that I did start with a negative adjustment to account for my local machine system time being ahead of the remote server system time. I also added out of an abundance of caution some overlap in the testing between invocations of the remote token generator to account for any jitter.

## Solution ##

The final [pwntools](https://docs.pwntools.com/en/stable/) script used to capture the flag:

    #!/usr/bin/env python3

    from pwn import *

    import random
    import time

    def get_random(length, seed):
        alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        random.seed(seed) #, version=1)
        s = ""
        for i in range(length):
            s += random.choice(alphabet)
        return s

    found_flag = False

    for adjust in range(-50, 1000, 40):
      print('=================== Adjustment: ' + str(adjust) + ' ==================')
      
      start_time = time.time()
      target_proc = remote(sys.argv[1], sys.argv[2])

      for i in range(0, 50):
        print('Tweak: ' + str(i + adjust))
        token = get_random(20, (int(start_time * 1000) + i + adjust)).encode("utf-8")

        x = target_proc.recvuntil(b'(or exit):')
        print(b'Tryng: ' + token)
        target_proc.sendline(token)
        y = target_proc.readline()
        if y[:5] != b'Sorry':
          print('*' * 160)
          print(y)
          print(target_proc.recvline())
          found_flag = True;
          break
        print(y)
      
      target_proc.close()

      if found_flag == True:
        break

With the output from the challenge server looking like:

    $ ./pwn-chronohack.py verbal-sleep.picoctf.net 56904
    =================== Adjustment: -50 ==================
    [+] Opening connection to verbal-sleep.picoctf.net on port 56904: Done
    Tweak: -50
    b'Tryng: 5lOKufCOih9K8NZvnuDU'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: -49
    b'Tryng: 8W7iojpCCJbtJ8kFWwBa'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: -48
    b'Tryng: qWjh1UJc8mmrystqq1wc'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: -47
    b'Tryng: oTvayGVYBGepi4Ang67h'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: -46

    ..... (snip - lots of iterations) ....

    b'Tryng: UzRRzr1VTHwtrVgkNtNZ'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: 330
    b'Tryng: mmzIZ3PSrMGDMZGjPrtk'
    b'Sorry, your token does not match. Try again!\n'
    Tweak: 331
    b'Tryng: 85t0EybGlH4X8A3HE6E0'
    ****************************************************************************************************************************************************************
    b'Congratulations! You found the correct token.\n'
    b'picoCTF{...........redacted.............}\n'
    [*] Closed connection to verbal-sleep.picoctf.net port 56904

Where the actual flag value has been redacted for the purposes of this write up.

So for me, the required adjustment ended up being +331 milliseconds from my local system time to align the timestamps for seeding.
