# vault-door-3

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
This vault uses for-loops and byte arrays. 

The source code for this vault is here: VaultDoor3.java

Hints:
1. Make a table that contains each value of the loop variables and the corresponding buffer index that it writes to.
```

Challenge link: [https://play.picoctf.org/practice/challenge/60](https://play.picoctf.org/practice/challenge/60)

## Solutions

The source code looks like this

```java
import java.util.*;

class VaultDoor3 {
    public static void main(String args[]) {
        VaultDoor3 vaultDoor = new VaultDoor3();
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

    // Our security monitoring team has noticed some intrusions on some of the
    // less secure doors. Dr. Evil has asked me specifically to build a stronger
    // vault door to protect his Doomsday plans. I just *know* this door will
    // keep all of those nosy agents out of our business. Mwa ha!
    //
    // -Minion #2671
    public boolean checkPassword(String password) {
        if (password.length() != 32) {
            return false;
        }
        char[] buffer = new char[32];
        int i;
        for (i=0; i<8; i++) {
            buffer[i] = password.charAt(i);
        }
        for (; i<16; i++) {
            buffer[i] = password.charAt(23-i);
        }
        for (; i<32; i+=2) {
            buffer[i] = password.charAt(46-i);
        }
        for (i=31; i>=17; i-=2) {
            buffer[i] = password.charAt(i);
        }
        String s = new String(buffer);
        return s.equals("jU5t_a_sna_3lpm13g34c_u_4_m3rf48");
    }
}
```

In the `checkPassword` method we see that flag characters are scrambled out of order.

Let's write a small python script to re-create the flag

```python
#!/usr/bin/python

password = list("--------------------------------")
buffer = "jU5t_a_sna_3lpm13g34c_u_4_m3rf48"

for i in range(0, 8):
    password[i] = buffer[i]

for i in range(8, 16):
    password[23-i] = buffer[i]

for i in range(16, 32, 2):
    password[46-i] = buffer[i]

for i in range(31, 16, -2):
    password[i] = buffer[i]

flag = ''.join(password)
print(f"Flag: picoCTF{{{flag}}}")
```

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-3]
└─$ ./decode.py
Flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
- [Java String substring()](https://www.javatpoint.com/java-string-substring)
- [Java String charAt()](https://www.javatpoint.com/java-string-charat)
- [Java String length()](https://www.javatpoint.com/java-string-length)
- [Java For Loop](https://www.javatpoint.com/java-for-loop)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
