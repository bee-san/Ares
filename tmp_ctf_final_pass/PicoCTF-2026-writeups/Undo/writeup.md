# Undo - picoCTF 2026

**Category:** General Skills
**Points:** 100

## Challenge Description

Can you reverse a series of Linux text transformations to recover the original flag?

We are given a file (or a command pipeline) that shows how a flag was transformed using a series of standard Linux text-processing utilities. Our task is to reverse each transformation in the correct order to recover the original `picoCTF{...}` flag.

## Approach

This is a classic "General Skills" challenge that tests familiarity with Linux command-line text-processing tools. The challenge provides a transformed string and either shows or implies the sequence of transformations that were applied. To recover the flag, we must apply the **inverse** of each transformation in **reverse order**.

### Common Linux text transformations and their inverses

| Forward Command | What It Does | Inverse Command |
|---|---|---|
| `base64` | Base64 encode | `base64 -d` |
| `rev` | Reverse each line character-by-character | `rev` (self-inverse) |
| `tac` | Reverse line order (last line first) | `tac` (self-inverse) |
| `tr 'a-z' 'A-Z'` | Lowercase to uppercase | `tr 'A-Z' 'a-z'` |
| `tr 'A-Za-z' 'N-ZA-Mn-za-m'` | ROT13 | `tr 'A-Za-z' 'N-ZA-Mn-za-m'` (self-inverse) |
| `xxd` | Hex dump | `xxd -r` |
| `xxd -p` | Plain hex dump | `xxd -r -p` |
| `gzip` / `zlib` | Compression | `gzip -d` / `zlib-flate -uncompress` |
| `sed 's/old/new/g'` | String substitution | `sed 's/new/old/g'` |
| `tr 'abc' 'xyz'` | Character transliteration | `tr 'xyz' 'abc'` |
| `cut -c N-` | Remove first N-1 characters | (cannot directly undo without knowing removed chars) |
| `sort` | Sort lines alphabetically | (cannot directly undo) |
| `fold -w N` | Wrap lines to N characters | `tr -d '\n'` (rejoin) |

### Strategy

1. **Read the challenge instructions carefully.** The transformations are usually given explicitly (e.g., as a shell pipeline or a script).
2. **Start from the final output** and work backwards, applying the inverse of each transformation.
3. **Verify at each step** that the intermediate result looks reasonable (e.g., valid base64, readable ASCII).
4. **The final result** should be a valid `picoCTF{...}` flag.

## Solution

### Step 1: Examine the provided file/instructions

The challenge typically provides:
- A transformed data file (e.g., `transformed.txt`)
- The sequence of commands used to transform it (e.g., in a script or description)

For example, if the challenge says the flag was transformed with:
```bash
cat flag.txt | base64 | rev | tr 'a-z' 'A-Z' > transformed.txt
```

### Step 2: Reverse the pipeline

Apply the inverse of each command in reverse order:
```bash
cat transformed.txt | tr 'A-Z' 'a-z' | rev | base64 -d
```

Breaking this down:
1. **Last transformation was `tr 'a-z' 'A-Z'`** (uppercase) -> Undo with `tr 'A-Z' 'a-z'` (lowercase)
2. **Second transformation was `rev`** (reverse) -> Undo with `rev` (self-inverse)
3. **First transformation was `base64`** (encode) -> Undo with `base64 -d` (decode)

### Step 3: Handle multi-step or nested transformations

If the challenge involves multiple rounds or nested encodings, keep peeling layers:
```bash
# Example: double base64 + rev + ROT13
cat data.txt | tr 'A-Za-z' 'N-ZA-Mn-za-m' | rev | base64 -d | base64 -d
```

### Step 4: Verify the flag

The output should match the pattern `picoCTF{...}`.

## Solution Script

```
python3 solve.py
```

The script reads the transformed data, applies common inverse transformations, and uses pattern detection to automatically determine and reverse the correct sequence.

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
