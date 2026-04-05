# bytemancy 0 - picoCTF 2026

**Category:** General Skills
**Points:** 50

## Challenge Description

Can you conjure the right bytes? The program's source code can be downloaded. (First in the bytemancy series - basic byte manipulation)

## Approach

This is the introductory challenge in the "bytemancy" series, focused on basic byte manipulation. The challenge provides a program's source code that performs some transformation on input bytes, and we need to reverse-engineer the transformation to produce the expected output and retrieve the flag.

### Key Concepts

1. **Byte representation**: Understanding how data is represented at the byte level -- integers, characters, and hex values are all just bytes under the hood.
2. **Encoding/Decoding**: Converting between different representations: ASCII, hexadecimal, decimal, binary.
3. **Basic byte operations**: Simple operations like XOR, addition/subtraction modulo 256, bit shifting, and byte reversal.

### Typical bytemancy 0 Pattern

As the first challenge in the series (50 points, 3175 solves), this is likely a straightforward byte conversion task. The program probably:
- Reads input from the user (as hex, decimal, or raw bytes)
- Performs a simple check or transformation (e.g., compare to a target sequence)
- Prints the flag if the correct bytes are supplied

Common patterns for introductory byte challenges:
- **Hex string to bytes**: Convert a given hex string to raw bytes and send it to the program
- **ASCII to byte values**: Identify specific byte values that correspond to certain characters
- **Simple XOR**: XOR each byte with a single key byte
- **Byte order reversal**: Convert between little-endian and big-endian representations

### Analysis Strategy

1. Download and read the source code carefully
2. Identify what input the program expects
3. Determine the transformation/check being performed
4. Compute the required input to satisfy the check
5. Send the correct bytes to receive the flag

## Solution

### Step 1: Download and examine the source code
```bash
# Download the source code from the challenge
wget <challenge_url>/bytemancy0.py
# or
cat bytemancy0.py
```

### Step 2: Analyze the program logic
The program likely reads input and compares it against expected byte values. For example, it might:
- Ask for hex input and compare decoded bytes to a target
- Ask for specific byte values in decimal format
- Read raw stdin bytes and check them against a hardcoded sequence

### Step 3: Determine the required bytes
Read the source to find the target bytes. For a 50-point General Skills challenge, the transformation is typically minimal:
- Direct hex decoding: `bytes.fromhex("deadbeef")`
- Character-to-int conversion: `[ord(c) for c in "target"]`
- Simple arithmetic: each byte XORed with a constant or shifted by a fixed amount

### Step 4: Connect and send the solution
```bash
# If it's a remote service:
python3 solve.py

# If it's a local program:
echo -ne '\xDE\xAD\xBE\xEF' | python3 bytemancy0.py
```

### Step 5: Retrieve the flag
The program prints the flag upon receiving the correct byte sequence.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
