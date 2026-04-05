# Secure Password Database - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 200

## Challenge Description

I made a new password authentication program that even shows you the password you entered saved in the database! Isn't that cool?

## Approach

This is a **reverse engineering** challenge where a binary program implements a password authentication system. The key hint in the description is that it "shows you the password you entered saved in the database" -- this suggests the program transforms your input in some way and displays the result, which we can use to reverse-engineer the expected password or directly extract the flag.

### Vulnerability Analysis

The description implies the program:
1. Takes a password as input
2. Performs some transformation/encoding on it
3. Stores/displays the "database" representation
4. Compares against a hardcoded expected value

Several common patterns for this type of challenge:

1. **Direct string comparison**: The expected password or flag is hardcoded in the binary and compared against input using `strcmp()` or a byte-by-byte comparison. The flag can be extracted with `strings` or by examining the binary.

2. **Character-by-character transformation**: The program applies a transformation (XOR, shift, substitution) to each input character and compares against a stored array. By reversing the transformation on the stored values, we recover the password/flag.

3. **Format string / buffer display vulnerability**: The program "shows you the password saved in the database" -- it may actually be displaying memory contents that include the real password or flag if we provide the right input.

4. **Obfuscated comparison**: The password check is split across multiple functions or uses anti-debugging tricks, but the core comparison values are still in the binary.

### Analysis Tools

- **strings**: Quick check for readable strings including the flag format
- **Ghidra/IDA**: Full disassembly and decompilation for understanding the logic
- **ltrace/strace**: Trace library calls to see `strcmp()`, `memcmp()`, etc. with arguments
- **gdb**: Dynamic debugging to inspect memory and registers during comparison
- **objdump**: Quick disassembly of key functions

### Key Observations

The phrase "shows you the password you entered saved in the database" is the major clue. This likely means:
- The program echoes back a transformed version of your input
- By observing how input maps to output, we can deduce the transformation
- We can then reverse the transformation on the stored "correct" database entry to recover the flag
- Alternatively, the program might have a bug where it leaks the actual stored password

## Solution

1. **Initial reconnaissance**:
   ```bash
   file binary          # Check architecture and type
   strings binary | grep -i pico   # Quick flag search
   strings binary | grep -i flag   # Look for flag references
   checksec binary      # Check security mitigations
   ```

2. **Dynamic analysis with ltrace**:
   ```bash
   ltrace ./binary      # Trace library calls -- look for strcmp/memcmp
   ```
   If the program uses `strcmp(user_input, "picoCTF{...}")`, ltrace will show both arguments.

3. **Static analysis**: Open in Ghidra and look for:
   - The `main()` function and its control flow
   - String references to "password", "database", "correct", "wrong"
   - Comparison functions and their arguments
   - Hardcoded byte arrays that might be the encoded flag

4. **Reverse the transformation**: If the binary XORs, shifts, or otherwise encodes the password before comparison:
   - Identify the transformation function
   - Extract the target (encoded) values from the binary
   - Apply the inverse transformation to recover the plaintext

5. **Input-output analysis**: Since the program shows the "database" version:
   - Input known strings (e.g., "AAAA", "BBBB") and observe the output
   - Determine the encoding: is it XOR with a key? Caesar shift? Custom substitution?
   - Once understood, reverse-engineer the expected output back to the flag

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
