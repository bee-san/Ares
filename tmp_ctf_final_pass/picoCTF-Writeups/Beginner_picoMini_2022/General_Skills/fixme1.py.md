# fixme1.py

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
Fix the syntax error in this Python script to print the flag.

Download Python script

Hints:
1. Indentation is very meaningful in Python
2. To view the file in the webshell, do: $ nano fixme1.py
3. To exit nano, press Ctrl and x and follow the on-screen prompts.
4. The str_xor function does not need to be reverse engineered for this challenge.
```

Challenge link: [https://play.picoctf.org/practice/challenge/240](https://play.picoctf.org/practice/challenge/240)

## Solution

Try running the script and see what happens

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme1.py]
└─$ python fixme1.py
  File "/mnt/hgfs/CTFs/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme1.py/fixme1.py", line 20
    print('That is correct! Here\'s your flag: ' + flag)
IndentationError: unexpected indent
```

Python uses [indentation](https://www.w3schools.com/python/gloss_python_indentation.asp) to indicate what lines of code are included in blocks of code.  
The indentation consists of spaces or tabs. You can choose either but you cannot mix in the same script.

Lets look at lines of code around line 20

```python
<---snip--->
flag_enc = chr(0x15) + chr(0x07) + chr(0x08) + chr(0x06) + chr(0x27) + chr(0x21) + chr(0x23) + chr(0x15) + chr(0x5a) + chr(0x07) + chr(0x00) + chr(0x46) + chr(0x0b) + chr(0x1a) + chr(0x5a) + chr(0x1d) + chr(0x1d) + chr(0x2a) + chr(0x06) + chr(0x1c) + chr(0x5a) + chr(0x5c) + chr(0x55) + chr(0x40) + chr(0x3a) + chr(0x5e) + chr(0x52) + chr(0x0c) + chr(0x01) + chr(0x42) + chr(0x57) + chr(0x59) + chr(0x0a) + chr(0x14)
  
flag = str_xor(flag_enc, 'enkidu')
  print('That is correct! Here\'s your flag: ' + flag)
```

The `print` statement is indented but shouldn't be. Remove the spaces before `print` and save the script.

Then try to run the script again

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Fixme1.py]
└─$ python fixme1.py
That is correct! Here's your flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [Indentation style - Wikipedia](https://en.wikipedia.org/wiki/Indentation_style)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
- [Python Indentation - W3Schools](https://www.w3schools.com/python/gloss_python_indentation.asp)
