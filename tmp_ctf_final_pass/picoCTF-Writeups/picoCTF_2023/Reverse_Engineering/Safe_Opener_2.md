# Safe Opener 2

- [Challenge information](#challenge-information)
- [Grepping for the flag solution](#grepping-for-the-flag-solution)
- [Decompiling with JD-GUI solution](#decompiling-with-jd-gui-solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2023, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL
 
Description:
What can you do with this file?

I forgot the key to my safe but this file is supposed to help me with retrieving the lost key.  
Can you help me unlock my safe?
 
Hints:
1. Download and try to decompile the file.
```

Challenge link: [https://play.picoctf.org/practice/challenge/375](https://play.picoctf.org/practice/challenge/375)

There are several ways to solve this challenge. Here are two solutions presented in increasing difficulty.

## Grepping for the flag solution

On easy challenges it's always recommended to search for the flag in plain text with `strings` and `grep`.

```bash
┌──(kali㉿kali)-[/picoCTF/picoCTF_2023/Reverse_Engineering/Safe_Opener_2]
└─$ strings -a -n 8 SafeOpener.class | grep picoCTF
,picoCTF{<REDACTED>}
```

## Decompiling with JD-GUI solution

A more sofisticated solution is to decompile the file in [JD-GUI](https://github.com/java-decompiler/jd-gui) and study the code.

You find the flag in the openSafe function (but it's redacted here).

```C
  public static boolean openSafe(String password)
  {
    String encodedkey = "picoCTF{<REDACTED>}";
    if (password.equals(encodedkey))
    {
      System.out.println("Sesame open");
      return true;
    }
    System.out.println("Password is incorrect\n");
    return false;
  }
```

For additional information, please see the references below.

## References

- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [JD-GUI - GitHub](https://github.com/java-decompiler/jd-gui)
- [String (computer science) - Wikipedia](https://en.wikipedia.org/wiki/String_(computer_science))
- [strings - Linux manual page](https://man7.org/linux/man-pages/man1/strings.1.html)
