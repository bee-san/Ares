# vault-door-training

- [Challenge information](#challenge-information)
- [Solutions](#solutions)
- [References](#references)

## Challenge information

```text
Level: Easy
Tags: picoCTF 2019, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MARK E. HAASE

Description:
Your mission is to enter Dr. Evil's laboratory and retrieve the blueprints for his Doomsday Project. 
The laboratory is protected by a series of locked vault doors. Each door is controlled by a computer 
and requires a password to open. Unfortunately, our undercover agents have not been able to obtain 
the secret passwords for the vault doors, but one of our junior agents obtained the source code for 
each vault's computer! You will need to read the source code for each level to figure out what the 
password is for that vault door. As a warmup, we have created a replica vault in our training facility. 

The source code for the training vault is here: VaultDoorTraining.java

Hints:
1. The password is revealed in the program's source code.
```

Challenge link: [https://play.picoctf.org/practice/challenge/7](https://play.picoctf.org/practice/challenge/7)

## Solutions

The source code looks like this

```java
import java.util.*;

class VaultDoorTraining {
    public static void main(String args[]) {
        VaultDoorTraining vaultDoor = new VaultDoorTraining();
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

    // The password is below. Is it safe to put the password in the source code?
    // What if somebody stole our source code? Then they would know what our
    // password is. Hmm... I will think of some ways to improve the security
    // on the other doors.
    //
    // -Minion #9567
    public boolean checkPassword(String password) {
        return password.equals("w4rm1ng_Up_w1tH_jAv4_eec0716b713");
    }
}
```

We see that a [substring](https://www.javatpoint.com/java-string-substring) of the `userInput` is extracted and later compared with [equals](https://www.javatpoint.com/java-string-equals) in the `checkPassword` method where most of the flag is visible in plain text.

For additional information, please see the references below.

## References

- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
- [Java String substring()](https://www.javatpoint.com/java-string-substring)
- [Java String equals()](https://www.javatpoint.com/java-string-equals)
