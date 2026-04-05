# vault-door-1

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MARK E. HAASE

Description:
This vault uses some complicated arrays! I hope you can make sense of it, special agent. 

The source code for this vault is here: VaultDoor1.java

Hints:
1. Look up the charAt() method online.
```

Challenge link: [https://play.picoctf.org/practice/challenge/12](https://play.picoctf.org/practice/challenge/12)

## Solutions

The source code looks like this

```java
import java.util.*;

class VaultDoor1 {
    public static void main(String args[]) {
        VaultDoor1 vaultDoor = new VaultDoor1();
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter vault password: ");
    String userInput = scanner.next();
    String input = userInput.substring("picoCTF{".length(),userInput.length()-1);
    if (vaultDoor.checkPassword(input)) {
        System.out.println("Access granted.");
    } else {
        System.out.println("Access denied!");
    }
    }

    // I came up with a more secure way to check the password without putting
    // the password itself in the source code. I think this is going to be
    // UNHACKABLE!! I hope Dr. Evil agrees...
    //
    // -Minion #8728
    public boolean checkPassword(String password) {
        return password.length() == 32 &&
               password.charAt(0)  == 'd' &&
               password.charAt(29) == '3' &&
               password.charAt(4)  == 'r' &&
               password.charAt(2)  == '5' &&
               password.charAt(23) == 'r' &&
               password.charAt(3)  == 'c' &&
               password.charAt(17) == '4' &&
               password.charAt(1)  == '3' &&
               password.charAt(7)  == 'b' &&
               password.charAt(10) == '_' &&
               password.charAt(5)  == '4' &&
               password.charAt(9)  == '3' &&
               password.charAt(11) == 't' &&
               password.charAt(15) == 'c' &&
               password.charAt(8)  == 'l' &&
               password.charAt(12) == 'H' &&
               password.charAt(20) == 'c' &&
               password.charAt(14) == '_' &&
               password.charAt(6)  == 'm' &&
               password.charAt(24) == '5' &&
               password.charAt(18) == 'r' &&
               password.charAt(13) == '3' &&
               password.charAt(19) == '4' &&
               password.charAt(21) == 'T' &&
               password.charAt(16) == 'H' &&
               password.charAt(27) == 'f' &&
               password.charAt(30) == 'b' &&
               password.charAt(25) == '_' &&
               password.charAt(22) == '3' &&
               password.charAt(28) == '6' &&
               password.charAt(26) == 'f' &&
               password.charAt(31) == '0';
    }
}
```

In the `checkPassword` method we see a lot of [charAt](https://www.javatpoint.com/java-string-charat) checks. The `&&` is the [logical AND operator](https://www.freecodecamp.org/news/java-operator-and-or-logical-operators/).

We want to extract the characters and append them in a sorted order. We need to use a number of commandline tools to achieve this.

First we `grep` all the lines with charAt

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-1]
└─$ grep charAt VaultDoor1.java 
               password.charAt(0)  == 'd' &&
               password.charAt(29) == '3' &&
               password.charAt(4)  == 'r' &&
               password.charAt(2)  == '5' &&
               password.charAt(23) == 'r' &&
               password.charAt(3)  == 'c' &&
               password.charAt(17) == '4' &&
               password.charAt(1)  == '3' &&
               password.charAt(7)  == 'b' &&
               password.charAt(10) == '_' &&
               password.charAt(5)  == '4' &&
               password.charAt(9)  == '3' &&
               password.charAt(11) == 't' &&
               password.charAt(15) == 'c' &&
               password.charAt(8)  == 'l' &&
               password.charAt(12) == 'H' &&
               password.charAt(20) == 'c' &&
               password.charAt(14) == '_' &&
               password.charAt(6)  == 'm' &&
               password.charAt(24) == '5' &&
               password.charAt(18) == 'r' &&
               password.charAt(13) == '3' &&
               password.charAt(19) == '4' &&
               password.charAt(21) == 'T' &&
               password.charAt(16) == 'H' &&
               password.charAt(27) == 'f' &&
               password.charAt(30) == 'b' &&
               password.charAt(25) == '_' &&
               password.charAt(22) == '3' &&
               password.charAt(28) == '6' &&
               password.charAt(26) == 'f' &&
               password.charAt(31) == '0';
```

Next we use `cut` with a delimmiter of '('

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-1]
└─$ grep charAt VaultDoor1.java | cut -d '(' -f2
0)  == 'd' &&
29) == '3' &&
4)  == 'r' &&
2)  == '5' &&
23) == 'r' &&
3)  == 'c' &&
17) == '4' &&
1)  == '3' &&
7)  == 'b' &&
10) == '_' &&
5)  == '4' &&
9)  == '3' &&
11) == 't' &&
15) == 'c' &&
8)  == 'l' &&
12) == 'H' &&
20) == 'c' &&
14) == '_' &&
6)  == 'm' &&
24) == '5' &&
18) == 'r' &&
13) == '3' &&
19) == '4' &&
21) == 'T' &&
16) == 'H' &&
27) == 'f' &&
30) == 'b' &&
25) == '_' &&
22) == '3' &&
28) == '6' &&
26) == 'f' &&
31) == '0';
```

Then we `sort` numerically

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-1]
└─$ grep charAt VaultDoor1.java | cut -d '(' -f2 | sort -n
0)  == 'd' &&
1)  == '3' &&
2)  == '5' &&
3)  == 'c' &&
4)  == 'r' &&
5)  == '4' &&
6)  == 'm' &&
7)  == 'b' &&
8)  == 'l' &&
9)  == '3' &&
10) == '_' &&
11) == 't' &&
12) == 'H' &&
13) == '3' &&
14) == '_' &&
15) == 'c' &&
16) == 'H' &&
17) == '4' &&
18) == 'r' &&
19) == '4' &&
20) == 'c' &&
21) == 'T' &&
22) == '3' &&
23) == 'r' &&
24) == '5' &&
25) == '_' &&
26) == 'f' &&
27) == 'f' &&
28) == '6' &&
29) == '3' &&
30) == 'b' &&
31) == '0';
```

Next we `cut` again but with a delimiter of "'"

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-1]
└─$ grep charAt VaultDoor1.java | cut -d '(' -f2 | sort -n | cut -d \' -f2
d
3
5
c
r
4
m
b
l
3
_
t
H
3
_
c
H
4
r
4
c
T
3
r
5
_
f
f
6
3
b
0
```

Finally, we remove newlines with `tr`

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-1]
└─$ grep charAt VaultDoor1.java | cut -d '(' -f2 | sort -n | cut -d \' -f2 | tr -d '\n'
d35cr4mbl3_<REDACTED>
```

To get the flag we need to add 'picoCTF{' in the beginning and an '}' at the end.

For additional information, please see the references below.

## References

- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
- [Java String substring()](https://www.javatpoint.com/java-string-substring)
- [Java String charAt()](https://www.javatpoint.com/java-string-charat)
- [Java String length()](https://www.javatpoint.com/java-string-length)
- [cut - Linux manual page](https://man7.org/linux/man-pages/man1/cut.1.html)
- [grep - Linux manual page](https://man7.org/linux/man-pages/man1/grep.1.html)
- [sort - Linux manual page](https://man7.org/linux/man-pages/man1/sort.1.html)
- [tr - Linux manual page](https://man7.org/linux/man-pages/man1/tr.1.html)
