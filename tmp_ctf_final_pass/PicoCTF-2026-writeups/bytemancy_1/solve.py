#!/usr/bin/env python3
"""
bytemancy 1 - picoCTF 2026 (General Skills, 100 pts)

Second challenge in the bytemancy series -- byte/encoding manipulation.
This solver reverses the byte transformations in the challenge source code
to produce the correct input bytes and retrieve the flag.

The general approach:
  1. Parse the challenge source code to extract targets, keys, and operations.
  2. Invert each transformation to recover the required input.
  3. Deliver the payload to the local program or remote service.

Usage:
  python3 solve.py                              # auto-detect source & solve
  python3 solve.py --source bytemancy1.py       # parse specific source file
  python3 solve.py --host HOST --port PORT      # solve against remote service
  python3 solve.py --local bytemancy1.py        # pipe solution to local program

Dependencies: pwntools (optional, for remote connections)
              pip install pwntools
"""

import argparse
import os
import re
import struct
import subprocess
import sys

try:
    from pwn import *
    HAS_PWNTOOLS = True
except ImportError:
    HAS_PWNTOOLS = False


# ── Byte inversion utilities ─────────────────────────────────────────

def xor_bytes(data, key):
    """XOR each byte with key (single int or byte sequence, cyclic)."""
    if isinstance(key, int):
        return bytes(b ^ key for b in data)
    return bytes(b ^ key[i % len(key)] for i, b in enumerate(data))


def sub_bytes_mod256(data, key):
    """Invert (b + key) % 256 -> (b - key) % 256."""
    if isinstance(key, int):
        return bytes((b - key) % 256 for b in data)
    return bytes((b - key[i % len(key)]) % 256 for i, b in enumerate(data))


def add_bytes_mod256(data, key):
    """Invert (b - key) % 256 -> (b + key) % 256."""
    if isinstance(key, int):
        return bytes((b + key) % 256 for b in data)
    return bytes((b + key[i % len(key)]) % 256 for i, b in enumerate(data))


def ror_byte(val, n, bits=8):
    """Rotate a byte right by n bits."""
    n %= bits
    return ((val >> n) | (val << (bits - n))) & ((1 << bits) - 1)


def rol_byte(val, n, bits=8):
    """Rotate a byte left by n bits."""
    n %= bits
    return ((val << n) | (val >> (bits - n))) & ((1 << bits) - 1)


def swap_nibbles(data):
    """Swap high and low nibbles of each byte (self-inverse)."""
    return bytes(((b >> 4) | ((b & 0x0F) << 4)) & 0xFF for b in data)


def invert_bits(data):
    """Bitwise NOT each byte (self-inverse)."""
    return bytes(~b & 0xFF for b in data)


def reverse_bits_byte(val, bits=8):
    """Reverse the bit order of a single byte."""
    result = 0
    for _ in range(bits):
        result = (result << 1) | (val & 1)
        val >>= 1
    return result


def reverse_bits(data):
    """Reverse bits in each byte (self-inverse)."""
    return bytes(reverse_bits_byte(b) for b in data)


# ── Source code parser ───────────────────────────────────────────────

def parse_source(filepath):
    """
    Parse the bytemancy challenge source code to extract:
    - Target/expected byte values
    - Keys and constants
    - Transformation operations (in order)
    """
    print(f"[*] Parsing source: {filepath}")
    with open(filepath, 'r') as f:
        source = f.read()

    print(f"[*] Source ({len(source)} chars):")
    print("-" * 50)
    for i, line in enumerate(source.splitlines(), 1):
        print(f"  {i:3d} | {line}")
    print("-" * 50)

    info = {
        'expected': None,
        'keys': [],
        'operations': [],
        'input_format': 'raw',  # raw, hex, decimal, binary
        'source': source,
    }

    # Detect input format
    if re.search(r'input.*hex|hex.*input|fromhex|\.hex\(\)', source, re.I):
        info['input_format'] = 'hex'
    elif re.search(r'input.*decimal|int\(.*input|split.*int', source, re.I):
        info['input_format'] = 'decimal'
    elif re.search(r'input.*bin|binary.*input', source, re.I):
        info['input_format'] = 'binary'

    # Extract byte arrays: [0x41, 0x42, ...] or bytearray([...])
    hex_list_re = r'\[(\s*0x[0-9a-fA-F]{1,2}(?:\s*,\s*0x[0-9a-fA-F]{1,2})*\s*)\]'
    for match in re.finditer(hex_list_re, source):
        values = [int(x.strip(), 16) for x in match.group(1).split(',')]
        if len(values) >= 2:
            if info['expected'] is None:
                info['expected'] = values
                print(f"[+] Expected values: {[hex(v) for v in values]}")
            else:
                info['keys'].append(values)
                print(f"[+] Key array: {[hex(v) for v in values]}")

    # Extract decimal arrays: [65, 66, ...]
    dec_list_re = r'\[(\s*\d{1,3}(?:\s*,\s*\d{1,3})+\s*)\]'
    for match in re.finditer(dec_list_re, source):
        values = [int(x.strip()) for x in match.group(1).split(',')]
        if all(0 <= v <= 255 for v in values) and len(values) >= 2:
            if info['expected'] is None:
                info['expected'] = values
                print(f"[+] Expected values (decimal): {values}")
            elif not info['keys']:
                info['keys'].append(values)
                print(f"[+] Key array (decimal): {values}")

    # Extract bytes.fromhex("...")
    for match in re.finditer(r'bytes\.fromhex\s*\(\s*["\']([0-9a-fA-F]+)["\']', source):
        values = list(bytes.fromhex(match.group(1)))
        if info['expected'] is None:
            info['expected'] = values
            print(f"[+] Expected from fromhex: {[hex(v) for v in values]}")

    # Extract byte string literals: b'\x41\x42...'
    for match in re.finditer(r"b'((?:\\x[0-9a-fA-F]{2})+)'", source):
        values = list(bytes.fromhex(
            match.group(1).replace('\\x', '')
        ))
        if len(values) >= 2:
            if info['expected'] is None:
                info['expected'] = values
                print(f"[+] Expected from byte literal: {[hex(v) for v in values]}")

    # Detect operations (in order of appearance)
    lines = source.splitlines()
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('#') or not stripped:
            continue

        # XOR
        if re.search(r'\^', stripped) and not re.search(r'#.*\^', line):
            if 'xor' not in info['operations']:
                info['operations'].append('xor')
                # Try to extract XOR key constant
                xor_const = re.search(r'\^\s*(0x[0-9a-fA-F]+|\d+)', stripped)
                if xor_const:
                    val = int(xor_const.group(1), 0)
                    if val not in [v for sublist in info['keys'] for v in (sublist if isinstance(sublist, list) else [sublist])]:
                        info['keys'].append(val)
                        print(f"[+] XOR constant: {hex(val)}")

        # Addition mod 256
        if re.search(r'[+]\s*(0x[0-9a-fA-F]+|\d+)\s*[)%]', stripped):
            if 'add' not in info['operations']:
                info['operations'].append('add')
                add_match = re.search(r'[+]\s*(0x[0-9a-fA-F]+|\d+)', stripped)
                if add_match:
                    val = int(add_match.group(1), 0)
                    info['keys'].append(val)
                    print(f"[+] ADD constant: {hex(val)}")

        # Subtraction mod 256
        if re.search(r'[-]\s*(0x[0-9a-fA-F]+|\d+)\s*[)%]', stripped):
            if 'sub' not in info['operations']:
                info['operations'].append('sub')
                sub_match = re.search(r'[-]\s*(0x[0-9a-fA-F]+|\d+)', stripped)
                if sub_match:
                    val = int(sub_match.group(1), 0)
                    info['keys'].append(val)
                    print(f"[+] SUB constant: {hex(val)}")

        # Bit shift / rotation
        if '<<' in stripped or '>>' in stripped:
            if re.search(r'<<.*>>', stripped) or re.search(r'>>.*<<', stripped):
                if 'rotate' not in info['operations']:
                    info['operations'].append('rotate')
                    rot_match = re.search(r'[<>]{2}\s*(\d+)', stripped)
                    if rot_match:
                        info['keys'].append(int(rot_match.group(1)))
                        print(f"[+] Rotation amount: {rot_match.group(1)}")

        # Nibble swap
        if re.search(r'>> 4.*<< 4|<< 4.*>> 4|swap.*nibble|nibble.*swap', stripped, re.I):
            if 'swap_nibbles' not in info['operations']:
                info['operations'].append('swap_nibbles')

        # Bitwise NOT
        if re.search(r'~\s*\w|NOT|invert', stripped, re.I):
            if 'not' not in info['operations']:
                info['operations'].append('not')

    print(f"[*] Detected operations: {info['operations']}")
    print(f"[*] Input format: {info['input_format']}")

    return info


def solve_from_info(info):
    """Compute the required input bytes by inverting the detected transformations."""
    if info['expected'] is None:
        print("[-] No expected values found in source.", file=sys.stderr)
        return None

    result = bytes(info['expected'])
    keys_iter = iter(info['keys'])

    # Apply inverse operations in REVERSE order
    for op in reversed(info['operations']):
        try:
            key = next(keys_iter) if info['keys'] else 0
        except StopIteration:
            key = 0

        if op == 'xor':
            result = xor_bytes(result, key)
            print(f"[*] Inverse XOR with {hex(key) if isinstance(key, int) else key}: {result.hex()}")
        elif op == 'add':
            result = sub_bytes_mod256(result, key)
            print(f"[*] Inverse ADD (subtract {hex(key) if isinstance(key, int) else key}): {result.hex()}")
        elif op == 'sub':
            result = add_bytes_mod256(result, key)
            print(f"[*] Inverse SUB (add {hex(key) if isinstance(key, int) else key}): {result.hex()}")
        elif op == 'rotate':
            # Determine direction from context (default: assume left rotation -> invert with right)
            if isinstance(key, int):
                result = bytes(ror_byte(b, key) for b in result)
            print(f"[*] Inverse rotation by {key}: {result.hex()}")
        elif op == 'swap_nibbles':
            result = swap_nibbles(result)
            print(f"[*] Swap nibbles: {result.hex()}")
        elif op == 'not':
            result = invert_bits(result)
            print(f"[*] Bitwise NOT: {result.hex()}")

    return result


def deliver_remote(host, port, payload, input_format):
    """Send the payload to a remote service."""
    if not HAS_PWNTOOLS:
        print("[!] pwntools not installed (pip install pwntools)")
        print(f"[*] Manual: echo -ne '{format_echo(payload)}' | nc {host} {port}")
        return

    print(f"[*] Connecting to {host}:{port}...")
    r = remote(host, port)

    try:
        prompt = r.recvuntil(b':', timeout=5)
        print(f"[*] Prompt: {prompt.decode(errors='replace')}")
    except Exception:
        prompt = r.recv(timeout=3)
        print(f"[*] Received: {prompt.decode(errors='replace')}")

    # Send in the detected format
    if input_format == 'hex':
        r.sendline(payload.hex().encode())
    elif input_format == 'decimal':
        r.sendline(' '.join(str(b) for b in payload).encode())
    elif input_format == 'binary':
        r.sendline(' '.join(format(b, '08b') for b in payload).encode())
    else:
        r.sendline(payload)

    try:
        response = r.recvall(timeout=5).decode(errors='replace')
        print(f"\n[*] Response:\n{response}")
        flag_match = re.search(r'picoCTF\{[^}]+\}', response)
        if flag_match:
            print(f"\n[+] FLAG: {flag_match.group()}")
    except Exception as e:
        print(f"[!] Error: {e}")

    r.close()


def deliver_local(program_path, payload, input_format):
    """Run the local program and pipe the payload."""
    formats_to_try = []

    if input_format == 'hex':
        formats_to_try = [('hex', payload.hex().encode() + b'\n')]
    elif input_format == 'decimal':
        formats_to_try = [('decimal', ' '.join(str(b) for b in payload).encode() + b'\n')]
    elif input_format == 'binary':
        formats_to_try = [('binary', ' '.join(format(b, '08b') for b in payload).encode() + b'\n')]
    else:
        # Try all formats
        formats_to_try = [
            ('hex', payload.hex().encode() + b'\n'),
            ('raw+newline', payload + b'\n'),
            ('raw', payload),
            ('decimal', ' '.join(str(b) for b in payload).encode() + b'\n'),
        ]

    for fmt_name, data in formats_to_try:
        print(f"\n[*] Trying {fmt_name}: {data[:60]!r}{'...' if len(data) > 60 else ''}")
        try:
            result = subprocess.run(
                ['python3', program_path],
                input=data,
                capture_output=True,
                timeout=10,
            )
            output = result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace')
            if output.strip():
                print(f"[*] Output: {output.strip()}")

            flag_match = re.search(r'picoCTF\{[^}]+\}', output)
            if flag_match:
                print(f"\n[+] FLAG: {flag_match.group()}")
                return True
        except subprocess.TimeoutExpired:
            print(f"[!] Timeout with {fmt_name}")
        except Exception as e:
            print(f"[!] Error: {e}")

    return False


def format_echo(payload):
    """Format bytes for use with echo -ne."""
    return ''.join(f'\\x{b:02x}' for b in payload)


def main():
    parser = argparse.ArgumentParser(description='bytemancy 1 solver - picoCTF 2026')
    parser.add_argument('--source', '-s', help='Path to challenge source file')
    parser.add_argument('--host', help='Remote host')
    parser.add_argument('--port', type=int, help='Remote port')
    parser.add_argument('--local', '-l', help='Path to local challenge program')
    parser.add_argument('--hex', action='store_true', help='Output payload as hex')
    parser.add_argument('--raw', action='store_true', help='Output raw bytes to stdout')
    args = parser.parse_args()

    print("=" * 60)
    print("  bytemancy 1 - picoCTF 2026 Solver")
    print("  General Skills | 100 pts")
    print("=" * 60)
    print()

    # Find source code
    source_path = args.source
    if not source_path:
        candidates = [
            'bytemancy1.py', 'bytemancy.py', 'challenge.py', 'program.py',
            'main.py', 'chall.py', 'source.py', 'bytemancy_1.py',
        ]
        for c in candidates:
            if os.path.exists(c):
                source_path = c
                break

    payload = None
    input_format = 'raw'

    if source_path and os.path.exists(source_path):
        info = parse_source(source_path)
        input_format = info['input_format']
        payload = solve_from_info(info)
    else:
        print("[*] No source code found.")
        print("[*] Download the source from the challenge page and run:")
        print("    python3 solve.py --source <source_file.py>")
        print()
        print("[*] Or place the source file in the current directory as one of:")
        print("    bytemancy1.py, bytemancy.py, challenge.py, program.py")
        print()

    if payload is None:
        print("[*] Could not auto-compute payload.")
        print("[*] Common bytemancy 1 patterns to try manually:")
        print()
        print("  # Pattern 1: XOR with single key byte")
        print("  expected = [...]  # from source code")
        print("  key = 0x42       # from source code")
        print("  payload = bytes(b ^ key for b in expected)")
        print()
        print("  # Pattern 2: Addition mod 256")
        print("  payload = bytes((b - offset) % 256 for b in expected)")
        print()
        print("  # Pattern 3: Multi-step (XOR then add)")
        print("  step1 = bytes(b ^ xor_key for b in expected)")
        print("  payload = bytes((b - add_key) % 256 for b in step1)")
        print()
        print("  # Pattern 4: Byte-by-byte with different keys")
        print("  payload = bytes(expected[i] ^ keys[i] for i in range(len(expected)))")
        print()
        sys.exit(1)

    # Display the solution
    print(f"\n[+] Solution ({len(payload)} bytes):")
    print(f"    Hex:     {payload.hex()}")
    print(f"    Decimal: {list(payload)}")
    try:
        if all(0x20 <= b <= 0x7e for b in payload):
            print(f"    ASCII:   {payload.decode('ascii')}")
        else:
            print(f"    Repr:    {payload!r}")
    except Exception:
        print(f"    Repr:    {payload!r}")

    # Deliver the payload
    if args.host and args.port:
        deliver_remote(args.host, args.port, payload, input_format)
    elif args.local:
        deliver_local(args.local, payload, input_format)
    elif args.raw:
        sys.stdout.buffer.write(payload)
        sys.stdout.buffer.flush()
    elif args.hex:
        print(payload.hex())
    else:
        print(f"\n[*] To solve:")
        print(f"    python3 solve.py --local <program.py>       # local")
        print(f"    python3 solve.py --host <HOST> --port <PORT> # remote")
        print(f"    python3 solve.py --raw | python3 bytemancy1.py")
        print(f"\n[*] Manual echo command:")
        print(f"    echo -ne '{format_echo(payload)}' | python3 bytemancy1.py")
        print(f"    echo -ne '{format_echo(payload)}' | nc <HOST> <PORT>")


if __name__ == '__main__':
    main()
