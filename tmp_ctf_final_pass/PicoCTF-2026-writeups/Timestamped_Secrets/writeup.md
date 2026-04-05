# Timestamped Secrets - picoCTF 2026

**Category:** Cryptography
**Points:** 200

## Challenge Description

Someone encrypted a message using AES in ECB mode but they weren't very careful with their key. Turns out it's derived from a timestamp -- can you figure out when and crack it?

## Approach

This challenge exploits a classic cryptographic weakness: using a **predictable, low-entropy value** (a timestamp) as the basis for an encryption key. AES-128/256 in ECB mode is already problematic because it encrypts identical plaintext blocks to identical ciphertext blocks, but the primary vulnerability here is that the keyspace is trivially small when the key is derived from a timestamp.

### Key Concepts

1. **AES-ECB mode**: Each 16-byte block of plaintext is encrypted independently with the same key. Identical plaintext blocks produce identical ciphertext blocks. While this is a known weakness, it is not the main attack vector in this challenge.

2. **Timestamp-derived key**: Instead of using a truly random 128-bit or 256-bit key, the encryption key is derived from a Unix timestamp (seconds since epoch). This means:
   - A Unix timestamp is ~10 digits (currently ~1.7 billion)
   - Even across a full year, there are only ~31.5 million possible timestamps
   - If we know the approximate time window, the search space shrinks to thousands or hundreds of thousands of keys

3. **Key derivation**: The timestamp is likely converted to a key via one of these methods:
   - Direct use: `key = str(timestamp).encode().ljust(16, b'\x00')` (pad to 16 bytes)
   - Hashing: `key = hashlib.md5(str(timestamp).encode()).digest()` (MD5 produces 16 bytes)
   - SHA256 truncated: `key = hashlib.sha256(str(timestamp).encode()).digest()[:16]`
   - `random.seed(timestamp)` then generate key bytes

4. **Brute-force feasibility**: Even brute-forcing all timestamps from the Unix epoch (1970) to present day is computationally feasible -- that is roughly 1.7 billion AES decryptions, which modern hardware can perform in minutes. With a narrower time window (e.g., a single year or month), it takes seconds.

### Attack Strategy

1. Obtain the ciphertext (provided by the challenge)
2. Determine or guess the key derivation method (from source code, hints, or trial)
3. Establish a search range for the timestamp (use file metadata, challenge hints, or a wide range)
4. For each candidate timestamp:
   a. Derive the AES key
   b. Decrypt the ciphertext
   c. Check if the plaintext looks valid (e.g., contains "picoCTF{", is valid ASCII, has correct padding)
5. Report the flag

## Solution

### Step 1: Download challenge files

The challenge provides:
- An encrypted ciphertext file (e.g., `ciphertext.bin` or a hex-encoded ciphertext)
- Possibly the encryption script or a hint about the key derivation method
- Possibly metadata indicating the approximate encryption time

### Step 2: Identify the key derivation method

Examine any provided source code. Common patterns:

```python
import time, hashlib
from Crypto.Cipher import AES

timestamp = int(time.time())
key = hashlib.md5(str(timestamp).encode()).digest()  # 16 bytes for AES-128
cipher = AES.new(key, AES.MODE_ECB)
ciphertext = cipher.encrypt(pad(plaintext, 16))
```

### Step 3: Determine the search range

- If a source file has a modification time, use that as the center of the search window
- If the challenge mentions a date, convert it to Unix timestamp
- As a fallback, search from the competition start to the current time
- The challenge may also provide the ciphertext as a hex string with a timestamp hint

### Step 4: Brute-force the timestamp

```python
for ts in range(start_time, end_time):
    key = derive_key(ts)
    plaintext = decrypt(ciphertext, key)
    if is_valid(plaintext):
        print(f"Found at timestamp {ts}: {plaintext}")
        break
```

### Step 5: Validation heuristics

The decrypted plaintext is valid if:
- It contains `picoCTF{` or `flag{`
- It is valid UTF-8/ASCII
- It has correct PKCS#7 padding (last N bytes are all value N)
- It does not contain excessive non-printable characters

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
