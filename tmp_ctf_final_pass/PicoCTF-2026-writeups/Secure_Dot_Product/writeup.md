# Secure Dot Product - picoCTF 2026

**Category:** Cryptography
**Points:** 300

## Challenge Description
Our intern thought it was a great idea to vibe code a secure dot product server using our AES key. Having taken a class in homomorphic encryption, they tried to make a server that computes a dot product over encrypted data.

## Approach

The server implements a flawed "homomorphic" dot product computation over AES-encrypted data. The key vulnerability is that AES-ECB (Electronic Codebook) mode is used, where each 16-byte block is encrypted independently with the same key. This means identical plaintext blocks always produce identical ciphertext blocks.

The server accepts a vector from the user, encrypts each element with AES-ECB, and computes a dot product with a secret vector (which contains the flag). The critical flaw is that because AES-ECB is deterministic and the server returns the encrypted dot product result, we can use a **chosen plaintext attack** to recover the secret vector byte by byte.

### Vulnerability Analysis

1. **AES-ECB determinism**: The same plaintext always encrypts to the same ciphertext. This means we can build a dictionary of plaintext-to-ciphertext mappings.

2. **Dot product oracle**: The server computes `sum(user_vector[i] * secret_vector[i])` and returns the encrypted result. By sending carefully crafted unit vectors (e.g., `[1, 0, 0, ..., 0]`), we can isolate individual elements of the secret vector.

3. **Unit vector attack**: If we send a vector with a `1` in position `i` and `0` everywhere else, the dot product result is exactly `secret_vector[i]`. The server encrypts this result and returns it. By iterating over all positions, we recover each element of the secret vector.

4. **Reconstructing the flag**: Each element of the secret vector corresponds to a byte (or character) of the flag. Once we recover all elements, we decode them to get the flag.

## Solution

1. Connect to the server.
2. Determine the length of the secret vector (from server prompts or by probing).
3. For each position `i` in the vector, send a unit vector with `1` at position `i` and `0` elsewhere.
4. The server returns `E(secret[i])` -- the encrypted value of the i-th secret element.
5. Build a lookup table: encrypt all possible byte values (0-255) by sending a vector `[v, 0, 0, ...]` for each value `v` and recording the ciphertext.
6. Match the ciphertexts from step 4 against the lookup table to recover each `secret[i]`.
7. Convert the recovered byte values to ASCII to get the flag.

Alternatively, if the server returns the plaintext dot product result (not encrypted), then the unit vector approach directly reveals each character.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
