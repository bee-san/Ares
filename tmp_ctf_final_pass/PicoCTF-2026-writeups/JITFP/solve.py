#!/usr/bin/env python3
"""
JITFP - picoCTF 2026 (Reverse Engineering, 500 pts)

This solver connects to the remote JITFP service, retrieves the JIT-compiled
password checker binary (if served), and uses Z3 to symbolically solve for the
correct password. If the binary must be analyzed locally, it extracts the JIT
code buffer, disassembles it to find the per-byte transformations, and inverts
them to recover the password.

Approach:
  1. Connect to the remote service.
  2. Download or dump the binary's JIT code region.
  3. Parse the emitted x86 instructions to extract XOR keys and expected values.
  4. Solve the constraints (invert the per-character transformation).
  5. Submit the password and capture the flag.

Usage:
  python3 solve.py <host> <port>         -- solve against remote
  python3 solve.py --binary ./jitfp      -- extract from local binary with GDB
"""

import argparse
import struct
import sys
import subprocess
import tempfile
import os
import re

# ── Attempt to import optional dependencies ──────────────────────────────
try:
    from pwn import *
    HAS_PWN = True
except ImportError:
    HAS_PWN = False

try:
    from z3 import *
    HAS_Z3 = True
except ImportError:
    HAS_Z3 = False

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    HAS_CAPSTONE = True
except ImportError:
    HAS_CAPSTONE = False


# ── JIT code extraction via GDB ─────────────────────────────────────────

GDB_SCRIPT = """
set pagination off
set confirm off
break mmap
run <<< "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
finish
set $buf = $rax
continue
# After JIT code is written, dump it
dump binary memory /tmp/jit_dump.bin $buf ($buf + 4096)
quit
"""

def extract_jit_code_gdb(binary_path):
    """Use GDB to run the binary and dump the JIT code buffer."""
    print("[*] Extracting JIT code via GDB...")
    with tempfile.NamedTemporaryFile(mode='w', suffix='.gdb', delete=False) as f:
        f.write(GDB_SCRIPT)
        gdb_script_path = f.name

    try:
        result = subprocess.run(
            ['gdb', '-batch', '-x', gdb_script_path, binary_path],
            capture_output=True, text=True, timeout=30
        )
        print(f"[*] GDB stdout: {result.stdout[:500]}")
        if os.path.exists('/tmp/jit_dump.bin'):
            with open('/tmp/jit_dump.bin', 'rb') as f:
                return f.read()
        else:
            print("[-] JIT dump not created. Trying alternate extraction...")
            return None
    except (subprocess.TimeoutExpired, FileNotFoundError) as e:
        print(f"[-] GDB extraction failed: {e}")
        return None
    finally:
        os.unlink(gdb_script_path)


# ── Disassemble and parse JIT code ──────────────────────────────────────

def disassemble_jit(code_bytes, base_addr=0x0):
    """Disassemble JIT code and extract per-byte XOR keys and expected values."""
    if not HAS_CAPSTONE:
        print("[-] capstone not installed. Install with: pip install capstone")
        return None, None

    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = True

    xor_keys = []
    expected_values = []
    current_xor = None

    instructions = list(md.disasm(code_bytes, base_addr))
    print(f"[*] Disassembled {len(instructions)} instructions from JIT buffer")

    for i, insn in enumerate(instructions):
        mnem = insn.mnemonic
        op_str = insn.op_str

        # Look for patterns like: xor <reg>, <immediate>
        # These are the per-byte XOR keys
        if mnem == 'xor' and '0x' in op_str:
            match = re.search(r'0x([0-9a-fA-F]+)', op_str)
            if match:
                current_xor = int(match.group(1), 16) & 0xFF
                xor_keys.append(current_xor)

        # Look for patterns like: cmp <reg>, <immediate>
        # These are the expected values after transformation
        if mnem == 'cmp' and '0x' in op_str:
            match = re.search(r'0x([0-9a-fA-F]+)', op_str)
            if match:
                val = int(match.group(1), 16) & 0xFF
                expected_values.append(val)

        # Also look for: add <reg>, <immediate> (rotation/shift constants)
        # sub <reg>, <immediate>
        # rol/ror <reg>, <immediate>

    return xor_keys, expected_values


# ── Solve with Z3 ───────────────────────────────────────────────────────

def solve_z3(xor_keys, expected_values, add_consts=None):
    """Use Z3 to solve for the password given the extracted constraints."""
    if not HAS_Z3:
        print("[-] z3 not installed. Install with: pip install z3-solver")
        return solve_manual(xor_keys, expected_values, add_consts)

    n = min(len(xor_keys), len(expected_values))
    print(f"[*] Solving {n} character constraints with Z3...")

    solver = Solver()
    chars = [BitVec(f'c{i}', 8) for i in range(n)]

    for i in range(n):
        # Each character must be printable ASCII
        solver.add(chars[i] >= 0x20)
        solver.add(chars[i] <= 0x7e)

        # Apply the transformation: (input[i] ^ xor_key[i]) + add_const[i] == expected[i]
        transformed = chars[i] ^ xor_keys[i]
        if add_consts and i < len(add_consts):
            transformed = transformed + add_consts[i]
        solver.add(transformed == expected_values[i])

    if solver.check() == sat:
        model = solver.model()
        password = ''.join(chr(model[c].as_long()) for c in chars)
        return password
    else:
        print("[-] Z3 found no solution. Transformation model may be incorrect.")
        return None


# ── Manual solve (invert XOR) ───────────────────────────────────────────

def solve_manual(xor_keys, expected_values, add_consts=None):
    """Invert the transformation manually: input[i] = expected[i] ^ xor_key[i]"""
    n = min(len(xor_keys), len(expected_values))
    password = []
    for i in range(n):
        val = expected_values[i]
        if add_consts and i < len(add_consts):
            val = (val - add_consts[i]) & 0xFF
        ch = val ^ xor_keys[i]
        password.append(chr(ch) if 0x20 <= ch <= 0x7e else '?')
    return ''.join(password)


# ── Remote interaction ──────────────────────────────────────────────────

def solve_remote(host, port, password):
    """Connect to the remote service, submit the password, and get the flag."""
    if not HAS_PWN:
        print("[-] pwntools not installed. Install with: pip install pwntools")
        print(f"[*] Password to submit manually: {password}")
        print(f"[*] Connect with: nc {host} {port}")
        return None

    print(f"[*] Connecting to {host}:{port}...")
    r = remote(host, int(port))

    # Wait for the password prompt
    r.recvuntil(b':', timeout=10)
    print(f"[*] Sending password: {password}")
    r.sendline(password.encode())

    # Receive the response (hopefully the flag)
    try:
        response = r.recvall(timeout=10).decode(errors='replace')
        print(f"[*] Response:\n{response}")

        # Extract flag
        flag_match = re.search(r'picoCTF\{[^}]+\}', response)
        if flag_match:
            flag = flag_match.group(0)
            print(f"\n[+] FLAG: {flag}")
            return flag
        else:
            print("[-] No flag found in response.")
            return None
    except Exception as e:
        print(f"[-] Error receiving response: {e}")
        return None
    finally:
        r.close()


# ── Alternative: Brute-force with side-channel timing ────────────────────

def timing_attack(host, port, max_len=32):
    """
    If the JIT code checks characters one at a time and exits early on mismatch,
    we can use a timing side-channel to brute-force the password character by
    character.
    """
    if not HAS_PWN:
        print("[-] pwntools not installed for timing attack.")
        return None

    import time
    charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_!@#$%^&*()'
    password = ''

    print("[*] Attempting timing side-channel attack...")

    for pos in range(max_len):
        best_char = None
        best_time = 0
        samples = 5  # Average over multiple samples for reliability

        for ch in charset:
            candidate = password + ch + 'A' * (max_len - pos - 1)
            total_time = 0

            for _ in range(samples):
                try:
                    r = remote(host, int(port), level='error')
                    r.recvuntil(b':', timeout=5)
                    start = time.perf_counter()
                    r.sendline(candidate.encode())
                    r.recvline(timeout=5)
                    elapsed = time.perf_counter() - start
                    total_time += elapsed
                    r.close()
                except Exception:
                    total_time += 0
                    try:
                        r.close()
                    except Exception:
                        pass

            avg_time = total_time / samples
            if avg_time > best_time:
                best_time = avg_time
                best_char = ch

        password += best_char
        print(f"[*] Position {pos}: '{best_char}' (avg {best_time:.4f}s) -> {password}")

        # Check if we've found the full password (significant time drop for next position)
        try:
            r = remote(host, int(port), level='error')
            r.recvuntil(b':', timeout=5)
            r.sendline(password.encode())
            resp = r.recvall(timeout=5).decode(errors='replace')
            r.close()
            if 'picoCTF{' in resp:
                flag_match = re.search(r'picoCTF\{[^}]+\}', resp)
                if flag_match:
                    print(f"\n[+] FLAG: {flag_match.group(0)}")
                    return flag_match.group(0)
        except Exception:
            pass

    return password


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description='JITFP solver - picoCTF 2026')
    parser.add_argument('host', nargs='?', help='Remote host')
    parser.add_argument('port', nargs='?', help='Remote port')
    parser.add_argument('--binary', help='Path to local binary for JIT extraction')
    parser.add_argument('--timing', action='store_true', help='Use timing side-channel attack')
    parser.add_argument('--jit-dump', help='Path to pre-dumped JIT code file')
    args = parser.parse_args()

    password = None

    # Strategy 1: Extract and solve from JIT code dump
    if args.jit_dump:
        print(f"[*] Loading JIT dump from {args.jit_dump}")
        with open(args.jit_dump, 'rb') as f:
            jit_code = f.read()
        xor_keys, expected = disassemble_jit(jit_code)
        if xor_keys and expected:
            password = solve_z3(xor_keys, expected)
            if password:
                print(f"[+] Recovered password: {password}")

    # Strategy 2: Extract JIT code from local binary via GDB
    elif args.binary:
        jit_code = extract_jit_code_gdb(args.binary)
        if jit_code:
            xor_keys, expected = disassemble_jit(jit_code)
            if xor_keys and expected:
                password = solve_z3(xor_keys, expected)
                if password:
                    print(f"[+] Recovered password: {password}")

    # Strategy 3: Timing side-channel (if --timing flag is set)
    elif args.timing and args.host and args.port:
        flag = timing_attack(args.host, args.port)
        if flag:
            return
        else:
            print("[-] Timing attack did not recover the flag.")
            return

    # Submit to remote if we have a password
    if password and args.host and args.port:
        solve_remote(args.host, args.port, password)
    elif password:
        print(f"\n[+] Recovered password: {password}")
        print("[*] Submit this to the remote service to get the flag.")
    elif args.host and args.port:
        print("[*] No local binary or JIT dump provided.")
        print("[*] Attempting timing side-channel as fallback...")
        timing_attack(args.host, args.port)
    else:
        print("Usage:")
        print("  python3 solve.py <host> <port>                    # timing attack")
        print("  python3 solve.py <host> <port> --binary ./jitfp   # JIT extraction + remote")
        print("  python3 solve.py --binary ./jitfp                 # JIT extraction only")
        print("  python3 solve.py --jit-dump ./jit_dump.bin        # from pre-dumped JIT code")
        print("  python3 solve.py <host> <port> --timing           # timing side-channel")


if __name__ == '__main__':
    main()
