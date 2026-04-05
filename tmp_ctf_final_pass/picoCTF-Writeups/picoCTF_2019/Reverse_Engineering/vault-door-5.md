# vault-door-5

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
In the last challenge, you mastered octal (base 8), decimal (base 10), and hexadecimal (base 16) numbers, 
but this vault door uses a different change of base as well as URL encoding! 
 
The source code for this vault is here: VaultDoor5.java

Hints:
1. You may find an encoder/decoder tool helpful, such as https://encoding.tools/
2. Read the wikipedia articles on URL encoding and base 64 encoding to understand how they work and what the results look like.
```

Challenge link: [https://play.picoctf.org/practice/challenge/77](https://play.picoctf.org/practice/challenge/77)

## Solutions

The java source code looks like this

```java
import java.net.URLDecoder;
import java.util.*;

class VaultDoor5 {
    public static void main(String args[]) {
        VaultDoor5 vaultDoor = new VaultDoor5();
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

    // Minion #7781 used base 8 and base 16, but this is base 64, which is
    // like... eight times stronger, right? Riiigghtt? Well that's what my twin
    // brother Minion #2415 says, anyway.
    //
    // -Minion #2414
    public String base64Encode(byte[] input) {
        return Base64.getEncoder().encodeToString(input);
    }

    // URL encoding is meant for web pages, so any double agent spies who steal
    // our source code will think this is a web site or something, defintely not
    // vault door! Oh wait, should I have not said that in a source code
    // comment?
    //
    // -Minion #2415
    public String urlEncode(byte[] input) {
        StringBuffer buf = new StringBuffer();
        for (int i=0; i<input.length; i++) {
            buf.append(String.format("%%%2x", input[i]));
        }
        return buf.toString();
    }

    public boolean checkPassword(String password) {
        String urlEncoded = urlEncode(password.getBytes());
        String base64Encoded = base64Encode(urlEncoded.getBytes());
        String expected = "JTYzJTMwJTZlJTc2JTMzJTcyJTc0JTMxJTZlJTY3JTVm"
                        + "JTY2JTcyJTMwJTZkJTVmJTYyJTYxJTM1JTY1JTVmJTM2"
                        + "JTM0JTVmJTY0JTYyJTM2JTM5JTM0JTM2JTYyJTYx";
        return base64Encoded.equals(expected);
    }
}
```

In the `checkPassword` method we see that flag characters are both [URL-encoded](https://en.wikipedia.org/wiki/Percent-encoding) and [Base64](https://en.wikipedia.org/wiki/Base64) encoded.

### Write a Python decoder

Let's write a small python script to re-create the flag

```python
#!/usr/bin/python

from base64 import b64decode
from urllib.parse import unquote

expected = "JTYzJTMwJTZlJTc2JTMzJTcyJTc0JTMxJTZlJTY3JTVm" + "JTY2JTcyJTMwJTZkJTVmJTYyJTYxJTM1JTY1JTVmJTM2" + "JTM0JTVmJTY0JTYyJTM2JTM5JTM0JTM2JTYyJTYx"

result = unquote(b64decode(expected).decode())

print(f"Flag: picoCTF{{{result}}}")
```

### Get the flag

Then we run the script to get the flag

```bash
┌──(kali㉿kali)-[/mnt/…/picoCTF/picoCTF_2019/Reverse_Engineering/Vault-door-5]
└─$ ./decode.py
Flag: picoCTF{<REDACTED>}
```

For additional information, please see the references below.

## References

- [Base64 - Wikipedia](https://en.wikipedia.org/wiki/Base64)
- [Java (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Java_(programming_language))
- [Percent-encoding - Wikipedia](https://en.wikipedia.org/wiki/Percent-encoding)
- [Python (programming language) - Wikipedia](https://en.wikipedia.org/wiki/Python_(programming_language))
