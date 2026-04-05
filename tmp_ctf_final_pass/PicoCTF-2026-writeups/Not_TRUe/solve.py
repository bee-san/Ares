#!/usr/bin/env python3
"""
Not TRUe - picoCTF 2026 (Cryptography, 400 pts)

Truncation attack on a cryptographic MAC/hash.
The server uses a truncated hash to verify authenticity, making it
feasible to brute-force a valid tag.

Usage:
    python3 solve.py [HOST] [PORT]

Adjust HOST and PORT to match the challenge instance.
"""

import hashlib
import itertools
import struct
import sys
from pwn import *

# ============================================================
# Configuration - update these with the actual challenge values
# ============================================================
HOST = sys.argv[1] if len(sys.argv) > 1 else "rescued-float.picoctf.net"
PORT = int(sys.argv[2]) if len(sys.argv) > 2 else 1337

# Truncation length in bytes (common values: 2 = 16-bit, 3 = 24-bit, 4 = 32-bit)
# Adjust based on what the challenge server uses
TRUNC_BYTES = 2


def compute_truncated_hash(data: bytes, trunc_len: int = TRUNC_BYTES) -> bytes:
    """Compute SHA-256 hash and truncate to trunc_len bytes."""
    full_hash = hashlib.sha256(data).digest()
    return full_hash[:trunc_len]


def compute_truncated_hmac(key: bytes, data: bytes, trunc_len: int = TRUNC_BYTES) -> bytes:
    """Compute HMAC-SHA256 and truncate to trunc_len bytes."""
    import hmac
    full_mac = hmac.new(key, data, hashlib.sha256).digest()
    return full_mac[:trunc_len]


def brute_force_truncated_hash(target_tag: bytes, prefix: bytes = b"", trunc_len: int = TRUNC_BYTES):
    """
    Brute-force a message whose truncated SHA-256 hash matches target_tag.
    Appends different suffixes to prefix until the truncated hash matches.
    """
    log.info(f"Brute-forcing truncated hash ({trunc_len} bytes = {trunc_len*8} bits)...")
    for i in range(2 ** (trunc_len * 8 + 4)):  # Generous upper bound
        candidate = prefix + struct.pack("<I", i)
        h = hashlib.sha256(candidate).digest()[:trunc_len]
        if h == target_tag:
            log.success(f"Found collision after {i+1} attempts")
            return candidate
    return None


def brute_force_message_for_tag(target_tag_hex: str, trunc_len: int = TRUNC_BYTES):
    """
    Given a hex-encoded target tag, find a message whose truncated hash matches.
    """
    target_tag = bytes.fromhex(target_tag_hex)
    return brute_force_truncated_hash(target_tag, b"forge_", trunc_len)


def solve():
    """
    Main solve routine. Connects to the server and exploits the truncated
    hash/MAC to forge a valid authentication token.
    """
    conn = remote(HOST, PORT)

    # ============================================================
    # Phase 1: Read the server banner and understand the protocol
    # ============================================================
    banner = conn.recvuntil(b"\n", timeout=5)
    log.info(f"Banner: {banner.decode().strip()}")

    # Read all available initial data
    try:
        initial_data = conn.recvuntil(b"\n", timeout=3)
        log.info(f"Server says: {initial_data.decode().strip()}")
    except:
        pass

    # ============================================================
    # Phase 2: Interact with the server
    # The general approach for truncation attacks:
    #   - Server sends a challenge or asks for message + tag
    #   - We brute-force the truncated portion
    # ============================================================

    # Strategy A: If server asks us to provide a message with valid tag
    # We try random messages until one has the right truncated hash
    log.info("Attempting truncation brute-force attack...")

    # Generate candidate messages and try them
    for attempt in range(65536):
        # Craft a candidate message
        msg = f"admin:{attempt}".encode()
        tag = hashlib.sha256(msg).hexdigest()[:TRUNC_BYTES * 2]  # hex-encoded truncated hash

        # Send to server (adjust format based on actual protocol)
        try:
            conn.sendline(msg.hex() + ":" + tag)
        except:
            break

        response = conn.recvline(timeout=2)
        resp_str = response.decode().strip() if response else ""

        if "picoCTF{" in resp_str:
            log.success(f"Flag found: {resp_str}")
            print(resp_str)
            conn.close()
            return

        if "correct" in resp_str.lower() or "success" in resp_str.lower() or "flag" in resp_str.lower():
            log.success(f"Possible success: {resp_str}")
            # Try to receive more data that might contain the flag
            try:
                more = conn.recvall(timeout=3)
                full_response = resp_str + more.decode()
                if "picoCTF{" in full_response:
                    flag_start = full_response.index("picoCTF{")
                    flag_end = full_response.index("}", flag_start) + 1
                    flag = full_response[flag_start:flag_end]
                    log.success(f"Flag: {flag}")
                    print(flag)
            except:
                pass
            conn.close()
            return

        if attempt % 1000 == 0 and attempt > 0:
            log.info(f"Tried {attempt} candidates...")

    # ============================================================
    # Strategy B: If we need to forge a specific message
    # Server gives us a known-good (message, tag) pair and we need
    # to produce a *different* message with the same truncated tag
    # ============================================================
    log.warning("Strategy A did not work. The protocol may differ from what was assumed.")
    log.info("Please examine the server's actual protocol and adjust the script accordingly.")
    log.info("Key variables to adjust: TRUNC_BYTES, message format, and tag computation.")

    conn.close()


def offline_demo():
    """
    Demonstrate the truncation attack offline.
    Shows how easy it is to find collisions with truncated hashes.
    """
    log.info("=== Offline Truncation Attack Demo ===")

    original_msg = b"hello world"
    full_hash = hashlib.sha256(original_msg).hexdigest()
    trunc_hash = full_hash[:TRUNC_BYTES * 2]

    log.info(f"Original message: {original_msg}")
    log.info(f"Full SHA-256:     {full_hash}")
    log.info(f"Truncated ({TRUNC_BYTES} bytes): {trunc_hash}")

    # Find a collision
    target = bytes.fromhex(trunc_hash)
    collision = brute_force_truncated_hash(target, b"forged_", TRUNC_BYTES)

    if collision:
        collision_hash = hashlib.sha256(collision).hexdigest()[:TRUNC_BYTES * 2]
        log.success(f"Forged message:   {collision}")
        log.success(f"Truncated hash:   {collision_hash}")
        log.success(f"Match: {trunc_hash == collision_hash}")


if __name__ == "__main__":
    if "--demo" in sys.argv:
        offline_demo()
    else:
        solve()
