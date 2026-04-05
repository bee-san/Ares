# Heap Havoc - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 200

## Challenge Description

A seemingly harmless program takes two names as arguments, but there's a catch. By overflowing the input buffer, you can overwrite heap metadata and redirect execution.

## Approach

This is a **heap overflow** challenge. The program allocates two buffers on the heap (for two "names"), and a lack of bounds checking on input allows us to overflow the first buffer into adjacent heap memory. By overwriting data in the second allocation -- or overwriting a function pointer / metadata stored on the heap -- we can redirect program execution to a win function or trigger the flag.

### Binary Analysis

Key observations:

1. **Two heap allocations**: The program uses `malloc()` to allocate space for two name strings on the heap. Since `malloc` allocates memory contiguously (when chunks are adjacent in the heap), the second allocation sits right after the first in memory.
2. **Unbounded input**: The program reads into the first buffer using an unsafe function like `scanf("%s")`, `gets()`, or `strcpy()` without checking the length against the allocated buffer size.
3. **Win condition**: The program compares the second buffer's contents against an expected value. If the second buffer has been modified (overwritten by our overflow), a different code path is triggered that prints the flag. Alternatively, there may be a function pointer on the heap that we overwrite to point to a `win()` function.
4. **No stack protections needed**: Since the overflow happens on the heap, stack canaries are irrelevant.

### Heap Memory Layout

```
+---------------------------+
| Chunk 1 header (metadata) |
+---------------------------+
| name1 buffer (e.g. 32B)  |  <-- We write here (overflow this)
+---------------------------+
| Chunk 2 header (metadata) |  <-- We overwrite through this
+---------------------------+
| name2 buffer / safe_var   |  <-- Target: overwrite this value
+---------------------------+
```

The distance from `name1` to `name2` depends on the `malloc` chunk size (which includes alignment and metadata overhead). Typically for a 32-byte allocation, the total chunk size is 48 bytes (32 data + 16 metadata on 64-bit), so we need about **48+ bytes** of input to reach into the second buffer.

### Exploitation Strategy

1. **Determine the overflow distance**: Calculate or brute-force the number of bytes needed to overflow from buffer 1 into buffer 2. This is the size of buffer 1 plus the heap chunk metadata between them.
2. **Craft the payload**: Fill buffer 1 with padding bytes, then append the value we want to write into buffer 2.
3. **Trigger the win condition**: The program checks buffer 2's contents. If it no longer matches the original value (or matches a specific target value), the flag is printed.

### Finding the Exact Offset

The offset from the start of `name1` to the start of `name2` can be found by:

- **Static analysis** (Ghidra/IDA): Look at the `malloc` sizes and calculate chunk distances.
- **Dynamic analysis** (GDB): Set breakpoints after both `malloc` calls, note the returned addresses, and subtract them.
- **Trial and error**: Send increasingly longer strings until the win condition triggers. Common offsets for this type of challenge are 32, 33, 36, 40, or 48 bytes.

## Solution

### Step 1: Analyze the binary

```bash
checksec ./vuln
file ./vuln
```

Use Ghidra or `objdump` to find:
- The sizes passed to `malloc()`
- The win condition (comparison on the second buffer)
- Whether there is a `win()` function address

### Step 2: Find the heap offset

In GDB:
```
b *main+<offset_after_second_malloc>
r AAAA BBBB
# Note the two malloc return values
# offset = addr2 - addr1
```

### Step 3: Craft the overflow payload

If the offset is 32 bytes and the win condition checks that `name2 != "bico"`:
```python
payload = b"A" * 32 + b"OVERFLOW"
```

If a specific value is required (e.g., overwriting a function pointer to a `win` function):
```python
payload = b"A" * 32 + p64(win_addr)
```

### Step 4: Run the exploit

```bash
./vuln "$(python3 -c "print('A'*32 + 'WIN')")" "anything"
```

Or via the solve script against the remote service.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
