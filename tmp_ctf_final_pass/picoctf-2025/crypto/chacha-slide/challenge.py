import secrets
import hashlib
from Crypto.Cipher import ChaCha20_Poly1305

flag = open("flag.txt").read().strip()

def shasum(x):
    return hashlib.sha256(x).digest()

key = shasum(shasum(secrets.token_bytes(32) + flag.encode()))

# Generate a random nonce to be extra safe
nonce = secrets.token_bytes(12)

messages = [
    "Did you know that ChaCha20-Poly1305 is an authenticated encryption algorithm?",
    "That means it protects both the confidentiality and integrity of data!"
]

goal = "But it's only secure if used correctly!"

def encrypt(message):
    cipher = ChaCha20_Poly1305.new(key=key, nonce=nonce)
    ciphertext, tag = cipher.encrypt_and_digest(message)
    return ciphertext + tag + nonce

def decrypt(message_enc):
    ciphertext = message_enc[:-28]
    tag = message_enc[-28:-12]
    nonce = message_enc[-12:]
    cipher = ChaCha20_Poly1305.new(key=key, nonce=nonce)
    plaintext = cipher.decrypt_and_verify(ciphertext, tag)
    return plaintext

for message in messages:
    print("Plaintext: " + repr(message))
    message = message.encode()
    print("Plaintext (hex): " + message.hex())
    ciphertext = encrypt(message)
    print("Ciphertext (hex): " + ciphertext.hex())
    print()
    print()

user = bytes.fromhex(input("What is your message? "))
user_message = decrypt(user)
print("User message (decrypted): " + repr(user_message))

if goal in repr(user_message):
    print(flag)

