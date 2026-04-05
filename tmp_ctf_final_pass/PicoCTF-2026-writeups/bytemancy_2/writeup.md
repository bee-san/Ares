# bytemancy 2 - picoCTF 2026

**Category:** General Skills
**Points:** 200

## Challenge Description

Can you conjure the right bytes? The program's source code can be downloaded.

## Approach

This is the second challenge in the "bytemancy" series, which focuses on **byte manipulation** in Python. The challenge provides source code for a program that expects the user to supply specific byte sequences (via stdin or as command-line arguments) that satisfy a set of conditions. "Conjuring the right bytes" means crafting input that passes all validation checks.

### Source Code Analysis

The program likely performs a series of byte-level transformations and checks on user input. Common patterns in bytemancy-style challenges include:

1. **Byte value constraints**: Each byte of the input must satisfy arithmetic conditions (e.g., `input[i] ^ key[i] == target[i]`).
2. **Non-printable byte requirements**: Some expected bytes may fall outside the printable ASCII range (0x20-0x7E), requiring the use of raw byte input rather than regular text.
3. **Endianness and packing**: Values may need to be packed in little-endian or big-endian format using `struct.pack()`.
4. **Bitwise operations**: The program may use XOR, AND, OR, bit shifts, or rotations to transform the input before comparison.
5. **Multi-stage validation**: The input passes through multiple transformation stages, each of which must produce the correct intermediate result.

### Key Insight for "bytemancy 2"

Building on the first bytemancy challenge, this level adds more complexity to the byte transformations. The source code reveals the exact operations performed on the input, and we must invert them to determine the correct input bytes.

The typical structure is:

```python
# Pseudocode from source
def check(user_input):
    for i in range(len(expected)):
        transformed = some_operation(user_input[i], keys[i])
        if transformed != expected[i]:
            return False
    return True
```

We reverse-engineer `some_operation` to compute `user_input[i]` from `expected[i]` and `keys[i]`.

## Solution

### Step 1: Download and read the source code

Download the provided source file and examine the transformation logic. Identify:
- The expected output values
- The transformation function(s)
- Any keys, constants, or lookup tables used

### Step 2: Invert the transformation

For each byte position, compute the input byte that produces the expected output. For example:

- If the transform is `XOR`: `input[i] = expected[i] ^ key[i]`
- If the transform is `ADD`: `input[i] = (expected[i] - key[i]) % 256`
- If the transform is `ROT`: apply the reverse rotation

### Step 3: Construct the byte sequence

The required input may contain non-printable bytes, so we must pipe raw bytes to the program:

```bash
python3 solve.py | python3 bytemancy2.py
```

Or send them to the remote service:

```bash
python3 solve.py | nc <host> <port>
```

### Step 4: Capture the flag

The program validates our input and prints the flag if all checks pass.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
