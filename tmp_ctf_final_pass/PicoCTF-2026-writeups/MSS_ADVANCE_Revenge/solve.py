#!/usr/bin/env python3
"""
MSS_ADVANCE Revenge - picoCTF 2026
Category: Cryptography (400 points)

Exploit: The Mignotte Secret Sharing scheme evaluates a polynomial over
the integers (not a finite field). By evaluating at prime x-values, we
can recover the secret key modulo each prime, then reconstruct the full
key via the Chinese Remainder Theorem (CRT).

Usage:
    python3 solve.py
    python3 solve.py --host <HOST> --port <PORT>
"""

import json
import argparse
from hashlib import sha256

# Try importing pwntools for remote interaction; fall back to socket
try:
    from pwn import remote, log
    HAS_PWNTOOLS = True
except ImportError:
    HAS_PWNTOOLS = False
    import socket

# Try importing sympy for CRT; fall back to manual implementation
try:
    from sympy.ntheory.modular import crt as sympy_crt
    HAS_SYMPY = True
except ImportError:
    HAS_SYMPY = False

# Try importing pycryptodome for AES decryption
try:
    from Crypto.Cipher import AES
    from Crypto.Util.Padding import unpad
    HAS_CRYPTO = True
except ImportError:
    HAS_CRYPTO = False


# --- Math Utilities ---

def is_prime(n):
    """Simple primality test."""
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


def get_primes(count, bitsize=15):
    """Generate `count` distinct primes of approximately `bitsize` bits."""
    primes = []
    # Start from a value near the middle of the bit range to get primes spread out
    candidate = (1 << (bitsize - 1)) + 1
    while len(primes) < count:
        if is_prime(candidate):
            primes.append(candidate)
        candidate += 2
    return primes


def extended_gcd(a, b):
    """Extended Euclidean Algorithm. Returns (gcd, x, y) such that a*x + b*y = gcd."""
    if a == 0:
        return b, 0, 1
    gcd, x1, y1 = extended_gcd(b % a, a)
    return gcd, y1 - (b // a) * x1, x1


def crt(moduli, remainders):
    """
    Chinese Remainder Theorem.
    Given pairwise coprime moduli and corresponding remainders,
    returns the unique solution x such that x ≡ r_i (mod m_i) for all i,
    and x is in the range [0, product of all moduli).
    """
    if HAS_SYMPY:
        result = sympy_crt(moduli, remainders)
        if result is None:
            raise ValueError("CRT failed - moduli may not be pairwise coprime")
        return result  # Returns (solution, product_of_moduli)

    # Manual CRT implementation
    M = 1
    for m in moduli:
        M *= m

    x = 0
    for m_i, r_i in zip(moduli, remainders):
        M_i = M // m_i
        _, inv, _ = extended_gcd(M_i, m_i)
        inv = inv % m_i
        x += r_i * M_i * inv

    return x % M, M


# --- Communication Utilities ---

class Connection:
    """Wrapper for server communication, supports pwntools or raw sockets."""

    def __init__(self, host, port):
        self.host = host
        self.port = port
        if HAS_PWNTOOLS:
            self.conn = remote(host, port)
        else:
            self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.sock.connect((host, port))
            self.buffer = b""

    def recvuntil(self, delim):
        if HAS_PWNTOOLS:
            return self.conn.recvuntil(delim)
        if isinstance(delim, str):
            delim = delim.encode()
        while delim not in self.buffer:
            data = self.sock.recv(4096)
            if not data:
                break
            self.buffer += data
        idx = self.buffer.find(delim)
        result = self.buffer[:idx + len(delim)]
        self.buffer = self.buffer[idx + len(delim):]
        return result

    def recvline(self):
        return self.recvuntil(b"\n")

    def sendline(self, data):
        if isinstance(data, str):
            data = data.encode()
        if HAS_PWNTOOLS:
            self.conn.sendline(data)
        else:
            self.sock.sendall(data + b"\n")

    def recv(self, n=4096):
        if HAS_PWNTOOLS:
            return self.conn.recv(n)
        if self.buffer:
            result = self.buffer[:n]
            self.buffer = self.buffer[n:]
            return result
        return self.sock.recv(n)

    def close(self):
        if HAS_PWNTOOLS:
            self.conn.close()
        else:
            self.sock.close()


def send_command(conn, command_dict):
    """Send a JSON command to the server and receive the JSON response."""
    conn.sendline(json.dumps(command_dict))
    # Read response - may need to handle different server formats
    response_line = conn.recvline().decode().strip()
    # Try to parse JSON from the response
    try:
        return json.loads(response_line)
    except json.JSONDecodeError:
        # Server might include prefix text; try to find JSON in the line
        for i, ch in enumerate(response_line):
            if ch == '{':
                try:
                    return json.loads(response_line[i:])
                except json.JSONDecodeError:
                    continue
        return {"raw": response_line}


def solve(host, port, num_shares=19, prime_bits=15, key_bits=256):
    """
    Main solve routine:
    1. Connect to the server
    2. Request shares at prime x-values
    3. Apply CRT to recover the secret key
    4. Decrypt the flag
    """
    print(f"[*] Connecting to {host}:{port}...")
    conn = Connection(host, port)

    # Absorb any initial banner/prompt
    try:
        banner = conn.recvuntil(b"query")
        print(f"[*] Banner received")
    except Exception:
        pass

    # Step 1: Generate prime x-values
    primes = get_primes(num_shares, prime_bits)
    product = 1
    for p in primes:
        product *= p
    print(f"[*] Generated {num_shares} primes (~{prime_bits} bits each)")
    print(f"[*] Product bit-length: {product.bit_length()} (need > {key_bits})")

    if product.bit_length() <= key_bits:
        print(f"[!] Warning: product of primes may be too small. Try more primes.")

    # Step 2: Collect shares (polynomial evaluations at each prime)
    remainders = []
    for i, p in enumerate(primes):
        try:
            # Try JSON-based protocol first
            resp = send_command(conn, {"command": "get_share", "x": p})
            y = resp.get("y") or resp.get("share") or resp.get("result")
            if y is None:
                # Might be a different protocol format
                print(f"[!] Unexpected response format: {resp}")
                continue
            y = int(y)
        except Exception as e:
            print(f"[!] Error getting share {i}: {e}")
            continue

        # The key is the constant term: P(p) mod p = key mod p
        r = y % p
        remainders.append(r)
        print(f"[+] Share {i+1}/{num_shares}: P({p}) mod {p} = {r}")

    if len(remainders) < num_shares:
        print(f"[!] Only got {len(remainders)}/{num_shares} shares")

    # Step 3: Apply CRT to recover the key
    print(f"[*] Applying Chinese Remainder Theorem...")
    key, _ = crt(primes[:len(remainders)], remainders)
    print(f"[+] Recovered key: {key}")
    print(f"[+] Key bit-length: {key.bit_length()}")

    # Step 4: Request encrypted flag
    try:
        resp = send_command(conn, {"command": "encrypt_flag"})
        print(f"[*] Encrypted flag response: {resp}")
    except Exception as e:
        print(f"[!] Error getting encrypted flag: {e}")
        resp = {}

    # Step 5: Decrypt the flag
    iv_hex = resp.get("iv")
    enc_flag_hex = resp.get("enc_flag") or resp.get("ciphertext") or resp.get("ct")

    if iv_hex and enc_flag_hex:
        iv = bytes.fromhex(iv_hex)
        enc_flag = bytes.fromhex(enc_flag_hex)

        # The key is typically SHA-256 hashed before use as AES key
        key_bytes = sha256(str(key).encode()).digest()

        if HAS_CRYPTO:
            cipher = AES.new(key_bytes, AES.MODE_CBC, iv)
            try:
                flag = unpad(cipher.decrypt(enc_flag), AES.block_size).decode()
                print(f"\n[+] FLAG: {flag}")
            except Exception as e:
                # Try without unpadding
                flag = cipher.decrypt(enc_flag)
                print(f"\n[+] FLAG (raw): {flag}")
        else:
            print("[!] pycryptodome not installed. Install with: pip install pycryptodome")
            print(f"[*] Key (hex): {key_bytes.hex()}")
            print(f"[*] IV (hex): {iv_hex}")
            print(f"[*] Ciphertext (hex): {enc_flag_hex}")
    else:
        # The flag might be returned directly after key verification
        flag_data = resp.get("flag") or resp.get("message") or resp.get("raw", "")
        if flag_data:
            print(f"\n[+] FLAG: {flag_data}")
        else:
            print("[*] Could not automatically extract flag.")
            print(f"[*] Recovered key value: {key}")
            print("[*] Try using this key to manually decrypt the flag.")

    conn.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="MSS_ADVANCE Revenge Solver - picoCTF 2026")
    parser.add_argument("--host", default="rescued-float.picoctf.net",
                        help="Challenge server hostname")
    parser.add_argument("--port", type=int, default=61898,
                        help="Challenge server port")
    parser.add_argument("--shares", type=int, default=19,
                        help="Number of shares to request (default: 19)")
    parser.add_argument("--prime-bits", type=int, default=15,
                        help="Bit size of primes to use (default: 15)")
    parser.add_argument("--key-bits", type=int, default=256,
                        help="Expected key bit size (default: 256)")
    args = parser.parse_args()

    solve(args.host, args.port, args.shares, args.prime_bits, args.key_bits)
