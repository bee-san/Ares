# Not TRUe - picoCTF 2026

**Category:** Cryptography
**Points:** 400

## Challenge Description
no. no. that's Not TRUe. that's impossible!

We are given a remote service (nc) that implements a custom authentication scheme. The server uses a truncated hash/MAC to verify messages. The name "Not TRUe" hints at a **truncation attack** -- where a cryptographic hash or MAC is truncated to fewer bits, making it feasible to brute-force a valid tag.

## Approach

The challenge name "Not TRUe" is a play on "TRU" / truncation. In proper cryptographic implementations, MACs and hashes use their full output length (e.g., 256 bits for SHA-256). However, when the output is **truncated** to a small number of bits (e.g., 16-32 bits), it becomes computationally trivial to forge a valid tag through brute force.

### Vulnerability Analysis

1. **Truncated MAC/Hash**: The server computes a MAC or hash over user-supplied data but only checks a truncated portion (e.g., the first 2-4 bytes). This drastically reduces the search space from 2^256 to 2^16 or 2^32.

2. **Attack Strategy**: We can forge a message with a valid truncated tag by:
   - Observing the server's behavior and understanding the truncation length
   - Brute-forcing the truncated portion by trying random inputs until one produces a matching truncated hash
   - Alternatively, if the server uses `HMAC-SHA256` truncated to N bits, we generate candidate messages and check if the first N bits of their hash match what the server expects

3. **Birthday-style collision**: With a 16-bit truncation, we only need ~2^8 = 256 attempts on average (birthday bound) or at most 2^16 = 65536 attempts for a guaranteed match against a fixed target.

### Server Interaction Pattern

The typical flow is:
1. Connect to the server
2. Server provides a challenge or expects a message + tag pair
3. We craft a message and brute-force the truncated tag
4. Submit the forged message to retrieve the flag

## Solution

1. Connect to the challenge server using netcat or pwntools
2. Parse the server's response to understand the expected format
3. Brute-force the truncated hash/MAC by iterating through possible values
4. Send the forged message + valid truncated tag
5. Receive the flag

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
