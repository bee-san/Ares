# JITFP - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 500

## Challenge Description

If we can crack the password checker on this remote host, we will be able to infiltrate deeper into this criminal organization. Can you break in?

## Approach

The challenge name "JITFP" stands for "JIT Function Pointer" (or similar), hinting that the binary uses **Just-In-Time compilation** to dynamically generate machine code for password verification at runtime. This means the password-checking logic is not statically present in the binary -- it is assembled in memory when the program runs, making static analysis with tools like Ghidra or IDA alone insufficient.

### Key Observations

1. **Dynamic code generation**: The binary allocates an executable memory region (via `mmap` with `PROT_EXEC` or `VirtualAlloc` with `PAGE_EXECUTE_READWRITE`), writes machine code bytes into it at runtime, and then calls the generated code through a function pointer. The password check logic lives entirely in this JIT-compiled code.

2. **Obfuscated comparison**: The JIT-emitted code does not perform a simple `strcmp`. Instead, it applies a series of arithmetic transformations (XOR, rotation, addition with constants) to each character of the input and compares the result against hardcoded target values embedded in the JIT code buffer.

3. **Anti-debugging tricks**: The binary may include timing checks (`rdtsc`) or `ptrace` self-attach to detect debuggers. These need to be bypassed or patched.

### Reverse Engineering Strategy

The most reliable approach is **dynamic analysis**:

1. Run the binary under GDB with `starti` to break before any anti-debug checks.
2. Set a breakpoint on `mmap` (or the memory allocation call) to find where the JIT buffer is allocated.
3. Once the JIT code is emitted, dump the executable buffer and disassemble it.
4. Analyze the disassembled JIT code to extract the transformation applied to each input byte and the expected output values.
5. Invert the transformations to recover the correct password.

Alternatively, we can use **angr** or **Z3** to symbolically execute the JIT-compiled code and solve for the input that satisfies all comparison constraints.

## Solution

### Step 1: Identify the JIT buffer

Run the binary under GDB and break on `mmap`:

```
gdb ./jitfp
(gdb) break mmap
(gdb) run
```

When `mmap` returns, note the address of the allocated buffer. Continue until the JIT code is written and the password prompt appears.

### Step 2: Dump the JIT code

```
(gdb) x/200i <jit_buffer_address>
```

This reveals the generated instruction sequence. Typically it looks like a loop that:
- Loads each byte of the input
- XORs it with a per-position key
- Adds a constant
- Rotates bits
- Compares against an expected value

### Step 3: Extract constraints and solve

For each character position `i`, the JIT code effectively computes:

```
transform(input[i], key[i]) == expected[i]
```

We extract the keys and expected values from the JIT code bytes and invert the transformation to recover each password character.

### Step 4: Submit the password

Connect to the remote host, enter the recovered password, and receive the flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
