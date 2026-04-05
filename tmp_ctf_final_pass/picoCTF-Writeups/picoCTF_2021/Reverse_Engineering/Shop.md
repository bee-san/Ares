# Shop

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2021, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: THELSHELL

Description:
Best Stuff - Cheap Stuff, Buy Buy Buy... 
Store Instance: source. 

The shop is open for business at nc mercury.picoctf.net 42159.
 
Hints:
1. Always check edge cases when programming
```

Challenge link: [https://play.picoctf.org/practice/challenge/134](https://play.picoctf.org/practice/challenge/134)

## Solutions

### Analyze the given information

Let's start by looking at what we have.

We have file called `source` that despite its name is a binary

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ file source                                                                                                     
source: ELF 32-bit LSB executable, Intel 80386, version 1 (SYSV), statically linked, Go BuildID=PjavkptB2tPNbBJewQBD/KlDP1g_fpBnKyhti11wQ/JIWBEgtPAt3YPE6g8qd7/pWlMkjZuAYGqbSv46xuR, with debug_info, not stripped
```

A 32-bit ELF-binary.

We also check if the flag is available as a plain text string

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ strings -a -n 8 -e s source | grep -i picoCTF

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ strings -a -n 8 -e S source | grep -i picoCTF

```

Nope, not that easy!

### Do a test run of the binary

Next, we run the program and try to give it unexpected input as suggested in the hint.

We can try invalid menu options

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ ./source
Welcome to the market!
=====================
You have 40 coins
        Item            Price   Count
(0) Quiet Quiches       10      12
(1) Average Apple       15      8
(2) Fruitful Flag       100     1
(3) Sell an Item
(4) Exit
Choose an option: 
-1

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ ./source
Welcome to the market!
=====================
You have 40 coins
        Item            Price   Count
(0) Quiet Quiches       10      12
(1) Average Apple       15      8
(2) Fruitful Flag       100     1
(3) Sell an Item
(4) Exit
Choose an option: 
5

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ 
```

Nothing happens!

And we can try buying a negative number of items

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ ./source
Welcome to the market!
=====================
You have 40 coins
        Item            Price   Count
(0) Quiet Quiches       10      12
(1) Average Apple       15      8
(2) Fruitful Flag       100     1
(3) Sell an Item
(4) Exit
Choose an option: 
0
How many do you want to buy?
-10
You have 140 coins
        Item            Price   Count
(0) Quiet Quiches       10      22
(1) Average Apple       15      8
(2) Fruitful Flag       100     1
(3) Sell an Item
(4) Exit
Choose an option: 
2
How many do you want to buy?
1
Flag is:  [112 105 99 111 67 84 70 123 98 52 100 95 98 114 111 103 114 97 109 109 101 114 95 55 57 55 98 50 57 50 99 125 13 10]
```

Ah, that worked! The flag seems to be encoded as decimal [ASCII](https://en.wikipedia.org/wiki/ASCII).

### Decode the flag

To decode the flag we can use an online site such as [CyberChef](https://gchq.github.io/CyberChef/) and the 'From Decimal' recipe.  
Enter 'decimal' in the `Operations` search bar, then drag and drop `From Decimal` to the `Recipe`.  
Copy the numbers to the `Input` pane and press `BAKE`.  
The flag will be shown in the `Output` pane.

Alternativly, we can use an interactive Python session

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ python                  
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> num_array = "112 105 99 111 67 84 70 123 98 52 100 95 98 114 111 103 114 97 109 109 101 114 95 55 57 55 98 50 57 50 99 125 13 10".split()
>>> int_array = map(int, num_array)
>>> ''.join(map(chr,int_array))
'picoCTF{b4d_<REDACTED>}\r\n'
```

### Automate everything with pwntools

We can use [pwntools](https://docs.pwntools.com/en/stable/index.html) to automate this with a Python script

```python
#!/usr/bin/python

from pwn import *

SERVER = 'mercury.picoctf.net'
PORT = 42159

# Set output level (critical, error, warning, info (default), debug)
context.log_level = "warning"

io = remote(SERVER, PORT)
# Buy -10 Quiet Quiches
io.sendlineafter(b"Choose an option: \n", b"0")
io.sendlineafter(b"How many do you want to buy?\n", b"-10")
# Buy the flag
io.sendlineafter(b"Choose an option: \n", b"2")
io.sendlineafter(b"How many do you want to buy?\n", b"1")
# Retreive the encoded flag
num_array = io.recvallS().split('[')[1][:-2].split()
# Convert to plain text flag
int_array = map(int, num_array)
print(''.join(map(chr,int_array)))
io.close()
```

And run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2021/Reverse_Engineering/Shop]
└─$ ~/python_venvs/pwntools/bin/python get_flag.py
picoCTF{b4d_<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [CyberChef - GitHub](https://github.com/gchq/CyberChef)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Edge case - Wikipedia](https://en.wikipedia.org/wiki/Edge_case)
- [file - Linux manual page](https://man7.org/linux/man-pages/man1/file.1.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [pwntools - Documentation](https://docs.pwntools.com/en/stable/index.html)
- [pwntools - GitHub](https://github.com/Gallopsled/pwntools)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
