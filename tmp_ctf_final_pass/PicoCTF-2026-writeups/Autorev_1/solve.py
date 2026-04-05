#!/usr/bin/env python3
"""
Autorev 1 - picoCTF 2026
Category: Reverse Engineering | Points: 200

Automated reverse engineering solver that uses angr for symbolic execution
to quickly solve binaries within a time constraint.

The challenge likely sends binaries that must be solved rapidly. This script:
1. Connects to the challenge server
2. Downloads/receives the binary
3. Uses angr to find the correct input via symbolic execution
4. Sends the answer back before the timeout

Usage:
    python3 solve.py <host> <port>
    python3 solve.py [path_to_binary]
"""

import angr
import claripy
import sys
import os
import re
import base64
import tempfile
import struct
import subprocess

# Optional: for network interaction
try:
    from pwn import *
    HAS_PWNTOOLS = True
except ImportError:
    import socket
    HAS_PWNTOOLS = False

FLAG_PATTERN = re.compile(rb"picoCTF\{[^}]+\}")


# ──────────────────────────────────────────────────────────────────
# angr-based binary solver
# ──────────────────────────────────────────────────────────────────

def solve_with_angr(binary_path, find_strs=None, avoid_strs=None):
    """
    Use angr symbolic execution to find input that reaches a 'success' state.

    Args:
        binary_path: Path to the ELF binary
        find_strs: List of byte strings indicating success (e.g., [b"Correct"])
        avoid_strs: List of byte strings indicating failure (e.g., [b"Wrong"])

    Returns:
        The solution input as bytes, or None if no solution found.
    """
    if find_strs is None:
        find_strs = [b"Correct", b"correct", b"Success", b"success",
                      b"Good", b"Right", b"right", b"Yes", b"FLAG",
                      b"picoCTF", b"Congrat", b"Well done", b"Access granted"]
    if avoid_strs is None:
        avoid_strs = [b"Wrong", b"wrong", b"Incorrect", b"incorrect",
                       b"Fail", b"fail", b"No", b"denied", b"Invalid",
                       b"Try again", b"Bad"]

    print(f"[*] Loading binary: {binary_path}")
    proj = angr.Project(binary_path, auto_load_libs=False)

    # ── Strategy 1: Use stdin as symbolic input ──
    print("[*] Strategy 1: Symbolic stdin exploration")
    state = proj.factory.entry_state(
        stdin=angr.SimFile("/dev/stdin", content=claripy.BVS("stdin", 200 * 8)),
        add_options={
            angr.options.ZERO_FILL_UNCONSTRAINED_MEMORY,
            angr.options.ZERO_FILL_UNCONSTRAINED_REGISTERS,
        }
    )

    simgr = proj.factory.simulation_manager(state)

    # Define find/avoid conditions based on output strings
    def is_find(s):
        output = s.posix.dumps(1)  # stdout
        return any(fs in output for fs in find_strs)

    def is_avoid(s):
        output = s.posix.dumps(1)  # stdout
        return any(av in output for av in avoid_strs)

    print("[*] Exploring paths...")
    simgr.explore(find=is_find, avoid=is_avoid)

    if simgr.found:
        found_state = simgr.found[0]
        solution = found_state.posix.dumps(0)  # stdin
        # Clean the solution: strip null bytes and trailing garbage
        solution = solution.split(b"\x00")[0].strip()
        print(f"[+] Solution found: {solution}")

        # Check if the flag is in stdout
        stdout_output = found_state.posix.dumps(1)
        flag_match = FLAG_PATTERN.search(stdout_output)
        if flag_match:
            print(f"[+] Flag in output: {flag_match.group().decode()}")

        return solution

    print("[-] Strategy 1 failed, trying Strategy 2...")

    # ── Strategy 2: Use argv as symbolic input ──
    print("[*] Strategy 2: Symbolic argv exploration")
    argv_sym = claripy.BVS("argv1", 100 * 8)
    state2 = proj.factory.entry_state(
        args=[binary_path, argv_sym],
        add_options={
            angr.options.ZERO_FILL_UNCONSTRAINED_MEMORY,
            angr.options.ZERO_FILL_UNCONSTRAINED_REGISTERS,
        }
    )

    simgr2 = proj.factory.simulation_manager(state2)
    simgr2.explore(find=is_find, avoid=is_avoid)

    if simgr2.found:
        found_state = simgr2.found[0]
        solution = found_state.solver.eval(argv_sym, cast_to=bytes)
        solution = solution.split(b"\x00")[0].strip()
        print(f"[+] Solution found (argv): {solution}")

        stdout_output = found_state.posix.dumps(1)
        flag_match = FLAG_PATTERN.search(stdout_output)
        if flag_match:
            print(f"[+] Flag in output: {flag_match.group().decode()}")

        return solution

    print("[-] Strategy 2 failed, trying Strategy 3...")

    # ── Strategy 3: Find target address from strings ──
    print("[*] Strategy 3: Address-based exploration")
    cfg = proj.analyses.CFGFast()

    find_addrs = []
    avoid_addrs = []

    for func_addr in cfg.functions:
        func = cfg.functions[func_addr]
        for block in func.blocks:
            try:
                block_bytes = proj.loader.memory.load(block.addr, block.size)
                for fs in find_strs:
                    if fs in block_bytes:
                        find_addrs.append(block.addr)
                for av in avoid_strs:
                    if av in block_bytes:
                        avoid_addrs.append(block.addr)
            except:
                pass

    if find_addrs:
        state3 = proj.factory.entry_state(
            stdin=angr.SimFile("/dev/stdin", content=claripy.BVS("stdin2", 200 * 8)),
            add_options={
                angr.options.ZERO_FILL_UNCONSTRAINED_MEMORY,
                angr.options.ZERO_FILL_UNCONSTRAINED_REGISTERS,
            }
        )
        simgr3 = proj.factory.simulation_manager(state3)
        simgr3.explore(find=find_addrs, avoid=avoid_addrs)

        if simgr3.found:
            found_state = simgr3.found[0]
            solution = found_state.posix.dumps(0)
            solution = solution.split(b"\x00")[0].strip()
            print(f"[+] Solution found (addr-based): {solution}")
            return solution

    print("[-] All angr strategies exhausted.")
    return None


def solve_with_strings(binary_path):
    """Quick heuristic: look for hardcoded flags or passwords in strings."""
    print("[*] Checking strings for embedded flag/password...")
    result = subprocess.run(
        ["strings", binary_path], capture_output=True, text=True
    )
    # Check for flag directly in strings
    flag_match = re.search(r"picoCTF\{[^}]+\}", result.stdout)
    if flag_match:
        print(f"[+] Flag found in strings: {flag_match.group()}")
        return flag_match.group().encode()
    return None


# ──────────────────────────────────────────────────────────────────
# Network interaction (for server-based challenges)
# ──────────────────────────────────────────────────────────────────

def solve_remote(host, port):
    """
    Connect to the challenge server, receive binaries, solve them,
    and send answers back within the time limit.
    """
    if HAS_PWNTOOLS:
        conn = remote(host, int(port))
    else:
        conn = socket.create_connection((host, int(port)))

    round_num = 0
    while True:
        round_num += 1
        print(f"\n{'=' * 60}")
        print(f"Round {round_num}")
        print(f"{'=' * 60}")

        if HAS_PWNTOOLS:
            data = conn.recvuntil(b"\n", timeout=10)
            data_str = data.decode(errors="replace")
        else:
            data = conn.recv(65536)
            data_str = data.decode(errors="replace")

        print(f"[*] Received: {data_str[:200]}...")

        # Check if we got a flag already
        flag_match = FLAG_PATTERN.search(data)
        if flag_match:
            print(f"[+] FLAG: {flag_match.group().decode()}")
            break

        # Try to extract a binary (base64 encoded)
        b64_match = re.search(r"[A-Za-z0-9+/]{100,}={0,2}", data_str)
        if b64_match:
            binary_data = base64.b64decode(b64_match.group())
        elif b"ELF" in data:
            # Raw binary data
            elf_start = data.index(b"\x7fELF")
            binary_data = data[elf_start:]
        else:
            # Maybe there's a download URL
            url_match = re.search(r"(https?://\S+)", data_str)
            if url_match:
                import urllib.request
                binary_data = urllib.request.urlopen(url_match.group()).read()
            else:
                print("[-] Could not extract binary from server data")
                if HAS_PWNTOOLS:
                    # Try receiving more
                    more_data = conn.recv(timeout=5)
                    data += more_data
                    data_str = data.decode(errors="replace")
                    flag_match = FLAG_PATTERN.search(data)
                    if flag_match:
                        print(f"[+] FLAG: {flag_match.group().decode()}")
                        break
                continue

        # Write binary to temp file
        with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as f:
            f.write(binary_data)
            bin_path = f.name
        os.chmod(bin_path, 0o755)

        print(f"[*] Binary saved to {bin_path} ({len(binary_data)} bytes)")

        # Solve with angr
        solution = solve_with_strings(bin_path) or solve_with_angr(bin_path)

        # Cleanup temp binary
        os.unlink(bin_path)

        if solution:
            print(f"[*] Sending solution: {solution}")
            if HAS_PWNTOOLS:
                conn.sendline(solution)
            else:
                conn.sendall(solution + b"\n")
        else:
            print("[-] Could not solve this round")
            break

    # Final receive to catch any flag output
    try:
        if HAS_PWNTOOLS:
            final = conn.recvall(timeout=5)
        else:
            final = conn.recv(65536)
        flag_match = FLAG_PATTERN.search(final)
        if flag_match:
            print(f"[+] FLAG: {flag_match.group().decode()}")
    except:
        pass

    if HAS_PWNTOOLS:
        conn.close()


def solve_local(binary_path):
    """Solve a single local binary."""
    binary_path = os.path.abspath(binary_path)
    if not os.path.exists(binary_path):
        print(f"[-] File not found: {binary_path}")
        return

    print(f"[*] Solving local binary: {binary_path}")

    # Try strings first (fastest)
    result = solve_with_strings(binary_path)
    if result:
        print(f"[+] Solution: {result.decode(errors='replace')}")
        # Run binary with found input
        proc = subprocess.run(
            [binary_path], input=result, capture_output=True, timeout=10
        )
        print(f"[*] Output: {proc.stdout.decode(errors='replace')}")
        return

    # Try angr (comprehensive)
    result = solve_with_angr(binary_path)
    if result:
        print(f"[+] Solution: {result.decode(errors='replace')}")
        # Run binary with found input
        proc = subprocess.run(
            [binary_path], input=result, capture_output=True, timeout=10
        )
        output = proc.stdout.decode(errors="replace")
        print(f"[*] Output: {output}")
        flag_match = FLAG_PATTERN.search(output.encode())
        if flag_match:
            print(f"[+] FLAG: {flag_match.group().decode()}")
    else:
        print("[-] Could not solve binary automatically")
        print("    Manual analysis steps:")
        print("    1. ghidra or IDA: decompile and find check function")
        print("    2. gdb: set breakpoint at comparison, examine registers")
        print("    3. ltrace/strace: trace library/system calls")
        print("    4. r2 -A binary; afl; pdf @main")


def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python3 solve.py <host> <port>     # Remote challenge")
        print("  python3 solve.py <binary_path>      # Local binary")
        print("")
        print("Examples:")
        print("  python3 solve.py titan.picoctf.net 52847")
        print("  python3 solve.py ./challenge_binary")
        sys.exit(1)

    if len(sys.argv) == 3 and sys.argv[2].isdigit():
        # Remote mode
        solve_remote(sys.argv[1], sys.argv[2])
    else:
        # Local mode
        solve_local(sys.argv[1])


if __name__ == "__main__":
    main()
