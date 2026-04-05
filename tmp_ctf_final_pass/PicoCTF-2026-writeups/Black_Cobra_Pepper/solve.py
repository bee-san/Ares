#!/usr/bin/env python3
"""
Black Cobra Pepper - picoCTF 2026
Category: Cryptography (200 points)

Description: "i like peppers. (change!)"

This challenge uses a pepper (secret short key) combined with data before
hashing or encrypting. The pepper is short enough to brute-force.

This script covers the most common pepper-based CTF patterns:
1. Hash pepper: H(pepper || message) or H(message || pepper)
2. XOR pepper: ciphertext = plaintext XOR repeated_pepper
3. HMAC pepper: HMAC(pepper, message)

Usage:
    python3 solve.py
    python3 solve.py --host <HOST> --port <PORT>
    python3 solve.py --file <challenge_file>
"""

import hashlib
import hmac
import itertools
import string
import argparse
import json
import sys

try:
    from pwn import remote, log
    HAS_PWNTOOLS = True
except ImportError:
    HAS_PWNTOOLS = False
    import socket


# --- Utility Functions ---

def xor_bytes(data, key):
    """XOR data with a repeating key."""
    key_len = len(key)
    return bytes(d ^ key[i % key_len] for i, d in enumerate(data))


def brute_force_xor_pepper(ciphertext, pepper_len=1, max_pepper_len=4):
    """
    Brute-force XOR pepper. Try all pepper lengths from pepper_len to max_pepper_len.
    Returns the pepper and plaintext if flag format is found.
    """
    for plen in range(pepper_len, max_pepper_len + 1):
        print(f"[*] Trying XOR pepper length: {plen}")
        for pepper in itertools.product(range(256), repeat=plen):
            pepper_bytes = bytes(pepper)
            plaintext = xor_bytes(ciphertext, pepper_bytes)
            try:
                decoded = plaintext.decode('ascii', errors='strict')
                if 'picoCTF{' in decoded:
                    return pepper_bytes, decoded
            except (UnicodeDecodeError, ValueError):
                continue
    return None, None


def brute_force_hash_pepper(target_hash, known_data, hash_func='sha256',
                            pepper_len=1, max_pepper_len=4, prepend=True,
                            charset=None):
    """
    Brute-force hash pepper. Tries pepper || data and data || pepper.
    Returns the pepper if a match is found.
    """
    if charset is None:
        charset = list(range(256))

    hash_functions = {
        'md5': hashlib.md5,
        'sha1': hashlib.sha1,
        'sha256': hashlib.sha256,
        'sha512': hashlib.sha512,
    }

    h_func = hash_functions.get(hash_func, hashlib.sha256)

    if isinstance(known_data, str):
        known_data = known_data.encode()
    if isinstance(target_hash, str):
        target_hash = target_hash.lower()

    for plen in range(pepper_len, max_pepper_len + 1):
        print(f"[*] Trying hash pepper length: {plen} ({'prepend' if prepend else 'append'}) with {hash_func}")
        for pepper_tuple in itertools.product(charset, repeat=plen):
            pepper_bytes = bytes(pepper_tuple)

            if prepend:
                test_input = pepper_bytes + known_data
            else:
                test_input = known_data + pepper_bytes

            digest = h_func(test_input).hexdigest()
            if digest == target_hash:
                return pepper_bytes

    return None


def brute_force_hmac_pepper(target_mac, message, hash_func='sha256',
                            pepper_len=1, max_pepper_len=4):
    """
    Brute-force HMAC pepper.
    Returns the pepper if found.
    """
    hash_map = {
        'md5': 'md5',
        'sha1': 'sha1',
        'sha256': 'sha256',
        'sha512': 'sha512',
    }

    h_name = hash_map.get(hash_func, 'sha256')

    if isinstance(message, str):
        message = message.encode()
    if isinstance(target_mac, str):
        target_mac = target_mac.lower()

    for plen in range(pepper_len, max_pepper_len + 1):
        print(f"[*] Trying HMAC pepper length: {plen}")
        # Try printable ASCII characters first (more likely in CTF)
        for pepper_tuple in itertools.product(string.printable.encode(), repeat=plen):
            pepper_bytes = bytes(pepper_tuple)
            mac = hmac.new(pepper_bytes, message, h_name).hexdigest()
            if mac == target_mac:
                return pepper_bytes

    return None


# --- Server Interaction ---

class Connection:
    """Simple connection wrapper."""

    def __init__(self, host, port):
        if HAS_PWNTOOLS:
            self.conn = remote(host, port)
        else:
            self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.sock.connect((host, port))
            self.buffer = b""

    def recvuntil(self, delim):
        if isinstance(delim, str):
            delim = delim.encode()
        if HAS_PWNTOOLS:
            return self.conn.recvuntil(delim)
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


def solve_interactive(host, port):
    """
    Connect to the challenge server and solve interactively.
    Adapts based on the server's response format.
    """
    print(f"[*] Connecting to {host}:{port}...")
    conn = Connection(host, port)

    # Read initial data from server
    try:
        initial = conn.recv(4096).decode(errors='replace')
        print(f"[*] Server says:\n{initial}")
    except Exception as e:
        print(f"[!] Error reading initial data: {e}")
        return

    # Try to parse the challenge data
    # Common formats:
    # 1. JSON with hash/ciphertext
    # 2. Plain text with hex-encoded data
    # 3. Base64-encoded data

    try:
        data = json.loads(initial)
        print(f"[*] Parsed JSON response: {data}")

        # Look for common keys
        ct = data.get("ciphertext") or data.get("ct") or data.get("encrypted")
        target_hash = data.get("hash") or data.get("target") or data.get("digest")
        message = data.get("message") or data.get("plaintext") or data.get("data")

        if ct:
            ct_bytes = bytes.fromhex(ct)
            print(f"[*] Ciphertext ({len(ct_bytes)} bytes): {ct[:64]}...")
            pepper, flag = brute_force_xor_pepper(ct_bytes)
            if flag:
                print(f"\n[+] Pepper found: {pepper.hex()}")
                print(f"[+] FLAG: {flag}")
                conn.close()
                return

        if target_hash and message:
            print(f"[*] Target hash: {target_hash}")
            print(f"[*] Known message: {message}")
            for h in ['sha256', 'sha1', 'md5']:
                for prepend in [True, False]:
                    pepper = brute_force_hash_pepper(target_hash, message, h,
                                                     prepend=prepend, max_pepper_len=3)
                    if pepper:
                        print(f"\n[+] Pepper found ({h}, {'prepend' if prepend else 'append'}): {pepper}")
                        print(f"[+] Pepper (hex): {pepper.hex()}")
                        # Send pepper back to server to get flag
                        conn.sendline(pepper.hex())
                        response = conn.recv(4096).decode(errors='replace')
                        print(f"[+] Server response: {response}")
                        conn.close()
                        return

    except (json.JSONDecodeError, ValueError):
        # Not JSON -- try to find hex-encoded data in the text
        import re
        hex_pattern = re.compile(r'[0-9a-fA-F]{16,}')
        hex_matches = hex_pattern.findall(initial)

        if hex_matches:
            for hex_str in hex_matches:
                print(f"[*] Found hex data: {hex_str[:64]}...")
                try:
                    ct_bytes = bytes.fromhex(hex_str)
                    pepper, flag = brute_force_xor_pepper(ct_bytes)
                    if flag:
                        print(f"\n[+] Pepper found: {pepper.hex()}")
                        print(f"[+] FLAG: {flag}")
                        conn.close()
                        return
                except ValueError:
                    continue

    print("[*] Could not auto-solve. Displaying raw server output for manual analysis.")
    conn.close()


def solve_file(filepath):
    """
    Solve from a downloaded challenge file.
    Tries to read and brute-force XOR pepper or hash pepper.
    """
    print(f"[*] Reading challenge file: {filepath}")
    with open(filepath, 'rb') as f:
        data = f.read()

    # Try XOR brute-force first
    print("[*] Attempting XOR pepper brute-force...")
    pepper, flag = brute_force_xor_pepper(data, pepper_len=1, max_pepper_len=4)
    if flag:
        print(f"\n[+] Pepper found: {pepper.hex()}")
        print(f"[+] FLAG: {flag}")
        return

    # Try interpreting file as hex
    try:
        hex_data = data.decode().strip()
        ct_bytes = bytes.fromhex(hex_data)
        print("[*] Decoded hex data, trying XOR brute-force...")
        pepper, flag = brute_force_xor_pepper(ct_bytes)
        if flag:
            print(f"\n[+] Pepper found: {pepper.hex()}")
            print(f"[+] FLAG: {flag}")
            return
    except (ValueError, UnicodeDecodeError):
        pass

    print("[!] Could not auto-solve from file. Manual analysis needed.")
    print(f"[*] File size: {len(data)} bytes")
    print(f"[*] First 64 bytes (hex): {data[:64].hex()}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Black Cobra Pepper Solver - picoCTF 2026")
    parser.add_argument("--host", default="rescued-float.picoctf.net",
                        help="Challenge server hostname")
    parser.add_argument("--port", type=int, default=50000,
                        help="Challenge server port")
    parser.add_argument("--file", type=str, default=None,
                        help="Path to downloaded challenge file")
    parser.add_argument("--hash", type=str, default=None,
                        help="Target hash to crack (hex)")
    parser.add_argument("--message", type=str, default=None,
                        help="Known message/plaintext")
    parser.add_argument("--hash-type", type=str, default="sha256",
                        choices=["md5", "sha1", "sha256", "sha512"],
                        help="Hash algorithm (default: sha256)")
    args = parser.parse_args()

    if args.file:
        solve_file(args.file)
    elif args.hash and args.message:
        # Direct hash brute-force mode
        print(f"[*] Brute-forcing {args.hash_type} pepper...")
        for prepend in [True, False]:
            pepper = brute_force_hash_pepper(
                args.hash, args.message, args.hash_type,
                prepend=prepend, max_pepper_len=4
            )
            if pepper:
                pos = "prepend" if prepend else "append"
                print(f"\n[+] Pepper found ({pos}): {pepper}")
                print(f"[+] Pepper (hex): {pepper.hex()}")
                try:
                    print(f"[+] Pepper (ascii): {pepper.decode()}")
                except UnicodeDecodeError:
                    pass
                sys.exit(0)
        print("[!] Pepper not found. Try increasing max length or different hash type.")
    else:
        solve_interactive(args.host, args.port)
