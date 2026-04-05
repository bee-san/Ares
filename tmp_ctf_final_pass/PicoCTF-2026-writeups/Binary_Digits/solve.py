#!/usr/bin/env python3
"""
Binary Digits - picoCTF 2026 (Forensics, 100 pts)

A file containing 1s and 0s encodes hidden data. This script tries
multiple decoding strategies:
  1. Binary string -> ASCII text (8-bit groups)
  2. Binary string -> image (QR code / bitmap)
  3. Binary string -> raw file bytes (check for file signatures)

Usage:
    python3 solve.py                     # auto-detect input file
    python3 solve.py binary_digits.txt   # specify input file
    python3 solve.py flag.txt

Dependencies (for image approach): Pillow (pip install Pillow)
Optional: pyzbar for QR decoding (pip install pyzbar)
"""

import sys
import os
import math
import re


def load_binary_string(filepath):
    """Load and clean the binary string from the file."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Remove all whitespace and non-binary characters
    binary_str = re.sub(r'[^01]', '', content)
    print(f"[*] Loaded {len(binary_str)} binary digits from {filepath}")
    return binary_str


def try_ascii_decode(binary_str):
    """
    Approach 1: Interpret the binary string as 8-bit ASCII characters.
    """
    print("\n[*] === Approach 1: Binary -> ASCII ===")

    if len(binary_str) % 8 != 0:
        print(f"[!] Length {len(binary_str)} is not divisible by 8, trimming...")
        binary_str = binary_str[:len(binary_str) - (len(binary_str) % 8)]

    chars = []
    for i in range(0, len(binary_str), 8):
        byte = binary_str[i:i+8]
        char_val = int(byte, 2)
        chars.append(chr(char_val))

    result = ''.join(chars)

    # Check if result looks like printable text
    printable_ratio = sum(1 for c in result if c.isprintable() or c in '\n\r\t') / len(result)
    print(f"[*] Decoded {len(result)} characters (printable ratio: {printable_ratio:.1%})")

    if printable_ratio > 0.8:
        print(f"[+] Decoded text:\n{result}")
        if 'picoCTF' in result:
            # Extract flag
            match = re.search(r'picoCTF\{[^}]+\}', result)
            if match:
                print(f"\n[+] FLAG: {match.group()}")
                return match.group()
        return result
    else:
        print("[*] Result doesn't look like readable ASCII text")
        # Print hex for debugging
        raw_bytes = bytes(int(binary_str[i:i+8], 2) for i in range(0, len(binary_str), 8))
        print(f"[*] First 32 bytes (hex): {raw_bytes[:32].hex()}")

        # Check for known file signatures
        return try_file_signature(raw_bytes)


def try_file_signature(raw_bytes):
    """Check if the bytes form a known file type."""
    print("\n[*] === Checking file signatures ===")
    signatures = {
        b'\x89PNG': 'PNG image',
        b'PK': 'ZIP archive',
        b'\xff\xd8\xff': 'JPEG image',
        b'GIF8': 'GIF image',
        b'%PDF': 'PDF document',
        b'\x7fELF': 'ELF binary',
        b'BM': 'BMP image',
    }

    for sig, filetype in signatures.items():
        if raw_bytes[:len(sig)] == sig:
            ext = filetype.split()[-1].lower()
            if ext == 'image':
                ext = filetype.split()[0].lower()
            outfile = f"output.{ext}"
            with open(outfile, 'wb') as f:
                f.write(raw_bytes)
            print(f"[+] Detected {filetype}! Saved to {outfile}")
            return f"File saved as {outfile}"

    print("[*] No known file signature detected")
    return None


def try_image_decode(binary_str):
    """
    Approach 2: Render binary digits as a black-and-white image.
    This often produces a QR code encoding the flag.
    """
    print("\n[*] === Approach 2: Binary -> Image ===")

    length = len(binary_str)
    print(f"[*] Binary string length: {length}")

    # Find possible image dimensions
    sqrt_len = int(math.isqrt(length))
    possible_dims = []

    # Check perfect square (QR code)
    if sqrt_len * sqrt_len == length:
        possible_dims.append((sqrt_len, sqrt_len))
        print(f"[+] Perfect square! Dimension: {sqrt_len}x{sqrt_len}")

    # Find factor pairs close to square
    for w in range(max(1, sqrt_len - 50), sqrt_len + 50):
        if w > 0 and length % w == 0:
            h = length // w
            if h > 0 and abs(w - h) < max(w, h) * 0.5:  # roughly square-ish
                possible_dims.append((w, h))

    if not possible_dims:
        # Try common widths
        for w in [8, 16, 32, 64, 128, 256, 512]:
            if length % w == 0:
                h = length // w
                possible_dims.append((w, h))

    if not possible_dims:
        print("[!] Could not determine image dimensions")
        return None

    try:
        from PIL import Image
    except ImportError:
        print("[!] Pillow not installed. Install with: pip install Pillow")
        print("[*] Attempting manual PBM output instead...")
        # Create PBM (Portable Bitmap) which doesn't need Pillow
        for width, height in possible_dims[:3]:
            filename = f"output_{width}x{height}.pbm"
            with open(filename, 'w') as f:
                f.write(f"P1\n{width} {height}\n")
                for row in range(height):
                    row_data = binary_str[row * width:(row + 1) * width]
                    f.write(' '.join(row_data) + '\n')
            print(f"[+] Saved PBM image: {filename}")
        return None

    # Generate images for each possible dimension
    for width, height in possible_dims[:5]:
        print(f"[*] Trying {width}x{height}...")
        img = Image.new('1', (width, height))
        pixels = img.load()

        for idx, bit in enumerate(binary_str[:width * height]):
            x = idx % width
            y = idx // width
            # 1 = white, 0 = black (or invert if needed)
            pixels[x, y] = int(bit)

        filename = f"output_{width}x{height}.png"
        img.save(filename)
        print(f"[+] Saved image: {filename}")

        # Also save inverted version
        img_inv = Image.new('1', (width, height))
        pixels_inv = img_inv.load()
        for idx, bit in enumerate(binary_str[:width * height]):
            x = idx % width
            y = idx // width
            pixels_inv[x, y] = 1 - int(bit)

        filename_inv = f"output_{width}x{height}_inverted.png"
        img_inv.save(filename_inv)
        print(f"[+] Saved inverted image: {filename_inv}")

        # Try to decode QR code
        try:
            from pyzbar.pyzbar import decode as qr_decode
            # Scale up for better QR detection
            img_scaled = img.resize((width * 10, height * 10), Image.NEAREST)
            img_inv_scaled = img_inv.resize((width * 10, height * 10), Image.NEAREST)

            for test_img, label in [(img_scaled, "normal"), (img_inv_scaled, "inverted")]:
                results = qr_decode(test_img)
                if results:
                    for r in results:
                        decoded_data = r.data.decode('utf-8')
                        print(f"\n[+] QR Code decoded ({label}): {decoded_data}")
                        if 'picoCTF' in decoded_data:
                            print(f"[+] FLAG: {decoded_data}")
                            return decoded_data
        except ImportError:
            print("[*] pyzbar not installed -- open the image and scan the QR code manually")
            print("[*] Install with: pip install pyzbar")

    return None


def try_7bit_ascii(binary_str):
    """Try 7-bit ASCII decoding (some challenges use 7-bit encoding)."""
    print("\n[*] === Approach 3: 7-bit ASCII ===")
    if len(binary_str) % 7 != 0:
        return None

    chars = []
    for i in range(0, len(binary_str), 7):
        byte = binary_str[i:i+7]
        char_val = int(byte, 2)
        if 32 <= char_val <= 126 or char_val in (10, 13, 9):
            chars.append(chr(char_val))
        else:
            chars.append('?')

    result = ''.join(chars)
    printable_ratio = sum(1 for c in result if c.isprintable()) / max(len(result), 1)

    if printable_ratio > 0.8:
        print(f"[+] 7-bit ASCII decoded: {result}")
        if 'picoCTF' in result:
            match = re.search(r'picoCTF\{[^}]+\}', result)
            if match:
                print(f"\n[+] FLAG: {match.group()}")
                return match.group()
    return None


def main():
    # Find the input file
    if len(sys.argv) > 1:
        filepath = sys.argv[1]
    else:
        # Auto-detect: look for likely input files
        candidates = []
        for fname in os.listdir('.'):
            if fname.endswith(('.txt', '.bin', '.dat')) and fname != 'solve.py':
                candidates.append(fname)
            elif fname in ('flag', 'binary_digits', 'data', 'challenge'):
                candidates.append(fname)

        if not candidates:
            print("[!] No input file found. Usage: python3 solve.py <input_file>")
            print("[*] Expected a file containing binary digits (1s and 0s)")
            sys.exit(1)

        filepath = candidates[0]
        print(f"[*] Auto-detected input file: {filepath}")

    if not os.path.exists(filepath):
        print(f"[!] File not found: {filepath}")
        sys.exit(1)

    # Load the binary string
    binary_str = load_binary_string(filepath)

    if not binary_str:
        print("[!] No binary digits found in the file")
        sys.exit(1)

    flag = None

    # Try approach 1: binary to ASCII
    flag = try_ascii_decode(binary_str)
    if flag and 'picoCTF' in str(flag):
        return

    # Try approach 2: binary to image
    result = try_image_decode(binary_str)
    if result and 'picoCTF' in str(result):
        return

    # Try approach 3: 7-bit ASCII
    result = try_7bit_ascii(binary_str)
    if result and 'picoCTF' in str(result):
        return

    print("\n[*] === Summary ===")
    print("[*] If no flag was found automatically, check:")
    print("    1. The generated image files -- open them and look for a QR code or text")
    print("    2. Try scanning generated images with a QR reader")
    print("    3. Check if the encoding uses a different scheme (base64, hex, etc.)")


if __name__ == '__main__':
    main()
