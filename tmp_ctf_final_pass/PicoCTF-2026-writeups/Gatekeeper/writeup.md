# Gatekeeper - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 100

## Challenge Description

What's behind the numeric gate? You only get access if you enter the right kind of number. You can download the binary.

## Approach

This is a beginner-level reverse engineering challenge where a binary asks for numeric input and only grants access (reveals the flag) if you provide "the right kind of number." The key insight is figuring out what **type or property** of number the program expects, rather than finding a specific secret value.

### Static Analysis

Using a disassembler (Ghidra, IDA, or `objdump`), we can reverse engineer the binary to understand its input validation logic. Common patterns for this type of challenge include:

1. **Negative number check**: The program uses an `unsigned` comparison or checks `input > 0`, but the flag path requires a negative number (or vice versa)
2. **Integer overflow**: The program checks if a number is within a range, but you can overflow a 32-bit signed integer to bypass the check
3. **Specific numeric property**: The number must be negative, zero, a float that truncates in a specific way, or have some other special property
4. **Type confusion**: The input is read as a string but compared as an integer, allowing special values like negative numbers or very large numbers

### Typical Binary Logic

After decompilation, the core logic typically resembles:

```c
int main() {
    int input;
    printf("Enter the key: ");
    scanf("%d", &input);

    // Gate check - the "right kind of number"
    if (input < 0) {
        // Or some other condition like: if (input > MAX_INT/2)
        // Or: if ((unsigned)input > some_threshold)
        decrypt_and_print_flag(input);
    } else {
        printf("Access denied.\n");
    }
    return 0;
}
```

The trick is that the prompt implies you need a positive passcode, but the actual check requires a **negative number** (or some other non-obvious numeric property).

### Tools Used

- **Ghidra / IDA Free**: For decompilation and static analysis
- **`strings`**: For finding embedded strings and hints
- **`ltrace` / `strace`**: For dynamic analysis of library/system calls
- **`gdb`**: For debugging and stepping through the binary
- **Python**: For automating the solution

## Solution

### Step 1: Initial Reconnaissance

```bash
file gatekeeper
strings gatekeeper | grep -i "flag\|pico\|access\|denied\|correct\|wrong\|gate"
chmod +x gatekeeper
```

### Step 2: Dynamic Analysis

```bash
# Try basic inputs
echo "0" | ./gatekeeper
echo "1" | ./gatekeeper
echo "42" | ./gatekeeper
echo "-1" | ./gatekeeper
echo "2147483647" | ./gatekeeper     # INT_MAX
echo "-2147483648" | ./gatekeeper    # INT_MIN
```

### Step 3: Static Analysis with Ghidra

1. Open the binary in Ghidra
2. Find `main()` or the entry point
3. Look for the conditional branch that leads to the flag
4. Identify what numeric condition satisfies the check

### Step 4: Provide the Correct Input

Based on the analysis, provide the number that satisfies the gate condition. For example, if the check is for a negative number:

```bash
echo "-1" | ./gatekeeper
```

Or if it requires a specific value found during analysis:

```bash
echo "<value>" | ./gatekeeper
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
