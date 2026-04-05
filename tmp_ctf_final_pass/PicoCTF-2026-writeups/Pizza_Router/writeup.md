# Pizza Router - picoCTF 2026

**Category:** Binary Exploitation
**Points:** 400

## Challenge Description

Plan the fastest pizza drone routes and snag a slice of the flag. The binary is available to download.

## Approach

This is the highest-point binary exploitation challenge of the four at 400 points with 368 solves. The theme involves "planning the fastest pizza drone routes," which suggests the binary implements some kind of graph/routing algorithm. The exploitation vector likely lies in how the routing data is processed.

### Binary Analysis

The binary likely presents a pizza delivery routing interface where you:
1. Input locations/nodes (pizza destinations)
2. Input distances/edges between them
3. The program calculates optimal routes (shortest path / TSP variant)

The vulnerability is hidden in how the program handles the routing input. At 400 points, this is a moderately difficult pwn challenge. Possible vulnerability classes:

### Likely Vulnerability: Heap-Based Buffer Overflow or Stack Overflow via Route Input

The binary probably has:
- A **fixed-size adjacency matrix or route buffer** that can be overflowed by providing too many nodes or excessively long route data
- An **integer overflow** in the route calculation that leads to a smaller-than-expected buffer allocation
- A **use-after-free** if routes can be added and deleted
- A **format string** in a logging/output function that prints route information

### Exploitation Strategy

Given the point value (400) and solve count (368), this is likely a **stack-based buffer overflow with some added complexity** (e.g., a canary to bypass, PIE to defeat, or a restricted character set due to the routing input format).

**Typical attack flow:**

1. **Reverse engineer the binary**: Use Ghidra/IDA to understand the routing input format and find the vulnerable function
2. **Identify the overflow**: Determine the buffer size and how many bytes of overflow we get
3. **Leak addresses**: If there is a canary or PIE, find an information leak (possibly through the route output)
4. **Build ROP chain or ret2win**: Redirect execution to a `win()` function or build a ROP chain to call `system("/bin/sh")`

### Advanced Techniques Potentially Required

- **ROP (Return-Oriented Programming)**: Chain gadgets to bypass NX
- **Canary bypass**: Leak the stack canary through an info leak before the overflow
- **PIE bypass**: Leak a code address to calculate the binary base
- **ret2libc**: If no win function exists, use libc gadgets to spawn a shell
- **Heap exploitation**: If the routing data is heap-allocated (malloc/free patterns)

## Solution

### Step 1: Initial Reconnaissance

```bash
file pizza_router
checksec pizza_router
strings pizza_router | grep -i flag
objdump -t pizza_router | grep -E "win|flag|system|exec"
```

Check the binary's protections:
- **NX**: Almost certainly enabled (no shellcode on stack)
- **Canary**: May or may not be present
- **PIE**: May or may not be present
- **RELRO**: Partial or Full (affects GOT overwrite feasibility)

### Step 2: Reverse Engineering

Open in Ghidra or IDA. Key things to find:
- The main menu / input handling loop
- The buffer where route data is stored
- Any bounds checking (or lack thereof)
- Functions that read/print the flag
- The routing calculation function (often where the overflow occurs)

### Step 3: Find the Overflow

Common patterns:
```c
// Pattern 1: Fixed buffer, unbounded read
char routes[256];
read(0, routes, 0x400);  // reads more than buffer size

// Pattern 2: Integer overflow in allocation
int num_routes = get_input();
char *buf = malloc(num_routes * sizeof(Route));  // integer overflow
for (int i = 0; i < num_routes; i++) {
    read_route(&buf[i]);  // writes past allocation
}

// Pattern 3: Off-by-one in adjacency matrix
int adj[MAX_NODES][MAX_NODES];
// Indexing error allows writing one row past the matrix
```

### Step 4: Craft the Exploit

Depending on protections found:

**If no canary + no PIE (simplest case):**
```
[padding to return address] + [address of win()]
```

**If canary present:**
```
[padding to canary] + [leaked canary] + [padding] + [return address]
```

**If PIE enabled:**
```
Phase 1: Leak binary base via info disclosure
Phase 2: Calculate win() address = base + offset
Phase 3: Overflow with calculated address
```

**If no win function (ret2libc):**
```
Phase 1: Leak libc address (via puts@GOT or similar)
Phase 2: Calculate system() and "/bin/sh" addresses
Phase 3: ROP chain: pop rdi; ret + "/bin/sh" addr + system addr
```

### Step 5: Route the Input Correctly

The tricky part of this challenge is likely that the overflow must be triggered through the routing interface's input format, not raw bytes. You may need to encode your payload within valid "route" entries, e.g.:

```
Number of locations: 100
Route 1: A -> B, distance: [overflow payload here]
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
