# Surfing the Waves

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Hard
Tags: picoCTF 2021, Forensics
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: WILLIAM BATISTA

Description:
While you're going through the FBI's servers, you stumble across their incredible taste in music. 
One main.wav you found is particularly interesting, see if you can find the flag!
 
Hints:
1. Music is cool, but what other kinds of waves are there?
2. Look deep below the surface
```

Challenge link: [https://play.picoctf.org/practice/challenge/117](https://play.picoctf.org/practice/challenge/117)

## Solution

### Analyze in Sonic Visualiser

I first tried to view the wav-file in [Sonic Visualiser](https://www.sonicvisualiser.org/) and saw that the wave-form was always in a rather narrow range of values.  
What were these values?

### Analyze in SciPy - Part 1

Next, I used [SciPy](https://scipy.org/) which can [read wav-files](https://docs.scipy.org/doc/scipy/tutorial/io.html#wav-sound-files-scipy-io-wavfile) to analyze the values. If needed install SciPy with either `sudo apt-get install python3-scipy` or `python -m pip install scipy`.

My first try was just to read the file and print the 100 first values and all unique values

```python
#!/usr/bin/python

from scipy.io.wavfile import read
from numpy import unique

rate, array = read("main.wav")
print("First 100 values:")
print(array[:100])
print("Unique values:")
print(unique(array))
```

Running the script gave me this output

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Surfing_the_Waves]
└─$ ./analyze.py
First 100 values:
[2008 2506 2000 1508 2009 8504 4500 3500 4507 2502 4508 2006 2008 8509
 4000 2006 4006 5504 4004 8009 2005 8507 4005 3506 4003 8007 4500 4003
 2006 1008 4506 1000 4503 5508 4502 3001 4002 5009 4005 8506 4008 8001
 2500 2501 1005 6002 4005 5506 4000 7504 4507 1004 4005 8507 4508 2006
 4505 3000 2007 1006 4009 8000 4502 3509 4009 7506 4505 1008 4502 5509
 2007 1006 4008 1505 4500 2507 2005 1005 4006 8005 4505 1006 1000 6002
 4006 4006 4508 2003 4000 8505 4008 7505 2008 1001 4501 2508 4002 2507
 4005 5507]
Unique values:
[1000 1001 1002 1003 1004 1005 1006 1007 1008 1009 1500 1501 1502 1503
 1504 1505 1506 1507 1508 1509 2000 2001 2002 2003 2004 2005 2006 2007
 2008 2009 2500 2501 2502 2503 2504 2505 2506 2507 2508 2509 3000 3001
 3002 3003 3004 3005 3006 3007 3008 3009 3500 3501 3502 3503 3504 3505
 3506 3507 3508 3509 4000 4001 4002 4003 4004 4005 4006 4007 4008 4009
 4500 4501 4502 4503 4504 4505 4506 4507 4508 4509 5000 5001 5002 5003
 5004 5005 5006 5007 5008 5009 5500 5501 5502 5503 5504 5505 5506 5507
 5508 5509 6000 6001 6002 6003 6004 6005 6006 6007 6008 6009 6501 6502
 6503 6504 6505 6506 6507 6508 7000 7001 7002 7003 7004 7005 7007 7008
 7009 7500 7501 7502 7503 7504 7505 7506 7507 7508 7509 8000 8001 8002
 8003 8004 8005 8006 8007 8008 8009 8500 8501 8502 8503 8504 8505 8506
 8507 8508 8509]
```

Hm, the first two digits seems to be multiples of five, from 10, 15, 20, etc. to 80 and 85. This is 16 different values!  
And the last two digits seems to be only in the range 00 to 09.

What if the first two digits represent hexadecimal values encoding the flag?

### Analyze in SciPy - Part 2

Next, I converted the data to hexadecimal numbers

```python
#!/usr/bin/python

from scipy.io.wavfile import read
from string import hexdigits

rate, array = read("main.wav")
for val in array:
    i = val // 500 - 2
    print(hexdigits[i])
```

Running this script gave me the following output

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Surfing_the_Waves]
└─$ ./analyze_2.py
2
3
2
1
2
f
7
5
7
3
7
2
2
f
6
2
<---snip--->
```

### Decode the flag

Finally, I collected all the hex-digits in a string and converted it to ASCII with this script

```python
#!/usr/bin/python

from scipy.io.wavfile import read
from string import hexdigits

rate, array = read("main.wav")

hex_str = ""
for val in array:
    i = val // 500 - 2
    hex_str += hexdigits[i]

print(bytearray.fromhex(hex_str).decode())
```

Running the script gave me a Python-script with the flag at the end

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Forensics/Surfing_the_Waves]
└─$ ./decode.py
#!/usr/bin/env python3
import numpy as np
from scipy.io.wavfile import write
from binascii import hexlify
from random import random

with open('generate_wav.py', 'rb') as f:
        content = f.read()
        f.close()

# Convert this program into an array of hex values
hex_stuff = (list(hexlify(content).decode("utf-8")))

# Loop through the each character, and convert the hex a-f characters to 10-15
for i in range(len(hex_stuff)):
        if hex_stuff[i] == 'a':
                hex_stuff[i] = 10
        elif hex_stuff[i] == 'b':
                hex_stuff[i] = 11
        elif hex_stuff[i] == 'c':
                hex_stuff[i] = 12
        elif hex_stuff[i] == 'd':
                hex_stuff[i] = 13
        elif hex_stuff[i] == 'e':
                hex_stuff[i] = 14
        elif hex_stuff[i] == 'f':
                hex_stuff[i] = 15

        # To make the program actually audible, 100 hertz is added from the beginning, then the number is multiplied by
        # 500 hertz
        # Plus a cheeky random amount of noise
        hex_stuff[i] = 1000 + int(hex_stuff[i]) * 500 + (10 * random())


def sound_generation(name, rand_hex):
        # The hex array is converted to a 16 bit integer array
        scaled = np.int16(np.array(hex_stuff))
        # Sci Pi then writes the numpy array into a wav file
        write(name, len(hex_stuff), scaled)
        randomness = rand_hex


# Pump up the music!
# print("Generating main.wav...")
# sound_generation('main.wav')
# print("Generation complete!")

# Your ears have been blessed
# picoCTF{<REDACTED>} 
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [numpy - Homepage](https://numpy.org/)
- [numpy - PyPI](https://pypi.org/project/numpy)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [SciPy - Documentation](https://docs.scipy.org/doc/scipy/)
- [SciPy - Homepage](https://scipy.org/)
- [SciPy - PyPI](https://pypi.org/project/scipy/)
- [Sonic Visualiser - Homepage](https://www.sonicvisualiser.org/)
- [WAV - Wikipedia](https://en.wikipedia.org/wiki/WAV)
