# Bypass Me - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 100

## Challenge Description
Your task is to analyze and exploit a password-protected binary called bypassme.bin and binary performs a multi-step verification process before granting access.

## Approach
This is a classic reverse engineering challenge where a binary performs multiple password verification steps. The goal is to either:

1. **Extract the password** by reverse engineering the verification logic, or
2. **Bypass the checks entirely** by patching the binary (changing conditional jumps)

The "multi-step verification" suggests the binary has several sequential checks, such as:
- String length verification
- Character-by-character comparison
- Hash/checksum validation
- Anti-debugging checks
- Obfuscated comparisons

### Key Techniques:
- **Static analysis**: Decompile with Ghidra/IDA to understand the verification logic
- **Dynamic analysis**: Use GDB to step through checks and observe comparisons
- **Binary patching**: Modify conditional jumps (e.g., `jne` -> `je` or `jmp`) to bypass checks
- **ltrace/strace**: Intercept library calls like `strcmp`, `strncmp`, `memcmp`

### Understanding the Verification Steps:
Each step likely compares parts of the input against expected values. By analyzing the comparison instructions in the disassembly, we can extract what values are expected or simply patch the binary to skip the checks.

## Solution

### Step 1: Initial reconnaissance
```bash
file bypassme.bin
checksec --file=bypassme.bin
strings bypassme.bin | grep -i "pass\|flag\|correct\|wrong\|step\|verify"
```

### Step 2: Dynamic analysis with ltrace
```bash
# ltrace intercepts library function calls -- if strcmp is used, the
# expected password will appear in the trace output
ltrace ./bypassme.bin
# Enter a test password like "AAAA" and observe strcmp/strncmp calls
```

### Step 3: Static analysis with Ghidra/objdump
```bash
# Quick disassembly of main
objdump -d -M intel bypassme.bin | less
# Look for cmp, test, je, jne instructions after reads/scanf
```

In Ghidra:
1. Open bypassme.bin
2. Navigate to `main()` or search for string references ("Enter password", "Correct", etc.)
3. Identify the verification function(s)
4. Note the expected values at each comparison

### Step 4: Bypass via GDB
```bash
gdb ./bypassme.bin
# Set breakpoint at each comparison
b *<address_of_first_cmp>
b *<address_of_second_cmp>
run
# At each breakpoint, examine the comparison values:
# x/s $rdi    (first arg to strcmp)
# x/s $rsi    (second arg to strcmp)
# Or modify the zero flag to force the branch:
# set $eflags |= (1 << 6)    # Set ZF to force JE to succeed
continue
```

### Step 5: Bypass via binary patching
```python
# Patch conditional jumps to unconditional jumps or NOPs
# jne (0x75) -> je (0x74) or jmp (0xEB) or NOP (0x90)
```

### Step 6: Run the patched binary or supply the extracted password
```bash
./bypassme_patched.bin
# Or: echo "extracted_password" | ./bypassme.bin
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
