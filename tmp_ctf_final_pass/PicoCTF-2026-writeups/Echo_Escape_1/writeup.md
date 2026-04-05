# Echo Escape 1 - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 100

## Challenge Description
The "secure" echo service welcomes you politely... but what if you don't stay polite? Can you make it reveal the flag?

## Approach

This challenge is a classic **format string vulnerability** exploitation, following the same pattern as picoCTF's "Echo Valley" challenge from 2025. The program has an echo service that reads user input and prints it back using a vulnerable `printf(buf)` call instead of the safe `printf("%s", buf)`.

### Vulnerability Analysis

The vulnerable code pattern looks like:

```c
void echo_service() {
    char buf[100];
    while (1) {
        fgets(buf, sizeof(buf), stdin);
        if (strcmp(buf, "exit\n") == 0) break;
        printf(buf);  // VULNERABLE: user-controlled format string
    }
}

void print_flag() {
    // Reads and prints the flag file -- never called in normal execution
}
```

The key vulnerability is `printf(buf)` -- when the user supplies format specifiers like `%p`, `%x`, `%n`, etc., they can:
1. **Read from the stack** (information leak) using `%p` or `%x`
2. **Write to arbitrary memory** using `%n`

### Binary Protections

Based on the challenge difficulty (100 pts) and the series pattern:
- **PIE (Position Independent Executable)**: Likely enabled -- addresses are randomized
- **FULL RELRO**: Likely enabled -- GOT is read-only, cannot overwrite GOT entries
- **Stack Canary**: May or may not be present
- **NX**: Enabled -- stack is not executable

### Exploitation Strategy

Since the binary has PIE and FULL RELRO, the approach is:

1. **Leak the binary base address**: Use format string `%N$p` to leak a return address from the stack, then calculate the binary base by subtracting the known offset
2. **Leak a stack address**: Use another format string position to get a stack pointer, allowing us to calculate the location of the return address on the stack
3. **Overwrite the return address**: Use `fmtstr_payload()` from pwntools to write the address of `print_flag()` over the saved return address of `echo_service()`
4. **Trigger return**: Send "exit" to make the function return, which now jumps to `print_flag()`

### Key Offsets

From the Echo Valley reference:
- `%20$p` typically leaks a stack address
- `%21$p` typically leaks the return address (main function pointer)
- The format string offset (where our input appears on the stack) can be found using pwntools `FmtStr` auto-detection

## Solution

### Step 1: Find the format string offset
Send patterns like `AAAA%1$p.%2$p.%3$p...` and look for `0x41414141` in the output.

### Step 2: Leak addresses
```
%20$p.%21$p
```
This leaks a stack address and a code address.

### Step 3: Calculate base address and return address location
```python
base_address = leaked_code_addr - known_offset
ret_address = leaked_stack_addr + 8  # adjust based on binary
print_flag_addr = base_address + elf.sym['print_flag']
```

### Step 4: Overwrite return address
Use pwntools `fmtstr_payload()` to generate the write payload.

### Step 5: Exit to trigger the overwritten return
Send "exit" to make the function return to `print_flag()`.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
