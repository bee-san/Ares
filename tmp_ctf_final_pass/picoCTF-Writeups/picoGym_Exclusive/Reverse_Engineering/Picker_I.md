# Picker I

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoGym Exclusive, Reverse Engineering, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
This service can provide you with a random number, but can it do anything else?

Connect to the program with netcat:
`$ nc saturn.picoctf.net 58059`

Hints:
 1. Can you point the program to a function that does something useful for you?
```

Challenge link: [https://play.picoctf.org/practice/challenge/400](https://play.picoctf.org/practice/challenge/400)

## Solution

### Study the source code

Let's start by studying the "main" part of the python program.

```python
while(True):
  try:
    print('Try entering "getRandomNumber" without the double quotes...')
    user_input = input('==> ')
    eval(user_input + '()')
  except Exception as e:
    print(e)
```

Then check the `getRandomNumber` function

```python
def getRandomNumber():
  print(4)  # Chosen by fair die roll.
            # Guaranteed to be random.
            # (See XKCD)
```

The comment is referring to this [XKCD comic strip](https://xkcd.com/221/).

More interesting is this `win` function which output the flag as hex values.

```python
def win():
  # This line will not work locally unless you create your own 'flag.txt' in
  #   the same directory as this script
  flag = open('flag.txt', 'r').read()
  #flag = flag[:-1]
  flag = flag.strip()
  str_flag = ''
  for c in flag:
    str_flag += str(hex(ord(c))) + ' '
  print(str_flag)
```

### Do a test run

Next let's explore the program behavior by running it

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_I]
└─$ nc saturn.picoctf.net 58059
Try entering "getRandomNumber" without the double quotes...
==> getRandomNumber
4
```

### Get the encoded flag

Now let's call the `win` function instead to get the flag

```bash
Try entering "getRandomNumber" without the double quotes...
==> win
0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x34 0x5f 0x64 0x31 0x34 0x6d 0x30 0x6e 0x64 0x5f 0x31 0x6e 0x5f 0x37 0x68 0x33 0x5f 0x72 0x30 0x75 0x67 0x68 0x5f 0x36 0x65 0x30 0x34 0x34 0x34 0x30 0x64 0x7d 
```

We have the flag hexadecimal encoded.

### Get the plaintext flag

Finally, we need to decode the flag. This can be done with [CyberChef's 'From Hex' recipe](https://gchq.github.io/CyberChef/#recipe=From_Hex('Auto')) or with a python script.

Let's write a python script called `decode.py`

```python
#!/usr/bin/python

# Create an array of the hex string numbers
enc_flag_array = "0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x34 0x5f 0x64 0x31 0x34 0x6d 0x30 0x6e 0x64 0x5f 0x31 0x6e 0x5f 0x37 0x68 0x33 0x5f 0x72 0x30 0x75 0x67 0x68 0x5f 0x36 0x65 0x30 0x34 0x34 0x34 0x30 0x64 0x7d ".split()

# Convert to numbers
num_array = map(lambda x: int(x, 16), enc_flag_array)

# Convert to chars
char_array = map(chr, num_array)

# Join and print the flag
print(''.join(char_array))
```

Then set the script file as executable and run it to get the flag

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_I]
└─$ chmod a+x decode.py                                           

┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_I]
└─$ ./decode.py   
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII Table](https://www.asciitable.com/)
- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [chr function - Python](https://docs.python.org/3/library/functions.html#chr)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [join method - Python](https://docs.python.org/3/library/stdtypes.html#str.join)
- [lambda expression - Python](https://docs.python.org/3/reference/expressions.html#lambda)
- [map function - Python](https://docs.python.org/3/library/functions.html#map)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
