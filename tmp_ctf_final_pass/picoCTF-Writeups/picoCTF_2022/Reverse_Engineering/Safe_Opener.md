# Safe Opener

- [Challenge information](#challenge-information)
- [Solution](#solution)
- [References](#references)

## Challenge information

```text
Level: Medium
Tags: picoCTF 2022, Reverse Engineering
Meta Tags: Walkthrough, Walk-through, Write-up, Writeup
Author: MUBARAK MIKAIL

Description:
Can you open this safe?

I forgot the key to my safe but this program is supposed to help me with retrieving the lost key. 
Can you help me unlock my safe?

Put the password you recover into the picoCTF flag format like:
picoCTF{password}

Hints:
(None)
```

Challenge link: [https://play.picoctf.org/practice/challenge/294](https://play.picoctf.org/practice/challenge/294)

## Solution

Let's start by looking at the Java source code

```java
import java.io.*;
import java.util.*;  
public class SafeOpener {
    public static void main(String args[]) throws IOException {
        BufferedReader keyboard = new BufferedReader(new InputStreamReader(System.in));
        Base64.Encoder encoder = Base64.getEncoder();
        String encodedkey = "";
        String key = "";
        int i = 0;
        boolean isOpen;
        

        while (i < 3) {
            System.out.print("Enter password for the safe: ");
            key = keyboard.readLine();

            encodedkey = encoder.encodeToString(key.getBytes());
            System.out.println(encodedkey);
              
            isOpen = openSafe(encodedkey);
            if (!isOpen) {
                System.out.println("You have  " + (2 - i) + " attempt(s) left");
                i++;
                continue;
            }
            break;
        }
    }
    
    public static boolean openSafe(String password) {
        String encodedkey = "cGwzYXMzX2wzdF9tM18xbnQwX3RoM19zYWYz";
        
        if (password.equals(encodedkey)) {
            System.out.println("Sesame open");
            return true;
        }
        else {
            System.out.println("Password is incorrect\n");
            return false;
        }
    }
}
```

In the `main` function we see a Base64.Encoder initialized and in the `OpenSafe` function we see  
an encodedkey that looks like a [base64](https://en.wikipedia.org/wiki/Base64) encoded password: `cGwzYXMzX2wzdF9tM18xbnQwX3RoM19zYWYz`.

I used [CyberChef's 'From Base64' recipe](https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true,false)) to decode the password.

Finally, follow the instructions in the challenge description to create the flag.

For additional information, please see the references below.

## References

- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [CyberChef - Homepage](https://gchq.github.io/CyberChef/)
- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
