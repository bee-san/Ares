# spelling-quiz

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoMini by redpwn, Cryptography
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: BROWNIEINMOTION

Description:
I found the flag, but my brother wrote a program to encrypt all his text files. 
He has a spelling quiz study guide too, but I don't know if that helps.

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/210](https://play.picoctf.org/practice/challenge/210)

## Solution

### Analyze the given files

Let's start by looking at the given files. First the python script

```python
import random
import os

files = [
    os.path.join(path, file)
    for path, dirs, files in os.walk('.')
    for file in files
    if file.split('.')[-1] == 'txt'
]

alphabet = list('abcdefghijklmnopqrstuvwxyz')
random.shuffle(shuffled := alphabet[:])
dictionary = dict(zip(alphabet, shuffled))

for filename in files:
    text = open(filename, 'r').read()
    encrypted = ''.join([
        dictionary[c]
        if c in dictionary else c
        for c in text
    ])
    open(filename, 'w').write(encrypted)
```

The script encrypts all .txt files in the current directory and all its subdirectories.  
The files are encrypted with a simple substitution cipher.

Then we check the first lines of the `study-guide.txt` file and how many words it contains

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ head study-guide.txt                                                        
nxsuvjujrf
ipawfpzms
hsffrvfpsd
cpedjzdsl
ajdmsjrmjb
yjrbjupdmsl
zdmunsmmiz
wajrbfszjp
bjzphsffpcjajdl
wsmvajuv

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ wc -l study-guide.txt 
272543 study-guide.txt
```

And last but not least we check the encrypted flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ cat flag.txt                                                                              
efsvpez_dvf_bmh_kxiefb_myfs_gpz_kxzd_djsfb
```

It looks like the first `picoCTF{` part and the trailing `}` is omitted and we need to add them later.

### Break the cipher

To break the substitution cipher I installed and used [Subbreaker](https://gitlab.com/guballa/SubstitutionBreaker).

It was new tool for me so I needed to check its help

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ ~/python_venvs/subbreaker/bin/subbreaker -h        
usage: subbreaker [-h] {break,decode,encode,fitness,quadgrams,info,version} ...

A collection of tools to work with substitution ciphers

positional arguments:
  {break,decode,encode,fitness,quadgrams,info,version}
                        subcommands to execute
    break               break a substitution cipher
    decode              decode a substitution cipher with a given key
    encode              encode a plaintext with a given key
    fitness             calculate the fitness for a given plaintext
    quadgrams           create quadgrams from a given text corpus
    info                print various information about the quadgram file
    version             print the version of the tool

options:
  -h, --help            show this help message and exit


┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ ~/python_venvs/subbreaker/bin/subbreaker break -h
usage: subbreaker break [-h] [--lang {EN}] [--text <string> | --ciphertext <path>] [--consolidate <int>] [--max-tries <int>]

options:
  -h, --help           show this help message and exit
  --lang {EN}          language of the text. The default is EN for English.
  --text <string>      string containing the input text. Note, that line breaks and blanks might require shell escaping.
  --ciphertext <path>  name of the file containing the input text. If neither --text nor --ciphertext is given, the text will be read from STDIN.
  --consolidate <int>  how often the same key must be found before it is regarded as the best solution. Default is 3. Lower values provide faster but unreliable results. If unsure don't touch
                       it.
  --max-tries <int>    the maximum number of hill climbings attempts. If no solution is found before this value is reached the best solution so far will be provided.
```

I first tried the entire `study-guide.txt` file as input but it seemed to take quite a while so I aborted the run.  
Then I tried only the first 50 lines of the file but the key became corrupt with at least one faulty substitution.

However, the first 100 lines worked just fine

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ head -n 100 study-guide.txt | ~/python_venvs/subbreaker/bin/subbreaker break
Alphabet: abcdefghijklmnopqrstuvwxyz
Key:      pcubfwhvjknairmetszdxygolq
Fitness: 90.41
Nbr keys tried: 6825
Keys per second: 8261
Execution time (seconds): 0.826
Plaintext:
kurchicine
malfeasor
greenheart
baptistry
litorinoid
vindicatory
stockrooms
flindersia
<---snip--->
```

So the substitution key is `pcubfwhvjknairmetszdxygolq`.

### Get the flag

I used subbreaker to decode the flag but needed help with the parameters once again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ ~/python_venvs/subbreaker/bin/subbreaker decode -h      
usage: subbreaker decode [-h] (--key <string> | --keyword <string>) [--alphabet <string>] [--text <string> | --ciphertext <path>] [--plaintext <path>]

options:
  -h, --help           show this help message and exit
  --key <string>       key containing all characters from the alphabet. The key can be specified case insensitive, may not contain any non-alphabetical characters, and every character of the
                       alphabet must be present exactly once.
  --keyword <string>   a case-insensitive keyword used to build the key. The key is created by writing out the keyword, removing repeated letters in it, then writing all the remaining letters in
                       the alphabet in the usual order. E.g., the keyword "ZEBRAS" leads to the key "zebrascdfghijklmnopqtuvwxy".
  --alphabet <string>  a string of characters which build the alphabet. Lower and upper characters are treated the same. By default, 'abcdefghijklmnopqrstuvwxyz' is used, but any character is
                       allowed (including the blank and e.g., umlauts). The length of the alphabet may not exceed 32 characters.
  --text <string>      string containing the input text. Note, that line breaks and blanks might require shell escaping.
  --ciphertext <path>  name of the file containing the input text. If neither --text nor --ciphertext is given, the text will be read from STDIN.
  --plaintext <path>   name of the file the output is written to. If it is not given, the output is printed to STDOUT.
```

Finally, time to get the plain text (with its ending redacted)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoMini_by_redpwn/Cryptography/spelling-quiz]
└─$ ~/python_venvs/subbreaker/bin/subbreaker decode --key pcubfwhvjknairmetszdxygolq --ciphertext flag.txt
perhaps_the_dog_<REDACTED>
```

For additional information, please see the references below.

## References

- [head - Linux manual page](https://man7.org/linux/man-pages/man1/head.1.html)
- [os.path — Common pathname manipulations](https://docs.python.org/3/library/os.path.html)
- [os.walk function](https://docs.python.org/3/library/os.html#os.walk)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [random module — Python](https://docs.python.org/3/library/random.html)
- [SubstitutionBreaker](https://gitlab.com/guballa/SubstitutionBreaker)
- [Substitution cipher - Wikipedia](https://en.wikipedia.org/wiki/Substitution_cipher)
- [What does colon equal (:=) in Python mean? - Stack Overflow](https://stackoverflow.com/questions/26000198/what-does-colon-equal-in-python-mean)
- [wc - Linux manual page](https://man7.org/linux/man-pages/man1/wc.1.html)
