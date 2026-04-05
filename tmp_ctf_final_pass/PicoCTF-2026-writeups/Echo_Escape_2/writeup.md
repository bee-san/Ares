# Echo Escape 2 - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 100

## Challenge Description

The developer has learned their lesson from unsafe input functions and tried to secure the program by using `fgets()`. Unfortunately, they missed a critical vulnerability -- a format string bug.

## Approach

This challenge is a classic format string vulnerability. The developer replaced dangerous input functions like `gets()` or `scanf("%s")` with the safer `fgets()`, which prevents buffer overflow by limiting input length. However, they made the mistake of passing user input directly to `printf()` without a format specifier:

```c
// Vulnerable pattern:
fgets(buf, sizeof(buf), stdin);
printf(buf);    // FORMAT STRING BUG -- should be printf("%s", buf);
```

This is the sequel to "Echo Escape" (likely a basic buffer overflow), where the developer tried to fix the input handling but introduced a new vulnerability class entirely.

### Format String Exploitation

A format string vulnerability allows an attacker to:

1. **Read from the stack** using `%p`, `%x`, or `%s` format specifiers
2. **Write to arbitrary memory** using the `%n` format specifier
3. **Leak memory addresses** to defeat ASLR/PIE

### Exploitation Strategy

Since this is a 100-point challenge with 1368 solves, it is likely a straightforward format string exploit. The typical approach:

1. **Leak addresses**: Use `%p` specifiers to dump stack values and find useful addresses (return address, libc addresses, binary base if PIE is enabled).
2. **Find the offset**: Determine at which position on the stack our input buffer appears (the format string offset).
3. **Overwrite the return address** (or GOT entry) to redirect execution to a `win`/`print_flag` function, or to a `system("/bin/sh")` call.

Given the point value and solve count, the binary likely has:
- A `win()` or `print_flag()` function that reads and displays the flag
- The goal is to overwrite the return address of the vulnerable function to jump to `win()`
- PIE may or may not be enabled; if enabled, we leak the base address first

### Finding the Format String Offset

Send a pattern like `AAAA%p.%p.%p.%p.%p.%p.%p.%p.%p.%p` and look for `0x41414141` (or `0x4141414141414141` on 64-bit) in the output. The position where it appears is the offset.

For example, if `0x4141414141414141` appears at position 6, then `%6$p` directly accesses our input on the stack.

## Solution

### Step 1: Find the format string offset

```
$ echo 'AAAAAAAA%p.%p.%p.%p.%p.%p.%p.%p.%p.%p' | ./echo_escape_2
```

Look for `0x4141414141414141` in the output. Count which position it is.

### Step 2: Leak addresses (if PIE is enabled)

```
$ echo '%21$p' | ./echo_escape_2
```

Leak the return address or a known function pointer. Calculate the binary base and the address of `win()`.

### Step 3: Overwrite return address

Use pwntools' `fmtstr_payload()` to craft a write-what-where payload:
- **What**: Address of `win()` / `print_flag()`
- **Where**: Return address location on the stack, or a GOT entry (e.g., `exit@GOT` if `exit()` is called after the vulnerable `printf`)

### Step 4: Trigger the overwritten pointer

When the function returns (or `exit()` is called), execution redirects to `win()` and the flag is printed.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
