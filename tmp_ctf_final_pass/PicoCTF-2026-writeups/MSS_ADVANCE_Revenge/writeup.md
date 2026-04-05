# MSS_ADVANCE Revenge - picoCTF 2026

**Category:** Cryptography
**Points:** 400

## Challenge Description

Last time we went easy on you. You'll never get the flag this time! (Mignotte Secret Sharing scheme)

## Approach

This challenge is based on the **Mignotte Secret Sharing Scheme (MSS)**, a threshold secret sharing mechanism built on the **Chinese Remainder Theorem (CRT)**. It is a follow-up ("Revenge") to an earlier MSS challenge, implying tighter constraints.

### Background: Mignotte Secret Sharing

In Mignotte's scheme, a secret `S` is shared among `n` parties using a specially chosen sequence of pairwise coprime integers `m_1 < m_2 < ... < m_n`. A `(k, n)`-threshold scheme means any `k` shares can reconstruct `S`, but `k-1` shares cannot. The secret is encoded as residues: each share `s_i = S mod m_i`.

To reconstruct the secret, one applies CRT to any `k` shares to recover `S` uniquely, provided the product of any `k` moduli exceeds the secret's value.

### The Vulnerability

The challenge likely implements a polynomial-based variant where:

1. A secret key is embedded as the constant term (or linear coefficient) of a polynomial `P(x)` of degree `d`.
2. Users can request evaluations `P(x_i)` for chosen `x` values (shares).
3. The number of shares provided is fewer than `d+1`, so standard polynomial interpolation (Lagrange) is impossible.

However, the polynomial is evaluated over the **integers** (not over a finite field). This is the critical flaw. If we choose **prime numbers** as our `x` values, then:

- `P(x_i) mod x_i` gives us the constant term (the secret key) modulo `x_i`
- This is because all higher-degree terms `a_j * x_i^j` vanish modulo `x_i`

By collecting enough such residues modulo distinct primes, we can apply CRT to recover the secret key, as long as the product of our chosen primes exceeds the key's value (e.g., 256-bit key).

### "Revenge" Hardening

The "Revenge" variant likely adds constraints such as:
- Smaller allowed range for `x` values
- Fewer allowed queries
- Larger polynomial degree
- Additional noise or obfuscation

The core attack remains the same: evaluate at primes, reduce modulo the prime, apply CRT. We just need to be more careful with prime selection and ensure the product of primes exceeds the secret size.

## Solution

### Step-by-step:

1. **Connect** to the challenge server (typically via netcat or pwntools).
2. **Request shares** at carefully chosen prime `x` values within the allowed range.
3. **Compute residues**: For each share `y_i = P(p_i)`, compute `r_i = y_i mod p_i`.
4. **Apply CRT**: Use the Chinese Remainder Theorem on the system `{ S ≡ r_i (mod p_i) }` to recover the secret key `S`.
5. **Decrypt the flag**: Use the recovered key (typically SHA-256 hashed) as an AES key to decrypt the encrypted flag.
6. **Request the encrypted flag** from the server and decrypt it.

### Key Math:

Given polynomial `P(x) = a_d*x^d + ... + a_1*x + key`:
- `P(p) mod p = key mod p` (since all `a_j * p^j ≡ 0 mod p` for j >= 1)
- Collect `(p_i, key mod p_i)` pairs
- CRT({key mod p_i}, {p_i}) --> unique key if `∏ p_i > key`

For a 256-bit key, we need primes whose product exceeds `2^256`. Using ~19 primes of ~15 bits each gives `19 * 15 = 285 > 256` bits of product.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
