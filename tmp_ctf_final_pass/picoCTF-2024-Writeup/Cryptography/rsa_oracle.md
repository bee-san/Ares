# Description
Can you abuse the oracle? <br>
An attacker was able to intercept communications <br>
between a bank and a fintech company. They managed <br>
to get the message (ciphertext) and the password that <br>
was used to encrypt the message. <br>
After some intensive reconassainance they found out <br>
that the bank has an oracle that was used to encrypt <br>
the password and can be found here nc <br>
titan.picoctf.net 62026. Decrypt the password and use <br>
it to decrypt the message. The oracle can decrypt <br>
anything except the password.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-rsa_oracle).

First download the files with this command: `wget https://artifacts.picoctf.net/c_titan/148/secret.enc https://artifacts.picoctf.net/c_titan/148/password.enc`

Followed this process: https://crypto.stackexchange.com/questions/2323/how-does-a-chosen-plaintext-attack-on-rsa-work/2331#2331

Script:
```
from pwn import *

context.log_level='critical'
p = remote("titan.picoctf.net", 61923)

p.recvuntil(b"decrypt.")

with open("password.enc") as file:
    c = int(file.read())

p.sendline(b"E")
p.recvuntil(b"keysize): ")
p.sendline(b"\x02")
p.recvuntil(b"mod n) ")

c_a = int(p.recvline())

p.sendline(b"D")
p.recvuntil(b"decrypt: ")
p.sendline(str(c_a*c).encode())
p.recvuntil(b"mod n): ")

password = int(p.recvline(), 16) // 2
password = password.to_bytes(len(str(password))-7, "big").decode("utf-8")

print("Password:", password)
```

This is a simple script that connects to the server with [pwn tools](https://docs.pwntools.com/en/stable/) to automate the process and easily send encoded text. The [context log level](https://docs.pwntools.com/en/stable/context.html#pwnlib.context.ContextType.log_level) was set to critical to remove unnecessary messages from pwntools when running. Note there is also a debug mode if needed.

First, taking in the `password.enc` text and storing it in c. Then encrypting 2 which is sent in hex and put into the c_a variable. The multiply c and c_a to where it is now in a format that the program will allow it to be decrypted. Once decrypted it takes the hex version which is why it is converted from hex with the `int(x, 16)` function. Then it uses integer division to divide by 2 to get the password.

Lastly, the password is converted to bytes and decoded to get the password that is needed to be used in the decryption.

By running this command and inputting the password after running the flag could be received: 

`openssl enc -aes-256-cbc -d -in secret.enc`

Flag: `picoCTF{su((3ss_(r@ck1ng_r3@_24bc...}`
