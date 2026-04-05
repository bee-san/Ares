# Binary Digits - picoCTF 2026

**Category:** Forensics
**Points:** 100

## Challenge Description

This file doesn't look like much... just a bunch of 1s and 0s. But maybe it's not just random noise. Can you recover anything meaningful?

## Approach

We are given a file containing a long string of binary digits (1s and 0s). The challenge is to determine what this binary data represents and decode it. There are several common encoding schemes used in forensics CTF challenges:

### Possible Interpretations

1. **Binary to ASCII text**: Each group of 8 bits (1 byte) represents an ASCII character. For example, `01110000` = `p`, `01101001` = `i`, etc. This is the simplest and most common approach.

2. **Binary to image (QR code or bitmap)**: The 1s and 0s represent black and white pixels in an image. If the total number of bits is a perfect square or has dimensions that suggest an image (e.g., factors that make a reasonable width/height), the binary string can be reshaped into a 2D grid to form an image -- often a QR code that encodes the flag.

3. **Binary to raw file**: The binary string, when converted to raw bytes, might produce a recognizable file format (PNG, ZIP, PDF, etc.) identifiable by its magic bytes/header.

### Analysis Steps

1. **Check the length**: Count the number of digits.
   - If divisible by 8, try ASCII conversion.
   - If it is a perfect square (e.g., 441 = 21x21, 625 = 25x25, 841 = 29x29), it is likely a QR code.
   - Check other factorizations for image dimensions.

2. **Try ASCII decode first** (simplest case for a 100-point challenge).

3. **If ASCII produces gibberish**, try rendering as an image -- swap 1s for black pixels and 0s for white (or vice versa).

4. **Check for file signatures** after byte conversion (e.g., `89 50 4E 47` for PNG, `50 4B` for ZIP).

## Solution

For a 100-point forensics challenge, the most likely solution is one of:
- **Direct binary-to-ASCII**: Split the binary string into 8-bit chunks and convert each to a character.
- **Binary-to-image (QR code)**: Render the bits as a black-and-white image and scan it.

The solve script tries both approaches automatically.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
