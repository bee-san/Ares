# Picker III

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
Can you figure out how this program works to get the flag?

Connect to the program with netcat:
`$ nc saturn.picoctf.net 60097`

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/402](https://play.picoctf.org/practice/challenge/402)

## Solution

### Study the source code

This time the python script is a bit larger. Let's start by looking at the "main" part

```python
import re

USER_ALIVE = True
FUNC_TABLE_SIZE = 4
FUNC_TABLE_ENTRY_SIZE = 32
CORRUPT_MESSAGE = 'Table corrupted. Try entering \'reset\' to fix it'

func_table = ''

<---function declations removed--->

reset_table()
 
while(USER_ALIVE):
  choice = input('==> ')
  if( choice == 'quit' or choice == 'exit' or choice == 'q' ):
    USER_ALIVE = False
  elif( choice == 'help' or choice == '?' ):
    help_text()
  elif( choice == 'reset' ):
    reset_table()
  elif( choice == '1' ):
    call_func(0)
  elif( choice == '2' ):
    call_func(1)
  elif( choice == '3' ):
    call_func(2)
  elif( choice == '4' ):
    call_func(3)
  else:
    print('Did not understand "'+choice+'" Have you tried "help"?')
```

One of the functions is the `win` function

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

However, the `win` function is not available in the function table

```python
def reset_table():
  global func_table
 
  # This table is formatted for easier viewing, but it is really one line
  func_table = \
'''\
print_table                     \
read_variable                   \
write_variable                  \
getRandomNumber                 \
'''
```

### Do a test run

Next let's explore the program behavior by running it as intended

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_III]
└─$ nc saturn.picoctf.net 60097
==> 1
1: print_table
2: read_variable
3: write_variable
4: getRandomNumber
==> 4
4
==> 2
Please enter variable name to read: func_table
print_table                     read_variable                   write_variable                  getRandomNumber                 
==> 
```

### Rewrite the function table to get the encoded flag

Now, let's try to overwrite the function table

```bash
==> 3
Please enter variable name to write: func_table
Please enter new value of variable: "win   read_variable   write_variable   getRandomNumber"
==> 1
Table corrupted. Try entering 'reset' to fix it
==> reset
```

Hhm, that didn't work. We need to study the format of the `func_table` in more detail.

Remember the global variables in the beginning of the script?

```python
FUNC_TABLE_SIZE = 4
FUNC_TABLE_ENTRY_SIZE = 32
CORRUPT_MESSAGE = 'Table corrupted. Try entering \'reset\' to fix it'
```

Also, see the `check_table` function

```python
def check_table():
  global func_table

  if( len(func_table) != FUNC_TABLE_ENTRY_SIZE * FUNC_TABLE_SIZE):
    return False

  return True
```

So the total length of the function table needs to be 32*4 = 128 bytes.

Create a 128-byte string with the letters 'win' left-aligned

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_III]
└─$ python -c "print('\"{0:<128}\"'.format('win'))" 
"win                                                                                                                             "
```

Then copy-and-paste this as input to the `func_table` variable.  
Then "call" the first function in the function table to get the encoded flag.

```bash
==> 3
Please enter variable name to write: func_table
Please enter new value of variable: "win                                                                                                                             "
==> 1
0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x37 0x68 0x31 0x35 0x5f 0x31 0x35 0x5f 0x77 0x68 0x34 0x37 0x5f 0x77 0x33 0x5f 0x67 0x33 0x37 0x5f 0x77 0x31 0x37 0x68 0x5f 0x75 0x35 0x33 0x72 0x35 0x5f 0x31 0x6e 0x5f 0x63 0x68 0x34 0x72 0x67 0x33 0x5f 0x61 0x31 0x38 0x36 0x66 0x39 0x61 0x63 0x7d 
```

### Get the plaintext flag

Finally, to get the plaintext flag you can use either [CyberChef](https://cyberchef.org/) or the `decode.py` script as in the [Picker I challenge](Picker_I.md) challenge.

Another way to get the flag is to use `sed` and `xxd` as below. With `sed` you substitute (with the s-command) '0x' and spaces with "nothing", effectively removing them. And then `xxd` will reverse (-r) the hexdump and output it in plain format (-p).

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_III]
└─$ echo "0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x37 0x68 0x31 0x35 0x5f 0x31 0x35 0x5f 0x77 0x68 0x34 0x37 0x5f 0x77 0x33 0x5f 0x67 0x33 0x37 0x5f 0x77 0x31 0x37 0x68 0x5f 0x75 0x35 0x33 0x72 0x35 0x5f 0x31 0x6e 0x5f 0x63 0x68 0x34 0x72 0x67 0x33 0x5f 0x61 0x31 0x38 0x36 0x66 0x39 0x61 0x63 0x7d" | sed 's/0x//g'

70 69 63 6f 43 54 46 7b 37 68 31 35 5f 31 35 5f 77 68 34 37 5f 77 33 5f 67 33 37 5f 77 31 37 68 5f 75 35 33 72 35 5f 31 6e 5f 63 68 34 72 67 33 5f 61 31 38 36 66 39 61 63 7d

┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_III]
└─$ echo "0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x37 0x68 0x31 0x35 0x5f 0x31 0x35 0x5f 0x77 0x68 0x34 0x37 0x5f 0x77 0x33 0x5f 0x67 0x33 0x37 0x5f 0x77 0x31 0x37 0x68 0x5f 0x75 0x35 0x33 0x72 0x35 0x5f 0x31 0x6e 0x5f 0x63 0x68 0x34 0x72 0x67 0x33 0x5f 0x61 0x31 0x38 0x36 0x66 0x39 0x61 0x63 0x7d" | sed 's/0x//g' | sed 's/ //g'

7069636f4354467b376831355f31355f776834375f77335f6733375f773137685f75353372355f316e5f6368347267335f61313836663961637d

┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_III]
└─$ echo "0x70 0x69 0x63 0x6f 0x43 0x54 0x46 0x7b 0x37 0x68 0x31 0x35 0x5f 0x31 0x35 0x5f 0x77 0x68 0x34 0x37 0x5f 0x77 0x33 0x5f 0x67 0x33 0x37 0x5f 0x77 0x31 0x37 0x68 0x5f 0x75 0x35 0x33 0x72 0x35 0x5f 0x31 0x6e 0x5f 0x63 0x68 0x34 0x72 0x67 0x33 0x5f 0x61 0x31 0x38 0x36 0x66 0x39 0x61 0x63 0x7d" | sed 's/0x//g' | sed 's/ //g' | xxd -r -p
picoCTF{<REDACTED>} 
```

For additional information, please see the references below.

## References

- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [sed - Linux manual page](https://man7.org/linux/man-pages/man1/sed.1.html)
- [xxd - Linux manual page](https://linux.die.net/man/1/xxd)
