#!/usr/bin/env python3
"""
Timestamped Secrets - picoCTF 2026 (Cryptography, 200 pts)

AES-ECB decryption brute-forcer that recovers a message encrypted with
a timestamp-derived key.

Attack: The encryption key is derived from a Unix timestamp, which has
far too little entropy for a cryptographic key. We brute-force all
plausible timestamps, derive the key for each, attempt decryption,
and check whether the resulting plaintext is valid.

Prerequisites:
  pip install pycryptodome   # or pycryptodomex

Usage:
  python3 solve.py                                  # interactive mode
  python3 solve.py --ciphertext ct.bin              # from binary file
  python3 solve.py --hex <hex_string>               # from hex string
  python3 solve.py --ciphertext ct.bin --start 1700000000 --end 1710000000
  python3 solve.py --ciphertext ct.bin --method sha256
"""

import argparse
import hashlib
import os
import re
import struct
import sys
import time
import random

# Try multiple crypto library imports
try:
    from Crypto.Cipher import AES
    from Crypto.Util.Padding import unpad
except ImportError:
    try:
        from Cryptodome.Cipher import AES
        from Cryptodome.Util.Padding import unpad
    except ImportError:
        print("[!] PyCryptodome not found. Install with:")
        print("    pip install pycryptodome")
        sys.exit(1)


# ── Key derivation methods ────────────────────────────────────────────

def key_from_md5(timestamp):
    """MD5(str(timestamp)) -> 16-byte AES-128 key."""
    return hashlib.md5(str(timestamp).encode()).digest()


def key_from_sha256_16(timestamp):
    """SHA256(str(timestamp))[:16] -> 16-byte AES-128 key."""
    return hashlib.sha256(str(timestamp).encode()).digest()[:16]


def key_from_sha256_32(timestamp):
    """SHA256(str(timestamp)) -> 32-byte AES-256 key."""
    return hashlib.sha256(str(timestamp).encode()).digest()


def key_from_direct_pad(timestamp):
    """str(timestamp) zero-padded to 16 bytes."""
    return str(timestamp).encode().ljust(16, b'\x00')


def key_from_direct_repeat(timestamp):
    """str(timestamp) repeated to fill 16 bytes."""
    ts_bytes = str(timestamp).encode()
    return (ts_bytes * (16 // len(ts_bytes) + 1))[:16]


def key_from_md5_millis(timestamp_ms):
    """MD5(str(timestamp_ms)) for millisecond-precision timestamps."""
    return hashlib.md5(str(timestamp_ms).encode()).digest()


def key_from_random_seed(timestamp):
    """Use timestamp to seed Python's random, then generate 16 key bytes."""
    r = random.Random(timestamp)
    return bytes([r.randint(0, 255) for _ in range(16)])


def key_from_random_seed_millis(timestamp_ms):
    """Use millisecond timestamp to seed Python's random."""
    r = random.Random(timestamp_ms)
    return bytes([r.randint(0, 255) for _ in range(16)])


def key_from_bytes_le(timestamp):
    """Pack timestamp as little-endian 64-bit int, pad to 16 bytes."""
    return struct.pack('<Q', timestamp).ljust(16, b'\x00')


def key_from_bytes_be(timestamp):
    """Pack timestamp as big-endian 64-bit int, pad to 16 bytes."""
    return struct.pack('>Q', timestamp).ljust(16, b'\x00')


# All key derivation methods to try
KEY_METHODS = {
    'md5':            key_from_md5,
    'sha256_16':      key_from_sha256_16,
    'sha256_32':      key_from_sha256_32,
    'direct_pad':     key_from_direct_pad,
    'direct_repeat':  key_from_direct_repeat,
    'random_seed':    key_from_random_seed,
    'bytes_le':       key_from_bytes_le,
    'bytes_be':       key_from_bytes_be,
}


# ── Decryption and validation ────────────────────────────────────────

def try_decrypt(ciphertext, key):
    """Attempt AES-ECB decryption with the given key."""
    try:
        if len(key) == 32:
            cipher = AES.new(key, AES.MODE_ECB)
        else:
            cipher = AES.new(key[:16], AES.MODE_ECB)
        plaintext = cipher.decrypt(ciphertext)
        return plaintext
    except Exception:
        return None


def check_pkcs7_padding(data):
    """Check if data has valid PKCS#7 padding."""
    if not data or len(data) == 0:
        return False
    pad_byte = data[-1]
    if pad_byte < 1 or pad_byte > 16:
        return False
    if data[-pad_byte:] != bytes([pad_byte]) * pad_byte:
        return False
    return True


def is_valid_plaintext(plaintext):
    """
    Heuristic check: does the decrypted data look like a valid message?
    Returns (confidence_score, cleaned_text).
    """
    if plaintext is None:
        return 0, ""

    # Check for picoCTF flag pattern
    try:
        text = plaintext.decode('ascii', errors='ignore')
        if 'picoCTF{' in text:
            return 100, text
        if 'pico' in text.lower() and 'ctf' in text.lower():
            return 80, text
        if 'flag{' in text.lower():
            return 90, text
    except Exception:
        pass

    # Check PKCS#7 padding and printability
    has_padding = check_pkcs7_padding(plaintext)
    if has_padding:
        pad_len = plaintext[-1]
        unpadded = plaintext[:-pad_len]
    else:
        unpadded = plaintext

    try:
        text = unpadded.decode('ascii')
        printable_ratio = sum(1 for c in text if 32 <= ord(c) <= 126) / max(len(text), 1)
        if printable_ratio > 0.9:
            score = 50 + int(printable_ratio * 30)
            if has_padding:
                score += 15
            if 'picoCTF{' in text:
                score = 100
            return score, text
    except (UnicodeDecodeError, ZeroDivisionError):
        pass

    return 0, ""


def load_ciphertext(args):
    """Load ciphertext from file or hex string."""
    if args.hex:
        hex_str = args.hex.strip().replace(' ', '').replace('\n', '')
        return bytes.fromhex(hex_str)

    if args.ciphertext and os.path.exists(args.ciphertext):
        with open(args.ciphertext, 'rb') as f:
            data = f.read()

        # Check if the file contains hex-encoded data
        try:
            text = data.decode('ascii').strip()
            if all(c in '0123456789abcdefABCDEF \n' for c in text):
                return bytes.fromhex(text.replace(' ', '').replace('\n', ''))
        except Exception:
            pass

        return data

    # Try to find ciphertext files in the current directory
    for candidate in ['ciphertext.bin', 'ciphertext.txt', 'ct.bin', 'ct.txt',
                      'encrypted.bin', 'encrypted.txt', 'flag.enc',
                      'message.enc', 'secret.enc', 'output.txt']:
        if os.path.exists(candidate):
            print(f"[*] Found ciphertext file: {candidate}")
            with open(candidate, 'rb') as f:
                data = f.read()
            try:
                text = data.decode('ascii').strip()
                if all(c in '0123456789abcdefABCDEF \n' for c in text):
                    return bytes.fromhex(text.replace(' ', '').replace('\n', ''))
            except Exception:
                pass
            return data

    return None


def determine_search_range(args):
    """Determine the timestamp search range."""
    if args.start and args.end:
        return int(args.start), int(args.end)

    # Default: search a reasonable window
    # picoCTF 2026 competition: March 9-19, 2026
    # But the challenge could use any timestamp

    now = int(time.time())

    if args.start:
        start = int(args.start)
        end = start + 86400 * 30  # 30 days after start
    elif args.end:
        end = int(args.end)
        start = end - 86400 * 30  # 30 days before end
    else:
        # Default search windows (most likely to least likely)
        # 1. Recent: last 30 days
        # 2. Competition window: March 2026
        # 3. Year 2025-2026
        start = now - 86400 * 365  # 1 year ago
        end = now + 86400  # tomorrow

    return start, end


# ── Main solver ───────────────────────────────────────────────────────

def brute_force(ciphertext, method_name, derive_func, start_ts, end_ts, use_millis=False):
    """
    Brute-force timestamps to find the correct AES key.
    Returns (timestamp, plaintext) on success, or (None, None).
    """
    total = end_ts - start_ts
    if use_millis:
        total *= 1000

    print(f"[*] Method: {method_name}")
    print(f"[*] Range: {start_ts} -> {end_ts} ({total:,} candidates)")

    best_score = 0
    best_result = None

    check_interval = 100000
    start_time = time.time()

    for ts in range(start_ts, end_ts):
        if use_millis:
            for ms in range(1000):
                ts_val = ts * 1000 + ms
                key = derive_func(ts_val)
                plaintext = try_decrypt(ciphertext, key)
                score, text = is_valid_plaintext(plaintext)

                if score >= 80:
                    elapsed = time.time() - start_time
                    print(f"\n[+] FOUND! Timestamp: {ts_val} (ms)")
                    print(f"[+] Key (hex): {key.hex()}")
                    print(f"[+] Plaintext: {text}")
                    print(f"[+] Time elapsed: {elapsed:.1f}s")
                    return ts_val, text

                if score > best_score:
                    best_score = score
                    best_result = (ts_val, text)
        else:
            key = derive_func(ts)
            plaintext = try_decrypt(ciphertext, key)
            score, text = is_valid_plaintext(plaintext)

            if score >= 80:
                elapsed = time.time() - start_time
                print(f"\n[+] FOUND! Timestamp: {ts}")
                print(f"[+] Key (hex): {key.hex()}")
                print(f"[+] Plaintext: {text}")
                print(f"[+] Time elapsed: {elapsed:.1f}s")
                return ts, text

            if score > best_score:
                best_score = score
                best_result = (ts, text)

        # Progress reporting
        idx = ts - start_ts
        if idx > 0 and idx % check_interval == 0:
            elapsed = time.time() - start_time
            rate = idx / elapsed if elapsed > 0 else 0
            remaining = (end_ts - ts) / rate if rate > 0 else 0
            pct = (idx / (end_ts - start_ts)) * 100
            print(f"    [{pct:5.1f}%] ts={ts}, {rate:.0f} keys/s, ETA {remaining:.0f}s", end='\r')

    return None, None


def main():
    parser = argparse.ArgumentParser(
        description='Timestamped Secrets solver - picoCTF 2026 Cryptography (200 pts)'
    )
    parser.add_argument('--ciphertext', '-c', help='Path to ciphertext file')
    parser.add_argument('--hex', help='Hex-encoded ciphertext string')
    parser.add_argument('--start', help='Start timestamp for search range')
    parser.add_argument('--end', help='End timestamp for search range')
    parser.add_argument('--method', choices=list(KEY_METHODS.keys()) + ['all'],
                        default='all', help='Key derivation method to use')
    parser.add_argument('--millis', action='store_true',
                        help='Also try millisecond-precision timestamps')

    args = parser.parse_args()

    print("=" * 60)
    print("  Timestamped Secrets - picoCTF 2026 Solver")
    print("  Cryptography | 200 pts")
    print("=" * 60)
    print()

    # Load ciphertext
    ciphertext = load_ciphertext(args)
    if ciphertext is None:
        print("[!] No ciphertext found.")
        print("[*] Provide ciphertext via:")
        print("    --ciphertext <file>  (binary or hex-encoded file)")
        print("    --hex <hex_string>   (hex string on command line)")
        print("[*] Or place the ciphertext file in the current directory.")
        print()
        print("[*] Example usage:")
        print("    python3 solve.py --hex 'aabbccdd...' --start 1709000000 --end 1711000000")
        print("    python3 solve.py --ciphertext encrypted.bin --method md5")
        sys.exit(1)

    print(f"[*] Ciphertext: {len(ciphertext)} bytes")
    print(f"[*] Ciphertext (hex): {ciphertext[:32].hex()}{'...' if len(ciphertext) > 32 else ''}")

    # Validate ciphertext length (must be multiple of 16 for AES-ECB)
    if len(ciphertext) % 16 != 0:
        print(f"[!] Warning: Ciphertext length ({len(ciphertext)}) is not a multiple of 16.")
        print("[*] This may indicate the ciphertext is hex-encoded or base64-encoded.")
        # Try hex decode
        try:
            decoded = bytes.fromhex(ciphertext.decode('ascii').strip())
            if len(decoded) % 16 == 0:
                print(f"[*] Hex-decoded to {len(decoded)} bytes.")
                ciphertext = decoded
        except Exception:
            pass

    # Determine search range
    start_ts, end_ts = determine_search_range(args)
    print(f"[*] Search range: {start_ts} -> {end_ts}")
    print(f"[*] Window: {(end_ts - start_ts):,} seconds ({(end_ts - start_ts) / 86400:.1f} days)")
    print()

    # Select methods to try
    if args.method == 'all':
        methods = list(KEY_METHODS.items())
    else:
        methods = [(args.method, KEY_METHODS[args.method])]

    # First pass: try each method with second-precision timestamps
    for method_name, derive_func in methods:
        print(f"\n{'─' * 50}")
        ts, plaintext = brute_force(ciphertext, method_name, derive_func, start_ts, end_ts)
        if ts is not None:
            # Extract flag
            flag_match = re.search(r'picoCTF\{[^}]+\}', plaintext)
            if flag_match:
                print(f"\n[+] FLAG: {flag_match.group()}")
            else:
                print(f"\n[+] Decrypted message: {plaintext}")
                # Wrap if it looks like a bare flag value
                cleaned = plaintext.strip().rstrip('\x00')
                if cleaned and '{' not in cleaned:
                    print(f"[*] Try: picoCTF{{{cleaned}}}")
            return

    # Second pass: try millisecond precision if requested
    if args.millis:
        print("\n[*] Trying millisecond-precision timestamps...")
        ms_methods = {
            'md5_millis': key_from_md5_millis,
            'random_seed_millis': key_from_random_seed_millis,
        }
        for method_name, derive_func in ms_methods.items():
            print(f"\n{'─' * 50}")
            ts, plaintext = brute_force(
                ciphertext, method_name, derive_func,
                start_ts, end_ts, use_millis=True
            )
            if ts is not None:
                flag_match = re.search(r'picoCTF\{[^}]+\}', plaintext)
                if flag_match:
                    print(f"\n[+] FLAG: {flag_match.group()}")
                else:
                    print(f"\n[+] Decrypted message: {plaintext}")
                return

    print("\n" + "=" * 60)
    print("[-] Flag not found in the searched range.")
    print("[*] Suggestions:")
    print("    1. Widen the search range (--start / --end)")
    print("    2. Try a specific key derivation method (--method)")
    print("    3. Try millisecond precision (--millis)")
    print("    4. Check the challenge for hints about the timestamp or key format")
    print("    5. Examine any provided source code for the exact key derivation")
    print("=" * 60)


if __name__ == '__main__':
    main()
