#!/usr/bin/env python3
"""
Shared Secrets - picoCTF 2026 (Cryptography, 100 pts)

Exploit a leaked private key in a Diffie-Hellman key exchange to compute
the shared secret and decrypt the flag.

Usage:
    python3 solve.py [challenge_file]

If no file is provided, the script will look for common filenames in the
current directory and also demonstrate the approach with example values.

Dependencies: pycryptodome (pip install pycryptodome)
"""

import sys
import os
import re
import json
import hashlib
from itertools import cycle

# Try to import pycryptodome for AES decryption
try:
    from Crypto.Cipher import AES
    from Crypto.Util.Padding import unpad
    from Crypto.Util.number import long_to_bytes
    HAS_CRYPTO = True
except ImportError:
    HAS_CRYPTO = False
    # Fallback long_to_bytes implementation
    def long_to_bytes(n, blocksize=0):
        s = b''
        n_int = int(n)
        while n_int > 0:
            s = bytes([n_int & 0xFF]) + s
            n_int >>= 8
        if blocksize > 0 and len(s) % blocksize:
            s = (blocksize - len(s) % blocksize) * b'\x00' + s
        return s or b'\x00'


def parse_challenge_file(filepath):
    """
    Parse the challenge file to extract DH parameters and ciphertext.
    Supports multiple common formats: JSON, key=value, Python-like assignments.
    """
    print(f"[*] Parsing: {filepath}")

    with open(filepath, 'r') as f:
        content = f.read()

    params = {}

    # Try JSON format first
    try:
        data = json.loads(content)
        params = {k.lower(): v for k, v in data.items()}
        print("    [*] Parsed as JSON")
        return params
    except (json.JSONDecodeError, AttributeError):
        pass

    # Try key=value or key: value format
    # Matches: p = 12345, p: 12345, p=12345, etc.
    patterns = [
        # Variable assignment: p = 12345 or p = 0x...
        (r'(\w+)\s*=\s*(0x[0-9a-fA-F]+|\d+)', 'assignment'),
        # Colon-separated: p: 12345
        (r'(\w+)\s*:\s*(0x[0-9a-fA-F]+|\d+)', 'colon'),
        # Quoted hex string values: ciphertext = "aabbcc..."
        (r'(\w+)\s*[=:]\s*["\']([0-9a-fA-F]+)["\']', 'hex_string'),
        # Base64 values: ciphertext = "base64..."
        (r'(\w+)\s*[=:]\s*["\']([A-Za-z0-9+/=]+)["\']', 'b64_string'),
    ]

    for pattern, fmt in patterns:
        for match in re.finditer(pattern, content):
            key = match.group(1).lower()
            val = match.group(2)

            if fmt in ('assignment', 'colon'):
                if val.startswith('0x'):
                    params[key] = int(val, 16)
                else:
                    params[key] = int(val)
            else:
                params[key] = val

    if params:
        print(f"    [*] Extracted {len(params)} parameters")
    else:
        print("    [!] Could not parse parameters from file")

    return params


def compute_shared_secret(params):
    """Compute the Diffie-Hellman shared secret from the given parameters."""

    # Identify the parameters (handle various naming conventions)
    p = params.get('p') or params.get('prime') or params.get('modulus')
    g = params.get('g') or params.get('generator') or params.get('base')
    a_pub = params.get('a') or params.get('a_pub') or params.get('alice_public') or params.get('public_a')
    b_pub = params.get('b') or params.get('b_pub') or params.get('bob_public') or params.get('public_b')
    a_priv = params.get('a_priv') or params.get('alice_private') or params.get('private_a') or params.get('a_secret')
    b_priv = params.get('b_priv') or params.get('bob_private') or params.get('private_b') or params.get('b_secret')

    if p is None:
        print("[!] Prime modulus 'p' not found in parameters")
        return None

    print(f"\n[*] DH Parameters:")
    print(f"    p = {str(p)[:60]}{'...' if len(str(p)) > 60 else ''}")
    if g: print(f"    g = {g}")
    if a_pub: print(f"    A (public) = {str(a_pub)[:60]}{'...' if len(str(a_pub)) > 60 else ''}")
    if b_pub: print(f"    B (public) = {str(b_pub)[:60]}{'...' if len(str(b_pub)) > 60 else ''}")
    if a_priv: print(f"    a (LEAKED private) = {str(a_priv)[:60]}{'...' if len(str(a_priv)) > 60 else ''}")
    if b_priv: print(f"    b (LEAKED private) = {str(b_priv)[:60]}{'...' if len(str(b_priv)) > 60 else ''}")

    # Compute the shared secret
    if a_priv and b_pub:
        print("\n[*] Computing shared secret: s = B^a mod p")
        shared_secret = pow(int(b_pub), int(a_priv), int(p))
    elif b_priv and a_pub:
        print("\n[*] Computing shared secret: s = A^b mod p")
        shared_secret = pow(int(a_pub), int(b_priv), int(p))
    elif a_priv and g:
        # If only one public key is available, and it's not labeled clearly
        # Check if 'a' is actually the other party's public key
        print("\n[*] Trying: s = A^(leaked_key) mod p")
        shared_secret = pow(int(a_pub), int(a_priv), int(p)) if a_pub else None
        if shared_secret is None:
            print("[!] Not enough information to compute shared secret")
            return None
    else:
        print("[!] Need a leaked private key and the other party's public key")
        return None

    print(f"[+] Shared secret = {str(shared_secret)[:60]}{'...' if len(str(shared_secret)) > 60 else ''}")
    return shared_secret


def derive_key(shared_secret, method='sha256_str'):
    """Derive an encryption key from the shared secret using common methods."""
    methods = {}

    # Method 1: SHA-256 of the string representation
    methods['sha256_str_16'] = hashlib.sha256(str(shared_secret).encode()).digest()[:16]
    methods['sha256_str_32'] = hashlib.sha256(str(shared_secret).encode()).digest()

    # Method 2: SHA-256 of the byte representation
    ss_bytes = long_to_bytes(shared_secret)
    methods['sha256_bytes_16'] = hashlib.sha256(ss_bytes).digest()[:16]
    methods['sha256_bytes_32'] = hashlib.sha256(ss_bytes).digest()

    # Method 3: MD5 of string representation
    methods['md5_str'] = hashlib.md5(str(shared_secret).encode()).digest()

    # Method 4: MD5 of byte representation
    methods['md5_bytes'] = hashlib.md5(ss_bytes).digest()

    # Method 5: Direct bytes (truncated/padded to 16 or 32)
    if len(ss_bytes) >= 16:
        methods['direct_16'] = ss_bytes[:16]
    if len(ss_bytes) >= 32:
        methods['direct_32'] = ss_bytes[:32]

    # Method 6: SHA-1 of string
    methods['sha1_str'] = hashlib.sha1(str(shared_secret).encode()).digest()[:16]

    return methods


def decrypt_message(ciphertext_raw, key, iv=None):
    """Try to decrypt the ciphertext using AES with the given key."""
    results = []

    if not HAS_CRYPTO:
        # Fallback: XOR decryption only
        decrypted = bytes(c ^ k for c, k in zip(ciphertext_raw, cycle(key)))
        return [('XOR', decrypted)]

    # Try AES-CBC
    if iv:
        try:
            cipher = AES.new(key, AES.MODE_CBC, iv)
            decrypted = cipher.decrypt(ciphertext_raw)
            try:
                decrypted = unpad(decrypted, AES.block_size)
            except ValueError:
                pass  # Might not be PKCS7 padded
            results.append(('AES-CBC', decrypted))
        except Exception:
            pass

    # Try AES-CBC with first 16 bytes as IV
    if len(ciphertext_raw) > 16 and not iv:
        try:
            iv_guess = ciphertext_raw[:16]
            ct = ciphertext_raw[16:]
            cipher = AES.new(key, AES.MODE_CBC, iv_guess)
            decrypted = cipher.decrypt(ct)
            try:
                decrypted = unpad(decrypted, AES.block_size)
            except ValueError:
                pass
            results.append(('AES-CBC (IV=first16)', decrypted))
        except Exception:
            pass

    # Try AES-ECB
    try:
        cipher = AES.new(key, AES.MODE_ECB)
        decrypted = cipher.decrypt(ciphertext_raw)
        try:
            decrypted = unpad(decrypted, AES.block_size)
        except ValueError:
            pass
        results.append(('AES-ECB', decrypted))
    except Exception:
        pass

    # Try AES-CTR with various nonce configurations
    if len(ciphertext_raw) > 8:
        try:
            from Crypto.Cipher import AES as AES2
            nonce = ciphertext_raw[:8]
            ct = ciphertext_raw[8:]
            cipher = AES2.new(key, AES2.MODE_CTR, nonce=nonce)
            decrypted = cipher.decrypt(ct)
            results.append(('AES-CTR (nonce=first8)', decrypted))
        except Exception:
            pass

    # Try XOR
    decrypted = bytes(c ^ k for c, k in zip(ciphertext_raw, cycle(key)))
    results.append(('XOR', decrypted))

    return results


def solve(params):
    """Main solving logic."""

    # Step 1: Compute shared secret
    shared_secret = compute_shared_secret(params)
    if shared_secret is None:
        return None

    # Step 2: Check if the shared secret itself contains the flag
    ss_str = str(shared_secret)
    ss_bytes = long_to_bytes(shared_secret)
    ss_hex = ss_bytes.hex()

    # Check if shared secret bytes decode to the flag
    try:
        decoded = ss_bytes.decode('utf-8', errors='ignore')
        if 'picoCTF' in decoded:
            match = re.search(r'picoCTF\{[^}]+\}', decoded)
            if match:
                return match.group(0)
    except Exception:
        pass

    # Check hex representation
    try:
        flag_bytes = bytes.fromhex(ss_hex)
        decoded = flag_bytes.decode('utf-8', errors='ignore')
        if 'picoCTF' in decoded:
            match = re.search(r'picoCTF\{[^}]+\}', decoded)
            if match:
                return match.group(0)
    except Exception:
        pass

    # Step 3: Get ciphertext
    ciphertext = params.get('ciphertext') or params.get('ct') or params.get('cipher') or params.get('encrypted') or params.get('message') or params.get('c')
    iv = params.get('iv') or params.get('nonce')

    if ciphertext is None:
        print("\n[!] No ciphertext found in parameters")
        print(f"[*] Shared secret (decimal): {shared_secret}")
        print(f"[*] Shared secret (hex): {ss_hex}")
        print(f"[*] Shared secret (bytes): {ss_bytes}")
        return None

    # Convert ciphertext to bytes
    if isinstance(ciphertext, str):
        try:
            ciphertext_raw = bytes.fromhex(ciphertext)
        except ValueError:
            import base64
            try:
                ciphertext_raw = base64.b64decode(ciphertext)
            except Exception:
                ciphertext_raw = ciphertext.encode()
    elif isinstance(ciphertext, int):
        ciphertext_raw = long_to_bytes(ciphertext)
    else:
        ciphertext_raw = ciphertext

    # Convert IV to bytes if present
    iv_raw = None
    if iv:
        if isinstance(iv, str):
            try:
                iv_raw = bytes.fromhex(iv)
            except ValueError:
                iv_raw = iv.encode()
        elif isinstance(iv, int):
            iv_raw = long_to_bytes(iv)
            # Pad IV to 16 bytes
            if len(iv_raw) < 16:
                iv_raw = b'\x00' * (16 - len(iv_raw)) + iv_raw

    print(f"\n[*] Ciphertext ({len(ciphertext_raw)} bytes): {ciphertext_raw[:32].hex()}...")
    if iv_raw:
        print(f"[*] IV ({len(iv_raw)} bytes): {iv_raw.hex()}")

    # Step 4: Derive keys and try decryption
    print("\n[*] Trying all key derivation and decryption combinations...")
    key_methods = derive_key(shared_secret)

    for key_name, key in key_methods.items():
        if len(key) not in (16, 24, 32):
            continue

        results = decrypt_message(ciphertext_raw, key, iv_raw)
        for mode, plaintext in results:
            # Check if decrypted text contains the flag
            try:
                decoded = plaintext.decode('utf-8', errors='ignore')
            except Exception:
                decoded = str(plaintext)

            if 'picoCTF' in decoded:
                match = re.search(r'picoCTF\{[^}]+\}', decoded)
                if match:
                    print(f"\n[+] Decrypted with key={key_name}, mode={mode}")
                    print(f"[+] Plaintext: {decoded}")
                    return match.group(0)

            # Also check raw bytes
            if b'picoCTF' in plaintext:
                match = re.search(rb'picoCTF\{[^}]+\}', plaintext)
                if match:
                    print(f"\n[+] Decrypted with key={key_name}, mode={mode}")
                    return match.group(0).decode()

    print("\n[!] No valid decryption found with standard methods")
    print("[*] The challenge may use a non-standard key derivation or cipher")
    return None


def main():
    # Try to find the challenge file
    challenge_file = None

    if len(sys.argv) >= 2:
        challenge_file = sys.argv[1]
    else:
        # Look for common challenge file names
        common_names = [
            'challenge.txt', 'params.txt', 'shared_secrets.txt',
            'exchange.txt', 'data.txt', 'output.txt', 'out.txt',
            'challenge.json', 'params.json', 'values.txt',
            'intercept.txt', 'captured.txt', 'dh_params.txt',
        ]
        for name in common_names:
            if os.path.isfile(name):
                challenge_file = name
                print(f"[*] Found challenge file: {name}")
                break

    if challenge_file is None:
        print("Shared Secrets - picoCTF 2026 (Cryptography, 100 pts)")
        print()
        print("Usage: python3 solve.py <challenge_file>")
        print()
        print("The challenge file should contain the DH parameters:")
        print("  p, g, A (Alice's public), B (Bob's public),")
        print("  leaked private key (a or b), and the ciphertext.")
        print()
        print("Supported formats: JSON, key=value, Python-like assignments")
        print()
        print("Example challenge file (params.txt):")
        print("  p = 13407807929942597099574024998205846127479365820592393377723561443721764...")
        print("  g = 2")
        print("  A = 89234872348723487...")
        print("  B = 72348723487234872...")
        print("  a_priv = 12345678901234...")
        print("  ciphertext = aabbccdd...")
        print("  iv = 00112233...")
        print()

        # Run with example values to demonstrate
        print("=" * 50)
        print("[*] Running demonstration with example values...")
        print("=" * 50)

        # Small example for demonstration
        demo_p = 0xFFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA237327FFFFFFFFFFFFFFFF
        demo_g = 2
        demo_a_priv = 123456789  # Leaked private key (small for demo)
        demo_A = pow(demo_g, demo_a_priv, demo_p)
        demo_b_priv = 987654321
        demo_B = pow(demo_g, demo_b_priv, demo_p)
        demo_shared = pow(demo_B, demo_a_priv, demo_p)

        # Encrypt a demo flag
        demo_flag = b"picoCTF{d1ff13_h3llm4n_1s_n0t_s0_s3cr3t}"
        demo_key = hashlib.sha256(str(demo_shared).encode()).digest()[:16]

        if HAS_CRYPTO:
            demo_iv = os.urandom(16)
            cipher = AES.new(demo_key, AES.MODE_CBC, demo_iv)
            from Crypto.Util.Padding import pad
            demo_ct = cipher.encrypt(pad(demo_flag, AES.block_size))

            demo_params = {
                'p': demo_p,
                'g': demo_g,
                'a_pub': demo_A,
                'b_pub': demo_B,
                'a_priv': demo_a_priv,
                'ciphertext': demo_ct.hex(),
                'iv': demo_iv.hex(),
            }

            flag = solve(demo_params)
            if flag:
                print(f"\n{'='*50}")
                print(f"DEMO FLAG: {flag}")
                print(f"{'='*50}")
                print("\n[*] This was a demonstration. Run with your challenge file:")
                print("    python3 solve.py <challenge_file>")
        else:
            print("[!] pycryptodome not installed, cannot run demo")
            print("    pip install pycryptodome")

        sys.exit(0)

    if not os.path.isfile(challenge_file):
        print(f"[!] File not found: {challenge_file}")
        sys.exit(1)

    # Parse the challenge file
    params = parse_challenge_file(challenge_file)
    if not params:
        print("[!] Failed to parse challenge file")
        sys.exit(1)

    print(f"[*] Parameters found: {list(params.keys())}")

    # Solve
    flag = solve(params)

    # Final output
    print()
    if flag:
        print(f"{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("[!] Flag not found automatically.")
        print()
        print("[*] Manual steps:")
        print("    1. Verify the parameter names match what the script expects")
        print("    2. Check the key derivation method (SHA-256 of string? of bytes?)")
        print("    3. Check the cipher mode (AES-CBC? AES-ECB? AES-CTR? XOR?)")
        print("    4. Try: shared_secret = pow(B, a, p) in a Python shell")
        print("    5. The shared secret itself might encode the flag")


if __name__ == '__main__':
    main()
