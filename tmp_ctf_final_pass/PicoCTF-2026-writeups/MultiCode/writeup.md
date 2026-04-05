# MultiCode - picoCTF 2026

**Category:** General Skills
**Points:** 200

## Challenge Description
We intercepted a suspiciously encoded message, but it's clearly hiding a flag. No encryption, just multiple layers of obfuscation/encoding.

## Approach

This is a classic multi-layer encoding challenge. The description explicitly states "no encryption, just multiple layers of obfuscation/encoding," which tells us the flag has been run through a pipeline of reversible encoding schemes -- one after another -- and we need to undo each layer in the correct order.

### Common Encoding Layers in CTF Challenges

The typical encoding schemes seen in picoCTF multi-encoding challenges include:

1. **Base64** -- Recognizable by its character set (`A-Za-z0-9+/`) and optional `=`/`==` padding at the end. Encoded data is roughly 4/3 the size of the original.

2. **Hexadecimal (Base16)** -- A string consisting only of `0-9a-fA-F` characters. Each pair of hex characters represents one byte.

3. **ROT13** -- A Caesar cipher with a rotation of 13. Only affects alphabetic characters; numbers and symbols remain unchanged.

4. **Binary (Base2)** -- A string of `0`s and `1`s, typically in groups of 8 (one byte per character).

5. **Octal** -- A string of numbers in groups of 3 (e.g., `160 151 143 157`), where each group is an octal (base-8) representation of an ASCII character.

6. **Morse Code** -- Dots (`.`) and dashes (`-`) separated by spaces or slashes.

7. **URL Encoding (Percent Encoding)** -- Characters represented as `%XX` where `XX` is the hex value.

8. **Decimal ASCII** -- A string of space-separated decimal numbers, each representing an ASCII code.

9. **Atbash Cipher** -- A simple substitution cipher where `A<->Z`, `B<->Y`, `C<->X`, etc.

10. **Base32** -- Uses characters `A-Z2-7` with `=` padding; common in CTF challenges.

### Strategy

The approach is iterative:
1. Look at the current encoded string and identify which encoding was applied last (outermost layer).
2. Decode that layer.
3. Look at the result and identify the next layer.
4. Repeat until the plaintext flag `picoCTF{...}` is revealed.

The solve script automates this by repeatedly attempting all known decodings and checking if the result looks like a valid next layer or the final flag.

## Solution

### Step 1: Examine the given encoded message

Download or copy the encoded message from the challenge. It will be a string that looks like one of the encoding formats listed above.

### Step 2: Identify and peel layers

For example, a typical multi-layer encoding might look like this:

```
Layer 5 (outermost): Base64
Layer 4: Hex
Layer 3: ROT13
Layer 2: Base64
Layer 1 (innermost): Binary
Original: picoCTF{...}
```

Working from the outside in:
1. Base64 decode the input
2. Hex decode the result
3. ROT13 the result
4. Base64 decode again
5. Convert binary to ASCII
6. Read the flag

### Step 3: Use CyberChef or the solve script

**CyberChef** (https://gchq.github.io/CyberChef/) is excellent for this -- you can chain operations together in its "Recipe" panel and experiment interactively.

Alternatively, run the automated solve script which tries all common decodings iteratively.

### Step 4: Read the flag

Once all layers are peeled, the plaintext `picoCTF{...}` flag will be revealed.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
