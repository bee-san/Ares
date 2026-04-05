#!/usr/bin/env python3
"""
bytemancy 2 - picoCTF 2026 (General Skills, 200 pts)

This solver reverses the byte transformations in the bytemancy 2 challenge
to produce the correct input bytes. It analyzes the challenge source code's
transformation logic and computes the inverse.

The general approach for bytemancy challenges:
  1. Parse the source code to extract keys, expected values, and transforms.
  2. Invert each transformation to recover the required input bytes.
  3. Output the raw bytes (which may include non-printable characters).

Usage:
  python3 solve.py                              # print the solution bytes
  python3 solve.py | python3 bytemancy2.py      # pipe to local challenge
  python3 solve.py | nc <host> <port>           # pipe to remote challenge
  python3 solve.py --source bytemancy2.py       # auto-parse source file
"""

import sys
import struct
import re
import os
import argparse


# ── Byte inversion utilities ─────────────────────────────────────────────

def xor_bytes(data, key):
    """XOR each byte of data with corresponding key byte (cyclic)."""
    if isinstance(key, int):
        return bytes(b ^ key for b in data)
    return bytes(b ^ key[i % len(key)] for i, b in enumerate(data))


def sub_bytes(expected, key):
    """Invert addition: input[i] = (expected[i] - key[i]) % 256"""
    if isinstance(key, int):
        return bytes((e - key) % 256 for e in expected)
    return bytes((e - key[i % len(key)]) % 256 for i, e in enumerate(expected))


def add_bytes(expected, key):
    """Invert subtraction: input[i] = (expected[i] + key[i]) % 256"""
    if isinstance(key, int):
        return bytes((e + key) % 256 for e in expected)
    return bytes((e + key[i % len(key)]) % 256 for i, e in enumerate(expected))


def ror_byte(val, n, bits=8):
    """Rotate right by n bits."""
    n = n % bits
    return ((val >> n) | (val << (bits - n))) & ((1 << bits) - 1)


def rol_byte(val, n, bits=8):
    """Rotate left by n bits."""
    n = n % bits
    return ((val << n) | (val >> (bits - n))) & ((1 << bits) - 1)


def inv_ror_bytes(expected, n):
    """Invert rotate-right: apply rotate-left."""
    return bytes(rol_byte(e, n) for e in expected)


def inv_rol_bytes(expected, n):
    """Invert rotate-left: apply rotate-right."""
    return bytes(ror_byte(e, n) for e in expected)


def swap_nibbles(data):
    """Swap high and low nibbles of each byte."""
    return bytes(((b >> 4) | ((b & 0x0F) << 4)) & 0xFF for b in data)


def reverse_bits(val, bits=8):
    """Reverse the bits of a byte."""
    result = 0
    for _ in range(bits):
        result = (result << 1) | (val & 1)
        val >>= 1
    return result


def inv_reverse_bits(expected):
    """Invert bit-reversal (it is its own inverse)."""
    return bytes(reverse_bits(e) for e in expected)


# ── Source code parser ───────────────────────────────────────────────────

def parse_source_file(filepath):
    """
    Attempt to auto-parse the challenge source code to extract:
    - Expected/target values
    - Transformation operations
    - Keys/constants
    """
    with open(filepath, 'r') as f:
        source = f.read()

    info = {
        'expected': None,
        'keys': None,
        'operations': [],
    }

    # Look for byte arrays / lists that could be expected values
    # Patterns: [0x41, 0x42, ...] or b'\x41\x42...' or bytearray(...)
    hex_list_pattern = r'\[(\s*0x[0-9a-fA-F]{1,2}(?:\s*,\s*0x[0-9a-fA-F]{1,2})*\s*)\]'
    matches = re.findall(hex_list_pattern, source)
    if matches:
        for match in matches:
            values = [int(x.strip(), 16) for x in match.split(',')]
            if len(values) >= 4:  # Likely a meaningful array
                if info['expected'] is None:
                    info['expected'] = values
                elif info['keys'] is None:
                    info['keys'] = values

    # Look for byte string literals
    byte_str_pattern = r"b'((?:\\x[0-9a-fA-F]{2})+)'"
    byte_matches = re.findall(byte_str_pattern, source)
    if byte_matches:
        for match in byte_matches:
            values = [int(x, 16) for x in re.findall(r'\\x([0-9a-fA-F]{2})', match)]
            if len(values) >= 4:
                if info['expected'] is None:
                    info['expected'] = values
                elif info['keys'] is None:
                    info['keys'] = values

    # Detect operations
    if 'xor' in source.lower() or '^' in source:
        info['operations'].append('xor')
    if re.search(r'[+]\s*\d', source) or 'add' in source.lower():
        info['operations'].append('add')
    if re.search(r'[-]\s*\d', source) or 'sub' in source.lower():
        info['operations'].append('sub')
    if 'rol' in source.lower() or '<<' in source:
        info['operations'].append('rol')
    if 'ror' in source.lower() or '>>' in source:
        info['operations'].append('ror')
    if 'swap' in source.lower():
        info['operations'].append('swap_nibbles')
    if 'reverse' in source.lower():
        info['operations'].append('reverse_bits')

    return info


# ── Solver ───────────────────────────────────────────────────────────────

def solve_from_source(info):
    """Given parsed source info, attempt to recover the input bytes."""
    expected = info['expected']
    keys = info['keys']
    ops = info['operations']

    if expected is None:
        print("[-] Could not extract expected values from source.", file=sys.stderr)
        return None

    result = bytes(expected)
    print(f"[*] Expected values ({len(expected)} bytes): {expected}", file=sys.stderr)
    if keys:
        print(f"[*] Keys ({len(keys)} bytes): {keys}", file=sys.stderr)
    print(f"[*] Detected operations: {ops}", file=sys.stderr)

    # Apply inverse operations in reverse order
    # This is a heuristic -- the actual order depends on the source code
    for op in reversed(ops):
        if op == 'xor' and keys:
            result = xor_bytes(result, keys)
            print(f"[*] Applied inverse XOR", file=sys.stderr)
        elif op == 'add' and keys:
            result = sub_bytes(result, keys)
            print(f"[*] Applied inverse ADD (subtraction)", file=sys.stderr)
        elif op == 'sub' and keys:
            result = add_bytes(result, keys)
            print(f"[*] Applied inverse SUB (addition)", file=sys.stderr)
        elif op == 'swap_nibbles':
            result = swap_nibbles(result)
            print(f"[*] Applied nibble swap", file=sys.stderr)
        elif op == 'reverse_bits':
            result = inv_reverse_bits(result)
            print(f"[*] Applied bit reversal", file=sys.stderr)
        elif op == 'rol':
            # Default rotation amount; adjust based on source
            result = inv_rol_bytes(result, 3)
            print(f"[*] Applied inverse ROL (ROR by 3)", file=sys.stderr)
        elif op == 'ror':
            result = inv_ror_bytes(result, 3)
            print(f"[*] Applied inverse ROR (ROL by 3)", file=sys.stderr)

    return result


def solve_generic():
    """
    Generic solver template for bytemancy 2.
    Update the expected values, keys, and transformation based on the
    actual challenge source code.
    """
    # ══════════════════════════════════════════════════════════════════
    # UPDATE THESE VALUES from the challenge source code:
    # ══════════════════════════════════════════════════════════════════

    # Example: expected output values after transformation
    expected = [
        # Replace with actual values from the challenge source
        # e.g., 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, ...
    ]

    # Example: XOR key / transformation key
    key = [
        # Replace with actual key values from the challenge source
        # e.g., 0x12, 0x34, 0x56, 0x78, ...
    ]

    if not expected:
        print("[!] No expected values configured.", file=sys.stderr)
        print("[!] Please update this script with values from the challenge source.", file=sys.stderr)
        print("[!] Or use: python3 solve.py --source <challenge_source.py>", file=sys.stderr)
        return None

    # ══════════════════════════════════════════════════════════════════
    # Apply inverse transformations (adjust order based on source code)
    # ══════════════════════════════════════════════════════════════════

    # Step 1: Invert XOR
    result = xor_bytes(bytes(expected), key)

    # Step 2: Invert any additional operations
    # Uncomment/modify as needed:
    # result = sub_bytes(result, add_constant)
    # result = inv_rol_bytes(result, rotation_amount)
    # result = swap_nibbles(result)

    return result


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description='bytemancy 2 solver - picoCTF 2026')
    parser.add_argument('--source', help='Path to the challenge source file')
    parser.add_argument('--hex', action='store_true', help='Output as hex string')
    parser.add_argument('--python', action='store_true', help='Output as Python bytes literal')
    args = parser.parse_args()

    result = None

    if args.source and os.path.exists(args.source):
        print(f"[*] Parsing source file: {args.source}", file=sys.stderr)
        info = parse_source_file(args.source)
        result = solve_from_source(info)
    else:
        if args.source:
            print(f"[-] Source file not found: {args.source}", file=sys.stderr)
        result = solve_generic()

    if result is None:
        print("[-] No solution computed.", file=sys.stderr)
        print("\n[*] Manual usage:", file=sys.stderr)
        print("    1. Download the challenge source code", file=sys.stderr)
        print("    2. Extract expected values and keys", file=sys.stderr)
        print("    3. Run: python3 solve.py --source <challenge_source.py>", file=sys.stderr)
        print("    4. Or: Update solve_generic() with the values and re-run", file=sys.stderr)
        sys.exit(1)

    # Output the result
    print(f"[+] Solution ({len(result)} bytes):", file=sys.stderr)
    print(f"[+] Hex: {result.hex()}", file=sys.stderr)
    print(f"[+] Repr: {result!r}", file=sys.stderr)

    # Check if the result looks like printable ASCII
    if all(0x20 <= b <= 0x7e for b in result):
        print(f"[+] ASCII: {result.decode()}", file=sys.stderr)

    if args.hex:
        print(result.hex())
    elif args.python:
        print(repr(result))
    else:
        # Write raw bytes to stdout (for piping to the challenge program)
        sys.stdout.buffer.write(result)
        sys.stdout.buffer.write(b'\n')
        sys.stdout.buffer.flush()


if __name__ == '__main__':
    main()
