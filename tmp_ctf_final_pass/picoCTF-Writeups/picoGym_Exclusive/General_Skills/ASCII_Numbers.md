# ASCII Numbers

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoGym Exclusive, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Convert the following string of ASCII numbers into a readable string:
0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x34 0x35 0x63 0x31 0x31 0x5f 0x6e 0x30 0x5f 0x71 0x75 0x33 0x35 0x37 0x31 0x30 0x6e 0x35 0x5f 0x31 0x6c 0x6c 0x5f 0x74 0x33 0x31 0x31 0x5f 0x79 0x33 0x5f 0x6e 0x30 0x5f 0x6c 0x31 0x33 0x35 0x5f 0x34 0x34 0x35 0x64 0x34 0x31 0x38 0x30 0x7d

Hints:
1. CyberChef is a great tool for any encoding but especially ASCII.
2. Try CyberChef's 'From Hex' function
```

Challenge link: [https://play.picoctf.org/practice/challenge/390](https://play.picoctf.org/practice/challenge/390)

## Solution

This challenge can easily be solved with [CyberChef's 'From Hex' recipe](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')) but that's no fun.

Let's write a python script called `solve.py` instead. The script uses both [lambda](https://docs.python.org/3/reference/expressions.html#lambda) and [map](https://docs.python.org/3/library/functions.html#map) functions.

```python
#!/usr/bin/python

# Create an array of the hex string numbers
enc_flag_array = "0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x34 0x35 0x63 0x31 0x31 0x5f 0x6e 0x30 0x5f 0x71 0x75 0x33 0x35 0x37 0x31 0x30 0x6e 0x35 0x5f 0x31 0x6c 0x6c 0x5f 0x74 0x33 0x31 0x31 0x5f 0x79 0x33 0x5f 0x6e 0x30 0x5f 0x6c 0x31 0x33 0x35 0x5f 0x34 0x34 0x35 0x64 0x34 0x31 0x38 0x30 0x7d".split()

# Convert to numbers
num_array = map(lambda x: int(x, 16), enc_flag_array)

# Convert to chars
char_array = map(chr, num_array)

# Print the flag
print(''.join(char_array))
```

Then run the script to get the flag

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/General_Skills/ASCII_Numbers]
└─$ python solve.py
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII Table](https://www.asciitable.com/)
- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [chr function - Python](https://docs.python.org/3/library/functions.html#chr)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [lambda expression - Python](https://docs.python.org/3/reference/expressions.html#lambda)
- [map function - Python](https://docs.python.org/3/library/functions.html#map)
