# vault-door-4

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
This vault uses ASCII encoding for the password. 

The source code for this vault is here: VaultDoor4.java

Hints:
1. Use a search engine to find an "ASCII table".
2. You will also need to know the difference between octal, decimal, and hexadecimal numbers.
```

Challenge link: [https://play.picoctf.org/practice/challenge/71](https://play.picoctf.org/practice/challenge/71)

## Solutions

The java source code looks like this

```java
import java.util.*;

class VaultDoor4 {
    public static void main(String args[]) {
        VaultDoor4 vaultDoor = new VaultDoor4();
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

    // I made myself dizzy converting all of these numbers into different bases,
    // so I just *know* that this vault will be impenetrable. This will make Dr.
    // Evil like me better than all of the other minions--especially Minion
    // #5620--I just know it!
    //
    //  .:::.   .:::.
    // :::::::.:::::::
    // :::::::::::::::
    // ':::::::::::::'
    //   ':::::::::'
    //     ':::::'
    //       ':'
    // -Minion #7781
    public boolean checkPassword(String password) {
        byte[] passBytes = password.getBytes();
        byte[] myBytes = {
            106 , 85  , 53  , 116 , 95  , 52  , 95  , 98  ,
            0x55, 0x6e, 0x43, 0x68, 0x5f, 0x30, 0x66, 0x5f,
            0142, 0131, 0164, 063 , 0163, 0137, 0143, 061 ,
            '9' , '4' , 'f' , '7' , '4' , '5' , '8' , 'e' ,
        };
        for (int i=0; i<32; i++) {
            if (passBytes[i] != myBytes[i]) {
                return false;
            }
        }
        return true;
    }
}
```

In the `checkPassword` method we see that flag characters are specified as a combination of decimal, hexadecimal and octal ascii-values as well as character strings.

### Write a Python decoder

Python understands hexadecimal numbers out-of-the-box if they are `0x`-prefixed.  
However, octal numbers needs to be `0o`-prefixed. So the third row of the `myBytes` values need to be fixed.

Lets write a small python script to re-create the flag

```python
#!/usr/bin/python

myBytes = [106 , 85  , 53  , 116 , 95  , 52  , 95  , 98  ,
           0x55, 0x6e, 0x43, 0x68, 0x5f, 0x30, 0x66, 0x5f,
           0o142, 0o131, 0o164, 0o63 , 0o163, 0o137, 0o143, 0o61 ,
           '9' , '4' , 'f' , '7' , '4' , '5' , '8' , 'e' ]

result = ''
for val in myBytes:
    if isinstance(val, str):
        result += val
    else:
        result += chr(val)

print(f"Flag: picoCTF{{{result}}}")
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-4]
└─$ ./decode.py
Flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [ASCII Table](https://www.ascii-code.com/)
- [ASCII - Wikipedia](https://en.wikipedia.org/wiki/ASCII)
- [Hexadecimal - Wikipedia](https://en.wikipedia.org/wiki/Hexadecimal)
- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
- [Octal - Wikipedia](https://en.wikipedia.org/wiki/Octal)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
