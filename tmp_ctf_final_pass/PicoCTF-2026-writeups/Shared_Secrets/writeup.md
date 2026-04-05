# Shared Secrets - picoCTF 2026

**Category:** Cryptography
**Points:** 100

## Challenge Description

A message was encrypted using a shared secret... but it looks like one side of the exchange leaked something. Can you piece it together?

## Approach

This is a **Diffie-Hellman key exchange** challenge where one party's private key has been leaked. With access to the public parameters and the leaked private key, we can reconstruct the shared secret and decrypt the message.

### Diffie-Hellman Key Exchange Background

The Diffie-Hellman (DH) protocol allows two parties (Alice and Bob) to establish a shared secret over an insecure channel:

1. **Public parameters**: A large prime `p` and a generator `g` are agreed upon publicly.
2. **Private keys**: Alice chooses a secret `a`, Bob chooses a secret `b`.
3. **Public keys**:
   - Alice computes `A = g^a mod p` and sends `A` to Bob
   - Bob computes `B = g^b mod p` and sends `B` to Alice
4. **Shared secret**:
   - Alice computes `s = B^a mod p`
   - Bob computes `s = A^b mod p`
   - Both arrive at the same value: `s = g^(a*b) mod p`

### The Vulnerability

If either private key is leaked, the shared secret can be computed by anyone who also knows the corresponding public key:
- If Alice's private key `a` is leaked: `s = B^a mod p`
- If Bob's private key `b` is leaked: `s = A^b mod p`

### Encryption Scheme

Once the shared secret is derived, it is typically used to encrypt a message. Common methods in CTF challenges:
- **AES encryption**: The shared secret (or its hash) is used as an AES key (often SHA-256 of the shared secret, truncated to 16 or 32 bytes)
- **XOR encryption**: The message is XORed with the shared secret (or bytes derived from it)
- **Simple modular conversion**: The shared secret directly contains or encodes the flag

### What We Are Given (Typical)

The challenge likely provides a file or data containing:
- `p` -- the prime modulus
- `g` -- the generator
- `A` -- Alice's public key
- `B` -- Bob's public key
- One of `a` or `b` -- the leaked private key
- `ciphertext` -- the encrypted message (possibly with an `iv` for AES-CBC)

## Solution

1. **Extract the parameters** from the provided file(s):
   - Prime `p`, generator `g`
   - Public keys `A` and `B`
   - Leaked private key (`a` or `b`)
   - Ciphertext and optionally IV/nonce

2. **Compute the shared secret**:
   ```python
   # If 'a' (Alice's private key) is leaked:
   shared_secret = pow(B, a, p)

   # If 'b' (Bob's private key) is leaked:
   shared_secret = pow(A, b, p)
   ```

3. **Derive the decryption key**: The shared secret is usually processed before use:
   ```python
   import hashlib
   key = hashlib.sha256(str(shared_secret).encode()).digest()[:16]  # AES-128
   # or
   key = hashlib.sha256(long_to_bytes(shared_secret)).digest()       # AES-256
   # or
   key = shared_secret  # direct use for XOR
   ```

4. **Decrypt the message**:
   - For AES-CBC: `AES.new(key, AES.MODE_CBC, iv).decrypt(ciphertext)`
   - For AES-ECB: `AES.new(key, AES.MODE_ECB).decrypt(ciphertext)`
   - For XOR: `bytes(c ^ k for c, k in zip(ciphertext, cycle(key_bytes)))`

5. **Extract the flag** from the decrypted plaintext.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
