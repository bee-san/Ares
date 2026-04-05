# Related Messages - picoCTF 2026

**Category:** Cryptography
**Points:** 200

## Challenge Description

Oops! I have a typo in my first message so I sent it again! I used RSA twice so this is secure right?

## Approach

### Understanding the Vulnerability

This challenge is a textbook **Franklin-Reiter Related Message Attack** on RSA. The scenario:

1. A sender encrypts a message `m1` with RSA public key `(n, e)` to get ciphertext `c1`.
2. The sender realizes there is a typo, so they "fix" it and send a corrected message `m2`, encrypted with the **same** public key to get `c2`.
3. The two messages are related by a known linear function: `m2 = a * m1 + b` (e.g., a single character difference amounts to `m2 = m1 + b` for some known `b`).

The sender assumes encrypting twice with RSA is safe, but when two plaintexts have a known polynomial relationship and share the same modulus and exponent, the Franklin-Reiter attack can recover both messages.

### Mathematical Foundation

Given:
- Public key `(n, e)` (typically `e = 65537`, but the attack is most efficient with small `e` like `e = 3`)
- `c1 = m1^e mod n`
- `c2 = m2^e mod n`
- Known relationship: `m2 = f(m1)` where `f(x) = a*x + b`

We construct two polynomials in the ring `Z_n[x]`:
- `g1(x) = x^e - c1`  (has root `m1`)
- `g2(x) = f(x)^e - c2 = (a*x + b)^e - c2`  (also has root `m1`)

Since `m1` is a common root, `(x - m1)` divides both polynomials. Computing `gcd(g1, g2)` in the polynomial ring modulo `n` yields a linear polynomial `x - m1`, from which we recover `m1`.

### Why This Works

- For small `e` (especially `e = 3`), the polynomial GCD computation is efficient and direct.
- For larger `e`, the attack still works in principle but may require more sophisticated polynomial GCD algorithms.
- The attack requires no factoring of `n` -- it operates entirely in the polynomial ring `Z_n[x]`.

### Challenge Files (Typical)

The challenge typically provides:
- `output.txt` containing `n`, `e`, `c1`, `c2`, and the relationship parameters `a` and `b` (or these can be inferred from the "typo" description)
- Sometimes a Python script showing how the encryption was performed

## Solution

### Step 1: Read the Challenge Data

Parse the provided values: `n`, `e`, `c1`, `c2`, and determine the relationship `f(x) = a*x + b`.

In the "typo" scenario, the relationship is often:
- `m2 = m1 + b` where `b` is the difference caused by the typo (e.g., a small integer offset), meaning `a = 1`.

### Step 2: Construct the Polynomials

In the polynomial ring `Z_n[x]`:
```
g1(x) = x^e - c1
g2(x) = (a*x + b)^e - c2
```

### Step 3: Compute the GCD

Using SageMath or Python (sympy), compute:
```
result = gcd(g1, g2)
```

The result should be a linear polynomial `x - m1` (or a scalar multiple thereof).

### Step 4: Extract the Message

From the GCD result, extract `m1 = -result.coefficients[-1]` (the negation of the constant term of the monic GCD). Then convert the integer back to bytes to reveal the flag.

### Step 5: Recover the Flag

```python
from Crypto.Util.number import long_to_bytes
flag = long_to_bytes(m1)
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
