# Picker II

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
`$ nc saturn.picoctf.net 59461`

Hints:
 1. Can you do what win does with your input to the program?
```

Challenge link: [https://play.picoctf.org/practice/challenge/401](https://play.picoctf.org/practice/challenge/401)

## Solution

### Study the source code

Let's start by studying the "main" part of the python program.

```python
while(True):
  try:
    user_input = input('==> ')
    if( filter(user_input) ):
      eval(user_input + '()')
    else:
      print('Illegal input')
  except Exception as e:
    print(e)
```

The `filter` function is new and will make things somewhat harder for us

```python
def filter(user_input):
  if 'win' in user_input:
    return False
  return True
```

The `win` function is the same as in the previous 'Picker I' challenge

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

Let's try to call the `win` function directly

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/Reverse_Engineering/Picker_II]
└─$ nc saturn.picoctf.net 59461
==> win
Illegal input
==> Win
name 'Win' is not defined
```

### Get the flag

Finally, let's read the flag directly as suggested in the hint

```bash
==> print(open('flag.txt', 'r').read())
picoCTF{<REDACTED>}
'NoneType' object is not callable
==> 
```

For additional information, please see the references below.

## References

- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [netcat - Wikipedia](https://en.wikipedia.org/wiki/Netcat)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Reading and writing files - Python](https://docs.python.org/3/tutorial/inputoutput.html#reading-and-writing-files)
