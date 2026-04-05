def decrypt(ciphertext, shift):
    plaintext = ""
    for char in ciphertext:
        if char.isalpha():
            if char.isupper():
                plaintext += chr((ord(char) - shift - 65) % 26 + 65)
            else:
                plaintext += chr((ord(char) - shift - 97) % 26 + 97)
        else:
            plaintext += char
    return plaintext

ciphertext = "ynkooejcpdanqxeykjrbdofgkq"
shift = 3
for i in range(1,26):
    plaintext = decrypt(ciphertext, i)
    print(plaintext)