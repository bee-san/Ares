# bytemancy 1 - picoCTF 2026

**Category:** General Skills
**Points:** 100

## Challenge Description

Can you conjure the right bytes? The program's source code can be downloaded. (Second in the bytemancy series -- basic byte/encoding manipulation)

## Approach

This is the second challenge in the "bytemancy" series (following bytemancy 0 at 50 points). At 100 points with 2097 solves, it remains in the accessible range but introduces slightly more complex byte manipulation than the first level.

### Key Concepts

1. **Byte representation**: Data can be expressed in multiple forms -- decimal, hexadecimal, octal, binary, and ASCII. Converting fluently between these representations is the core skill tested.
2. **Encoding schemes**: Base64, hex encoding, URL encoding, and other standard transformations that convert binary data to text-safe formats.
3. **Byte-level operations**: XOR, bitwise AND/OR, bit shifting, nibble swapping, and modular arithmetic on individual bytes.
4. **Endianness**: The byte order in which multi-byte values are stored -- little-endian (least significant byte first) vs. big-endian (most significant byte first).

### What Differentiates bytemancy 1 from bytemancy 0

Building on the introductory bytemancy 0, this level likely introduces one or more of these additional complexities:

- **Multi-step transformations**: Instead of a single operation (e.g., just hex decoding), the input may need to pass through two or more transformations in sequence.
- **Mixed encoding formats**: The program may expect input in one format (e.g., decimal) but display targets in another (e.g., hex), requiring the solver to convert between them.
- **Simple cipher operations**: A single-byte XOR cipher, Caesar-style byte rotation, or byte substitution table.
- **Byte ordering constraints**: The correct answer may require understanding little-endian vs. big-endian packing.
- **Non-printable bytes**: Some required bytes may fall outside the printable ASCII range (0x20-0x7E), requiring raw byte input.

### Analysis Strategy

1. Download the provided source code
2. Read it carefully to understand:
   - What input format the program expects (hex string, raw bytes, space-separated decimals, etc.)
   - What transformations are applied to the input
   - What the target/expected output is
3. Reverse the transformations to compute the required input
4. Send the correct bytes to the program to receive the flag

## Solution

### Step 1: Download and examine the source code

```bash
# Download the source code from the challenge page
wget <challenge_url>/bytemancy1.py
cat bytemancy1.py
```

### Step 2: Identify the transformation logic

The source code typically contains a validation function that:
1. Reads user input
2. Applies one or more transformations
3. Compares the result to hardcoded expected values
4. Prints the flag if the comparison succeeds

Example pseudocode pattern:

```python
expected = [0xa3, 0xf1, 0x42, ...]  # target byte values
key = 0x55                            # XOR key

user_input = read_input()             # e.g., hex string from stdin
transformed = [b ^ key for b in user_input]

if transformed == expected:
    print(flag)
```

### Step 3: Reverse the transformation

For each type of operation, the inverse is:

| Forward Operation            | Inverse Operation               |
|------------------------------|---------------------------------|
| `b ^ key`                    | `b ^ key` (XOR is self-inverse) |
| `(b + offset) % 256`        | `(b - offset) % 256`           |
| `(b - offset) % 256`        | `(b + offset) % 256`           |
| `(b << n) \| (b >> (8-n))`  | `(b >> n) \| (b << (8-n))`     |
| Nibble swap `(b>>4)\|(b<<4)` | Same (self-inverse)            |
| `~b & 0xFF`                 | `~b & 0xFF` (self-inverse)     |

### Step 4: Construct and send the payload

```bash
# If the program runs locally:
python3 solve.py | python3 bytemancy1.py

# If the program runs as a remote service:
python3 solve.py | nc <host> <port>

# Or use the solve script directly:
python3 solve.py --source bytemancy1.py
python3 solve.py --host <HOST> --port <PORT>
```

### Step 5: Capture the flag

The program validates the input and prints the flag upon success.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
