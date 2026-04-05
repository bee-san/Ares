#!/usr/bin/env python3
"""
ClusterRSA - picoCTF 2026 (Cryptography, 400 pts)

Multi-prime RSA challenge. The modulus n is composed of many small primes,
making it easy to factor. Once factored, we compute phi(n) and decrypt.

Usage:
    python3 solve.py
    python3 solve.py --file output.txt   # if challenge data is in a file

Dependencies: sympy (pip install sympy)
"""

import sys
import re
from sympy import factorint
from functools import reduce

# ============================================================
# CHALLENGE DATA - Paste your values here from the challenge
# ============================================================
# These are placeholders. Replace with actual challenge values.
n = None
e = None
c = None

def load_from_file(filepath):
    """Parse n, e, c from a challenge output file."""
    global n, e, c
    with open(filepath, 'r') as f:
        content = f.read()

    # Try common formats: "n = ...", "n: ...", "N = ...", etc.
    patterns = {
        'n': r'[nN]\s*[=:]\s*(\d+)',
        'e': r'[eE]\s*[=:]\s*(\d+)',
        'c': r'[cC](?:iphertext)?\s*[=:]\s*(\d+)',
    }
    for var, pattern in patterns.items():
        match = re.search(pattern, content)
        if match:
            globals()[var] = int(match.group(1))

    # Also try to parse Python-style assignments
    if n is None:
        try:
            exec_globals = {}
            exec(content, exec_globals)
            n = exec_globals.get('n', exec_globals.get('N', n))
            e = exec_globals.get('e', exec_globals.get('E', e))
            c = exec_globals.get('c', exec_globals.get('C', exec_globals.get('ct', exec_globals.get('ciphertext', c))))
        except Exception:
            pass


def int_to_bytes(n):
    """Convert a large integer to bytes."""
    byte_length = (n.bit_length() + 7) // 8
    return n.to_bytes(byte_length, byteorder='big')


def solve():
    global n, e, c

    # Load from file if specified
    if len(sys.argv) > 2 and sys.argv[1] == '--file':
        load_from_file(sys.argv[2])
    elif len(sys.argv) > 1 and not sys.argv[1].startswith('-'):
        load_from_file(sys.argv[1])

    # Validate that we have all required values
    if n is None or e is None or c is None:
        # Try loading from common filenames in current directory
        import os
        for fname in ['output.txt', 'ciphertext.txt', 'data.txt', 'challenge.txt', 'flag.enc']:
            if os.path.exists(fname):
                load_from_file(fname)
                break

    if n is None or e is None or c is None:
        print("[!] Missing challenge values. Please either:")
        print("    1. Edit this script and set n, e, c directly")
        print("    2. Run: python3 solve.py output.txt")
        print("    3. Place challenge data in output.txt in the same directory")
        sys.exit(1)

    print(f"[*] n = {str(n)[:80]}... ({n.bit_length()} bits)")
    print(f"[*] e = {e}")
    print(f"[*] c = {str(c)[:80]}...")
    print()

    # Step 1: Factor n using sympy
    print("[*] Factoring n (multi-prime RSA -- expecting many small factors)...")
    factors = factorint(n)
    print(f"[+] Found {len(factors)} prime factors!")

    for p, exp in sorted(factors.items()):
        print(f"    p = {str(p)[:60]}{'...' if len(str(p)) > 60 else ''} (exp={exp}, {p.bit_length()} bits)")

    # Step 2: Compute Euler's totient phi(n)
    # For n = p1^a1 * p2^a2 * ... * pk^ak:
    # phi(n) = n * product((1 - 1/pi) for each prime pi)
    # Which simplifies to: product(pi^(ai-1) * (pi - 1))
    phi_n = 1
    for p, exp in factors.items():
        phi_n *= (p ** (exp - 1)) * (p - 1)

    print(f"\n[*] phi(n) computed ({phi_n.bit_length()} bits)")

    # Step 3: Compute private exponent d = e^(-1) mod phi(n)
    d = pow(e, -1, phi_n)
    print(f"[*] Private exponent d computed ({d.bit_length()} bits)")

    # Step 4: Decrypt: m = c^d mod n
    m = pow(c, d, n)
    print(f"[*] Decrypted message integer: {str(m)[:80]}...")

    # Step 5: Convert to bytes and extract flag
    plaintext = int_to_bytes(m)
    print(f"\n[+] Decrypted plaintext (raw bytes): {plaintext}")

    # Try to decode as UTF-8
    try:
        flag = plaintext.decode('utf-8').strip('\x00')
        print(f"\n[+] FLAG: {flag}")
    except UnicodeDecodeError:
        # Strip leading null bytes and try again
        plaintext = plaintext.lstrip(b'\x00')
        try:
            flag = plaintext.decode('utf-8')
            print(f"\n[+] FLAG: {flag}")
        except UnicodeDecodeError:
            print(f"\n[+] Raw plaintext (hex): {plaintext.hex()}")
            # Try to find picoCTF flag in the raw bytes
            if b'picoCTF' in plaintext:
                start = plaintext.index(b'picoCTF')
                end = plaintext.index(b'}', start) + 1
                print(f"[+] FLAG: {plaintext[start:end].decode()}")


if __name__ == '__main__':
    solve()
