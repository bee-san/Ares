# Autorev 1 - picoCTF 2026

**Category:** Reverse Engineering
**Points:** 200

## Challenge Description
You think you can reverse engineer? Let's test out your speed.

## Approach
This challenge tests your ability to quickly reverse engineer binaries. The "speed" element strongly suggests that:

1. The server provides a binary (or series of binaries) that must be analyzed and solved within a time limit.
2. Manual reversing would be too slow -- you need automated analysis.
3. Common techniques include:
   - **angr** (symbolic execution) to automatically find inputs that reach a "success" path
   - **z3** (SMT solver) to solve constraint systems extracted from the binary
   - **Binary instrumentation** with tools like Frida or PIN
   - **Automated disassembly** with Ghidra scripting or radare2/rizin

The typical pattern for "speed reversing" challenges:
- Connect to a server via netcat
- Receive a binary (base64 encoded or downloadable via URL)
- Analyze the binary to find the correct input/password/key
- Send it back before the timeout expires
- Repeat for multiple rounds

The binary likely contains a `check_flag()` or `verify()` function that compares user input against some computed value. Using angr, we can symbolically execute the binary to find the input that leads to a "Correct" output.

## Solution

### Step 1: Connect to the challenge server
```bash
nc <challenge_host> <challenge_port>
```
The server sends a binary (possibly base64-encoded) and expects the correct input within a time limit.

### Step 2: Download and analyze the binary
```bash
# If a URL is provided:
wget <binary_url> -O challenge_binary
chmod +x challenge_binary

# Quick analysis
file challenge_binary
strings challenge_binary | grep -i "correct\|wrong\|flag\|success\|fail"
```

### Step 3: Use angr for automated solving
angr uses symbolic execution to explore all possible paths through a binary. We tell it to find the path that reaches the "success" output and avoid the "failure" output.

### Step 4: Alternatively, use Ghidra headless for decompilation
```bash
analyzeHeadless /tmp ghidra_project -import challenge_binary -postScript DecompileScript.py
```

### Step 5: Submit the answer before timeout

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
