#!/usr/bin/env python3
"""
Small Trouble - picoCTF 2026
Category: Cryptography | Points: 200

Exploit: RSA with a small public exponent (likely e=3).
When e is small and the plaintext m is not much larger than n^(1/e),
the ciphertext c = m^e mod n can be reversed by iterating over
k values: m = iroot(c + k*n, e).

This script also includes fallback methods for:
  - Small prime factorization (if n has a small factor)
  - Wiener's attack (if d is small)

Usage:
    1. Replace the placeholder values for n, e, c with challenge values.
    2. Run: python3 solve.py
"""

import sys

# ============================================================
# Try to import required libraries
# ============================================================
try:
    import gmpy2
except ImportError:
    print("[!] gmpy2 not installed. Install with: pip install gmpy2")
    print("[!] Falling back to pure Python (slower)...")
    gmpy2 = None

try:
    from Crypto.Util.number import long_to_bytes, inverse
except ImportError:
    try:
        from Cryptodome.Util.number import long_to_bytes, inverse
    except ImportError:
        print("[!] pycryptodome not installed. Install with: pip install pycryptodome")
        # Minimal fallback
        def long_to_bytes(n):
            return n.to_bytes((n.bit_length() + 7) // 8, byteorder='big')
        def inverse(a, m):
            return pow(a, -1, m)


# ============================================================
# CHALLENGE PARAMETERS -- Replace with actual values!
# ============================================================
# These are placeholder values. Paste the real n, e, c from the challenge.

n = int(input("Enter n (or paste below): ")) if len(sys.argv) < 2 else int(sys.argv[1]) if len(sys.argv) >= 4 else None
e = None
c = None

# If running interactively, uncomment this block and paste values:
# n = 0x...
# e = 3
# c = 0x...

# For automated solve, you can also read from a file:
# with open("params.txt") as f:
#     exec(f.read())  # expects n=..., e=..., c=...


def read_params_interactive():
    """Read RSA parameters interactively if not set."""
    global n, e, c
    if n is None:
        print("[*] Enter RSA parameters (integers, hex with 0x prefix accepted):")
        n = int(input("n = "), 0)
    if e is None:
        e = int(input("e = "), 0)
    if c is None:
        c = int(input("c = "), 0)


def iroot(x, n_root):
    """Integer n-th root using gmpy2 or pure Python fallback."""
    if gmpy2:
        root, exact = gmpy2.iroot(x, n_root)
        return int(root), exact
    else:
        # Newton's method fallback
        if x < 0:
            return None, False
        if x == 0:
            return 0, True
        guess = int(x ** (1.0 / n_root)) + 1
        # Refine with Newton's method
        while True:
            new_guess = ((n_root - 1) * guess + x // (guess ** (n_root - 1))) // n_root
            if new_guess >= guess:
                break
            guess = new_guess
        exact = (guess ** n_root == x)
        return guess, exact


def attack_small_exponent(n, e, c, max_k=100000):
    """
    Small exponent attack (cube root attack for e=3).
    If m^e is only slightly larger than n, then c = m^e - k*n
    for some small k. We iterate over k and check for a perfect root.
    """
    print(f"[*] Attempting small exponent attack (e={e})")
    print(f"[*] Trying k = 0 to {max_k}...")

    for k in range(max_k):
        candidate = c + k * n
        root, is_perfect = iroot(candidate, e)

        if is_perfect:
            plaintext = long_to_bytes(root)
            # Sanity check: does it look like ASCII / flag?
            try:
                text = plaintext.decode("utf-8", errors="ignore")
                if "pico" in text.lower() or "ctf" in text.lower() or text.isprintable():
                    print(f"\n[+] SUCCESS at k = {k}")
                    print(f"[+] m = {root}")
                    print(f"[+] Plaintext: {plaintext}")
                    print(f"[+] Decoded: {text}")
                    return plaintext
            except Exception:
                pass

            # Even if not printable, report it
            print(f"\n[+] Perfect {e}-th root found at k = {k}")
            print(f"[+] m = {root}")
            print(f"[+] Bytes: {plaintext}")
            return plaintext

        if k % 10000 == 0 and k > 0:
            print(f"[*] Tried k = {k}...")

    print("[-] Small exponent attack failed (try increasing max_k)")
    return None


def attack_small_prime(n, e, c, limit=1000000):
    """
    Try to factor n by trial division with small primes.
    If one factor of n is small, this will find it quickly.
    """
    print(f"[*] Attempting small prime factorization (up to {limit})...")

    # Check even
    if n % 2 == 0:
        p = 2
        q = n // 2
        print(f"[+] n is even! p=2, q=n//2")
        return _decrypt_with_factors(p, q, e, c)

    # Trial division with odd numbers
    for i in range(3, limit, 2):
        if n % i == 0:
            p = i
            q = n // i
            print(f"[+] Found small factor: p = {p}")
            return _decrypt_with_factors(p, q, e, c)

    print("[-] No small prime factor found")
    return None


def attack_wiener(n, e, c):
    """
    Wiener's attack for small private exponent d.
    Uses continued fraction expansion of e/n to find d.
    """
    print("[*] Attempting Wiener's attack (small d)...")

    def continued_fraction(a, b):
        cf = []
        while b:
            cf.append(a // b)
            a, b = b, a % b
        return cf

    def convergents(cf):
        convs = []
        for i in range(len(cf)):
            if i == 0:
                num, den = cf[0], 1
            elif i == 1:
                num = cf[0] * cf[1] + 1
                den = cf[1]
            else:
                num = cf[i] * convs[-1][0] + convs[-2][0]
                den = cf[i] * convs[-1][1] + convs[-2][1]
            convs.append((num, den))
        return convs

    cf = continued_fraction(e, n)
    convs = convergents(cf)

    for k, d in convs:
        if k == 0 or d == 0:
            continue

        # Check if d is valid: phi = (e*d - 1) / k should be integer
        phi_candidate = (e * d - 1) // k
        if (e * d - 1) % k != 0:
            continue

        # phi(n) = n - p - q + 1, so p + q = n - phi + 1
        s = n - phi_candidate + 1
        # p and q are roots of: x^2 - s*x + n = 0
        discriminant = s * s - 4 * n
        if discriminant < 0:
            continue

        sqrt_disc, is_perfect = iroot(discriminant, 2)
        if is_perfect:
            p = (s + sqrt_disc) // 2
            q = (s - sqrt_disc) // 2
            if p * q == n:
                print(f"[+] Wiener's attack succeeded! d = {d}")
                m = pow(c, d, n)
                plaintext = long_to_bytes(m)
                print(f"[+] Plaintext: {plaintext}")
                return plaintext

    print("[-] Wiener's attack failed")
    return None


def _decrypt_with_factors(p, q, e, c):
    """Standard RSA decryption given p, q, e, c."""
    n = p * q
    phi = (p - 1) * (q - 1)
    d = inverse(e, phi)
    m = pow(c, d, n)
    plaintext = long_to_bytes(m)
    print(f"[+] Decrypted: {plaintext}")
    return plaintext


def main():
    global n, e, c

    print("=" * 60)
    print("  Small Trouble - picoCTF 2026 Solver")
    print("  RSA Small Exponent / Small Prime / Wiener Attack")
    print("=" * 60)
    print()

    # Read parameters
    try:
        read_params_interactive()
    except (EOFError, KeyboardInterrupt):
        print("\n[!] No input provided. Using example values for demonstration.")
        # Example values (replace with real challenge data)
        print("[!] Set n, e, c in the script or provide via stdin.")
        sys.exit(1)

    print(f"\n[*] Parameters loaded:")
    print(f"    n = {str(n)[:80]}...")
    print(f"    e = {e}")
    print(f"    c = {str(c)[:80]}...")
    print()

    # Strategy 1: Small exponent attack (most likely for this challenge)
    if e <= 17:
        result = attack_small_exponent(n, e, c)
        if result:
            print(f"\n[FLAG] {result.decode('utf-8', errors='replace')}")
            return

    # Strategy 2: Small prime factorization
    result = attack_small_prime(n, e, c)
    if result:
        print(f"\n[FLAG] {result.decode('utf-8', errors='replace')}")
        return

    # Strategy 3: Wiener's attack (small d)
    result = attack_wiener(n, e, c)
    if result:
        print(f"\n[FLAG] {result.decode('utf-8', errors='replace')}")
        return

    print("\n[-] All attacks failed. The vulnerability may require a different approach.")
    print("[-] Try checking factordb.com for n, or look for other clues in the challenge.")


if __name__ == "__main__":
    main()
