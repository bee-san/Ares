# bytemancy 3 - picoCTF 2026

**Category:** General Skills
**Points:** 400

## Challenge Description

Can you conjure the right bytes? The program's source code can be downloaded. (Final in the bytemancy series - likely involves complex byte manipulation/encoding)

## Approach

This is the final and most advanced challenge in the "bytemancy" series (400 points). Building on the techniques from bytemancy 0-2, this challenge involves complex, multi-layered byte manipulation that must be reversed to produce the correct input.

### Key Concepts

1. **Multi-stage byte transformations**: Unlike the simpler entries, bytemancy 3 likely chains multiple operations together (e.g., XOR then rotate then substitute).
2. **Substitution tables / S-boxes**: A lookup table that maps each byte value to a different byte value, requiring inversion.
3. **Byte permutation**: Rearranging the order of bytes according to a fixed permutation, requiring the inverse permutation to reverse.
4. **Bitwise rotations**: Circular bit shifts (rotate left/right) within each byte.
5. **Block-based operations**: Bytes may be processed in groups/blocks where operations depend on neighboring bytes.
6. **Key-dependent transformations**: Operations may use a key or seed derived from the challenge, combining XOR, addition, and other operations.

### Expected Complexity (400 pts, final in series)

The progression from bytemancy 0 (50 pts) to bytemancy 3 (400 pts) suggests:
- **bytemancy 0**: Single operation (e.g., hex decode or single XOR)
- **bytemancy 1**: Two operations chained (e.g., XOR + shift)
- **bytemancy 2**: Multiple operations with a key schedule or table
- **bytemancy 3**: Full multi-stage cipher with substitution, permutation, and key mixing -- essentially a simplified block cipher that must be fully reversed

### Reverse Engineering Strategy

1. **Map the full transformation pipeline**: Read the source code and document each step in order.
2. **Invert each step**: For each operation, determine the mathematical inverse:
   - XOR is its own inverse: `a ^ k ^ k = a`
   - Addition mod 256 inverses to subtraction: `(a + k) % 256` -> `(a - k) % 256`
   - Left rotation inverses to right rotation and vice versa
   - Substitution table inverses require building an inverse lookup table
   - Permutation inverses require computing the inverse permutation
3. **Apply inversions in reverse order**: If the forward pass is `f3(f2(f1(input)))`, the reverse is `f1_inv(f2_inv(f3_inv(target)))`.
4. **Handle rounds**: If the cipher uses multiple rounds, unroll all rounds in reverse.

## Solution

### Step 1: Download and thoroughly study the source code
```bash
wget <challenge_url>/bytemancy3.py
cat bytemancy3.py
```

### Step 2: Document the transformation pipeline
Map out every operation applied to the input bytes, noting:
- Order of operations
- Constants/keys used
- Whether operations are byte-wise or block-wise
- Number of rounds

### Step 3: Build the inverse transformation
For each step in the forward pipeline, implement its inverse:

```python
# Example: Forward pipeline
def encrypt(data, key):
    # Step 1: XOR with key
    data = bytes(d ^ k for d, k in zip(data, key))
    # Step 2: Substitute using S-box
    data = bytes(SBOX[b] for b in data)
    # Step 3: Permute byte positions
    data = bytes(data[PERM[i]] for i in range(len(data)))
    # Step 4: Rotate each byte left by 3
    data = bytes(((b << 3) | (b >> 5)) & 0xFF for b in data)
    return data

# Inverse pipeline (reversed order)
def decrypt(data, key):
    # Step 4 inv: Rotate each byte right by 3
    data = bytes(((b >> 3) | (b << 5)) & 0xFF for b in data)
    # Step 3 inv: Inverse permutation
    out = bytearray(len(data))
    for i in range(len(data)):
        out[PERM[i]] = data[i]
    data = bytes(out)
    # Step 2 inv: Inverse S-box
    data = bytes(INV_SBOX[b] for b in data)
    # Step 1 inv: XOR with key (self-inverse)
    data = bytes(d ^ k for d, k in zip(data, key))
    return data
```

### Step 4: Compute the required input
```python
target = <target_bytes_from_source>
solution = decrypt(target, key)
```

### Step 5: Send to the challenge and retrieve the flag
```bash
python3 solve.py --host <HOST> --port <PORT>
# or
python3 solve.py --source bytemancy3.py --local bytemancy3.py
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
