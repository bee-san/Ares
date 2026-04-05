#!/usr/bin/env python3
"""
Related Messages - picoCTF 2026
Category: Cryptography (200 pts)

Franklin-Reiter Related Message Attack on RSA.
Two messages with a known linear relationship encrypted under the same RSA key.

The sender had a typo in message m1, so they sent a corrected m2.
The relationship is m2 = a*m1 + b (mod n), with a and b known.

Usage:
    python3 solve.py                     # reads from output.txt in current directory
    python3 solve.py --file output.txt   # specify the file
    python3 solve.py --n N --e E --c1 C1 --c2 C2 --a A --b B  # manual values

Dependencies:
    pip install pycryptodome sympy
"""

import argparse
import re
import sys
from sympy import Poly, symbols, gcd, GF, ZZ, invert
from Crypto.Util.number import long_to_bytes


def parse_args():
    parser = argparse.ArgumentParser(description="Related Messages - Franklin-Reiter Attack")
    parser.add_argument("--file", default="output.txt", help="File containing challenge output")
    parser.add_argument("--n", type=int, default=None, help="RSA modulus N")
    parser.add_argument("--e", type=int, default=None, help="RSA public exponent e")
    parser.add_argument("--c1", type=int, default=None, help="Ciphertext 1")
    parser.add_argument("--c2", type=int, default=None, help="Ciphertext 2")
    parser.add_argument("--a", type=int, default=1, help="Linear coefficient a in m2 = a*m1 + b (default: 1)")
    parser.add_argument("--b", type=int, default=None, help="Constant b in m2 = a*m1 + b")
    return parser.parse_args()


def parse_output_file(filepath):
    """Parse the challenge output file to extract n, e, c1, c2, a, b."""
    values = {}
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        return None

    # Try various common output formats
    patterns = {
        'n': [r'[nN]\s*[=:]\s*(\d+)', r'modulus\s*[=:]\s*(\d+)'],
        'e': [r'[eE]\s*[=:]\s*(\d+)', r'exponent\s*[=:]\s*(\d+)'],
        'c1': [r'c1\s*[=:]\s*(\d+)', r'ct1\s*[=:]\s*(\d+)', r'ciphertext1\s*[=:]\s*(\d+)'],
        'c2': [r'c2\s*[=:]\s*(\d+)', r'ct2\s*[=:]\s*(\d+)', r'ciphertext2\s*[=:]\s*(\d+)'],
        'a': [r'\ba\s*[=:]\s*(\d+)'],
        'b': [r'\bb\s*[=:]\s*(\d+)', r'diff(?:erence)?\s*[=:]\s*(\d+)'],
    }

    for key, pats in patterns.items():
        for pat in pats:
            m = re.search(pat, content)
            if m:
                values[key] = int(m.group(1))
                break

    return values


def franklin_reiter_related_message(n, e, c1, c2, a, b):
    """
    Franklin-Reiter Related Message Attack.

    Given:
        c1 = m1^e mod n
        c2 = (a*m1 + b)^e mod n

    Recover m1.

    Uses polynomial GCD in Z_n[x]:
        g1(x) = x^e - c1
        g2(x) = (a*x + b)^e - c2
        gcd(g1, g2) should yield (x - m1)
    """
    x = symbols('x')

    print(f"[*] Setting up polynomials in Z_{n}[x]...")
    print(f"[*] e = {e}")

    # For small e, use sympy's polynomial GCD directly
    if e <= 65537:
        print(f"[*] Using sympy polynomial GCD method...")
        g1 = Poly(x ** e - c1, x).set_modulus(n)
        g2 = Poly((a * x + b) ** e - c2, x).set_modulus(n)

        print("[*] Computing GCD of g1(x) and g2(x)...")
        result = gcd(g1, g2)
        print(f"[*] GCD degree: {result.degree()}")

        coeffs = result.all_coeffs()
        if len(coeffs) == 2:
            # Linear polynomial: c0*x + c1 => root is -c1/c0 mod n
            # For monic polynomial (c0=1): root is -c1 mod n
            c0, c1_coeff = coeffs
            if c0 != 1:
                c0_inv = int(invert(c0, n))
                m1 = (-c1_coeff * c0_inv) % n
            else:
                m1 = (-c1_coeff) % n
            return int(m1)
        else:
            print(f"[!] GCD is not linear (degree {result.degree()}). Attack may have failed.")
            print(f"[!] GCD coefficients: {coeffs}")
            return None
    else:
        print("[!] e is large; sympy may be slow. Consider using SageMath.")
        # Attempt anyway
        g1 = Poly(x ** e - c1, x).set_modulus(n)
        g2 = Poly((a * x + b) ** e - c2, x).set_modulus(n)
        result = gcd(g1, g2)
        coeffs = result.all_coeffs()
        if len(coeffs) == 2:
            c0, c1_coeff = coeffs
            if c0 != 1:
                c0_inv = int(invert(c0, n))
                m1 = (-c1_coeff * c0_inv) % n
            else:
                m1 = (-c1_coeff) % n
            return int(m1)
        return None


def franklin_reiter_e3(n, c1, c2, a, b):
    """
    Optimized version for e=3 using direct algebraic formula.

    When e=3 and f(x) = ax + b:
        g1 = x^3 - c1
        g2 = (ax+b)^3 - c2

    The GCD can be computed via the extended Euclidean algorithm for polynomials.
    For e=3 with a=1, this simplifies considerably.
    """
    # For e=3, a=1: We can use a direct approach
    # g1(x) = x^3 - c1
    # g2(x) = (x+b)^3 - c2 = x^3 + 3bx^2 + 3b^2*x + b^3 - c2
    #
    # g2 - g1 = 3bx^2 + 3b^2*x + (b^3 - c2 + c1)
    # Continue with polynomial GCD steps...

    # Use the general method - it handles e=3 very efficiently
    return franklin_reiter_related_message(n, 3, c1, c2, a, b)


def main():
    args = parse_args()

    print("=" * 60)
    print("  Related Messages - Franklin-Reiter Attack")
    print("  picoCTF 2026 - Cryptography (200 pts)")
    print("=" * 60)
    print()

    # Load values from arguments or file
    n = args.n
    e = args.e
    c1 = args.c1
    c2 = args.c2
    a = args.a
    b = args.b

    if any(v is None for v in [n, e, c1, c2]):
        print(f"[*] Loading values from {args.file}...")
        file_values = parse_output_file(args.file)
        if file_values is None:
            print(f"[!] Could not read {args.file}")
            print("[!] Provide values via command line: --n N --e E --c1 C1 --c2 C2 --b B")
            sys.exit(1)

        n = n or file_values.get('n')
        e = e or file_values.get('e')
        c1 = c1 or file_values.get('c1')
        c2 = c2 or file_values.get('c2')
        a = a if args.a != 1 else file_values.get('a', 1)
        b = b or file_values.get('b')

    # Validate
    if any(v is None for v in [n, e, c1, c2, b]):
        print("[!] Missing required values. Need: n, e, c1, c2, b")
        print(f"[!] Got: n={'set' if n else 'MISSING'}, e={'set' if e else 'MISSING'}, "
              f"c1={'set' if c1 else 'MISSING'}, c2={'set' if c2 else 'MISSING'}, "
              f"b={'set' if b else 'MISSING'}")
        sys.exit(1)

    print(f"[*] n = {str(n)[:60]}... ({n.bit_length()} bits)")
    print(f"[*] e = {e}")
    print(f"[*] c1 = {str(c1)[:60]}...")
    print(f"[*] c2 = {str(c2)[:60]}...")
    print(f"[*] Relationship: m2 = {a}*m1 + {b}")
    print()

    # Run the attack
    print("[*] Running Franklin-Reiter Related Message Attack...")
    m1 = franklin_reiter_related_message(n, e, c1, c2, a, b)

    if m1 is None:
        print("[!] Attack failed. The relationship may be incorrect or e may be too large.")
        sys.exit(1)

    # Convert to bytes
    plaintext = long_to_bytes(m1)
    print()
    print(f"[+] Recovered m1 (int): {m1}")
    print(f"[+] Recovered m1 (bytes): {plaintext}")
    print()

    # Try to extract the flag
    try:
        text = plaintext.decode('utf-8', errors='ignore')
        print(f"[+] Decoded message: {text}")

        # Search for the flag pattern
        import re
        flag_match = re.search(r'picoCTF\{[^}]+\}', text)
        if flag_match:
            print()
            print("=" * 60)
            print(f"[+] FLAG: {flag_match.group()}")
            print("=" * 60)
        else:
            print("[*] No picoCTF{...} pattern found in message.")
            print("[*] The flag may be in m2 instead. Computing m2 = a*m1 + b...")
            m2 = (a * m1 + b) % n
            plaintext2 = long_to_bytes(m2)
            text2 = plaintext2.decode('utf-8', errors='ignore')
            print(f"[+] Decoded m2: {text2}")
            flag_match2 = re.search(r'picoCTF\{[^}]+\}', text2)
            if flag_match2:
                print()
                print("=" * 60)
                print(f"[+] FLAG: {flag_match2.group()}")
                print("=" * 60)
    except Exception as ex:
        print(f"[!] Could not decode plaintext: {ex}")
        print(f"[*] Raw bytes: {plaintext.hex()}")


if __name__ == "__main__":
    main()
