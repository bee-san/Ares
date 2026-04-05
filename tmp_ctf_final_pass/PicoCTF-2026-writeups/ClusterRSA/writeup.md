# ClusterRSA - picoCTF 2026

**Category:** Cryptography
**Points:** 400

## Challenge Description

A message has been encrypted using RSA, but this time something feels... more crowded than usual. Can you decrypt it?

## Approach

The challenge name "ClusterRSA" strongly hints at **multi-prime RSA** -- an RSA variant where the modulus `n` is the product of many small primes rather than the standard two large primes. The word "crowded" in the description reinforces this: instead of `n = p * q`, we have `n = p1 * p2 * p3 * ... * pk` for some number of primes k.

### Why Multi-Prime RSA is Vulnerable

Standard RSA security relies on the difficulty of factoring a large semiprime (product of two large primes). When the modulus is instead composed of many smaller primes, each individual prime is much smaller and easier to find. For example, a 2048-bit modulus composed of 16 primes means each prime is only ~128 bits -- well within range of modern factoring algorithms.

### Attack Strategy

1. **Factor the modulus `n`**: Since each prime factor is relatively small, we can use services like [factordb.com](http://factordb.com) or tools like `sympy.factorint()`, `yafu`, or `msieve` to factor `n`.
2. **Compute Euler's totient**: For multi-prime RSA, the totient is:
   ```
   phi(n) = (p1 - 1) * (p2 - 1) * ... * (pk - 1)
   ```
3. **Compute the private exponent**: `d = e^(-1) mod phi(n)`
4. **Decrypt**: `m = c^d mod n`

The challenge typically provides `n`, `e`, and `c` in a file (e.g., `output.txt` or `ciphertext.txt`).

## Solution

1. Download the challenge files. You should receive values for `n` (modulus), `e` (public exponent, commonly 65537), and `c` (ciphertext).
2. Factor `n` using an automated tool. Because the primes are small, factoring should complete quickly.
3. Calculate `phi(n)` as the product of `(p_i - 1)` for each prime factor.
4. Compute `d = inverse(e, phi(n))`.
5. Decrypt `m = pow(c, d, n)` and convert the resulting integer to bytes to reveal the flag.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
