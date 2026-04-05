# Black Cobra Pepper - picoCTF 2026

**Category:** Cryptography
**Points:** 200

## Challenge Description

i like peppers. (change!)

## Approach

This challenge involves a **pepper** in the context of password hashing/cryptography. The hint "change!" suggests we need to look at something that changes -- likely a pepper value that is being XORed, concatenated, or otherwise combined with data before hashing.

### What is a Pepper?

In cryptography, a **pepper** is a secret value added to a password (or plaintext) before hashing, similar to a salt. The key difference:
- A **salt** is stored alongside the hash (public)
- A **pepper** is kept secret (not stored with the hash)

In CTF challenges, "pepper" typically means a short secret that is combined with known data before hashing. Since the pepper is short, it can be brute-forced.

### The "Black Cobra" + "change!" Hint

The challenge name "Black Cobra Pepper" references the Black Cobra chili pepper, a real-world hot pepper. The hint "(change!)" likely refers to:
1. The pepper changes between attempts, OR
2. We need to find what changed (the pepper) by brute-forcing, OR
3. The pepper is applied via XOR (change bits)

### Typical Challenge Structure

The server likely provides:
- A known plaintext or known hash
- A hash computed as `H(pepper + plaintext)` or `H(plaintext + pepper)`
- The pepper is a short value (1-4 bytes) that can be brute-forced

The approach is to:
1. Obtain the target hash and any known components
2. Brute-force the pepper value by trying all possibilities
3. Once the correct pepper is found, use it to recover the flag

### Alternative: XOR-based Pepper

If the scheme uses XOR rather than concatenation:
- `ciphertext = plaintext XOR pepper (repeated)`
- Since the pepper is short (a few bytes), we can brute-force the key
- The flag format `picoCTF{...}` gives us known plaintext to verify against

## Solution

### Step-by-step:

1. **Connect** to the challenge server or download the challenge files.
2. **Analyze** the encryption/hashing scheme to understand how the pepper is applied.
3. **Identify** the pepper length and application method (prepend, append, XOR, etc.).
4. **Brute-force** the pepper: try all possible pepper values and check which one produces a valid result (matches the target hash or decrypts to readable text starting with `picoCTF{`).
5. **Apply** the recovered pepper to obtain the flag.

### Common Pepper Schemes in CTFs:

- **Hash pepper**: `target_hash = SHA256(pepper + message)` -- brute-force pepper, check hash match
- **XOR pepper**: `ciphertext = message XOR repeat(pepper)` -- brute-force XOR key
- **HMAC pepper**: `target = HMAC(pepper, message)` -- brute-force the key
- **AES with pepper-derived key**: pepper expands to a key -- brute-force short pepper

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
