#!/usr/bin/env python3
"""
bytemancy 3 - picoCTF 2026 (General Skills, 400 pts)

Final challenge in the bytemancy series -- complex multi-stage byte
manipulation that must be fully reversed.

This script:
  1. Parses the challenge source code to extract the transformation pipeline
  2. Builds inverse operations for each transformation stage
  3. Computes the required input bytes
  4. Sends them to the challenge to retrieve the flag

Usage:
    python3 solve.py                          # Auto-detect source and solve
    python3 solve.py --host HOST --port PORT  # Remote service mode
    python3 solve.py --local program.py       # Local program mode
    python3 solve.py --source source.py       # Analyze specific source file

Dependencies: pwntools (pip install pwntools) [optional, for remote]
"""

import argparse
import sys
import re
import os
import struct

try:
    from pwn import *
    HAS_PWNTOOLS = True
except ImportError:
    HAS_PWNTOOLS = False


# ============================================================
# Common byte operation inverses used across bytemancy variants
# ============================================================

def xor_bytes(data, key):
    """XOR data with key (repeating key if shorter). Self-inverse."""
    if isinstance(key, int):
        return bytes(b ^ key for b in data)
    return bytes(b ^ key[i % len(key)] for i, b in enumerate(data))


def rotate_left(byte_val, n):
    """Rotate a single byte left by n bits."""
    n = n % 8
    return ((byte_val << n) | (byte_val >> (8 - n))) & 0xFF


def rotate_right(byte_val, n):
    """Rotate a single byte right by n bits."""
    n = n % 8
    return ((byte_val >> n) | (byte_val << (8 - n))) & 0xFF


def add_bytes(data, key_bytes):
    """Add key bytes mod 256."""
    if isinstance(key_bytes, int):
        return bytes((b + key_bytes) % 256 for b in data)
    return bytes((b + key_bytes[i % len(key_bytes)]) % 256 for i, b in enumerate(data))


def sub_bytes(data, key_bytes):
    """Subtract key bytes mod 256 (inverse of add_bytes)."""
    if isinstance(key_bytes, int):
        return bytes((b - key_bytes) % 256 for b in data)
    return bytes((b - key_bytes[i % len(key_bytes)]) % 256 for i, b in enumerate(data))


def apply_sbox(data, sbox):
    """Apply substitution box."""
    return bytes(sbox[b] for b in data)


def invert_sbox(sbox):
    """Build inverse substitution box."""
    inv = [0] * 256
    for i, v in enumerate(sbox):
        inv[v] = i
    return inv


def apply_permutation(data, perm):
    """Apply byte permutation: out[i] = data[perm[i]]."""
    return bytes(data[perm[i]] for i in range(len(data)))


def invert_permutation(perm):
    """Build inverse permutation."""
    inv = [0] * len(perm)
    for i, v in enumerate(perm):
        inv[v] = i
    return inv


def swap_pairs(data):
    """Swap adjacent byte pairs."""
    out = bytearray(data)
    for i in range(0, len(out) - 1, 2):
        out[i], out[i + 1] = out[i + 1], out[i]
    return bytes(out)


def reverse_bytes(data):
    """Reverse byte order."""
    return data[::-1]


def nibble_swap(data):
    """Swap high and low nibbles of each byte."""
    return bytes(((b << 4) | (b >> 4)) & 0xFF for b in data)


# ============================================================
# Source code analysis
# ============================================================

def analyze_source(source_path):
    """
    Analyze bytemancy 3 source code to extract:
    - Target/expected bytes
    - Transformation pipeline
    - Keys, S-boxes, permutations
    """
    print(f"[*] Analyzing source: {source_path}")

    with open(source_path, 'r') as f:
        source = f.read()

    print(f"[*] Source code ({len(source)} bytes):")
    print("-" * 60)
    print(source)
    print("-" * 60)

    # Extract components from the source
    info = {
        'target': None,
        'sbox': None,
        'perm': None,
        'key': None,
        'xor_key': None,
        'rotation': None,
        'add_val': None,
        'rounds': 1,
        'operations': [],  # ordered list of operations
    }

    # Find target bytes
    fromhex = re.findall(r'bytes\.fromhex\s*\(\s*["\']([0-9a-fA-F]+)["\']', source)
    if fromhex:
        info['target'] = bytes.fromhex(fromhex[-1])  # usually the last one is the target
        print(f"[+] Target bytes: {info['target'].hex()}")

    # Find bytearray/list of ints
    ba_match = re.findall(r'(?:target|expected|goal|check|ciphertext|encrypted|output)\s*=\s*(?:bytearray|bytes)\s*\(\s*\[([^\]]+)\]', source)
    if ba_match:
        nums = [int(x.strip(), 0) for x in ba_match[0].split(',') if x.strip()]
        info['target'] = bytes(nums)
        print(f"[+] Target bytes: {info['target'].hex()}")

    # Find S-box
    sbox_match = re.findall(r'(?:sbox|SBOX|s_box|S_BOX|substitution|sub_table)\s*=\s*\[([^\]]+)\]', source)
    if sbox_match:
        info['sbox'] = [int(x.strip(), 0) for x in sbox_match[0].split(',') if x.strip()]
        print(f"[+] Found S-box ({len(info['sbox'])} entries)")

    # Find permutation
    perm_match = re.findall(r'(?:perm|PERM|permutation|perm_table|shuffle)\s*=\s*\[([^\]]+)\]', source)
    if perm_match:
        info['perm'] = [int(x.strip(), 0) for x in perm_match[0].split(',') if x.strip()]
        print(f"[+] Found permutation: {info['perm']}")

    # Find XOR key
    xor_match = re.findall(r'(?:key|KEY|xor_key|XOR_KEY)\s*=\s*(?:0x([0-9a-fA-F]+)|(\d+))', source)
    if xor_match:
        val = xor_match[0][0] or xor_match[0][1]
        info['xor_key'] = int(val, 16) if xor_match[0][0] else int(val)
        print(f"[+] Found XOR key: {info['xor_key']:#x}")

    # Find key bytes
    key_hex = re.findall(r'(?:key|KEY)\s*=\s*bytes\.fromhex\s*\(\s*["\']([0-9a-fA-F]+)["\']', source)
    if key_hex:
        info['key'] = bytes.fromhex(key_hex[0])
        print(f"[+] Found key: {info['key'].hex()}")

    key_bytes_match = re.findall(r'(?:key|KEY)\s*=\s*(?:bytearray|bytes)\s*\(\s*\[([^\]]+)\]', source)
    if key_bytes_match:
        info['key'] = bytes(int(x.strip(), 0) for x in key_bytes_match[0].split(',') if x.strip())
        print(f"[+] Found key: {info['key'].hex()}")

    # Find rotation amount
    rot_match = re.findall(r'(?:<<|>>|rotate_left|rotate_right|rot_left|rot_right)\s*[,(]\s*(\d+)', source)
    if rot_match:
        info['rotation'] = int(rot_match[0])
        print(f"[+] Found rotation: {info['rotation']} bits")

    # Find addition value
    add_match = re.findall(r'[+]\s*(0x[0-9a-fA-F]+|\d+)\s*[)]\s*%\s*256', source)
    if add_match:
        info['add_val'] = int(add_match[0], 0)
        print(f"[+] Found addition: {info['add_val']}")

    # Find number of rounds
    rounds_match = re.findall(r'(?:rounds|ROUNDS|num_rounds|NUM_ROUNDS)\s*=\s*(\d+)', source)
    if rounds_match:
        info['rounds'] = int(rounds_match[0])
        print(f"[+] Found rounds: {info['rounds']}")

    # Detect operation order from the source
    # Look for function calls or inline operations in the encrypt/transform function
    transform_section = re.search(
        r'def\s+(?:encrypt|transform|encode|process|encipher)\s*\([^)]*\)\s*:(.*?)(?=\ndef\s|\Z)',
        source, re.DOTALL
    )
    if transform_section:
        body = transform_section.group(1)
        lines = body.split('\n')
        for line in lines:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            if 'xor' in line.lower() or '^' in line:
                info['operations'].append('xor')
            elif 'sbox' in line.lower() or 'substitut' in line.lower():
                info['operations'].append('sbox')
            elif 'perm' in line.lower() or 'shuffle' in line.lower():
                info['operations'].append('perm')
            elif 'rotate' in line.lower() or '<<' in line or '>>' in line:
                info['operations'].append('rotate')
            elif 'swap' in line.lower():
                info['operations'].append('swap')
            elif 'reverse' in line.lower() or '[::-1]' in line:
                info['operations'].append('reverse')
            elif 'nibble' in line.lower():
                info['operations'].append('nibble_swap')
            elif '+' in line and '% 256' in line:
                info['operations'].append('add')
            elif '-' in line and '% 256' in line:
                info['operations'].append('sub')

        if info['operations']:
            print(f"[+] Detected operation order: {' -> '.join(info['operations'])}")

    return info, source


def build_decryptor(info):
    """
    Build a decryption function based on the detected operations.
    Applies inverse operations in reverse order.
    """
    operations = info.get('operations', [])
    if not operations:
        # Default assumption for bytemancy 3: full cipher pipeline
        operations = ['xor', 'sbox', 'perm', 'rotate']
        print(f"[*] Using default operation order: {' -> '.join(operations)}")

    # Build inverse S-box if needed
    inv_sbox = None
    if info.get('sbox'):
        inv_sbox = invert_sbox(info['sbox'])

    # Build inverse permutation if needed
    inv_perm = None
    if info.get('perm'):
        inv_perm = invert_permutation(info['perm'])

    def decrypt(data):
        """Apply inverse operations in reverse order for each round."""
        result = bytearray(data)

        for round_num in range(info.get('rounds', 1)):
            # Process operations in reverse
            for op in reversed(operations):
                if op == 'xor':
                    key = info.get('key') or info.get('xor_key') or 0
                    result = bytearray(xor_bytes(bytes(result), key))

                elif op == 'sbox' and inv_sbox:
                    result = bytearray(apply_sbox(bytes(result), inv_sbox))

                elif op == 'perm' and inv_perm:
                    result = bytearray(apply_permutation(bytes(result), inv_perm))

                elif op == 'rotate':
                    rot = info.get('rotation', 3)
                    # If forward was rotate-left, inverse is rotate-right
                    result = bytearray(rotate_right(b, rot) for b in result)

                elif op == 'add':
                    val = info.get('add_val', 0)
                    key = info.get('key')
                    if key:
                        result = bytearray(sub_bytes(bytes(result), key))
                    else:
                        result = bytearray(sub_bytes(bytes(result), val))

                elif op == 'sub':
                    val = info.get('add_val', 0)
                    key = info.get('key')
                    if key:
                        result = bytearray(add_bytes(bytes(result), key))
                    else:
                        result = bytearray(add_bytes(bytes(result), val))

                elif op == 'swap':
                    result = bytearray(swap_pairs(bytes(result)))

                elif op == 'reverse':
                    result = bytearray(reverse_bytes(bytes(result)))

                elif op == 'nibble_swap':
                    result = bytearray(nibble_swap(bytes(result)))

        return bytes(result)

    return decrypt


def solve_remote(host, port, payload_bytes):
    """Connect to remote service and send the payload."""
    if not HAS_PWNTOOLS:
        print("[!] pwntools not installed. Install with: pip install pwntools")
        print(f"[*] Payload (hex): {payload_bytes.hex()}")
        return

    print(f"[*] Connecting to {host}:{port}")
    r = remote(host, port)

    # Receive initial prompt
    try:
        initial = r.recvuntil(b':', timeout=5)
        print(f"[*] Prompt: {initial.decode(errors='replace')}")
    except Exception:
        initial = r.recv(timeout=3)
        print(f"[*] Received: {initial.decode(errors='replace')}")

    prompt_text = initial.decode(errors='replace').lower()

    # Send in appropriate format
    if 'hex' in prompt_text:
        print(f"[*] Sending hex: {payload_bytes.hex()}")
        r.sendline(payload_bytes.hex().encode())
    elif 'base64' in prompt_text:
        import base64
        b64 = base64.b64encode(payload_bytes).decode()
        print(f"[*] Sending base64: {b64}")
        r.sendline(b64.encode())
    elif 'decimal' in prompt_text or 'number' in prompt_text:
        dec_str = ' '.join(str(b) for b in payload_bytes)
        print(f"[*] Sending decimal: {dec_str}")
        r.sendline(dec_str.encode())
    else:
        print(f"[*] Sending raw bytes: {payload_bytes.hex()}")
        r.send(payload_bytes)

    # Get response
    try:
        response = r.recvall(timeout=5).decode(errors='replace')
        print(f"\n[*] Response:\n{response}")

        flag_match = re.search(r'picoCTF\{[^}]+\}', response)
        if flag_match:
            print(f"\n[+] FLAG: {flag_match.group()}")
        else:
            print("[!] No flag found. Try different input format.")
    except Exception as e:
        print(f"[!] Error: {e}")

    r.close()


def solve_local(program_path, payload_bytes):
    """Run local program with the payload."""
    import subprocess

    for fmt_name, fmt_data in [
        ("hex", payload_bytes.hex().encode() + b'\n'),
        ("raw", payload_bytes + b'\n'),
        ("space-separated hex", ' '.join(f'0x{b:02x}' for b in payload_bytes).encode() + b'\n'),
        ("decimal", ' '.join(str(b) for b in payload_bytes).encode() + b'\n'),
    ]:
        print(f"\n[*] Trying {fmt_name} input...")
        try:
            result = subprocess.run(
                ['python3', program_path],
                input=fmt_data,
                capture_output=True,
                timeout=10,
            )
            output = result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace')
            if output.strip():
                print(f"[*] Output: {output.strip()}")

            flag_match = re.search(r'picoCTF\{[^}]+\}', output)
            if flag_match:
                print(f"\n[+] FLAG: {flag_match.group()}")
                return
        except subprocess.TimeoutExpired:
            print(f"[!] Timeout with {fmt_name} input")
        except Exception as e:
            print(f"[!] Error: {e}")

    print("\n[!] Could not get flag automatically.")
    print("[*] Verify the source analysis and try manually.")


def main():
    parser = argparse.ArgumentParser(description='bytemancy 3 solver - picoCTF 2026')
    parser.add_argument('--host', help='Remote host')
    parser.add_argument('--port', type=int, help='Remote port')
    parser.add_argument('--local', help='Path to local program')
    parser.add_argument('--source', help='Path to source code to analyze')
    args = parser.parse_args()

    print("=" * 60)
    print("  bytemancy 3 - picoCTF 2026 Solver")
    print("  General Skills | 400 pts")
    print("=" * 60)
    print()

    # Step 1: Find and analyze source code
    source_files = [
        'bytemancy3.py', 'bytemancy.py', 'challenge.py',
        'program.py', 'main.py', 'cipher.py',
    ]
    source_path = args.source

    if not source_path:
        for sf in source_files:
            if os.path.exists(sf):
                source_path = sf
                break

    info = None
    if source_path and os.path.exists(source_path):
        info, source = analyze_source(source_path)
    else:
        print("[!] No source code found.")
        print("[*] Download the source from the challenge and use --source <path>")
        print()
        print("[*] General approach for bytemancy 3:")
        print("    1. Read the source code -- identify the encrypt/transform function")
        print("    2. List every operation in order (XOR, S-box, permutation, rotation, etc.)")
        print("    3. For each operation, implement the inverse")
        print("    4. Apply inverses in REVERSE order to the target bytes")
        print("    5. Handle multiple rounds if present")
        print()
        print("[*] Common inverse operations:")
        print("    XOR(key)         -> XOR(key)           [self-inverse]")
        print("    ADD(k) % 256     -> SUB(k) % 256")
        print("    Rotate Left(n)   -> Rotate Right(n)")
        print("    S-box lookup     -> Inverse S-box lookup")
        print("    Permutation P    -> Inverse permutation P^-1")
        print("    Reverse bytes    -> Reverse bytes       [self-inverse]")
        print("    Nibble swap      -> Nibble swap         [self-inverse]")
        sys.exit(0)

    if not info or not info.get('target'):
        print("[!] Could not extract target bytes from source.")
        print("[*] Manually identify the target and update this script.")
        sys.exit(1)

    # Step 2: Build decryptor and compute solution
    print(f"\n[*] Target bytes: {info['target'].hex()}")
    print(f"[*] Target length: {len(info['target'])} bytes")

    decrypt = build_decryptor(info)
    solution = decrypt(info['target'])

    print(f"\n[+] Computed solution (hex):     {solution.hex()}")
    print(f"[+] Computed solution (decimal): {list(solution)}")
    try:
        print(f"[+] Computed solution (ASCII):   {solution.decode('ascii', errors='replace')}")
    except Exception:
        pass

    # Step 3: Deliver the solution
    if args.host and args.port:
        solve_remote(args.host, args.port, solution)
    elif args.local:
        solve_local(args.local, solution)
    else:
        print(f"\n[*] To send to a remote service:")
        print(f"    python3 solve.py --host <HOST> --port <PORT>")
        print(f"\n[*] To run against local program:")
        print(f"    python3 solve.py --local <program.py>")
        echo_payload = ''.join(f'\\x{b:02x}' for b in solution)
        print(f"\n[*] Manual command:")
        print(f"    echo -ne '{echo_payload}' | nc <HOST> <PORT>")


if __name__ == '__main__':
    main()
