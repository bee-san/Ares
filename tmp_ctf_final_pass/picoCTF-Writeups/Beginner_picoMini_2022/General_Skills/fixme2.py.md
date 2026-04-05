# fixme2.py

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Fix the syntax error in the Python script to print the flag.

Download Python script
 
Hints:
1. Are equality and assignment the same symbol?
2. To view the file in the webshell, do: $ nano fixme2.py
3. To exit nano, press Ctrl and x and follow the on-screen prompts.
4. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/241](https://play.picoctf.org/practice/challenge/241)

## Solution

Try running the script and see what happens

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme2.py]
└─$ python fixme2.py 
  File "/mnt/hgfs/CTFs/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme2.py/fixme2.py", line 22
    if flag = "":
       ^^^^^^^^^
SyntaxError: invalid syntax. Maybe you meant '==' or ':=' instead of '='?
```

Python is kind enough to suggest possible solutions. Change the '=' to a '==' and save the script.  
'=' is the assigment operator and '==' is the equal comparison operator.

Then try to run the script again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme2.py]
└─$ python fixme2.py
That is correct! Here's your flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [Operator - Python](https://docs.python.org/3/library/operator.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python Operators - W3Schools](https://www.w3schools.com/python/python_operators.asp)
