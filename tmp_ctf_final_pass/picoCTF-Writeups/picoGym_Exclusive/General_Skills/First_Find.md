# First Find

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoGym Exclusive, General Skills
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: LT 'SYREAL' JONES

Description:
Unzip this archive and find the file named 'uber-secret.txt'

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/320](https://play.picoctf.org/practice/challenge/320)

## Solution

Unzip the file

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/General_Skills/Fist_Find]
└─$ unzip files.zip 
Archive:  files.zip
   creating: files/
   creating: files/satisfactory_books/
   creating: files/satisfactory_books/more_books/
  inflating: files/satisfactory_books/more_books/37121.txt.utf-8  
  inflating: files/satisfactory_books/23765.txt.utf-8  
  inflating: files/satisfactory_books/16021.txt.utf-8  
  inflating: files/13771.txt.utf-8   
   creating: files/adequate_books/
   creating: files/adequate_books/more_books/
   creating: files/adequate_books/more_books/.secret/
   creating: files/adequate_books/more_books/.secret/deeper_secrets/
   creating: files/adequate_books/more_books/.secret/deeper_secrets/deepest_secrets/
 extracting: files/adequate_books/more_books/.secret/deeper_secrets/deepest_secrets/uber-secret.txt  
  inflating: files/adequate_books/more_books/1023.txt.utf-8  
  inflating: files/adequate_books/46804-0.txt  
  inflating: files/adequate_books/44578.txt.utf-8  
   creating: files/acceptable_books/
   creating: files/acceptable_books/more_books/
  inflating: files/acceptable_books/more_books/40723.txt.utf-8  
  inflating: files/acceptable_books/17880.txt.utf-8  
  inflating: files/acceptable_books/17879.txt.utf-8  
  inflating: files/14789.txt.utf-8   
```

The path to the file is visible in the middle of the file listing (prefixed with extracting) but let's search for it anyway

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/General_Skills/Fist_Find]
└─$ find files -name uber-secret.txt
files/adequate_books/more_books/.secret/deeper_secrets/deepest_secrets/uber-secret.txt
```

Finally, display the flag with `cat`

```bash
┌──(kali㉿kali)-[/picoCTF/picoGym/General_Skills/Fist_Find]
└─$ cat files/adequate_books/more_books/.secret/deeper_secrets/deepest_secrets/uber-secret.txt
picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [cat - Linux manual page](https://man7.org/linux/man-pages/man1/cat.1.html)
- [find - Linux manual page](https://man7.org/linux/man-pages/man1/find.1.html)
- [unzip - Linux manual page](https://linux.die.net/man/1/unzip)
