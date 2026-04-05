# Hidden Cipher 2 - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 100

## Challenge Description
The flag is right in front of you... kind of. You just need to solve a basic math problem to see it. But to get the real flag, you'll have to figure out the cipher.

## Approach

This is a beginner-friendly reverse engineering challenge (100 points) where the flag has been obfuscated using a **mathematical cipher** applied to each character. The description gives two major hints:

1. **"solve a basic math problem"** -- The cipher involves simple arithmetic operations (addition, subtraction, XOR, multiplication, modular arithmetic) applied to the characters of the flag.

2. **"figure out the cipher"** -- We need to reverse-engineer the transformation to recover the original flag characters.

### Typical Pattern for This Type of Challenge

The challenge likely provides either:
- **A compiled binary (ELF/PE)** that contains the encrypted flag and the cipher logic, or
- **Source code** (C, Python, or Java) that shows how the flag was encrypted.

The cipher is usually a character-by-character transformation such as:
- `encrypted[i] = flag[i] + key` (Caesar-style shift)
- `encrypted[i] = flag[i] ^ key` (XOR with a constant or rotating key)
- `encrypted[i] = flag[i] + i` (position-dependent shift)
- `encrypted[i] = flag[i] * a + b` (affine cipher)
- A combination: even-index characters get one operation, odd-index characters get another

Given the "Hidden Cipher **2**" name (implying a sequel to a simpler version), the cipher likely involves a position-dependent or alternating arithmetic operation -- slightly more complex than a flat shift but still "basic math."

### Analysis Steps

1. **Examine the binary/source**: Use a disassembler (Ghidra, IDA) or just read the source code to find where the encrypted flag data is stored and how the cipher operates.

2. **Identify the cipher**: Look for loops that iterate over the flag characters and apply arithmetic operations. Note:
   - What operation is applied (add, subtract, XOR, multiply)?
   - Is the key constant or does it change per position (e.g., using the index `i`)?
   - Are different operations applied to even vs. odd indices?

3. **Reverse the cipher**: Apply the inverse operation to each encrypted character:
   - If `enc[i] = flag[i] + key`, then `flag[i] = enc[i] - key`
   - If `enc[i] = flag[i] ^ key`, then `flag[i] = enc[i] ^ key` (XOR is its own inverse)
   - If `enc[i] = flag[i] + i`, then `flag[i] = enc[i] - i`

4. **Recover the flag**: Apply the inverse to the encrypted data to get `picoCTF{...}`.

## Solution

### Step 1: Extract the encrypted data and cipher logic

Open the binary in Ghidra or strings it to find the encrypted flag data. Look for:
- An array of bytes or characters (the encrypted flag)
- A loop with arithmetic operations

For example, if the source/decompilation looks like:

```c
char encrypted[] = { ... };  // encrypted flag bytes
for (int i = 0; i < len; i++) {
    if (i % 2 == 0) {
        encrypted[i] = flag[i] + 5;
    } else {
        encrypted[i] = flag[i] - 3;
    }
}
```

### Step 2: Reverse the cipher

The inverse of the above example would be:

```python
for i in range(len(encrypted)):
    if i % 2 == 0:
        flag[i] = encrypted[i] - 5
    else:
        flag[i] = encrypted[i] + 3
```

### Step 3: Run the solve script

The solve script provides a generic framework that:
1. Reads the encrypted data (from the binary or as input).
2. Tries common cipher reversals (shift, XOR, position-dependent, alternating).
3. Checks if the result starts with `picoCTF{` and ends with `}`.

If the challenge provides a binary, use `strings` or Ghidra to extract the encrypted bytes first, then feed them to the script.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
