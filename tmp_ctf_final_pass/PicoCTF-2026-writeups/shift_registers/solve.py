#!/usr/bin/env python3
"""
shift registers - picoCTF 2026
Category: Cryptography | Points: 200

The challenge uses a Linear Feedback Shift Register (LFSR) to encrypt the flag.
LFSRs are not cryptographically secure -- with known plaintext (the "picoCTF{"
prefix), we can recover the keystream and break the cipher.

This script implements multiple attack strategies:
1. Known-plaintext XOR recovery with brute-force LFSR state search
2. Berlekamp-Massey algorithm for polynomial recovery
3. Direct brute force for small register sizes

Usage:
    python3 solve.py                     # Interactive mode
    python3 solve.py <ciphertext_file>   # Provide ciphertext file
    python3 solve.py --source <src.py>   # Analyze source code
"""

import sys
import os
import re
import struct
from itertools import product


# ============================================================
# LFSR Implementation
# ============================================================

class LFSR:
    """A Fibonacci LFSR implementation."""

    def __init__(self, state, taps, size):
        """
        Args:
            state: Initial state (integer)
            taps: List of tap positions (0-indexed from LSB) or feedback polynomial mask
            size: Number of bits in the register
        """
        self.state = state & ((1 << size) - 1)
        self.size = size
        if isinstance(taps, int):
            # taps is a bitmask / polynomial
            self.mask = taps & ((1 << size) - 1)
        else:
            # taps is a list of positions
            self.mask = 0
            for t in taps:
                self.mask |= (1 << t)

    def clock(self):
        """Clock the LFSR once, return the output bit."""
        output = self.state & 1
        # Compute feedback: XOR of tapped bits
        feedback = bin(self.state & self.mask).count('1') % 2
        self.state = (self.state >> 1) | (feedback << (self.size - 1))
        return output

    def generate(self, n_bits):
        """Generate n_bits of keystream."""
        bits = []
        for _ in range(n_bits):
            bits.append(self.clock())
        return bits

    def generate_bytes(self, n_bytes):
        """Generate n_bytes of keystream as a bytearray."""
        result = bytearray()
        for _ in range(n_bytes):
            byte = 0
            for bit_pos in range(8):
                bit = self.clock()
                byte |= (bit << bit_pos)
            result.append(byte)
        return bytes(result)


# ============================================================
# Berlekamp-Massey Algorithm
# ============================================================

def berlekamp_massey(sequence):
    """
    Berlekamp-Massey algorithm over GF(2).
    Given a binary sequence, find the shortest LFSR that generates it.

    Returns:
        (polynomial, length) where polynomial is a list of tap positions
        and length is the LFSR size.
    """
    n = len(sequence)
    # Connection polynomial coefficients (binary)
    c = [0] * (n + 1)
    b = [0] * (n + 1)
    c[0] = 1
    b[0] = 1
    L = 0  # Current LFSR length
    m = 1  # Number of iterations since L was last updated

    for i in range(n):
        # Compute discrepancy
        d = sequence[i]
        for j in range(1, L + 1):
            d ^= (c[j] & sequence[i - j])

        if d == 0:
            m += 1
        else:
            t = c[:]
            for j in range(m, n + 1):
                c[j] ^= b[j - m]
            if 2 * L <= i:
                L = i + 1 - L
                b = t[:]
                m = 1
            else:
                m += 1

    # Extract tap positions from the connection polynomial
    taps = []
    for j in range(1, L + 1):
        if c[j]:
            taps.append(j)

    return taps, L


# ============================================================
# Attack Functions
# ============================================================

def xor_bytes(a, b):
    """XOR two byte sequences."""
    return bytes(x ^ y for x, y in zip(a, b))


def bytes_to_bits(data):
    """Convert bytes to a list of bits (LSB first per byte)."""
    bits = []
    for byte in data:
        for i in range(8):
            bits.append((byte >> i) & 1)
    return bits


def bits_to_bytes(bits):
    """Convert a list of bits (LSB first per byte) to bytes."""
    result = bytearray()
    for i in range(0, len(bits), 8):
        byte = 0
        for j in range(min(8, len(bits) - i)):
            byte |= (bits[i + j] << j)
        result.append(byte)
    return bytes(result)


def recover_keystream(ciphertext, known_plaintext):
    """Recover keystream bits from known plaintext."""
    keystream_bytes = xor_bytes(ciphertext[:len(known_plaintext)], known_plaintext)
    return bytes_to_bits(keystream_bytes)


def brute_force_lfsr(keystream_bits, size, taps_mask):
    """
    Brute force the initial state of an LFSR given known keystream bits.

    Args:
        keystream_bits: Known output bits
        size: LFSR register size
        taps_mask: Known feedback polynomial mask

    Returns:
        The initial state if found, else None.
    """
    n_known = len(keystream_bits)

    for state in range(1, 1 << size):  # Skip 0 state (stuck)
        lfsr = LFSR(state, taps_mask, size)
        match = True
        for i in range(n_known):
            if lfsr.clock() != keystream_bits[i]:
                match = False
                break
        if match:
            return state

    return None


def attack_with_known_prefix(ciphertext, lfsr_size=None, taps=None):
    """
    Full attack using the known picoCTF{ prefix.

    Args:
        ciphertext: The encrypted flag (bytes)
        lfsr_size: LFSR register size (if known)
        taps: Tap positions or polynomial mask (if known)
    """
    known_prefix = b"picoCTF{"

    print(f"[*] Ciphertext length: {len(ciphertext)} bytes")
    print(f"[*] Known prefix: {known_prefix}")

    # Step 1: Recover keystream from known plaintext
    keystream_bits = recover_keystream(ciphertext, known_prefix)
    print(f"[*] Recovered {len(keystream_bits)} keystream bits from known prefix")

    if lfsr_size and taps:
        # We know both the size and taps -- just brute force the initial state
        print(f"[*] LFSR size: {lfsr_size}, taps/mask: {taps}")
        print(f"[*] Brute-forcing initial state (2^{lfsr_size} possibilities)...")

        if isinstance(taps, list):
            mask = 0
            for t in taps:
                mask |= (1 << t)
        else:
            mask = taps

        init_state = brute_force_lfsr(keystream_bits, lfsr_size, mask)

        if init_state:
            print(f"[+] Found initial state: {init_state} (0x{init_state:x})")

            # Generate full keystream and decrypt
            lfsr = LFSR(init_state, mask, lfsr_size)
            full_keystream = lfsr.generate_bytes(len(ciphertext))
            plaintext = xor_bytes(ciphertext, full_keystream)

            print(f"[+] Decrypted: {plaintext}")

            flag = extract_flag(plaintext.decode('utf-8', errors='replace'))
            if flag:
                print(f"\n[FLAG] {flag}")
                return flag
        else:
            print("[-] Could not find matching initial state")

    elif lfsr_size and not taps:
        # We know the size but not the taps -- use Berlekamp-Massey
        print(f"[*] LFSR size: {lfsr_size} (taps unknown)")
        print(f"[*] Applying Berlekamp-Massey algorithm...")

        recovered_taps, recovered_L = berlekamp_massey(keystream_bits)
        print(f"[+] Recovered LFSR length: {recovered_L}")
        print(f"[+] Recovered taps: {recovered_taps}")

        if recovered_L <= lfsr_size:
            mask = 0
            for t in recovered_taps:
                mask |= (1 << t)

            init_state = brute_force_lfsr(keystream_bits, lfsr_size, mask)
            if init_state:
                print(f"[+] Found initial state: {init_state} (0x{init_state:x})")
                lfsr = LFSR(init_state, mask, lfsr_size)
                full_keystream = lfsr.generate_bytes(len(ciphertext))
                plaintext = xor_bytes(ciphertext, full_keystream)
                print(f"[+] Decrypted: {plaintext}")

                flag = extract_flag(plaintext.decode('utf-8', errors='replace'))
                if flag:
                    print(f"\n[FLAG] {flag}")
                    return flag

    else:
        # Unknown size and taps -- try common sizes
        print("[*] LFSR parameters unknown. Trying common register sizes...")

        # First try Berlekamp-Massey to estimate size
        recovered_taps, recovered_L = berlekamp_massey(keystream_bits)
        print(f"[*] Berlekamp-Massey estimates LFSR length: {recovered_L}")
        print(f"[*] Estimated taps: {recovered_taps}")

        # Try sizes from the estimated length up to 32
        for size in range(max(1, recovered_L), 33):
            mask = 0
            for t in recovered_taps:
                if t < size:
                    mask |= (1 << t)

            if mask == 0:
                continue

            init_state = brute_force_lfsr(keystream_bits, size, mask)
            if init_state:
                print(f"\n[+] Match found! Size: {size}, State: {init_state} (0x{init_state:x})")
                lfsr = LFSR(init_state, mask, size)
                full_keystream = lfsr.generate_bytes(len(ciphertext))
                plaintext = xor_bytes(ciphertext, full_keystream)

                try:
                    decoded = plaintext.decode('utf-8', errors='strict')
                    if decoded.startswith('picoCTF{') and decoded.endswith('}'):
                        print(f"[+] Decrypted: {decoded}")
                        print(f"\n[FLAG] {decoded}")
                        return decoded
                except UnicodeDecodeError:
                    pass

                decoded = plaintext.decode('utf-8', errors='replace')
                if 'picoCTF{' in decoded:
                    print(f"[+] Decrypted: {decoded}")
                    flag = extract_flag(decoded)
                    if flag:
                        print(f"\n[FLAG] {flag}")
                        return flag

        print("[-] Could not find a match with common sizes.")

    return None


def extract_flag(text):
    """Extract picoCTF flag from text."""
    match = re.search(r'picoCTF\{[^}]+\}', text)
    return match.group(0) if match else None


def parse_ciphertext(data):
    """
    Try to parse ciphertext from various formats:
    - Raw bytes
    - Hex string
    - Base64
    - Comma-separated integers
    - Python list literal
    """
    import base64

    # If it's already bytes, return as is
    if isinstance(data, bytes):
        # Check if it looks like a hex string
        try:
            text = data.decode('ascii').strip()
        except UnicodeDecodeError:
            return data
    else:
        text = data.strip()

    # Try hex string (with or without 0x prefix, with or without spaces)
    hex_clean = text.replace('0x', '').replace(' ', '').replace(',', '').replace('\n', '')
    if all(c in '0123456789abcdefABCDEF' for c in hex_clean) and len(hex_clean) % 2 == 0:
        try:
            return bytes.fromhex(hex_clean)
        except ValueError:
            pass

    # Try base64
    try:
        decoded = base64.b64decode(text)
        if len(decoded) > 0:
            return decoded
    except Exception:
        pass

    # Try comma-separated integers (e.g., "72, 101, 108, 108, 111")
    try:
        # Handle Python list format: [72, 101, ...]
        cleaned = text.strip('[]() ')
        values = [int(x.strip()) for x in cleaned.split(',')]
        if all(0 <= v <= 255 for v in values):
            return bytes(values)
    except (ValueError, TypeError):
        pass

    # Return raw bytes
    if isinstance(text, str):
        return text.encode()
    return data


# ============================================================
# Main
# ============================================================

def main():
    print("=" * 60)
    print("shift registers - picoCTF 2026")
    print("LFSR Stream Cipher Attack")
    print("=" * 60)

    ciphertext = None
    lfsr_size = None
    taps = None

    # Check command-line arguments
    if len(sys.argv) > 1:
        arg = sys.argv[1]

        if arg == "--source" and len(sys.argv) > 2:
            # Analyze source code
            src_path = sys.argv[2]
            print(f"\n[*] Analyzing source: {src_path}")
            with open(src_path, 'r') as f:
                source = f.read()
            print(source)
            print("\n[*] Please extract the LFSR parameters from the source above")
            print("[*] and modify the script accordingly.")
            return

        elif os.path.isfile(arg):
            # Read ciphertext from file
            print(f"\n[*] Reading ciphertext from: {arg}")
            with open(arg, 'rb') as f:
                raw = f.read()
            ciphertext = parse_ciphertext(raw)
            print(f"[*] Parsed {len(ciphertext)} bytes of ciphertext")

    if ciphertext is None:
        # Demo mode with example
        print("\n[*] No ciphertext file provided. Running in demo/interactive mode.")
        print("[*] To use with actual challenge data:")
        print(f"    python3 {sys.argv[0]} <ciphertext_file>")
        print(f"    python3 {sys.argv[0]} --source <challenge_source.py>")

        print("\n" + "-" * 60)
        print("DEMO: Encrypting and breaking a sample LFSR cipher")
        print("-" * 60)

        # Demo: Create an LFSR, encrypt a sample flag, then break it
        demo_flag = b"picoCTF{lfsr_1s_n0t_s3cur3}"
        demo_size = 16
        demo_taps = 0b1011010000000001  # x^16 + x^14 + x^13 + x^11 + x^1
        demo_state = 0xACE1  # Initial state

        print(f"[Demo] Flag: {demo_flag.decode()}")
        print(f"[Demo] LFSR size: {demo_size} bits")
        print(f"[Demo] Taps mask: 0x{demo_taps:04x} (binary: {demo_taps:016b})")
        print(f"[Demo] Initial state: 0x{demo_state:04x}")

        # Encrypt
        lfsr = LFSR(demo_state, demo_taps, demo_size)
        keystream = lfsr.generate_bytes(len(demo_flag))
        demo_ciphertext = xor_bytes(demo_flag, keystream)
        print(f"[Demo] Ciphertext (hex): {demo_ciphertext.hex()}")

        # Now attack it
        print(f"\n[*] Attacking the demo ciphertext...")
        result = attack_with_known_prefix(demo_ciphertext, demo_size, demo_taps)

        if result:
            print(f"\n[SUCCESS] Recovered flag: {result}")
        else:
            print("\n[FAIL] Could not recover the flag in demo mode.")

        print("\n" + "-" * 60)
        print("For the actual challenge, provide the ciphertext file:")
        print(f"    python3 {sys.argv[0]} <ciphertext_file>")
        print("\nIf you know the LFSR parameters, edit the variables below in the script:")
        print("    lfsr_size = <register size>")
        print("    taps = <tap mask or list>")
        return

    # ================================================================
    # EDIT THESE VALUES based on the challenge source code if available
    # ================================================================
    # lfsr_size = 16        # Uncomment and set if known
    # taps = [0, 2, 3, 5]  # Uncomment and set if known (list of tap positions)
    # taps = 0x002D         # Or set as a bitmask
    # ================================================================

    print(f"\n[*] Starting attack on ciphertext ({len(ciphertext)} bytes)...")
    result = attack_with_known_prefix(ciphertext, lfsr_size, taps)

    if result:
        print(f"\n{'=' * 60}")
        print(f"FLAG: {result}")
        print(f"{'=' * 60}")
    else:
        print("\n[-] Automatic attack failed.")
        print("[-] You may need to:")
        print("    1. Examine the challenge source code for LFSR parameters")
        print("    2. Set lfsr_size and taps variables in the script")
        print("    3. Check if the LFSR generates keystream bytes in MSB or LSB order")
        print("    4. Check if there's additional encoding (base64, hex)")


if __name__ == "__main__":
    main()
