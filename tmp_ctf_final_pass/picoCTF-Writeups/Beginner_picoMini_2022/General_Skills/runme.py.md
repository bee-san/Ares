# runme.py

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: Beginner picoMini 2022, General Skills, Python
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: SUJEET KUMAR
 
Description:
Run the runme.py script to get the flag. 
Download the script with your browser or with wget in the webshell.
Download runme.py Python script

Hints:
1. If you have Python on your computer, you can download the script normally and run it. 
   Otherwise, use the wget command in the webshell.
2. To use wget in the webshell, first right click on the download link and select 'Copy Link' or 'Copy Link Address'
3. Type everything after the dollar sign in the webshell: 
   $ wget , then paste the link after the space after wget and press enter. 
   This will download the script for you in the webshell so you can run it!
4. Finally, to run the script, type everything after the dollar sign and then press enter: 
   $ python3 runme.py You should have the flag now!
```

Challenge link: [https://play.picoctf.org/practice/challenge/250](https://play.picoctf.org/practice/challenge/250)

## Solution

This challenge is very straight forward, but let's start with looking at the script

```python
#!/usr/bin/python3
################################################################################
# Python script which just prints the flag
################################################################################

flag ='picoCTF{<REDACTED>}'
print(flag)
```

And there is our flag (but it's redacted above).

If we still want to run the script we can certainly do that. Either explicitly with Python

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Runme.py]
└─$ python runme.py              
picoCTF{<REDACTED>}
```

Or by making sure it is executable and then run it stand-alone

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Runme.py]
└─$ chmod +x runme.py 

┌──(kali㉿kali)-[/mnt/…/picoCTF/Beginner_picoMini_2022/General_Skills/Runme.py]
└─$ ./runme.py              
picoCTF{<REDACTED>}
```

### References

- [chmod - Linux manual page](https://man7.org/linux/man-pages/man1/chmod.1.html)
- [python - Linux manual page](https://linux.die.net/man/1/python)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
