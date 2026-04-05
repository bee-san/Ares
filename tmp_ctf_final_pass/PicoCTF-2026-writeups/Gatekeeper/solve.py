#!/usr/bin/env python3
"""
Gatekeeper - picoCTF 2026
Category: Reverse Engineering | Points: 100

Reverse engineers the 'gatekeeper' binary to find the correct numeric input
that passes the gate check and reveals the flag.

This script:
  1. Attempts common numeric tricks (negative numbers, edge values, etc.)
  2. Uses static analysis (strings, objdump) to find clues
  3. Optionally uses angr for symbolic execution to find the correct input

Requirements:
    pip install pwntools

Optional (for automated symbolic execution):
    pip install angr

Usage:
    python3 solve.py [path_to_binary]

    If no path is provided, looks for ./gatekeeper in the current directory.
"""

import os
import sys
import re
import subprocess
import struct

# ─── Configuration ───────────────────────────────────────────────────────────
BINARY_PATH = sys.argv[1] if len(sys.argv) > 1 else "./gatekeeper"

# Candidate inputs to try - common numeric gate bypasses
CANDIDATE_INPUTS = [
    "-1",               # Negative number (most common trick)
    "-2",
    "0",                # Zero
    "-2147483648",      # INT_MIN (32-bit)
    "2147483647",       # INT_MAX (32-bit)
    "-2147483647",      # INT_MIN + 1
    "2147483648",       # INT_MAX + 1 (overflow to negative in 32-bit signed)
    "4294967295",       # UINT_MAX (32-bit)
    "4294967296",       # UINT_MAX + 1
    "-9999",
    "1",
    "1337",
    "31337",
    "999999999",
    "-999999999",
    "NaN",              # Not a number (scanf edge case)
    "inf",              # Infinity
    "-inf",
    "0x7fffffff",       # Hex INT_MAX
    "-0",
]


def check_binary():
    """Verify the binary exists and is executable."""
    if not os.path.isfile(BINARY_PATH):
        print(f"[!] Binary not found: {BINARY_PATH}")
        print("[*] Download the binary from the challenge page and place it here.")
        print(f"[*] Usage: python3 {sys.argv[0]} /path/to/gatekeeper")
        sys.exit(1)

    # Make executable
    os.chmod(BINARY_PATH, 0o755)
    print(f"[+] Binary: {BINARY_PATH}")

    # File info
    try:
        result = subprocess.run(["file", BINARY_PATH], capture_output=True, text=True)
        print(f"[+] Type: {result.stdout.strip()}")
    except FileNotFoundError:
        pass


def extract_strings():
    """Extract interesting strings from the binary."""
    print("\n[*] Extracting strings from binary...")
    try:
        result = subprocess.run(["strings", BINARY_PATH], capture_output=True, text=True)
        interesting = []
        for line in result.stdout.split("\n"):
            line_lower = line.lower()
            if any(kw in line_lower for kw in [
                "flag", "pico", "access", "denied", "correct", "wrong",
                "gate", "key", "password", "enter", "input", "number",
                "negative", "positive", "secret", "granted", "welcome",
            ]):
                interesting.append(line.strip())
        if interesting:
            print("[+] Interesting strings found:")
            for s in interesting:
                print(f"    {s}")
        return interesting
    except FileNotFoundError:
        print("[*] 'strings' not available, skipping")
        return []


def run_with_input(input_val):
    """Run the binary with a given input and return stdout+stderr."""
    try:
        proc = subprocess.run(
            [BINARY_PATH],
            input=input_val + "\n",
            capture_output=True,
            text=True,
            timeout=5,
        )
        output = proc.stdout + proc.stderr
        return output
    except subprocess.TimeoutExpired:
        return "[TIMEOUT]"
    except Exception as e:
        return f"[ERROR: {e}]"


def find_flag(text):
    """Search for a picoCTF flag in text."""
    match = re.search(r'picoCTF\{[^}]+\}', text)
    if match:
        return match.group(0)
    return None


def try_candidates():
    """Try each candidate input and check for the flag."""
    print(f"\n[*] Trying {len(CANDIDATE_INPUTS)} candidate inputs...")
    print("-" * 60)

    for i, candidate in enumerate(CANDIDATE_INPUTS, 1):
        output = run_with_input(candidate)
        output_clean = output.strip().replace("\n", " | ")

        flag = find_flag(output)
        if flag:
            print(f"  [{i}] Input: {candidate:<20} -> FLAG FOUND!")
            print(f"\n[+] Correct input: {candidate}")
            print(f"[+] FLAG: {flag}")
            return candidate, flag

        # Check for success indicators (without the flag directly visible)
        output_lower = output.lower()
        if any(word in output_lower for word in ["access granted", "correct", "welcome", "success"]):
            print(f"  [{i}] Input: {candidate:<20} -> POSSIBLE SUCCESS: {output_clean[:80]}")
            print(f"\n[+] Possible correct input: {candidate}")
            print(f"[+] Full output:\n{output}")
            return candidate, None
        else:
            status = "Denied" if any(w in output_lower for w in ["denied", "wrong", "invalid", "incorrect"]) else "Unknown"
            print(f"  [{i}] Input: {candidate:<20} -> {status}: {output_clean[:60]}")

    return None, None


def try_angr_solve():
    """Use angr symbolic execution to find the correct input automatically."""
    try:
        import angr
        import claripy
    except ImportError:
        print("\n[*] angr not installed. Install with: pip install angr")
        print("[*] angr can automatically find the correct input via symbolic execution.")
        return None

    print("\n[*] Attempting symbolic execution with angr...")

    proj = angr.Project(BINARY_PATH, auto_load_libs=False)

    # Look for "flag" or "picoCTF" or "access granted" in the binary
    # to identify the target (success) address
    cfg = proj.analyses.CFGFast()

    # Find addresses containing success/failure strings
    success_addr = None
    fail_addr = None

    for func in cfg.functions.values():
        try:
            blocks = list(func.blocks)
        except Exception:
            continue

    # Use exploration with string matching
    state = proj.factory.entry_state(
        args=[BINARY_PATH],
        stdin=angr.SimFile("/dev/stdin", content=claripy.BVS("stdin", 32 * 8)),
    )

    simgr = proj.factory.simulation_manager(state)

    def is_successful(state):
        output = state.posix.dumps(1)  # stdout
        return b"picoCTF{" in output or b"access granted" in output.lower() or b"correct" in output.lower()

    def is_failure(state):
        output = state.posix.dumps(1)
        return b"denied" in output.lower() or b"wrong" in output.lower() or b"invalid" in output.lower()

    print("[*] Exploring paths (this may take a moment)...")
    simgr.explore(find=is_successful, avoid=is_failure)

    if simgr.found:
        found_state = simgr.found[0]
        solution_input = found_state.posix.dumps(0).decode("utf-8", errors="ignore").strip()
        solution_output = found_state.posix.dumps(1).decode("utf-8", errors="ignore").strip()
        print(f"[+] angr found solution input: {solution_input}")
        print(f"[+] Program output: {solution_output}")

        flag = find_flag(solution_output)
        if flag:
            print(f"[+] FLAG: {flag}")
        return solution_input
    else:
        print("[-] angr could not find a successful path.")
        return None


def main():
    print("[*] Gatekeeper - picoCTF 2026 Solver")
    print("=" * 50)

    check_binary()
    strings_found = extract_strings()

    # Try candidate inputs first (fast)
    candidate, flag = try_candidates()

    if flag:
        print(f"\n{'=' * 50}")
        print(f"[+] SOLVED! Flag: {flag}")
        print(f"{'=' * 50}")
        sys.exit(0)

    if candidate:
        print(f"\n[+] Found a promising input: {candidate}")
        print("[*] Check the output above for the flag or try running manually:")
        print(f'    echo "{candidate}" | {BINARY_PATH}')
        sys.exit(0)

    # If brute-force didn't work, try angr
    print("\n[*] Candidate inputs exhausted. Trying symbolic execution...")
    result = try_angr_solve()

    if result:
        print(f"\n[+] Run: echo \"{result}\" | {BINARY_PATH}")
    else:
        print("\n[-] Could not find the correct input automatically.")
        print("[*] Manual reverse engineering recommended:")
        print(f"    1. ghidra {BINARY_PATH}")
        print(f"    2. Look for the comparison/branch in main()")
        print(f"    3. Identify what numeric property is checked")
        print(f"    4. echo '<correct_number>' | {BINARY_PATH}")


if __name__ == "__main__":
    main()
