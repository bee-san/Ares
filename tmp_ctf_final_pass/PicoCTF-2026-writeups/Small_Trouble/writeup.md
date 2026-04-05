# Small Trouble - picoCTF 2026

**Category:** Cryptography
**Points:** 200

## Challenge Description

Everything seems secure; strong numbers, familiar parameters but something small might ruin it all. Can you recover the secret?

We are given RSA parameters (n, e, c) where something "small" introduces a critical vulnerability. The challenge hints at either a small public exponent (`e`), a small prime factor, or a small private exponent (`d`).

## Approach

### Identifying the Vulnerability

The title "Small Trouble" and the description "something small might ruin it all" point to one of several classic RSA weaknesses related to small values:

1. **Small public exponent (e=3) with small message** -- If `m^e < n`, the ciphertext `c = m^e` and we can recover `m` by simply computing the integer e-th root of `c`. Even if `m^e` is slightly larger than `n`, we can iterate: `m = iroot(c + k*n, e)` for small values of `k`.

2. **Small prime factor** -- If one of the prime factors of `n` is small enough to be found by trial division or a factoring database (like factordb.com), we can factor `n` trivially.

3. **Small private exponent (Wiener's Attack)** -- If `d` is too small relative to `n`, the continued fraction expansion of `e/n` reveals `d`.

### Most Likely Scenario: Small Exponent (e=3) Cube Root Attack

Given the challenge's 200-point value, high solve count (1270), and the phrasing "familiar parameters," this is most likely an RSA challenge with `e=3` where the plaintext is small enough that `m^3` barely exceeds `n`. This is the classic "Mini RSA" / cube root attack.

**The math:**

- RSA encryption: `c = m^e mod n`
- If `e = 3` and `m` is small, then `m^3 = c + k*n` for some small integer `k`
- We iterate over `k = 0, 1, 2, ...` and check if `c + k*n` is a perfect cube
- When we find a perfect cube, `m = iroot(c + k*n, 3)`

### Alternative: Small Prime Factor

If the challenge provides a large `n` but one prime is suspiciously small:
- Try factoring with small primes or use factordb.com
- Once we have `p` and `q`, compute `phi(n) = (p-1)(q-1)`, then `d = inverse(e, phi(n))`, then `m = pow(c, d, n)`

## Solution

### Step-by-step:

1. **Extract the parameters** from the challenge files (n, e, c).
2. **Check the value of e** -- if `e = 3` (or another small value), try the cube root attack.
3. **Iterate** over `k = 0, 1, 2, ...`, computing `iroot(c + k*n, e)` until we find a perfect e-th root.
4. **Convert** the resulting integer `m` to bytes to get the flag.

### Cube Root Attack (e=3):

```python
import gmpy2
from Crypto.Util.number import long_to_bytes

# Given values from the challenge
n = ...  # RSA modulus
e = 3    # Small public exponent
c = ...  # Ciphertext

# Iterate: try c + k*n for k = 0, 1, 2, ...
for k in range(100000):
    candidate = c + k * n
    root, is_perfect = gmpy2.iroot(candidate, e)
    if is_perfect:
        m = int(root)
        plaintext = long_to_bytes(m)
        print(f"Found at k={k}: {plaintext}")
        break
```

### Small Prime Factorization (alternative):

```python
from Crypto.Util.number import long_to_bytes
from sympy import factorint

# If n has a small factor
factors = factorint(n)  # or use factordb
p, q = list(factors.keys())
phi = (p - 1) * (q - 1)
d = pow(e, -1, phi)
m = pow(c, d, n)
print(long_to_bytes(m))
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
