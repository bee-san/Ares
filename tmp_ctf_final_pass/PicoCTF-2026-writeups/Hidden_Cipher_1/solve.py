#!/usr/bin/env python3
"""
Hidden Cipher 1 - picoCTF 2026
Category: Reverse Engineering | Points: 100

The binary contains a flag encrypted with a simple cipher.
This script attempts multiple common cipher types to decrypt it.

Usage:
    python3 solve.py                     # Use embedded encrypted data
    python3 solve.py <encrypted_file>    # Read encrypted data from file
    python3 solve.py --binary <binary>   # Extract encrypted data from binary

Steps to adapt this script:
    1. Run the binary through Ghidra/IDA to identify the cipher
    2. Extract the encrypted flag bytes (from binary or data file)
    3. Update ENCRYPTED_FLAG and select the correct cipher method
    4. Run the script to decrypt
"""

import sys
import struct
import os

# ============================================================
# CONFIGURATION - Update these after reversing the binary
# ============================================================

# The encrypted flag bytes extracted from the binary or data file.
# Update this with the actual encrypted bytes from your instance.
# Example (from a typical picoCTF reverse cipher challenge):
ENCRYPTED_FLAG = b""

# If the encrypted flag is in a file, specify the filename here
# (or pass it as a command-line argument)
ENCRYPTED_FILE = "rev_this"

# Known plaintext prefix -- picoCTF flags always start with this
KNOWN_PREFIX = b"picoCTF{"

# Number of prefix characters left unencrypted (0 if all are encrypted)
UNENCRYPTED_PREFIX_LEN = 0

# ============================================================
# CIPHER IMPLEMENTATIONS
# ============================================================

def decrypt_alternating_add_sub(data, add_val=5, sub_val=2, start_index=0):
    """
    Decrypt alternating add/subtract cipher.
    Even indices had add_val added -> subtract it
    Odd indices had sub_val subtracted -> add it

    This is the most common cipher in picoCTF RE challenges.
    """
    result = bytearray()
    for i in range(len(data)):
        if i < start_index:
            # Characters before start_index were not encrypted
            result.append(data[i])
        elif (i - start_index) % 2 == 0:
            # Even offset: was encrypted by adding -> decrypt by subtracting
            result.append((data[i] - add_val) % 256)
        else:
            # Odd offset: was encrypted by subtracting -> decrypt by adding
            result.append((data[i] + sub_val) % 256)
    return bytes(result)


def decrypt_xor_single_byte(data, key):
    """Decrypt single-byte XOR cipher."""
    return bytes([b ^ key for b in data])


def decrypt_xor_multi_byte(data, key):
    """Decrypt multi-byte (repeating) XOR cipher."""
    if isinstance(key, str):
        key = key.encode()
    return bytes([b ^ key[i % len(key)] for i, b in enumerate(data)])


def decrypt_caesar(data, shift):
    """Decrypt Caesar cipher (shift all printable chars)."""
    result = bytearray()
    for b in data:
        if 32 <= b <= 126:
            result.append(((b - 32 - shift) % 95) + 32)
        else:
            result.append(b)
    return bytes(result)


def decrypt_subtract_constant(data, constant):
    """Decrypt by adding a constant (reverse of subtracting)."""
    return bytes([(b + constant) % 256 for b in data])


def decrypt_add_constant(data, constant):
    """Decrypt by subtracting a constant (reverse of adding)."""
    return bytes([(b - constant) % 256 for b in data])


# ============================================================
# KEY RECOVERY
# ============================================================

def try_recover_xor_key(data, known_prefix=KNOWN_PREFIX):
    """Try to recover XOR key using known plaintext attack."""
    if len(data) < len(known_prefix):
        return None

    # Recover key bytes from known prefix
    key_bytes = bytes([data[i] ^ known_prefix[i] for i in range(len(known_prefix))])

    # Check if the key repeats (common pattern)
    for key_len in range(1, len(known_prefix) + 1):
        candidate_key = key_bytes[:key_len]
        # Verify the key works for all known prefix bytes
        matches = all(
            data[i] ^ candidate_key[i % key_len] == known_prefix[i]
            for i in range(len(known_prefix))
        )
        if matches:
            return candidate_key

    return key_bytes


def try_recover_arithmetic_params(data, known_prefix=KNOWN_PREFIX, start_index=0):
    """Try to recover add/sub parameters using known plaintext."""
    if len(data) < len(known_prefix):
        return None, None

    diffs = []
    for i in range(start_index, min(len(data), len(known_prefix))):
        diffs.append(data[i] - known_prefix[i])

    if len(diffs) < 2:
        return None, None

    # Check for alternating pattern
    even_diffs = [diffs[i] for i in range(0, len(diffs), 2)]
    odd_diffs = [diffs[i] for i in range(1, len(diffs), 2)]

    if even_diffs and all(d == even_diffs[0] for d in even_diffs):
        add_val = even_diffs[0]
    else:
        add_val = None

    if odd_diffs and all(d == odd_diffs[0] for d in odd_diffs):
        sub_val = -odd_diffs[0]
    else:
        sub_val = None

    return add_val, sub_val


# ============================================================
# MAIN LOGIC
# ============================================================

def load_encrypted_data():
    """Load encrypted data from file or configuration."""
    # Check command line arguments
    if len(sys.argv) > 1 and sys.argv[1] != "--binary":
        filepath = sys.argv[1]
        if os.path.exists(filepath):
            with open(filepath, "rb") as f:
                data = f.read()
            print(f"[*] Loaded {len(data)} bytes from {filepath}")
            return data

    if len(sys.argv) > 2 and sys.argv[1] == "--binary":
        binary_path = sys.argv[2]
        if os.path.exists(binary_path):
            # Try to extract strings that look like encrypted flags
            with open(binary_path, "rb") as f:
                binary_data = f.read()
            # Look for data near "picoCTF" or known markers
            print(f"[*] Loaded binary {binary_path} ({len(binary_data)} bytes)")
            print("[*] Searching for encrypted flag data in binary...")
            return binary_data

    # Try default encrypted file
    if os.path.exists(ENCRYPTED_FILE):
        with open(ENCRYPTED_FILE, "rb") as f:
            data = f.read()
        print(f"[*] Loaded {len(data)} bytes from {ENCRYPTED_FILE}")
        return data

    # Try other common filenames
    for fname in ["enc_flag", "encrypted.txt", "flag.enc", "output.txt", "cipher.txt"]:
        if os.path.exists(fname):
            with open(fname, "rb") as f:
                data = f.read()
            print(f"[*] Loaded {len(data)} bytes from {fname}")
            return data

    # Use embedded data
    if ENCRYPTED_FLAG:
        print(f"[*] Using embedded encrypted data ({len(ENCRYPTED_FLAG)} bytes)")
        return ENCRYPTED_FLAG

    return None


def is_valid_flag(data):
    """Check if decrypted data looks like a valid picoCTF flag."""
    try:
        text = data.decode("ascii", errors="strict")
        if text.startswith("picoCTF{") and text.endswith("}"):
            # Check all chars are printable
            if all(32 <= ord(c) <= 126 for c in text):
                return True
    except Exception:
        pass

    # Partial check -- does it contain the flag format?
    try:
        text = data.decode("ascii", errors="replace")
        if "picoCTF{" in text:
            return True
    except Exception:
        pass

    return False


def extract_flag(data):
    """Extract picoCTF{...} from decrypted data."""
    try:
        text = data.decode("ascii", errors="replace")
        start = text.find("picoCTF{")
        if start == -1:
            return None
        end = text.find("}", start)
        if end == -1:
            return text[start:]
        return text[start:end + 1]
    except Exception:
        return None


def main():
    print("=" * 60)
    print(" Hidden Cipher 1 - picoCTF 2026 Solver")
    print("=" * 60)
    print()

    data = load_encrypted_data()

    if data is None:
        print("[!] No encrypted data found.")
        print("[!] Usage:")
        print("    python3 solve.py <encrypted_file>")
        print("    python3 solve.py --binary <binary_file>")
        print()
        print("[*] Or update ENCRYPTED_FLAG in this script with the")
        print("    encrypted bytes extracted from the binary/file.")
        print()
        print("[*] Example workflow:")
        print("    1. Open binary in Ghidra")
        print("    2. Find the encrypted flag data (byte array or string)")
        print("    3. Paste it into ENCRYPTED_FLAG in this script")
        print("    4. Run this script again")
        return

    print(f"[*] Encrypted data (hex): {data.hex()}")
    print(f"[*] Encrypted data (raw): {data}")
    print()

    # ----- Try all decryption methods -----

    results = []

    # Method 1: Alternating add/subtract (most common in picoCTF)
    print("[*] Trying alternating add/subtract ciphers...")
    for add_val in range(1, 20):
        for sub_val in range(1, 20):
            for start in [0, 8]:  # 0 = all encrypted, 8 = first 8 chars unencrypted
                dec = decrypt_alternating_add_sub(data, add_val, sub_val, start)
                if is_valid_flag(dec):
                    flag = extract_flag(dec)
                    results.append(("alternating_add_sub", f"add={add_val}, sub={sub_val}, start={start}", flag))

    # Method 1b: Use known-plaintext to recover arithmetic parameters
    add_val, sub_val = try_recover_arithmetic_params(data)
    if add_val is not None and sub_val is not None:
        print(f"[*] Recovered arithmetic params: add={add_val}, sub={sub_val}")
        for start in [0, 8]:
            dec = decrypt_alternating_add_sub(data, add_val, sub_val, start)
            flag = extract_flag(dec)
            if flag:
                results.append(("alternating_add_sub (recovered)", f"add={add_val}, sub={sub_val}, start={start}", flag))

    # Method 2: Single-byte XOR
    print("[*] Trying single-byte XOR...")
    for key in range(1, 256):
        dec = decrypt_xor_single_byte(data, key)
        if is_valid_flag(dec):
            flag = extract_flag(dec)
            results.append(("xor_single", f"key=0x{key:02x}", flag))

    # Method 3: Multi-byte XOR with known-plaintext recovery
    print("[*] Trying multi-byte XOR with known-plaintext recovery...")
    recovered_key = try_recover_xor_key(data)
    if recovered_key:
        dec = decrypt_xor_multi_byte(data, recovered_key)
        if is_valid_flag(dec):
            flag = extract_flag(dec)
            results.append(("xor_multi (recovered)", f"key={recovered_key.hex()}", flag))

    # Method 4: Caesar cipher
    print("[*] Trying Caesar cipher...")
    for shift in range(1, 95):
        dec = decrypt_caesar(data, shift)
        if is_valid_flag(dec):
            flag = extract_flag(dec)
            results.append(("caesar", f"shift={shift}", flag))

    # Method 5: Simple add/subtract constant
    print("[*] Trying constant add/subtract...")
    for const in range(1, 128):
        dec = decrypt_add_constant(data, const)
        if is_valid_flag(dec):
            flag = extract_flag(dec)
            results.append(("subtract_constant", f"const={const}", flag))

        dec = decrypt_subtract_constant(data, const)
        if is_valid_flag(dec):
            flag = extract_flag(dec)
            results.append(("add_constant", f"const={const}", flag))

    # ----- Report results -----
    print()
    if results:
        print("=" * 60)
        print(f" FOUND {len(results)} POSSIBLE DECRYPTION(S)")
        print("=" * 60)
        for method, params, flag in results:
            print(f"  Method: {method}")
            print(f"  Params: {params}")
            print(f"  Flag:   {flag}")
            print()
    else:
        print("[!] No valid decryption found with standard methods.")
        print("[*] Manual reverse engineering may be needed:")
        print("    1. Open the binary in Ghidra")
        print("    2. Find the encryption function")
        print("    3. Identify the exact cipher algorithm and key")
        print("    4. Update this script accordingly")
        print()
        print("[*] Hint: Look for loops with XOR, ADD, SUB, or ROL/ROR")
        print("    operations on character arrays in the binary.")


if __name__ == "__main__":
    main()
