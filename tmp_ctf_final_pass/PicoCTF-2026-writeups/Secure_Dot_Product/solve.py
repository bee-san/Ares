#!/usr/bin/env python3
"""
Secure Dot Product - picoCTF 2026 (Cryptography, 300 pts)

The server computes a dot product of our input vector with a secret vector
(the flag) using AES encryption. Because AES-ECB is deterministic, we can
use unit vectors to isolate each element of the secret vector.

Strategy:
  1. Send unit vectors e_i = [0, ..., 1, ..., 0] to get each secret[i].
  2. If the server returns encrypted results, build a lookup table by
     encrypting all possible byte values (0-255) and matching.
  3. If the server returns numeric results directly, just read them off.

Usage:
  python3 solve.py [HOST] [PORT]
  e.g.: python3 solve.py titan.picoctf.net 51234
"""

import sys
import json
from pwn import *

# ---- Configuration ----
HOST = sys.argv[1] if len(sys.argv) > 1 else "titan.picoctf.net"
PORT = int(sys.argv[2]) if len(sys.argv) > 2 else 51234

def connect():
    """Establish connection to the challenge server."""
    return remote(HOST, PORT)

def send_vector(io, vec):
    """
    Send a vector to the server and receive the dot product result.
    Adjust the send/receive protocol to match the actual server interface.
    """
    # Wait for prompt
    io.recvuntil(b":")  # adjust delimiter based on actual server prompt
    # Send the vector as JSON or space-separated, depending on server
    io.sendline(json.dumps(vec).encode())
    # Receive result
    result = io.recvline().strip()
    return result

def solve():
    print("[*] Secure Dot Product Solver")
    print(f"[*] Connecting to {HOST}:{PORT}")

    # Step 1: Determine vector length by connecting and reading server info
    io = connect()
    banner = io.recvuntil(b"\n", timeout=5)
    print(f"[*] Banner: {banner.decode().strip()}")

    # Try to determine the vector dimension from the server
    # The server likely tells us the expected vector size
    vec_len = None
    for line in banner.decode().split("\n"):
        for word in line.split():
            try:
                n = int(word)
                if 10 <= n <= 200:  # reasonable flag length
                    vec_len = n
            except ValueError:
                pass

    if vec_len is None:
        # Try reading more lines to find the dimension
        extra = io.recv(timeout=3)
        full_text = banner.decode() + extra.decode()
        print(f"[*] Full server text: {full_text}")
        # Default guess if we can't determine
        vec_len = 64
        print(f"[*] Using default vector length: {vec_len}")

    io.close()

    # Step 2: Use unit vectors to recover each element
    print(f"[*] Recovering secret vector of length {vec_len}")
    flag_bytes = []

    for i in range(vec_len):
        io = connect()

        # Construct unit vector: 1 at position i, 0 elsewhere
        unit_vec = [0] * vec_len
        unit_vec[i] = 1

        try:
            result = send_vector(io, unit_vec)
            # Parse the numeric result
            # The result might be a number (direct) or hex/base64 (encrypted)
            try:
                val = int(result)
                flag_bytes.append(val)
            except ValueError:
                # Might be hex-encoded encrypted result
                flag_bytes.append(result)

            if i % 10 == 0:
                print(f"[*] Progress: {i+1}/{vec_len}")
        except Exception as e:
            print(f"[!] Error at position {i}: {e}")
            flag_bytes.append(0)
        finally:
            io.close()

    # Step 3: If we got encrypted results, build lookup table
    if flag_bytes and isinstance(flag_bytes[0], bytes):
        print("[*] Results are encrypted, building lookup table...")
        lookup = {}
        io = connect()
        for byte_val in range(256):
            test_vec = [byte_val] + [0] * (vec_len - 1)
            enc_result = send_vector(io, test_vec)
            lookup[enc_result] = byte_val
        io.close()

        # Decrypt flag bytes using lookup
        decrypted = []
        for enc_byte in flag_bytes:
            if enc_byte in lookup:
                decrypted.append(lookup[enc_byte])
            else:
                decrypted.append(ord("?"))
        flag_bytes = decrypted

    # Step 4: Convert to flag string
    flag = "".join(chr(b) for b in flag_bytes if isinstance(b, int) and 0 < b < 128)
    print(f"\n[+] Recovered flag: {flag}")
    return flag


def solve_persistent_connection():
    """
    Alternative solver that uses a single persistent connection.
    Some servers allow multiple queries per connection.
    """
    print("[*] Secure Dot Product Solver (persistent connection)")
    print(f"[*] Connecting to {HOST}:{PORT}")

    io = connect()

    # Read initial server messages
    init_data = io.recv(timeout=3).decode()
    print(f"[*] Server says: {init_data}")

    # Try to parse vector length from server
    vec_len = 64  # default
    for word in init_data.split():
        try:
            n = int(word)
            if 10 <= n <= 200:
                vec_len = n
                break
        except ValueError:
            pass

    print(f"[*] Vector length: {vec_len}")

    flag_bytes = []
    for i in range(vec_len):
        unit_vec = [0] * vec_len
        unit_vec[i] = 1

        try:
            # Send vector
            io.recvuntil(b":", timeout=5)
            io.sendline(json.dumps(unit_vec).encode())
            result = io.recvline(timeout=5).strip()

            try:
                val = int(result)
                flag_bytes.append(val)
            except ValueError:
                # Try to decode hex
                try:
                    val = int(result, 16)
                    flag_bytes.append(val)
                except ValueError:
                    flag_bytes.append(0)

            if (i + 1) % 10 == 0:
                print(f"[*] Progress: {i+1}/{vec_len}")

        except Exception as e:
            print(f"[!] Error at position {i}: {e}")
            flag_bytes.append(0)

    io.close()

    # Convert to ASCII
    flag = "".join(chr(b) for b in flag_bytes if 0 < b < 128)
    print(f"\n[+] Recovered flag: {flag}")
    return flag


if __name__ == "__main__":
    context.log_level = "warn"

    print("=" * 60)
    print("  Secure Dot Product - picoCTF 2026 Solver")
    print("=" * 60)
    print()
    print(f"  Target: {HOST}:{PORT}")
    print()

    try:
        # Try persistent connection first (more efficient)
        flag = solve_persistent_connection()
    except Exception as e:
        print(f"[!] Persistent connection failed: {e}")
        print("[*] Falling back to per-query connections...")
        flag = solve()

    if "picoCTF{" in flag:
        print(f"\n{'=' * 60}")
        print(f"  FLAG: {flag}")
        print(f"{'=' * 60}")
    else:
        print(f"\n[!] Flag format not detected in output: {flag}")
        print("[!] The server protocol may need adjustment.")
        print("[!] Check the server's actual prompt format and adjust send_vector().")
