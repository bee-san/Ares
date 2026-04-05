#!/usr/bin/env python3
"""
bytemancy 0 - picoCTF 2026 (General Skills, 50 pts)

First challenge in the bytemancy series -- basic byte manipulation.
The program's source code is provided and expects specific byte input.

This script analyzes common bytemancy patterns and sends the correct
bytes to retrieve the flag.

Usage:
    python3 solve.py                          # Interactive / auto mode
    python3 solve.py --host HOST --port PORT  # Remote service mode
    python3 solve.py --local program.py       # Local program mode
    python3 solve.py --source source.py       # Analyze source and solve

Dependencies: pwntools (pip install pwntools)
"""

import argparse
import sys
import re
import os

# Try to import pwntools for remote connections
try:
    from pwn import *
    HAS_PWNTOOLS = True
except ImportError:
    HAS_PWNTOOLS = False


def analyze_source(source_path):
    """
    Analyze the bytemancy source code to determine:
    1. What input format it expects (hex, decimal, raw bytes)
    2. What transformation/check it performs
    3. What the target bytes are
    """
    print(f"[*] Analyzing source: {source_path}")

    with open(source_path, 'r') as f:
        source = f.read()

    print(f"[*] Source code ({len(source)} bytes):")
    print("-" * 60)
    print(source)
    print("-" * 60)

    # --- Pattern detection ---

    # Look for target byte arrays or hex strings
    hex_patterns = re.findall(r'(?:0x[0-9a-fA-F]+[,\s]*)+', source)
    byte_patterns = re.findall(r'b["\']([^"\']+)["\']', source)
    bytearray_patterns = re.findall(r'(?:bytearray|bytes)\s*\(\s*\[([^\]]+)\]', source)
    fromhex_patterns = re.findall(r'bytes\.fromhex\s*\(\s*["\']([0-9a-fA-F]+)["\']', source)
    hex_string_patterns = re.findall(r'["\']([0-9a-fA-F]{4,})["\']', source)

    target_bytes = None

    # Check for bytes.fromhex() calls
    if fromhex_patterns:
        print(f"[+] Found fromhex pattern: {fromhex_patterns[0]}")
        target_bytes = bytes.fromhex(fromhex_patterns[0])

    # Check for bytearray/bytes with list of ints
    elif bytearray_patterns:
        nums = [int(x.strip(), 0) for x in bytearray_patterns[0].split(',') if x.strip()]
        print(f"[+] Found byte array: {nums}")
        target_bytes = bytes(nums)

    # Check for hex string literals
    elif hex_string_patterns:
        for pat in hex_string_patterns:
            if len(pat) % 2 == 0:
                try:
                    candidate = bytes.fromhex(pat)
                    print(f"[+] Found hex string: {pat}")
                    target_bytes = candidate
                    break
                except ValueError:
                    continue

    # Detect XOR operations
    xor_patterns = re.findall(r'\^\s*(0x[0-9a-fA-F]+|\d+)', source)
    if xor_patterns:
        xor_key = int(xor_patterns[0], 0)
        print(f"[+] Found XOR key: {xor_key:#x}")
        if target_bytes:
            # Reverse the XOR to find the input
            target_bytes = bytes(b ^ xor_key for b in target_bytes)
            print(f"[+] XOR-reversed target: {target_bytes.hex()}")

    # Detect addition/subtraction
    add_patterns = re.findall(r'[+]\s*(0x[0-9a-fA-F]+|\d+)\s*[)%]', source)
    sub_patterns = re.findall(r'[-]\s*(0x[0-9a-fA-F]+|\d+)\s*[)%]', source)

    if add_patterns and target_bytes:
        add_val = int(add_patterns[0], 0)
        print(f"[+] Found addition: +{add_val}")
        target_bytes = bytes((b - add_val) % 256 for b in target_bytes)
        print(f"[+] Reversed addition: {target_bytes.hex()}")

    if sub_patterns and target_bytes:
        sub_val = int(sub_patterns[0], 0)
        print(f"[+] Found subtraction: -{sub_val}")
        target_bytes = bytes((b + sub_val) % 256 for b in target_bytes)
        print(f"[+] Reversed subtraction: {target_bytes.hex()}")

    if target_bytes:
        print(f"\n[+] Target bytes (hex): {target_bytes.hex()}")
        print(f"[+] Target bytes (decimal): {list(target_bytes)}")
        try:
            print(f"[+] Target bytes (ASCII): {target_bytes.decode('ascii', errors='replace')}")
        except Exception:
            pass

    return target_bytes, source


def solve_common_patterns():
    """
    Try common bytemancy patterns when no source is available.
    Returns a list of (description, payload) tuples to try.
    """
    payloads = []

    # Pattern 1: Simple hex string input
    # The program might ask for hex-encoded bytes
    payloads.append(("hex string input", None))

    # Pattern 2: Specific byte values
    # Common in introductory challenges
    payloads.append(("raw bytes", None))

    return payloads


def solve_remote(host, port, payload_bytes):
    """Connect to remote service and send the payload."""
    if not HAS_PWNTOOLS:
        print("[!] pwntools not installed. Install with: pip install pwntools")
        print(f"[*] Manual solution: echo -ne '{format_echo_payload(payload_bytes)}' | nc {host} {port}")
        return

    print(f"[*] Connecting to {host}:{port}")
    r = remote(host, port)

    # Read the initial prompt
    try:
        prompt = r.recvuntil(b':', timeout=5)
        print(f"[*] Prompt: {prompt.decode(errors='replace')}")
    except Exception:
        prompt = r.recv(timeout=3)
        print(f"[*] Received: {prompt.decode(errors='replace')}")

    # Determine input format from the prompt
    prompt_text = prompt.decode(errors='replace').lower()

    if 'hex' in prompt_text:
        # Send hex-encoded
        print(f"[*] Sending hex: {payload_bytes.hex()}")
        r.sendline(payload_bytes.hex().encode())
    elif 'decimal' in prompt_text or 'number' in prompt_text:
        # Send space-separated decimal values
        dec_str = ' '.join(str(b) for b in payload_bytes)
        print(f"[*] Sending decimal: {dec_str}")
        r.sendline(dec_str.encode())
    elif 'binary' in prompt_text or 'bit' in prompt_text:
        # Send binary representation
        bin_str = ' '.join(format(b, '08b') for b in payload_bytes)
        print(f"[*] Sending binary: {bin_str}")
        r.sendline(bin_str.encode())
    else:
        # Try sending raw bytes
        print(f"[*] Sending raw bytes: {payload_bytes.hex()}")
        r.send(payload_bytes)

    # Receive the response
    try:
        response = r.recvall(timeout=5).decode(errors='replace')
        print(f"\n[*] Response:\n{response}")

        # Extract flag
        flag_match = re.search(r'picoCTF\{[^}]+\}', response)
        if flag_match:
            print(f"\n[+] FLAG: {flag_match.group()}")
        else:
            print("[!] No flag found in response.")
            print("[*] Try different input format or check the source code.")
    except Exception as e:
        print(f"[!] Error receiving response: {e}")

    r.close()


def solve_local(program_path, payload_bytes):
    """Run local program with the payload."""
    import subprocess

    print(f"[*] Running: python3 {program_path}")

    # Try different input formats
    for fmt_name, fmt_data in [
        ("hex", payload_bytes.hex().encode() + b'\n'),
        ("raw", payload_bytes),
        ("decimal", (' '.join(str(b) for b in payload_bytes)).encode() + b'\n'),
    ]:
        print(f"\n[*] Trying {fmt_name} input: {fmt_data!r}")
        try:
            result = subprocess.run(
                ['python3', program_path],
                input=fmt_data,
                capture_output=True,
                timeout=10,
            )
            output = result.stdout.decode(errors='replace') + result.stderr.decode(errors='replace')
            print(f"[*] Output: {output}")

            flag_match = re.search(r'picoCTF\{[^}]+\}', output)
            if flag_match:
                print(f"\n[+] FLAG: {flag_match.group()}")
                return
        except subprocess.TimeoutExpired:
            print(f"[!] Timeout with {fmt_name} input")
        except Exception as e:
            print(f"[!] Error: {e}")


def format_echo_payload(payload_bytes):
    """Format bytes as an echo -ne compatible string."""
    return ''.join(f'\\x{b:02x}' for b in payload_bytes)


def main():
    parser = argparse.ArgumentParser(description='bytemancy 0 solver - picoCTF 2026')
    parser.add_argument('--host', help='Remote host')
    parser.add_argument('--port', type=int, help='Remote port')
    parser.add_argument('--local', help='Path to local program')
    parser.add_argument('--source', help='Path to source code to analyze')
    args = parser.parse_args()

    print("=" * 60)
    print("  bytemancy 0 - picoCTF 2026 Solver")
    print("  General Skills | 50 pts")
    print("=" * 60)
    print()

    payload_bytes = None

    # Step 1: Try to find and analyze source code
    source_files = ['bytemancy0.py', 'bytemancy.py', 'challenge.py', 'program.py', 'main.py']
    source_path = args.source

    if not source_path:
        for sf in source_files:
            if os.path.exists(sf):
                source_path = sf
                break

    if source_path and os.path.exists(source_path):
        payload_bytes, source = analyze_source(source_path)
    else:
        print("[*] No source code found locally.")
        print("[*] Download the source code from the challenge and either:")
        print("    - Place it in the current directory")
        print("    - Use --source <path> to specify its location")
        print()

    if payload_bytes is None:
        print("[*] Could not auto-detect payload from source.")
        print("[*] Demonstrating common bytemancy 0 solution patterns:")
        print()
        print("    # Pattern 1: Convert hex string to bytes")
        print("    payload = bytes.fromhex('DEADBEEF')")
        print()
        print("    # Pattern 2: XOR each byte with a key")
        print("    target = [0x70, 0x69, 0x63, 0x6f]  # 'pico'")
        print("    key = 0x42")
        print("    payload = bytes(b ^ key for b in target)")
        print()
        print("    # Pattern 3: Reverse byte order")
        print("    payload = target_bytes[::-1]")
        print()
        print("    # Pattern 4: Add/subtract mod 256")
        print("    payload = bytes((b - offset) % 256 for b in target)")
        print()

        # Use a placeholder for demo
        print("[*] Using placeholder payload for demonstration.")
        print("[*] Update this script with the actual target from the source code.")
        payload_bytes = b'\x00' * 4  # placeholder

    # Step 2: Deliver the payload
    if args.host and args.port:
        solve_remote(args.host, args.port, payload_bytes)
    elif args.local:
        solve_local(args.local, payload_bytes)
    else:
        print(f"\n[*] Computed payload (hex):     {payload_bytes.hex()}")
        print(f"[*] Computed payload (decimal): {list(payload_bytes)}")
        try:
            print(f"[*] Computed payload (ASCII):   {payload_bytes.decode('ascii', errors='replace')}")
        except Exception:
            pass
        print(f"\n[*] To send to a remote service:")
        print(f"    python3 solve.py --host <HOST> --port <PORT>")
        print(f"\n[*] To run against a local program:")
        print(f"    python3 solve.py --local <program.py>")
        print(f"\n[*] Manual netcat command:")
        print(f"    echo -ne '{format_echo_payload(payload_bytes)}' | nc <HOST> <PORT>")


if __name__ == '__main__':
    main()
