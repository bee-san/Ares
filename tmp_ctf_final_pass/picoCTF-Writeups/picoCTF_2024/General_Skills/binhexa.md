# binhexa

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2024, General Skills, browser_webshell_solvable
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: NANA AMA ATOMBO-SACKEY

Description:
How well can you perfom basic binary operations?

Start searching for the flag here nc titan.picoctf.net 62850

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/404](https://play.picoctf.org/practice/challenge/404)

## Solution

### Connect to the site

We connect to the site with netcat

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ nc titan.picoctf.net 62850

Welcome to the Binary Challenge!"
Your task is to perform the unique operations in the given order and find the final result in hexadecimal that yields the flag.

Binary Number 1: 10110101
Binary Number 2: 11011111


Question 1/6:
Operation 1: '<<'
Perform a left shift of Binary Number 1 by 1 bits.
Enter the binary result: 
```

We are expected to solve six logical computations of [binary numbers](https://en.wikipedia.org/wiki/Binary_number).  

### Manual solution

This could be done manually (which is good for your understanding but tedious)

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ nc titan.picoctf.net 62850

Welcome to the Binary Challenge!"
Your task is to perform the unique operations in the given order and find the final result in hexadecimal that yields the flag.

Binary Number 1: 10110101
Binary Number 2: 11011111


Question 1/6:
Operation 1: '<<'
Perform a left shift of Binary Number 1 by 1 bits.
Enter the binary result: 101101010
Correct!

Question 2/6:
Operation 2: '>>'
Perform a right shift of Binary Number 2 by 1 bits .
Enter the binary result: 1101111
Correct!

Question 3/6:
Operation 3: '|'
Perform the operation on Binary Number 1&2.
Enter the binary result: 11111111
Correct!

Question 4/6:
Operation 4: '&'
Perform the operation on Binary Number 1&2.
Enter the binary result: 10010101
Correct!

Question 5/6:
Operation 5: '*'
Perform the operation on Binary Number 1&2.
Enter the binary result: 1001110110101011
Correct!

Question 6/6:
Operation 6: '+'
Perform the operation on Binary Number 1&2.
Enter the binary result: 110010100
Correct!

Enter the results of the last operation in hexadecimal: 194

Correct answer!
The flag is: picoCTF{<REDACTED>}
```

If you aren't familiar with the operations they are:

```text
<< (Logical left shift)
>> (Logical right shift)
| (OR)
& (AND)
* (Multiplication)
+ (Addition)
```

### Python solution

Alternatively, we could use [Python](https://en.wikipedia.org/wiki/Python_(programming_language)) to solve one or more problems for us like this

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(a << 1)[2:])"
101101010

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(b >> 1)[2:])"
1101111

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(a | b)[2:])" 
11111111

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(a & b)[2:])"
10010101

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(a * b)[2:])"
1001110110101011

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(bin(a + b)[2:])"
110010100

┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2024/General_Skills/binhexa]
└─$ python -c "a=0b10110101; b=0b11011111; print(hex(a + b)[2:])"
194
```

For additional information, please see the references below.

## References

- [Binary number - Wikipedia](https://en.wikipedia.org/wiki/Binary_number)
- [Bitwise operation - Wikipedia](https://en.wikipedia.org/wiki/Bitwise_operation)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Logical shift - Wikipedia](https://en.wikipedia.org/wiki/Logical_shift)
- [nc - Linux manual page](https://linux.die.net/man/1/nc)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
