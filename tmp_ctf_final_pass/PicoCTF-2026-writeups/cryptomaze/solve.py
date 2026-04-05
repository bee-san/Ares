#!/usr/bin/env python3
"""
cryptomaze - picoCTF 2026 (Cryptography, 100 pts)

Recovers a flag encrypted with an LFSR-based stream cipher combined with
additional techniques. The solver uses known-plaintext attack (the flag
prefix 'picoCTF{') to recover the LFSR keystream, then applies the
Berlekamp-Massey algorithm or brute-force to recover the full LFSR state
and decrypt the complete flag.

Usage:
  python3 solve.py                                   # interactive mode
  python3 solve.py --ciphertext <hex_or_file>        # provide ciphertext
  python3 solve.py --ciphertext-file challenge.enc   # from file
  python3 solve.py --source challenge.py             # parse challenge source

Requirements: None (uses only standard library)
"""

import argparse
import sys
import os
import re
import struct
import base64
import json
from itertools import product


# ── LFSR Implementation ─────────────────────────────────────────────────

class LFSR:
    """Linear Feedback Shift Register."""

    def __init__(self, state, taps, nbits):
        """
        state: initial state as an integer
        taps: list of tap positions (0-indexed from MSB)
        nbits: number of bits in the register
        """
        self.state = state & ((1 << nbits) - 1)
        self.taps = taps
        self.nbits = nbits
        self.mask = (1 << nbits) - 1

    def clock(self):
        """Clock the LFSR once. Returns the output bit."""
        # Output bit is the LSB
        output = self.state & 1

        # Compute feedback bit from taps
        feedback = 0
        for tap in self.taps:
            feedback ^= (self.state >> tap) & 1

        # Shift right and insert feedback at MSB
        self.state = (self.state >> 1) | (feedback << (self.nbits - 1))

        return output

    def generate_bits(self, n):
        """Generate n bits of keystream."""
        return [self.clock() for _ in range(n)]

    def generate_bytes(self, n):
        """Generate n bytes of keystream."""
        result = []
        for _ in range(n):
            byte_val = 0
            for bit_pos in range(8):
                byte_val |= self.clock() << bit_pos
            result.append(byte_val)
        return bytes(result)


# ── Berlekamp-Massey Algorithm ───────────────────────────────────────────

def berlekamp_massey(bits):
    """
    Berlekamp-Massey algorithm over GF(2).
    Given a sequence of bits, finds the minimal LFSR that generates it.
    Returns (polynomial_taps, length).
    """
    n = len(bits)
    # Connection polynomial C(x) and auxiliary polynomial B(x)
    C = [1]
    B = [1]
    L = 0   # Current LFSR length
    m = 1   # Number of steps since last length change
    b = 1   # Previous discrepancy

    for i in range(n):
        # Compute discrepancy
        d = bits[i]
        for j in range(1, L + 1):
            if j < len(C):
                d ^= C[j] & bits[i - j]

        if d == 0:
            m += 1
        elif 2 * L <= i:
            # Update both C and L
            T = list(C)
            # C = C + x^m * B
            padded_B = [0] * m + B
            while len(C) < len(padded_B):
                C.append(0)
            for j in range(len(padded_B)):
                C[j] ^= padded_B[j]
            L = i + 1 - L
            B = T
            m = 1
        else:
            # Only update C
            padded_B = [0] * m + B
            while len(C) < len(padded_B):
                C.append(0)
            for j in range(len(padded_B)):
                C[j] ^= padded_B[j]
            m += 1

    # Extract tap positions from connection polynomial
    taps = [i - 1 for i in range(1, len(C)) if C[i] == 1]
    return taps, L


# ── XOR Utilities ────────────────────────────────────────────────────────

def xor_bytes(a, b):
    """XOR two byte sequences."""
    return bytes(x ^ y for x, y in zip(a, b))


def bytes_to_bits(data):
    """Convert bytes to a list of bits (LSB first per byte)."""
    bits = []
    for byte_val in data:
        for i in range(8):
            bits.append((byte_val >> i) & 1)
    return bits


def bits_to_bytes(bits):
    """Convert a list of bits (LSB first per byte) to bytes."""
    result = []
    for i in range(0, len(bits), 8):
        byte_val = 0
        for j in range(min(8, len(bits) - i)):
            byte_val |= bits[i + j] << j
        result.append(byte_val)
    return bytes(result)


# ── Known Plaintext Attack ───────────────────────────────────────────────

def known_plaintext_attack(ciphertext, known_prefix=b'picoCTF{'):
    """
    Use known plaintext to recover LFSR keystream bits,
    then use Berlekamp-Massey to find the LFSR parameters.
    """
    # XOR known plaintext with ciphertext to get keystream
    keystream = xor_bytes(ciphertext[:len(known_prefix)], known_prefix)
    keystream_bits = bytes_to_bits(keystream)

    print(f"[*] Recovered {len(keystream_bits)} keystream bits from known plaintext")
    print(f"[*] Keystream (hex): {keystream.hex()}")

    # Use Berlekamp-Massey to find minimal LFSR
    taps, lfsr_length = berlekamp_massey(keystream_bits)
    print(f"[*] Berlekamp-Massey: LFSR length = {lfsr_length}, taps = {taps}")

    if lfsr_length > len(keystream_bits) // 2:
        print(f"[!] Warning: LFSR length ({lfsr_length}) >= half of known bits ({len(keystream_bits) // 2})")
        print(f"[!] Result may be unreliable. Consider brute-force approach.")

    # Reconstruct the initial state from the first lfsr_length keystream bits
    initial_state = 0
    for i in range(min(lfsr_length, len(keystream_bits))):
        initial_state |= keystream_bits[i] << i

    return taps, lfsr_length, initial_state


# ── Brute-Force Attack ───────────────────────────────────────────────────

def brute_force_attack(ciphertext, taps, nbits, known_prefix=b'picoCTF{'):
    """
    Brute-force all possible LFSR initial states and check which one
    produces a decryption starting with the known prefix.
    """
    total = 1 << nbits
    print(f"[*] Brute-forcing {total} possible states for {nbits}-bit LFSR...")

    for state in range(1, total):  # Skip state 0 (all-zeros produces all-zeros)
        lfsr = LFSR(state, taps, nbits)
        keystream = lfsr.generate_bytes(len(known_prefix))
        decrypted_prefix = xor_bytes(ciphertext[:len(known_prefix)], keystream)

        if decrypted_prefix == known_prefix:
            print(f"[+] Found matching state: {state} (0x{state:x})")

            # Generate full keystream and decrypt
            lfsr = LFSR(state, taps, nbits)
            full_keystream = lfsr.generate_bytes(len(ciphertext))
            plaintext = xor_bytes(ciphertext, full_keystream)

            return state, plaintext

        if state % 10000 == 0:
            print(f"  [{state}/{total}]...", end='\r', file=sys.stderr)

    print("[-] No matching state found.")
    return None, None


# ── Source Code Parser ───────────────────────────────────────────────────

def parse_challenge_source(filepath):
    """Parse the challenge source to extract LFSR parameters and ciphertext."""
    with open(filepath, 'r') as f:
        source = f.read()

    params = {
        'ciphertext': None,
        'taps': None,
        'nbits': None,
        'initial_state': None,
        'additional_key': None,
    }

    # Look for ciphertext as hex string
    hex_pattern = r'(?:cipher|ct|encrypted|enc|output|flag_enc)\s*=\s*["\']([0-9a-fA-F]+)["\']'
    match = re.search(hex_pattern, source)
    if match:
        params['ciphertext'] = bytes.fromhex(match.group(1))

    # Look for ciphertext as bytes
    bytes_pattern = r'(?:cipher|ct|encrypted|enc|output|flag_enc)\s*=\s*b["\'](.+?)["\']'
    match = re.search(bytes_pattern, source)
    if match and params['ciphertext'] is None:
        try:
            params['ciphertext'] = eval(f"b'{match.group(1)}'")
        except Exception:
            pass

    # Look for base64-encoded ciphertext
    b64_pattern = r'(?:cipher|ct|encrypted|enc|output|flag_enc)\s*=\s*["\']([A-Za-z0-9+/=]{16,})["\']'
    match = re.search(b64_pattern, source)
    if match and params['ciphertext'] is None:
        try:
            params['ciphertext'] = base64.b64decode(match.group(1))
        except Exception:
            pass

    # Look for tap positions
    taps_pattern = r'(?:taps|poly|polynomial|feedback)\s*=\s*\[([0-9,\s]+)\]'
    match = re.search(taps_pattern, source)
    if match:
        params['taps'] = [int(x.strip()) for x in match.group(1).split(',')]

    # Look for LFSR size
    size_pattern = r'(?:nbits|size|length|n|bits|degree)\s*=\s*(\d+)'
    match = re.search(size_pattern, source)
    if match:
        params['nbits'] = int(match.group(1))

    # Look for initial state/seed
    seed_pattern = r'(?:state|seed|init|initial|key)\s*=\s*(0x[0-9a-fA-F]+|\d+)'
    match = re.search(seed_pattern, source)
    if match:
        val = match.group(1)
        params['initial_state'] = int(val, 16) if val.startswith('0x') else int(val)

    # Look for additional XOR key
    xor_key_pattern = r'(?:xor_key|key2|extra_key|mask)\s*=\s*(0x[0-9a-fA-F]+|\d+)'
    match = re.search(xor_key_pattern, source)
    if match:
        val = match.group(1)
        params['additional_key'] = int(val, 16) if val.startswith('0x') else int(val)

    return params


# ── Full Solver ──────────────────────────────────────────────────────────

def solve(ciphertext, taps=None, nbits=None, initial_state=None, additional_key=None):
    """Main solving logic."""

    # If we have a secondary XOR key, apply it first to get the LFSR-only ciphertext
    if additional_key is not None:
        print(f"[*] Removing additional XOR layer with key 0x{additional_key:x}")
        if isinstance(additional_key, int):
            key_bytes = (additional_key).to_bytes(
                max(1, (additional_key.bit_length() + 7) // 8), 'little'
            )
            ciphertext = bytes(c ^ key_bytes[i % len(key_bytes)] for i, c in enumerate(ciphertext))

    # If we already have all LFSR parameters, just decrypt
    if taps is not None and nbits is not None and initial_state is not None:
        print(f"[*] All LFSR parameters known. Decrypting directly.")
        lfsr = LFSR(initial_state, taps, nbits)
        keystream = lfsr.generate_bytes(len(ciphertext))
        plaintext = xor_bytes(ciphertext, keystream)
        return plaintext

    # Try known-plaintext attack
    print("[*] Attempting known-plaintext attack...")
    try:
        recovered_taps, lfsr_length, recovered_state = known_plaintext_attack(ciphertext)

        if taps is None:
            taps = recovered_taps
        if nbits is None:
            nbits = lfsr_length

        # Try decrypting with recovered parameters
        lfsr = LFSR(recovered_state, taps, nbits)
        keystream = lfsr.generate_bytes(len(ciphertext))
        plaintext = xor_bytes(ciphertext, keystream)

        if plaintext.startswith(b'picoCTF{') and plaintext.endswith(b'}'):
            return plaintext

        print("[*] Direct recovery didn't produce valid flag. Trying brute-force...")
    except Exception as e:
        print(f"[-] Known-plaintext attack failed: {e}")

    # Brute-force approach
    if taps is not None and nbits is not None:
        state, plaintext = brute_force_attack(ciphertext, taps, nbits)
        if plaintext:
            return plaintext

    # If no taps known, try common LFSR polynomials
    if taps is None:
        common_lfsrs = [
            # (nbits, taps) for common LFSR configurations
            (8, [7, 5, 4, 3]),          # x^8 + x^6 + x^5 + x^4 + 1
            (8, [7, 3, 2, 1]),          # x^8 + x^4 + x^3 + x^2 + 1
            (16, [15, 14, 12, 3]),      # x^16 + x^15 + x^13 + x^4 + 1
            (16, [15, 13, 12, 10]),     # x^16 + x^14 + x^13 + x^11 + 1
            (16, [15, 4, 2, 1]),        # x^16 + x^5 + x^3 + x^2 + 1
            (32, [31, 21, 1, 0]),       # x^32 + x^22 + x^2 + x + 1
            (32, [31, 30, 29, 27]),     # common 32-bit LFSR
            (24, [23, 22, 21, 16]),     # 24-bit LFSR
            (20, [19, 16]),             # x^20 + x^17 + 1
        ]

        print("[*] No taps specified. Trying common LFSR configurations...")
        for test_nbits, test_taps in common_lfsrs:
            print(f"  Trying {test_nbits}-bit LFSR with taps {test_taps}...", end='')
            state, plaintext = brute_force_attack(ciphertext, test_taps, test_nbits)
            if plaintext and plaintext.startswith(b'picoCTF{'):
                print(f" FOUND!")
                return plaintext
            print(" no match")

    return None


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description='cryptomaze solver - picoCTF 2026')
    parser.add_argument('--ciphertext', help='Ciphertext as hex string')
    parser.add_argument('--ciphertext-file', help='File containing ciphertext (raw bytes or hex)')
    parser.add_argument('--source', help='Path to challenge source file')
    parser.add_argument('--taps', help='LFSR tap positions (comma-separated)')
    parser.add_argument('--nbits', type=int, help='LFSR register size in bits')
    parser.add_argument('--state', help='LFSR initial state (hex or decimal)')
    parser.add_argument('--additional-key', help='Additional XOR key (hex or decimal)')
    args = parser.parse_args()

    ciphertext = None
    taps = None
    nbits = args.nbits
    initial_state = None
    additional_key = None

    # Parse tap positions
    if args.taps:
        taps = [int(x.strip()) for x in args.taps.split(',')]

    # Parse initial state
    if args.state:
        initial_state = int(args.state, 16) if args.state.startswith('0x') else int(args.state)

    # Parse additional key
    if args.additional_key:
        val = args.additional_key
        additional_key = int(val, 16) if val.startswith('0x') else int(val)

    # Load ciphertext from various sources
    if args.source and os.path.exists(args.source):
        print(f"[*] Parsing challenge source: {args.source}")
        params = parse_challenge_source(args.source)
        ciphertext = params.get('ciphertext') or ciphertext
        taps = params.get('taps') or taps
        nbits = params.get('nbits') or nbits
        initial_state = params.get('initial_state') or initial_state
        additional_key = params.get('additional_key') or additional_key

    if args.ciphertext:
        ciphertext = bytes.fromhex(args.ciphertext)

    if args.ciphertext_file and os.path.exists(args.ciphertext_file):
        with open(args.ciphertext_file, 'rb') as f:
            raw = f.read()
        # Try to interpret as hex first
        try:
            ciphertext = bytes.fromhex(raw.decode().strip())
        except (ValueError, UnicodeDecodeError):
            ciphertext = raw  # Raw binary

    if ciphertext is None:
        print("[!] No ciphertext provided.")
        print("\nUsage:")
        print("  python3 solve.py --source challenge.py")
        print("  python3 solve.py --ciphertext <hex_string>")
        print("  python3 solve.py --ciphertext-file challenge.enc")
        print("  python3 solve.py --ciphertext <hex> --taps 15,14,12,3 --nbits 16")
        print("\nExample:")
        print("  python3 solve.py --ciphertext 'a1b2c3...' --taps '7,5,4,3' --nbits 8")
        sys.exit(1)

    print(f"[*] Ciphertext: {len(ciphertext)} bytes")
    print(f"[*] Ciphertext (hex): {ciphertext.hex()}")
    if taps:
        print(f"[*] LFSR taps: {taps}")
    if nbits:
        print(f"[*] LFSR size: {nbits} bits")
    if initial_state is not None:
        print(f"[*] Initial state: 0x{initial_state:x}")
    print()

    # Solve
    plaintext = solve(ciphertext, taps, nbits, initial_state, additional_key)

    if plaintext:
        # Try to decode as UTF-8/ASCII
        try:
            text = plaintext.decode('ascii')
            print(f"\n[+] Decrypted plaintext: {text}")
        except UnicodeDecodeError:
            text = plaintext.decode('ascii', errors='replace')
            print(f"\n[+] Decrypted plaintext (lossy): {text}")
            print(f"[+] Hex: {plaintext.hex()}")

        # Extract flag
        flag_match = re.search(r'picoCTF\{[^}]+\}', text)
        if flag_match:
            print(f"\n[+] FLAG: {flag_match.group(0)}")
        else:
            print("\n[-] No picoCTF flag pattern found in decrypted text.")
            print("[*] The plaintext may need additional decoding steps.")
    else:
        print("\n[-] Could not recover the flag.")
        print("[*] Try providing more parameters (taps, nbits) or the challenge source.")


if __name__ == '__main__':
    main()
