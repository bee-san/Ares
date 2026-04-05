# Quantum Scrambler #
 
## Overview ##

200 points

Category: [Reverse Engineering](../)

Tags: `#reverseengineering #decrypt #cipher`

## Description ##

We invented a new cypher that uses "quantum entanglement" to encode the flag. Do you have what it takes to decode it? 
Connect to the program with netcat: 
$ nc verbal-sleep.picoctf.net 51316 
The program's source code can be downloaded here.

## Approach ##

Analysing the provided `quantum_scrambler.py` challenge source code, we see `main()` reads the flag string from the `flag.txt` file and creates a list of hex byte strings of each characters integer representation in the flag string, itself in a list (single element at this stage).

    import sys

    def exit():
      sys.exit(0)

    def scramble(L):
      A = L
      i = 2
      while (i < len(A)):
        A[i-2] += A.pop(i-1)
        A[i-1].append(A[:i-2])
        i += 1
        
      return L

    def get_flag():
      flag = open('flag.txt', 'r').read()
      flag = flag.strip()
      hex_flag = []
      for c in flag:
        hex_flag.append([str(hex(ord(c)))])

      return hex_flag

    def main():
      flag = get_flag()
      cypher = scramble(flag)
      print(cypher)

    if __name__ == '__main__':
      main()

Instrumenting `flag` string in `main()` that is passed to `scramble()` function as the `L` parameter for input reference:

    $ echo "dummyFlag{testing}" > flag.txt 

    flag: [['0x64'], ['0x75'], ['0x6d'], ['0x6d'], ['0x79'], ['0x46'], ['0x6c'], ['0x61'], ['0x67'], ['0x7b'], ['0x74'], ['0x65'], ['0x73'], ['0x74'], ['0x69'], ['0x6e'], ['0x67'], ['0x7d']] (length: 18)

`scramble()` then in a loop moves some of these elements from one list to another.

Further instrumentation was added to the `scramble()` function to better understand with trace output (visualisation) and confirm my understanding of the movements that were occurring.

    def scramble(L):
      A = L
      i = 2
      while (i < len(A)):
        print('-'*20)
        print('i=' + str(i) + ' (before)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))
        A[i-2] += A.pop(i-1)
        print('i=' + str(i) + ' (middle)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))    
        print('---- A[:i-2]=' + str(A[:i-2]))
        A[i-1].append(A[:i-2])
        print('i=' + str(i) + ' (after)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))
        print('---- A[0]=' + str(A[0]))
        print('---- A[1]=' + str(A[1]))
        i += 1
        
      return L

Output of the first two iterations of the loop using the dummy flag input:

    --------------------
    i=2 (before)
            A[i-2]=['0x64']
            A[i-1]=['0x75']
            A[i]=['0x6d']
    i=2 (middle)
            A[i-2]=['0x64', '0x75']
            A[i-1]=['0x6d']
            A[i]=['0x6d']
    ---- A[:i-2]=[]
    i=2 (after)
            A[i-2]=['0x64', '0x75']
            A[i-1]=['0x6d', []]
            A[i]=['0x6d']
    ---- A[0]=['0x64', '0x75']
    ---- A[1]=['0x6d', []]
    --------------------
    i=3 (before)
            A[i-2]=['0x6d', []]
            A[i-1]=['0x6d']
            A[i]=['0x79']
    i=3 (middle)
            A[i-2]=['0x6d', [], '0x6d']
            A[i-1]=['0x79']
            A[i]=['0x46']
    ---- A[:i-2]=[['0x64', '0x75']]
    i=3 (after)
            A[i-2]=['0x6d', [], '0x6d']
            A[i-1]=['0x79', [['0x64', '0x75']]]
            A[i]=['0x46']
    ---- A[0]=['0x64', '0x75']
    ---- A[1]=['0x6d', [], '0x6d']
    --------------------

From here I started to create a `descramble()` function to reverse the list element movements made by `scramble()`.

Looping from the end of the cipher list to the starting position `(i == 2)`, I reverse the `scramble()` process by `pop`ing the list `append`ed and re-`append`ing it to its original `pop`ed position.

    def descramble(C):
      P = C
      i = len(P)-1
      while i >= 2:
        print(i)
        print('i=' + str(i) + ' (before)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        P[i-1].pop()
        print('i=' + str(i) + ' (middle)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        P.insert((i-1), [])
        P[i-1].append(P[i-2].pop())
        print('i=' + str(i) + ' (after)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        i -= 1

      return P

As a sanity check of the process I fed the `cypher` list back through `descramble()` and also re-encoded the list of hex string lists back to characters to reform a `plaintext` string, to verify I could `descramble()` back to my original dummy flag input:

    def main():
      flag = get_flag()
      print('flag: ' + str(flag) + ' (length: ' + str(len(flag)) + ')')
      cypher = scramble(flag)
      print(cypher)
      print('*'*70)
      hex_plaintext = descramble(cypher)
      plaintext = ''
      for c in hex_plaintext:
        plaintext += chr(int(c[0], 16))
      print(plaintext) 

Which generates a lot of instrumentation output, but from the last iteration of the `descramble()` loop and printing of the resultant `plaintext`:

    2
    i=2 (before)
      A[i-2]=['0x64', '0x75']
      A[i-1]=['0x6d', []]
      A[i]=['0x6d']
    i=2 (middle)
      A[i-2]=['0x64', '0x75']
      A[i-1]=['0x6d']
      A[i]=['0x6d']
    i=2 (after)
      A[i-2]=['0x64']
      A[i-1]=['0x75']
      A[i]=['0x6d']
    dummyFlag{testing}

The final step was to obtain and use the encoded challenge flag and feed that through my `descramble()` function. I `tee`'d the output from the challenge server to a `chall.txt` for local input, then quickly realised I needed to get this string to an actual Python list structure (deserialised). Which was done use `eval()`.... I know..

    $ nc verbal-sleep.picoctf.net 51316 | tee chall.txt

Added further code to `main()` to load the encoded flag, deserialise, `descramble()` then re-encode the hex values back to a string for display:

    chall_flag_ciphertext = open('chall.txt', 'r').read()
    hex_chall_flag_ciphertext = eval(chall_flag_ciphertext)
    hex_chall_flag_plaintext = descramble(hex_chall_flag_ciphertext)
    chall_flag_plaintext = ''
    for c in hex_chall_flag_plaintext:
      chall_flag_plaintext += chr(int(c[0], 16))
    print(chall_flag_plaintext)  

## Solution ##

The final [pwntools](https://docs.pwntools.com/en/stable/) script used to capture the flag (including all my superfluous instrumentation code):

    import sys

    step = 4

    def exit():
      sys.exit(0)

    def scramble(L):
      A = L
      i = 2
      #while (i < step): #len(A)):
      while (i < len(A)):
        print('-'*20)
        print('i=' + str(i) + ' (before)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))
        A[i-2] += A.pop(i-1)
        print('i=' + str(i) + ' (middle)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))    
        print('---- A[:i-2]=' + str(A[:i-2]))
        A[i-1].append(A[:i-2])
        print('i=' + str(i) + ' (after)\n\tA[i-2]=' + str(A[i-2]) + '\n\tA[i-1]=' + str(A[i-1]) + '\n\tA[i]=' + str(A[i]))
        print('---- A[0]=' + str(A[0]))
        print('---- A[1]=' + str(A[1]))

        i += 1
        
      return L

    def descramble(C):
      P = C
      i = len(P)-1
      #i = (step-1)
      while i >= 2:
        print(i)
        print('i=' + str(i) + ' (before)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        P[i-1].pop()
        print('i=' + str(i) + ' (middle)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        P.insert((i-1), [])
        P[i-1].append(P[i-2].pop())
        print('i=' + str(i) + ' (after)\n\tA[i-2]=' + str(P[i-2]) + '\n\tA[i-1]=' + str(P[i-1]) + '\n\tA[i]=' + str(P[i]))
        i -= 1

      return P

    def get_flag():
      flag = open('flag.txt', 'r').read()
      flag = flag.strip()
      hex_flag = []
      for c in flag:
        hex_flag.append([str(hex(ord(c)))])

      return hex_flag

    def main():
      flag = get_flag()
      print('flag: ' + str(flag) + ' (length: ' + str(len(flag)) + ')')
      cypher = scramble(flag)
      print(cypher)
      print('*'*70)
      hex_plaintext = descramble(cypher)
      plaintext = ''
      for c in hex_plaintext:
        plaintext += chr(int(c[0], 16))
      print(plaintext)

      chall_flag_ciphertext = open('chall.txt', 'r').read()
      hex_chall_flag_ciphertext = eval(chall_flag_ciphertext)
      hex_chall_flag_plaintext = descramble(hex_chall_flag_ciphertext)
      chall_flag_plaintext = ''
      for c in hex_chall_flag_plaintext:
        chall_flag_plaintext += chr(int(c[0], 16))
      print(chall_flag_plaintext)  

    if __name__ == '__main__':
      main()

Running the script as such, generates a lot of output that I wont include in here, but the final iteration of `decramble()` and decoded flag:

    $ python3 quantum_scrambler_dec.py 

    ..
    ..
    ..

    2
    i=2 (before)
      A[i-2]=['0x70', '0x69']
      A[i-1]=['0x63', []]
      A[i]=['0x6f']
    i=2 (middle)
      A[i-2]=['0x70', '0x69']
      A[i-1]=['0x63']
      A[i]=['0x6f']
    i=2 (after)
      A[i-2]=['0x70']
      A[i-1]=['0x69']
      A[i]=['0x63']
    picoCTF{...........redacted.............}

Where the actual flag value has been redacted for the purposes of this write up.
