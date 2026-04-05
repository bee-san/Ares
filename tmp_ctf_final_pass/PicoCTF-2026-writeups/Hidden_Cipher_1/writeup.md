# Hidden Cipher 1 - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 100

## Challenge Description

The flag is right in front of you; just slightly encrypted. All you have to do is figure out the cipher and the key. You can download the binary.

## Approach

This is an introductory reverse engineering challenge where a binary contains an **encrypted flag** and a simple cipher algorithm. By reverse engineering the encryption logic, we can write a decryption routine to recover the original flag.

### Binary Analysis

The binary likely:

1. **Contains the encrypted flag as hardcoded data** -- either embedded as a byte array, a string literal, or stored in a separate file (e.g., `rev_this` or `encrypted.txt`).
2. **Implements a simple cipher** to encrypt the flag. At 100 points, this will be a basic transformation like:
   - **XOR cipher** with a single-byte or multi-byte key
   - **Caesar/shift cipher** with a fixed or alternating shift
   - **Alternating arithmetic** (e.g., add N on even indices, subtract M on odd indices)
   - **A combination** of the above

### Reverse Engineering the Cipher

Using a disassembler (Ghidra, IDA, or even `objdump`), we decompile the main function to understand the encryption logic. The typical pattern seen in picoCTF challenges of this type is:

```c
for (int i = 0; i < len; i++) {
    if (i % 2 == 0) {
        encrypted[i] = flag[i] + KEY1;   // e.g., +5
    } else {
        encrypted[i] = flag[i] - KEY2;   // e.g., -2
    }
}
```

Or for XOR-based:

```c
for (int i = 0; i < len; i++) {
    encrypted[i] = flag[i] ^ key[i % key_len];
}
```

The hint "figure out the cipher and the key" suggests there is a specific key value or set of values used in the transformation.

### Decryption Strategy

Once we identify the cipher:

1. **For additive/subtractive ciphers**: Reverse the operation (subtract where it added, add where it subtracted).
2. **For XOR ciphers**: XOR is its own inverse -- apply the same XOR key to the ciphertext to get plaintext.
3. **For Caesar ciphers**: Shift in the opposite direction by the same amount.

### Known Patterns from picoCTF

A very common pattern in picoCTF reverse engineering challenges:
- Characters at **even indices** (0, 2, 4, ...) have a value **added** (e.g., +5)
- Characters at **odd indices** (1, 3, 5, ...) have a value **subtracted** (e.g., -2)
- The first few characters (e.g., indices 0-7 for "picoCTF{") may be left unmodified

To decrypt:
- Even indices: **subtract** the added value
- Odd indices: **add** the subtracted value

## Solution

### Step 1: Download and examine the binary

```bash
file chall        # Identify the binary type
strings chall     # Look for readable strings or the encrypted flag
```

If there is an accompanying data file (e.g., `rev_this`, `enc_flag`), examine it:
```bash
xxd rev_this      # View hex dump of the encrypted data
```

### Step 2: Decompile with Ghidra

Open the binary in Ghidra, navigate to `main()`, and identify:
- Where the flag data is loaded
- The encryption loop
- The arithmetic/XOR operations and their constants (the "key")
- Which indices are treated differently (even vs. odd, or first N characters skipped)

### Step 3: Write the decryption script

Based on the identified cipher, write a Python script that applies the inverse operations. The solve script below covers the most common cipher variants seen in picoCTF.

### Step 4: Run the decryption

```bash
python3 solve.py
```

If the binary comes with an encrypted data file:
```bash
python3 solve.py rev_this
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
