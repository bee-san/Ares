# Description

Can you get sense of this code file and write the <br>
function that will decode the given encrypted file <br>
content. <br>
Find the encrypted file here flag_info and code file <br>
might be good to analyze and get the flag.

# Solution

Here is a better formatted version of this writeup on [picoCTF Solutions website](https://picoctfsolutions.com/picoctf-2024-custom-encryption).

To get the files: `wget https://artifacts.picoctf.net/c_titan/18/enc_flag https://artifacts.picoctf.net/c_titan/18/custom_encryption.py`

Here was the script to get the flag:

```
from random import randint
import sys


def generator(g, x, p):
    return pow(g, x) % p


def encrypt(plaintext, key):
    cipher = []
    for char in plaintext:
        cipher.append(((ord(char) * key*311)))
    return cipher


def is_prime(p):
    v = 0
    for i in range(2, p + 1):
        if p % i == 0:
            v = v + 1
    if v > 1:
        return False
    else:
        return True


def dynamic_xor_encrypt(plaintext, text_key):
    cipher_text = ""
    key_length = len(text_key)
    for i, char in enumerate(plaintext[::-1]):
        key_char = text_key[i % key_length]
        encrypted_char = chr(ord(char) ^ ord(key_char))
        cipher_text += encrypted_char
    return cipher_text

def dynamic_xor_decrypt(plaintext, text_key):
    cipher_text = ""
    key_length = len(text_key)

    for i, char in enumerate(plaintext[::-1]):
        key_char = text_key[i % key_length]
        encrypted_char = chr(ord(char) ^ ord(key_char))
        cipher_text += encrypted_char

    plaintext = cipher_text
    cipher_text = ""

    for i, char in enumerate(plaintext[::-1]):
        key_char = text_key[i % key_length]
        encrypted_char = chr(ord(char) ^ ord(key_char))
        cipher_text += encrypted_char

    plaintext = cipher_text
    cipher_text = ""

    for i, char in enumerate(plaintext[::-1]):
        key_char = text_key[i % key_length]
        encrypted_char = chr(ord(char) ^ ord(key_char))
        cipher_text += encrypted_char
    
    return cipher_text


def test(plain_text, text_key):
    p = 97
    g = 31
    if not is_prime(p) and not is_prime(g):
        print("Enter prime numbers")
        return
    a = randint(p-10, p)
    b = randint(g-10, g)
    print(f"a = {a}")
    print(f"b = {b}")
    u = generator(g, a, p)
    v = generator(g, b, p)
    key = generator(v, a, p)
    b_key = generator(u, b, p)
    shared_key = None
    if key == b_key:
        shared_key = key
    else:
        print("Invalid key")
        return
    semi_cipher = dynamic_xor_encrypt(plain_text, text_key)
    cipher = encrypt(semi_cipher, shared_key)
    print(f'cipher is: {cipher}')

def decrypt(cipher, key):
    plaintext = ""
    for encrypted_value in cipher:
        decrypted_value = encrypted_value // (key * 311)
        plaintext += chr(decrypted_value)
    return plaintext

def test2():
    p = 97
    g = 31
    a = 94
    b = 29

    u = generator(g, a, p)
    v = generator(g, b, p)
    key = generator(v, a, p)
    b_key = generator(u, b, p)

    shared_key = None
    if key == b_key:
        shared_key = key
    else:
        print("Invalid key")
        return

    cipher = [260307, 491691, 491691, 2487378, 2516301, 0, 1966764, 1879995, 1995687, 1214766, 0, 2400609, 607383, 144615, 1966764, 0, 636306, 2487378, 28923, 1793226, 694152, 780921, 173538, 173538, 491691, 173538, 751998, 1475073, 925536, 1417227, 751998, 202461, 347076, 491691]

    semi_cipher = decrypt(cipher, shared_key)

    flag = dynamic_xor_decrypt(semi_cipher, "trudeau")

    print(flag)


if __name__ == "__main__":
    # message = sys.argv[1]
    # test(message, "trudeau")
    test2()
```

The functions that were added were `dynamic_xor_decrypt`, `decrypt`, and `test2`. `

The `dynamic_xor_decrypt` function is the same as `dynamic_xor_encrypt` but runs three times.

The `decrypt` function was derived from the `encrypt` function and it is just the reversed process.

For the `test2` function there is a use of the previous 2 functions discussed and some values are needed. These are provided from `enc_flag` which gives a and b as well as the cipher while p and g were received from the test function that was used for encryption. The rest is very similar to the original test function other than for semi_cipher using the `decrypt` function and for the final flag the `dynamic_xor_decrypt` function.

Lastly, since the values were hard-coded into the `test2` function the changes to the main function were to statically call `test2()` to get the flag.

Flag: `picoCTF{custom_d2cr0pt6d_751a...}`
