#!/usr/bin/env python3
"""
Hidden Cipher 2 - picoCTF 2026
Category: Reverse Engineering | Points: 100

The flag is obfuscated using a simple mathematical cipher.  This script
tries common reversals (constant shift, XOR, position-dependent shift,
alternating operations) to recover the picoCTF{...} flag.

Usage:
    # If the encrypted bytes are hardcoded (edit ENCRYPTED_DATA below):
    python3 solve.py

    # If you want to provide encrypted bytes from a file:
    python3 solve.py encrypted.bin

    # If you have a C/binary and extracted the encrypted array as hex:
    python3 solve.py --hex "70 6a 65 71 47 56 4a ..."

    # If extracted as comma-separated decimal:
    python3 solve.py --dec "112,106,101,113,67,86,74,..."
"""

import sys
import re
import os

# =========================================================================
# ENCRYPTED DATA -- paste the encrypted flag bytes here after extracting
# them from the challenge binary (using Ghidra, strings, or hexdump).
# Examples of format:
#   As a byte string:  b'\x70\x6a\x65\x71...'
#   As a list of ints:  [112, 106, 101, 113, ...]
#   As a hex string:    "706a6571..."
# =========================================================================
ENCRYPTED_DATA = None  # <-- Replace with actual encrypted data from the challenge

# If you already know the cipher parameters, set them here:
# Otherwise the script will brute-force common patterns.
KNOWN_CIPHER = None  # e.g., {"type": "alternating_add", "even_key": 5, "odd_key": -3}


# ---------------------------------------------------------------------------
# Cipher reversal functions
# ---------------------------------------------------------------------------

def reverse_constant_shift(data, shift):
    """Reverse: enc[i] = flag[i] + shift  =>  flag[i] = enc[i] - shift"""
    return bytes([(b - shift) % 256 for b in data])


def reverse_constant_xor(data, key):
    """Reverse: enc[i] = flag[i] ^ key  =>  flag[i] = enc[i] ^ key"""
    return bytes([b ^ key for b in data])


def reverse_position_add(data):
    """Reverse: enc[i] = flag[i] + i  =>  flag[i] = enc[i] - i"""
    return bytes([(b - i) % 256 for i, b in enumerate(data)])


def reverse_position_sub(data):
    """Reverse: enc[i] = flag[i] - i  =>  flag[i] = enc[i] + i"""
    return bytes([(b + i) % 256 for i, b in enumerate(data)])


def reverse_position_xor(data):
    """Reverse: enc[i] = flag[i] ^ i  =>  flag[i] = enc[i] ^ i"""
    return bytes([b ^ i for i, b in enumerate(data)])


def reverse_alternating_add(data, even_key, odd_key):
    """Reverse: enc[i] = flag[i] + even_key if i%2==0 else flag[i] + odd_key"""
    result = []
    for i, b in enumerate(data):
        if i % 2 == 0:
            result.append((b - even_key) % 256)
        else:
            result.append((b - odd_key) % 256)
    return bytes(result)


def reverse_alternating_add_sub(data, add_val, sub_val):
    """Reverse: even positions had +add_val, odd positions had -sub_val applied."""
    result = []
    for i, b in enumerate(data):
        if i % 2 == 0:
            result.append((b - add_val) % 256)
        else:
            result.append((b + sub_val) % 256)
    return bytes(result)


def reverse_key_add(data, key):
    """Reverse: enc[i] = flag[i] + key[i % len(key)]"""
    result = []
    for i, b in enumerate(data):
        result.append((b - key[i % len(key)]) % 256)
    return bytes(result)


def reverse_key_xor(data, key):
    """Reverse: enc[i] = flag[i] ^ key[i % len(key)]"""
    return bytes([b ^ key[i % len(key)] for i, b in enumerate(data)])


def is_valid_flag(text):
    """Check if text looks like a valid picoCTF flag."""
    return text.startswith('picoCTF{') and text.endswith('}') and all(
        32 <= ord(c) <= 126 for c in text
    )


def looks_printable(data):
    """Check if most bytes are printable ASCII."""
    try:
        text = data.decode('ascii')
        return all(32 <= ord(c) <= 126 for c in text)
    except Exception:
        return False


# ---------------------------------------------------------------------------
# Brute-force all common cipher patterns
# ---------------------------------------------------------------------------

def try_all_ciphers(data):
    """Try all common cipher reversals and return any valid flags found."""
    results = []

    # 1. Constant shift (Caesar): try shifts 1-255
    for shift in range(1, 256):
        candidate = reverse_constant_shift(data, shift)
        try:
            text = candidate.decode('ascii')
            if is_valid_flag(text):
                results.append((f"Constant shift +{shift}", text))
        except Exception:
            pass

    # 2. Constant XOR: try keys 1-255
    for key in range(1, 256):
        candidate = reverse_constant_xor(data, key)
        try:
            text = candidate.decode('ascii')
            if is_valid_flag(text):
                results.append((f"XOR key=0x{key:02x}", text))
        except Exception:
            pass

    # 3. Position-dependent: enc[i] = flag[i] + i
    candidate = reverse_position_add(data)
    try:
        text = candidate.decode('ascii')
        if is_valid_flag(text):
            results.append(("Position add (enc[i]=flag[i]+i)", text))
    except Exception:
        pass

    # 4. Position-dependent: enc[i] = flag[i] - i
    candidate = reverse_position_sub(data)
    try:
        text = candidate.decode('ascii')
        if is_valid_flag(text):
            results.append(("Position sub (enc[i]=flag[i]-i)", text))
    except Exception:
        pass

    # 5. Position-dependent XOR: enc[i] = flag[i] ^ i
    candidate = reverse_position_xor(data)
    try:
        text = candidate.decode('ascii')
        if is_valid_flag(text):
            results.append(("Position XOR (enc[i]=flag[i]^i)", text))
    except Exception:
        pass

    # 6. Alternating add/sub: common in picoCTF reverse_cipher style
    for add_val in range(1, 20):
        for sub_val in range(1, 20):
            candidate = reverse_alternating_add_sub(data, add_val, sub_val)
            try:
                text = candidate.decode('ascii')
                if is_valid_flag(text):
                    results.append(
                        (f"Alternating +{add_val}/-{sub_val}", text)
                    )
            except Exception:
                pass

    # 7. Alternating two different shifts
    for even_key in range(-20, 21):
        for odd_key in range(-20, 21):
            if even_key == 0 and odd_key == 0:
                continue
            candidate = reverse_alternating_add(data, even_key, odd_key)
            try:
                text = candidate.decode('ascii')
                if is_valid_flag(text):
                    results.append(
                        (f"Alternating even+{even_key}/odd+{odd_key}", text)
                    )
            except Exception:
                pass

    # 8. Position + constant: enc[i] = flag[i] + i + c
    for c in range(1, 50):
        candidate = bytes([(b - i - c) % 256 for i, b in enumerate(data)])
        try:
            text = candidate.decode('ascii')
            if is_valid_flag(text):
                results.append((f"Position add + constant {c}", text))
        except Exception:
            pass

    # 9. Multiply (affine): enc[i] = (flag[i] * a + b) mod 256
    # Only try small values of a that have modular inverse
    for a in range(1, 20, 2):  # odd values have inverse mod 256
        # Compute modular inverse of a mod 256
        try:
            a_inv = pow(a, -1, 256)
        except ValueError:
            continue
        for b in range(0, 50):
            candidate = bytes([(a_inv * (byte - b)) % 256 for byte in data])
            try:
                text = candidate.decode('ascii')
                if is_valid_flag(text):
                    results.append((f"Affine a={a}, b={b}", text))
            except Exception:
                pass

    return results


def parse_input_data(raw):
    """Parse encrypted data from various formats."""
    raw = raw.strip()

    # Try as raw bytes (if it came from a binary file)
    if isinstance(raw, bytes):
        return raw

    # Try as hex string (e.g., "706a6571...")
    try:
        cleaned = re.sub(r'[\s,0x\\x]+', '', raw)
        if re.fullmatch(r'[0-9a-fA-F]+', cleaned) and len(cleaned) % 2 == 0:
            return bytes.fromhex(cleaned)
    except Exception:
        pass

    # Try as comma-separated decimals (e.g., "112,106,101,113,...")
    try:
        parts = re.split(r'[\s,]+', raw)
        nums = [int(p) for p in parts]
        if all(0 <= n <= 255 for n in nums):
            return bytes(nums)
    except Exception:
        pass

    # Try as space-separated hex (e.g., "70 6a 65 71 ...")
    try:
        parts = raw.split()
        if all(re.fullmatch(r'[0-9a-fA-F]{2}', p) for p in parts):
            return bytes(int(p, 16) for p in parts)
    except Exception:
        pass

    # Fall back to treating as raw ASCII bytes
    return raw.encode('latin-1')


def main():
    print("=" * 60)
    print("Hidden Cipher 2 - picoCTF 2026")
    print("Reverse the math cipher to recover the flag")
    print("=" * 60)

    data = None

    # Handle command-line arguments
    if len(sys.argv) > 1:
        if sys.argv[1] == '--hex' and len(sys.argv) > 2:
            data = parse_input_data(sys.argv[2])
        elif sys.argv[1] == '--dec' and len(sys.argv) > 2:
            data = parse_input_data(sys.argv[2])
        elif os.path.exists(sys.argv[1]):
            with open(sys.argv[1], 'rb') as f:
                data = f.read()
            print(f"[*] Read {len(data)} bytes from {sys.argv[1]}")
        else:
            data = parse_input_data(sys.argv[1])

    # Use hardcoded data if available
    if data is None and ENCRYPTED_DATA is not None:
        if isinstance(ENCRYPTED_DATA, (list, tuple)):
            data = bytes(ENCRYPTED_DATA)
        elif isinstance(ENCRYPTED_DATA, str):
            data = parse_input_data(ENCRYPTED_DATA)
        else:
            data = ENCRYPTED_DATA

    # Interactive input
    if data is None:
        if not sys.stdin.isatty():
            raw = sys.stdin.buffer.read()
            data = raw
        else:
            print("\n[*] No encrypted data provided.")
            print("[*] Paste the encrypted bytes (hex, decimal, or raw), then press Enter:")
            try:
                raw = input().strip()
                data = parse_input_data(raw)
            except EOFError:
                print("[!] No input.")
                sys.exit(1)

    if not data:
        print("[!] No data to analyze.")
        sys.exit(1)

    print(f"\n[*] Encrypted data ({len(data)} bytes):")
    hex_preview = ' '.join(f'{b:02x}' for b in data[:64])
    print(f"    Hex: {hex_preview}{'...' if len(data) > 64 else ''}")
    try:
        ascii_preview = data[:64].decode('ascii', errors='replace')
        print(f"    ASCII: {ascii_preview}")
    except Exception:
        pass

    # If a known cipher is set, use it directly
    if KNOWN_CIPHER:
        ct = KNOWN_CIPHER["type"]
        if ct == "alternating_add":
            result = reverse_alternating_add(data, KNOWN_CIPHER["even_key"],
                                             KNOWN_CIPHER["odd_key"])
        elif ct == "alternating_add_sub":
            result = reverse_alternating_add_sub(data, KNOWN_CIPHER["add_val"],
                                                 KNOWN_CIPHER["sub_val"])
        elif ct == "constant_shift":
            result = reverse_constant_shift(data, KNOWN_CIPHER["shift"])
        elif ct == "xor":
            result = reverse_constant_xor(data, KNOWN_CIPHER["key"])
        elif ct == "position_add":
            result = reverse_position_add(data)
        else:
            print(f"[!] Unknown cipher type: {ct}")
            sys.exit(1)
        text = result.decode('ascii', errors='replace')
        print(f"\n[+] Decrypted with known cipher ({ct}): {text}")
        if is_valid_flag(text):
            print(f"\n[+] FLAG: {text}")
        return

    # Brute-force all cipher types
    print("\n[*] Trying all common cipher reversals...")
    results = try_all_ciphers(data)

    if results:
        # Deduplicate
        seen = set()
        unique = []
        for method, flag in results:
            if flag not in seen:
                seen.add(flag)
                unique.append((method, flag))

        print(f"\n[+] Found {len(unique)} valid flag(s):\n")
        for method, flag in unique:
            print(f"    Method: {method}")
            print(f"    Flag:   {flag}\n")
    else:
        print("\n[!] No valid flag found with standard cipher reversals.")
        print("[*] The cipher might be more complex. Suggestions:")
        print("    1. Open the binary in Ghidra and look at the encryption function.")
        print("    2. Check if the first 8 bytes should decode to 'picoCTF{' --")
        print("       this gives you known plaintext to derive the key.")
        print("    3. Known plaintext attack:")
        known_plain = b'picoCTF{'
        if len(data) >= len(known_plain):
            print(f"\n[*] Known-plaintext analysis (first {len(known_plain)} bytes):")
            print(f"    Encrypted: {' '.join(f'{b:02x}' for b in data[:len(known_plain)])}")
            print(f"    Expected:  {' '.join(f'{b:02x}' for b in known_plain)}")
            diffs = [(e - p) % 256 for e, p in zip(data[:len(known_plain)], known_plain)]
            xors = [e ^ p for e, p in zip(data[:len(known_plain)], known_plain)]
            print(f"    Differences (enc-plain mod 256): {diffs}")
            print(f"    XOR values:                      {xors}")
            if len(set(diffs)) == 1:
                print(f"    -> Constant shift of {diffs[0]} detected!")
            elif diffs == list(range(len(diffs))):
                print(f"    -> Position-dependent shift (enc[i] = flag[i] + i) detected!")
            if len(set(xors)) == 1:
                print(f"    -> Constant XOR key 0x{xors[0]:02x} detected!")
        sys.exit(1)


if __name__ == "__main__":
    main()
