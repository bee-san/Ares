# cryptomaze - picoCTF 2026

**Category:** Cryptography
**Points:** 100

## Challenge Description

In this challenge, you are tasked with recovering a hidden flag that has been encrypted using a combination of Linear Feedback Shift Register and other techniques.

## Approach

This challenge combines a **Linear Feedback Shift Register (LFSR)** with additional encryption techniques to hide the flag. LFSRs are deterministic pseudo-random number generators that produce a sequence of bits based on an initial state (seed) and a feedback polynomial. The security of an LFSR-based cipher depends on keeping the seed and polynomial secret, but both can be recovered if enough output is known.

### What is an LFSR?

An LFSR is a shift register whose input bit is a linear function (typically XOR) of its previous state. Given:

- **State**: An n-bit register `[s_0, s_1, ..., s_{n-1}]`
- **Feedback polynomial**: Defines which bit positions are XORed to produce the new input bit
- **Output**: The bit shifted out at each step

The LFSR generates a pseudo-random bitstream that is XORed with the plaintext (the flag) to produce the ciphertext.

### Attack Strategy

For a 100-point challenge, the LFSR is likely breakable with one or more of these approaches:

1. **Known plaintext attack**: Since we know the flag starts with `picoCTF{`, we have at least 8 bytes (64 bits) of known plaintext. XORing the known plaintext with the ciphertext gives us the first 64 bits of the LFSR keystream. If the LFSR state is small enough (e.g., 16 or 32 bits), this is more than enough to recover the full state.

2. **Berlekamp-Massey algorithm**: Given a sequence of LFSR output bits, the Berlekamp-Massey algorithm can recover the minimal feedback polynomial and initial state. We need at least 2n consecutive output bits to recover an n-bit LFSR.

3. **Brute-force the seed**: If the LFSR state is small (e.g., 16 bits), we can brute-force all 2^16 = 65536 possible initial states and check which one produces a decryption starting with `picoCTF{`.

4. **Provided polynomial**: The challenge may provide the feedback polynomial (or it may be embedded in the source code), leaving only the initial state to recover.

### Additional Techniques

The description mentions "other techniques" beyond LFSR. This could include:
- A simple substitution or transposition cipher applied before/after LFSR encryption
- Base64 encoding of the ciphertext
- A second XOR layer with a static key
- Byte-level permutation or shuffling

## Solution

### Step 1: Extract the ciphertext and parameters

Download the challenge files. Extract:
- The ciphertext (encrypted flag)
- The LFSR feedback polynomial (tap positions)
- The LFSR state size (number of bits)
- Any additional encryption parameters

### Step 2: Recover the LFSR keystream

XOR the known plaintext prefix `picoCTF{` with the corresponding ciphertext bytes to recover the first bits of the LFSR output stream:

```python
known = b'picoCTF{'
keystream_bits = xor(ciphertext[:8], known)
```

### Step 3: Recover the LFSR state

Using the recovered keystream bits:

- If the polynomial is known, solve a system of linear equations (GF(2)) to find the initial state.
- If the polynomial is unknown, apply the Berlekamp-Massey algorithm.
- If the state is small, brute-force all possible initial states.

### Step 4: Decrypt the flag

Generate the full LFSR keystream from the recovered initial state and XOR it with the entire ciphertext to recover the flag.

### Step 5: Undo any additional transformations

If additional encryption layers were applied (base64, substitution, permutation), reverse them to obtain the final flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
