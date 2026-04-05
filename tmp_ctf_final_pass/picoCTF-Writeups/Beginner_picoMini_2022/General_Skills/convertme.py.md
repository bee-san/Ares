# convertme.py

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, base, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Run the Python script and convert the given number from decimal to binary to get the flag.

Download Python script

Hints:
1. Look up a decimal to binary number conversion app on the web or use your computer's calculator!
2. The str_xor function does not need to be reverse engineered for this challenge.
3. If you have Python on your computer, you can download the script normally and run it. 
   Otherwise, use the wget command in the webshell.
4. To use wget in the webshell, first right click on the download link and select 'Copy Link' or 'Copy Link Address'
5. Type everything after the dollar sign in the webshell: $ wget , then paste the link after the space after wget and press enter. 
   This will download the script for you in the webshell so you can run it!
6. Finally, to run the script, type everything after the dollar sign and then press enter: $ python3 convertme.py
```

Challenge link: [https://play.picoctf.org/practice/challenge/239](https://play.picoctf.org/practice/challenge/239)

## Solution

Start by running the script and see what random number is selected

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Convertme.py]
└─$ python convertme.py 
If 25 is in decimal base, what is it in binary base?
Answer: 
```

You can solve the conversion in a number of ways aside from calculating the result manually:

- Use an online service such as [RapidTables](https://www.rapidtables.com/convert/number/base-converter.html) to do the calculation
- Use an interactive Python session in another window
- Use the linux `bc` command in another window

### Convert using Python

Converting the number to binary with the `bin` function

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Convertme.py]
└─$ python             
Python 3.11.4 (main, Jun  7 2023, 10:13:09) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> bin(25)
'0b11001'
>>> quit()
```

Skip the initial '0b' when you enter your answer

### Convert using bc

Alternatively, you can use the linux `bc` command. Install it with `sudo apt install bc` if it isn't installed already.

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Convertme.py]
└─$ echo "obase=2; 25" | bc                                                      
11001
```

### Get the flag

Then we fill in the answer and get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Convertme.py]
└─$ python convertme.py 
If 25 is in decimal base, what is it in binary base?
Answer: 11001
That is correct! Here's your flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

### References

- [bc - Linux manual page](https://man7.org/linux/man-pages/man1/bc.1p.html)
- [Binary number - Wikipedia](https://en.wikipedia.org/wiki/Binary_number)
- [echo - Linux manual page](https://man7.org/linux/man-pages/man1/echo.1.html)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
