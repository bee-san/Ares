# Very Smooth

## Challenge

Forget safe primes... Here, we like to live life dangerously... >:)

* [gen.py](gen.py)
* [output.txt](output.txt)

## Solution

1. Searching online for "pollard smooth prime" finds [Pollard's p − 1 algorithm](https://en.wikipedia.org/wiki/Pollard%27s_p_%E2%88%92_1_algorithm).

2. Using [RsaCtfTool](https://github.com/Ganapati/RsaCtfTool/) with the `pollard_p_1` attack by running `python RsaCtfTool.py --uncipher [c] -e 65537 -n [n] --attack pollard_p_1` doesn't work since it doesn't try enough primes ([relevant source code](https://github.com/Ganapati/RsaCtfTool/blob/c13713a2808a03b15eb62e35605b9eb4271069cc/attacks/single_key/pollard_p_1.py)). So, we adapt their script to create the solution [script.py](script.py), which tries 7000 primes.

3. Interestingly, this prime is in factordb so [RsaCtfTool](https://github.com/Ganapati/RsaCtfTool/) will print the flag immediately when using the `factordb` attack.

### Flag

`picoCTF{376ebfe7}`
