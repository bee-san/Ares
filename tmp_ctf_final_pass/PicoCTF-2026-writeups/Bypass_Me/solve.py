#!/usr/bin/env python3
"""
Bypass Me - picoCTF 2026
Category: Reverse Engineering | Points: 100

Analyzes and bypasses a password-protected binary (bypassme.bin) that performs
multi-step verification. This script uses multiple approaches:
1. ltrace interception to capture strcmp/strncmp arguments
2. Binary patching to bypass conditional jumps
3. angr symbolic execution as a fallback

Usage:
    python3 solve.py [path_to_bypassme.bin]
    python3 solve.py              # defaults to ./bypassme.bin
"""

import subprocess
import sys
import os
import re
import struct
import tempfile
import shutil

FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")


# ──────────────────────────────────────────────────────────────────
# Method 1: ltrace interception
# ──────────────────────────────────────────────────────────────────

def try_ltrace(binary_path):
    """
    Use ltrace to intercept string comparison functions.
    If the binary uses strcmp/strncmp/memcmp, the expected password
    will appear as an argument in the trace output.
    """
    print("[*] Method 1: ltrace interception")
    test_input = b"AAAAAAAAAAAAAAAA\n"

    try:
        proc = subprocess.run(
            ["ltrace", "-s", "256", "-e", "strcmp+strncmp+memcmp+strcasecmp", binary_path],
            input=test_input,
            capture_output=True,
            text=True,
            timeout=10,
        )
        output = proc.stderr + proc.stdout  # ltrace outputs to stderr

        # Look for comparison functions with string arguments
        # ltrace format: strcmp("user_input", "expected_password") = -1
        passwords = []
        patterns = [
            r'str(?:n?)cmp\(".*?",\s*"([^"]+)"',
            r'str(?:n?)cmp\("([^"]+)",\s*".*?"',
            r'memcmp\(.*?,\s*"([^"]+)"',
            r'strcasecmp\(".*?",\s*"([^"]+)"',
        ]
        for pat in patterns:
            matches = re.findall(pat, output)
            for m in matches:
                if m != test_input.decode().strip() and len(m) > 0:
                    passwords.append(m)

        if passwords:
            print(f"[+] Intercepted password(s): {passwords}")
            return passwords
        else:
            print("[-] No passwords intercepted via ltrace")

    except FileNotFoundError:
        print("[-] ltrace not found, skipping")
    except subprocess.TimeoutExpired:
        print("[-] ltrace timed out")

    return []


# ──────────────────────────────────────────────────────────────────
# Method 2: String extraction
# ──────────────────────────────────────────────────────────────────

def try_strings(binary_path):
    """Search for hardcoded passwords/flags in binary strings."""
    print("[*] Method 2: String extraction")

    result = subprocess.run(
        ["strings", "-a", binary_path], capture_output=True, text=True
    )
    all_strings = result.stdout

    # Direct flag check
    flag_match = FLAG_PATTERN.search(all_strings.encode())
    if flag_match:
        print(f"[+] Flag found in strings: {flag_match.group().decode()}")
        return flag_match.group().decode()

    # Look for password-related strings
    interesting = []
    for line in all_strings.split("\n"):
        line = line.strip()
        # Filter for potential passwords (alphanumeric, reasonable length)
        if 4 <= len(line) <= 64 and re.match(r'^[a-zA-Z0-9_!@#$%^&*(){}]+$', line):
            interesting.append(line)

    return interesting


# ──────────────────────────────────────────────────────────────────
# Method 3: Binary patching
# ──────────────────────────────────────────────────────────────────

def try_patching(binary_path):
    """
    Patch conditional jumps to bypass verification checks.
    Strategy: find all jne/jnz (0x75, 0x0F 0x85) after cmp instructions
    near "wrong"/"fail" strings and replace with je/jz or NOP.
    """
    print("[*] Method 3: Binary patching")

    with open(binary_path, "rb") as f:
        data = bytearray(f.read())

    original_data = bytes(data)

    # Disassemble to find conditional jumps in verification sections
    try:
        result = subprocess.run(
            ["objdump", "-d", "-M", "intel", binary_path],
            capture_output=True, text=True
        )
        disasm = result.stdout
    except FileNotFoundError:
        print("[-] objdump not found")
        return None

    # Find all conditional jump instructions that could be verification checks
    # Pattern: cmp ... followed by jne/jnz (branch to fail) or je/jz (branch to success)
    patch_count = 0

    # Strategy A: Replace short jne (0x75 XX) with je (0x74 XX)
    # This inverts the condition so wrong passwords are accepted
    for match in re.finditer(
        r'^\s*([0-9a-f]+):\s+75 ([0-9a-f]{2})\s+jne?\s',
        disasm, re.MULTILINE
    ):
        addr = int(match.group(1), 16)
        # Find this byte sequence in the binary
        # We need the file offset, not the virtual address
        # For simple binaries, offset ~= addr - base_addr
        # Try to find the pattern in the raw data
        offset = data.find(b'\x75' + bytes([int(match.group(2), 16)]))
        if offset != -1:
            # Check context to avoid patching unrelated jumps
            data[offset] = 0x74  # jne -> je (invert condition)
            patch_count += 1

    # Strategy B: Replace long jne (0F 85 XX XX XX XX) with long je (0F 84)
    for match in re.finditer(
        r'^\s*([0-9a-f]+):\s+0f 85\s',
        disasm, re.MULTILINE
    ):
        addr = int(match.group(1), 16)
        idx = 0
        while True:
            idx = data.find(b'\x0f\x85', idx)
            if idx == -1:
                break
            data[idx + 1] = 0x84  # jne -> je
            patch_count += 1
            idx += 2

    if patch_count == 0:
        # Try the reverse: patch je to jne (if logic is inverted)
        data = bytearray(original_data)
        for match in re.finditer(
            r'^\s*([0-9a-f]+):\s+74 ([0-9a-f]{2})\s+je?\s',
            disasm, re.MULTILINE
        ):
            offset = data.find(b'\x74' + bytes([int(match.group(2), 16)]))
            if offset != -1:
                data[offset] = 0x75  # je -> jne
                patch_count += 1

    if patch_count > 0:
        print(f"[+] Applied {patch_count} patches")
        patched_path = binary_path + ".patched"
        with open(patched_path, "wb") as f:
            f.write(data)
        os.chmod(patched_path, 0o755)
        print(f"[+] Patched binary saved to: {patched_path}")
        return patched_path

    print("[-] No suitable patches found")
    return None


# ──────────────────────────────────────────────────────────────────
# Method 4: angr symbolic execution
# ──────────────────────────────────────────────────────────────────

def try_angr(binary_path):
    """Use angr to symbolically find the correct input."""
    print("[*] Method 4: angr symbolic execution")

    try:
        import angr
        import claripy
    except ImportError:
        print("[-] angr not installed (pip install angr)")
        return None

    proj = angr.Project(binary_path, auto_load_libs=False)

    # Create symbolic stdin
    sym_input = claripy.BVS("input", 200 * 8)
    state = proj.factory.entry_state(
        stdin=angr.SimFile("/dev/stdin", content=sym_input),
        add_options={
            angr.options.ZERO_FILL_UNCONSTRAINED_MEMORY,
            angr.options.ZERO_FILL_UNCONSTRAINED_REGISTERS,
        }
    )

    simgr = proj.factory.simulation_manager(state)

    # Success/failure strings
    find_strs = [b"Correct", b"correct", b"Success", b"success",
                  b"Access granted", b"Welcome", b"picoCTF", b"flag"]
    avoid_strs = [b"Wrong", b"wrong", b"Incorrect", b"incorrect",
                   b"Denied", b"denied", b"fail", b"Fail", b"Invalid"]

    def is_find(s):
        out = s.posix.dumps(1)
        return any(f in out for f in find_strs)

    def is_avoid(s):
        out = s.posix.dumps(1)
        return any(a in out for a in avoid_strs)

    simgr.explore(find=is_find, avoid=is_avoid)

    if simgr.found:
        solution = simgr.found[0].posix.dumps(0)
        solution = solution.split(b"\x00")[0].strip()
        stdout = simgr.found[0].posix.dumps(1)
        print(f"[+] angr found input: {solution}")
        print(f"[+] Program output: {stdout.decode(errors='replace')}")

        flag_match = FLAG_PATTERN.search(stdout)
        if flag_match:
            print(f"[+] FLAG: {flag_match.group().decode()}")

        return solution
    else:
        print("[-] angr could not find a solution")
        return None


# ──────────────────────────────────────────────────────────────────
# Method 5: GDB scripted analysis
# ──────────────────────────────────────────────────────────────────

def try_gdb_script(binary_path):
    """Use GDB with a script to break at comparisons and extract expected values."""
    print("[*] Method 5: GDB scripted analysis")

    gdb_script = """
set pagination off
set confirm off
set follow-fork-mode child

# Break at common comparison functions
catch syscall write
b strcmp
b strncmp
b memcmp

run <<< "TESTPASSWORD123"

# At each breakpoint, print the arguments
while 1
    # For strcmp(s1, s2): s1 in $rdi, s2 in $rsi (x86-64 SysV ABI)
    printf "RDI (arg1): "
    x/s $rdi
    printf "RSI (arg2): "
    x/s $rsi
    continue
end

quit
"""
    script_path = "/tmp/gdb_bypass_script.gdb"
    with open(script_path, "w") as f:
        f.write(gdb_script)

    try:
        result = subprocess.run(
            ["gdb", "-batch", "-x", script_path, binary_path],
            capture_output=True, text=True, timeout=15
        )
        output = result.stdout + result.stderr

        # Parse GDB output for string comparison values
        passwords = []
        for match in re.finditer(r'RSI \(arg2\):\s+\S+\s+"([^"]+)"', output):
            val = match.group(1)
            if val != "TESTPASSWORD123" and len(val) > 0:
                passwords.append(val)

        for match in re.finditer(r'RDI \(arg1\):\s+\S+\s+"([^"]+)"', output):
            val = match.group(1)
            if val != "TESTPASSWORD123" and len(val) > 0:
                passwords.append(val)

        if passwords:
            print(f"[+] GDB extracted password(s): {passwords}")
            return passwords

    except FileNotFoundError:
        print("[-] GDB not found")
    except subprocess.TimeoutExpired:
        print("[-] GDB timed out")

    return []


# ──────────────────────────────────────────────────────────────────
# Main solver orchestration
# ──────────────────────────────────────────────────────────────────

def run_binary(binary_path, password):
    """Run the binary with a given password and capture output."""
    try:
        proc = subprocess.run(
            [binary_path],
            input=password.encode() if isinstance(password, str) else password,
            capture_output=True,
            text=True,
            timeout=10,
        )
        return proc.stdout + proc.stderr
    except:
        return ""


def main():
    binary_path = sys.argv[1] if len(sys.argv) > 1 else "./bypassme.bin"
    binary_path = os.path.abspath(binary_path)

    if not os.path.exists(binary_path):
        print(f"[-] Binary not found: {binary_path}")
        print("Usage: python3 solve.py [path_to_bypassme.bin]")
        sys.exit(1)

    print(f"[*] Target: {binary_path}")
    print(f"[*] File info: {subprocess.getoutput(f'file {binary_path}')}")
    print()

    # ── Method 1: ltrace ──
    passwords = try_ltrace(binary_path)
    if passwords:
        for pw in passwords:
            output = run_binary(binary_path, pw)
            print(f"[*] Trying '{pw}': {output.strip()}")
            flag = FLAG_PATTERN.search(output)
            if flag:
                print(f"\n[+] FLAG: {flag.group()}")
                return
        # If multi-step, try concatenating passwords
        combined = "".join(passwords)
        output = run_binary(binary_path, combined)
        print(f"[*] Trying combined '{combined}': {output.strip()}")
        flag = FLAG_PATTERN.search(output)
        if flag:
            print(f"\n[+] FLAG: {flag.group()}")
            return

    print()

    # ── Method 2: Strings ──
    result = try_strings(binary_path)
    if isinstance(result, str) and "picoCTF" in result:
        print(f"\n[+] FLAG: {result}")
        return

    print()

    # ── Method 3: Binary patching ──
    patched = try_patching(binary_path)
    if patched:
        print(f"[*] Running patched binary with dummy input...")
        output = run_binary(patched, "anything")
        print(f"[*] Output: {output.strip()}")
        flag = FLAG_PATTERN.search(output)
        if flag:
            print(f"\n[+] FLAG: {flag.group()}")
            return

    print()

    # ── Method 4: GDB scripted analysis ──
    gdb_passwords = try_gdb_script(binary_path)
    if gdb_passwords:
        for pw in gdb_passwords:
            output = run_binary(binary_path, pw)
            print(f"[*] Trying '{pw}': {output.strip()}")
            flag = FLAG_PATTERN.search(output)
            if flag:
                print(f"\n[+] FLAG: {flag.group()}")
                return

    print()

    # ── Method 5: angr (slowest, most comprehensive) ──
    angr_result = try_angr(binary_path)
    if angr_result:
        output = run_binary(binary_path, angr_result)
        print(f"[*] Output: {output.strip()}")
        flag = FLAG_PATTERN.search(output)
        if flag:
            print(f"\n[+] FLAG: {flag.group()}")
            return

    print()
    print("=" * 60)
    print("MANUAL STEPS (if automatic methods failed):")
    print("=" * 60)
    print("1. Open in Ghidra: ghidra -> Import bypassme.bin -> Analyze")
    print("2. Find main() and trace the verification logic")
    print("3. In GDB:")
    print("   gdb ./bypassme.bin")
    print("   b main")
    print("   run")
    print("   # Step through, watching for cmp/test instructions")
    print("   # At each comparison, check register values:")
    print("   info registers")
    print("   x/s $rdi")
    print("   x/s $rsi")
    print("4. Patch binary:")
    print("   # In radare2: r2 -w bypassme.bin")
    print("   # s <addr_of_jne>")
    print("   # wa je <target>  OR  wa nop; nop")


if __name__ == "__main__":
    main()
