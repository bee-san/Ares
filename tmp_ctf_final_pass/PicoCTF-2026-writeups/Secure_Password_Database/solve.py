#!/usr/bin/env python3
"""
Secure Password Database - picoCTF 2026 (Reverse Engineering, 200 pts)

The binary takes a password, transforms it, shows the "database" representation,
and checks it against a stored value. We reverse-engineer the transformation to
extract the flag.

Multiple approaches:
  1. Run 'strings' to check for plaintext flag
  2. Use ltrace to capture strcmp/memcmp arguments
  3. Extract hardcoded comparison values and reverse the transformation
  4. Probe the "database display" feature to deduce the encoding

Usage:
    python3 solve.py [binary_path]

Dependencies: None required (uses subprocess). Optional: pip install pwntools
"""

import subprocess
import sys
import os
import re
import struct
import string


def find_flag_in_strings(binary_path):
    """Approach 1: Search for the flag directly in binary strings."""
    print("[*] Approach 1: Searching binary strings...")

    try:
        result = subprocess.run(
            ['strings', binary_path],
            capture_output=True, text=True, timeout=10
        )
        lines = result.stdout.strip().split('\n')

        # Search for direct flag
        for line in lines:
            if 'picoCTF{' in line:
                match = re.search(r'picoCTF\{[^}]+\}', line)
                if match:
                    print(f"    [+] Flag found in strings: {match.group(0)}")
                    return match.group(0)

        # Collect interesting strings for later analysis
        interesting = []
        keywords = ['password', 'flag', 'secret', 'correct', 'wrong',
                     'database', 'auth', 'key', 'enter', 'input']
        for line in lines:
            if any(kw in line.lower() for kw in keywords):
                interesting.append(line)

        if interesting:
            print("    [*] Interesting strings found:")
            for s in interesting[:20]:
                print(f"        {s}")

        # Look for hex-encoded or base64-encoded flag
        hex_pattern = re.compile(r'[0-9a-fA-F]{40,}')
        for line in lines:
            for match in hex_pattern.finditer(line):
                try:
                    decoded = bytes.fromhex(match.group(0)).decode('utf-8', errors='ignore')
                    if 'picoCTF' in decoded:
                        flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                        if flag_match:
                            print(f"    [+] Hex-encoded flag: {flag_match.group(0)}")
                            return flag_match.group(0)
                except Exception:
                    continue

        import base64
        b64_pattern = re.compile(r'[A-Za-z0-9+/]{20,}={0,2}')
        for line in lines:
            for match in b64_pattern.finditer(line):
                try:
                    decoded = base64.b64decode(match.group(0)).decode('utf-8', errors='ignore')
                    if 'picoCTF' in decoded:
                        flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                        if flag_match:
                            print(f"    [+] Base64-encoded flag: {flag_match.group(0)}")
                            return flag_match.group(0)
                except Exception:
                    continue

    except Exception as e:
        print(f"    [!] strings search failed: {e}")

    return None


def try_ltrace(binary_path):
    """Approach 2: Use ltrace to capture comparison function arguments."""
    print("\n[*] Approach 2: Running ltrace to capture comparisons...")

    test_inputs = ["test", "AAAA", "picoCTF{test}", "password"]

    for test_input in test_inputs:
        try:
            result = subprocess.run(
                ['ltrace', '-s', '256', binary_path],
                input=test_input + '\n',
                capture_output=True, text=True, timeout=10
            )

            output = result.stderr  # ltrace outputs to stderr
            if not output:
                continue

            print(f"    [*] Input: '{test_input}'")

            # Look for strcmp/memcmp/strncmp calls
            for line in output.split('\n'):
                if any(fn in line for fn in ['strcmp', 'memcmp', 'strncmp', 'strstr']):
                    print(f"        {line.strip()}")

                    # Extract the comparison string
                    match = re.search(r'picoCTF\{[^}]+\}', line)
                    if match:
                        print(f"    [+] Flag captured from ltrace: {match.group(0)}")
                        return match.group(0)

                    # Extract quoted strings from the comparison
                    str_matches = re.findall(r'"([^"]*)"', line)
                    for s in str_matches:
                        if s != test_input and len(s) > 5:
                            print(f"        [*] Comparison value: {s}")
                            if 'picoCTF' in s:
                                return s

        except FileNotFoundError:
            print("    [!] ltrace not found, skipping...")
            return None
        except subprocess.TimeoutExpired:
            print(f"    [!] Timeout with input '{test_input}'")
            continue
        except Exception as e:
            print(f"    [!] ltrace error: {e}")
            continue

    return None


def analyze_transformation(binary_path):
    """
    Approach 3: Probe the 'database display' to understand the transformation.
    Send known inputs and observe the output to reverse the encoding.
    """
    print("\n[*] Approach 3: Analyzing input/output transformation...")

    # Send test inputs and collect the "database" representations
    test_chars = string.ascii_uppercase[:26]
    io_map = {}

    for char in test_chars:
        test_input = char * 8  # Send "AAAAAAAA", "BBBBBBBB", etc.
        try:
            result = subprocess.run(
                [binary_path],
                input=test_input + '\n',
                capture_output=True, text=True, timeout=5
            )
            output = result.stdout
            if output:
                # The "database" output might contain the transformed version
                # Try to find the transformed representation
                io_map[char] = output
        except Exception:
            continue

    if not io_map:
        print("    [!] Could not capture output from binary")
        return None

    # Analyze the outputs to detect the transformation pattern
    print(f"    [*] Collected {len(io_map)} input/output pairs")

    # Check if it's a simple XOR
    print("    [*] Checking for XOR transformation...")
    for char, output in list(io_map.items())[:3]:
        print(f"        Input '{char}' -> Output: {output.strip()[:80]}")

    # Try to detect XOR key by sending known values
    # If output[i] = input[i] ^ key[i], then key[i] = input[i] ^ output[i]
    single_char_tests = {}
    for i in range(256):
        char = chr(i) if 32 <= i < 127 else None
        if char:
            single_char_tests[char] = i

    # Extract numeric/hex values from output for XOR analysis
    for char, output in io_map.items():
        hex_values = re.findall(r'(?:0x)?([0-9a-fA-F]{2})', output)
        if hex_values:
            input_val = ord(char)
            for j, hv in enumerate(hex_values):
                output_val = int(hv, 16)
                xor_key = input_val ^ output_val
                print(f"        Byte {j}: input=0x{input_val:02x} output=0x{output_val:02x} "
                      f"XOR key=0x{xor_key:02x} ('{chr(xor_key) if 32 <= xor_key < 127 else '?'}')")

    return None


def extract_hardcoded_values(binary_path):
    """
    Approach 4: Use objdump to find hardcoded comparison arrays in the binary.
    """
    print("\n[*] Approach 4: Extracting hardcoded values from binary...")

    try:
        # Disassemble and look for immediate values in comparison instructions
        result = subprocess.run(
            ['objdump', '-d', binary_path],
            capture_output=True, text=True, timeout=30
        )

        if not result.stdout:
            print("    [!] objdump produced no output")
            return None

        # Look for cmp instructions with immediate values (character comparisons)
        cmp_values = []
        lines = result.stdout.split('\n')

        for i, line in enumerate(lines):
            # x86: cmp $0xXX, ... patterns (byte comparisons)
            match = re.search(r'cmp\s+\$0x([0-9a-fA-F]+)', line)
            if match:
                val = int(match.group(1), 16)
                if 0x20 <= val <= 0x7e:  # Printable ASCII
                    cmp_values.append(val)

            # x86: mov immediate values that look like ASCII
            match = re.search(r'mov\s+\$0x([0-9a-fA-F]+)', line)
            if match:
                val = int(match.group(1), 16)
                # Check if it could be packed ASCII characters
                if 0x20 <= val <= 0x7e:
                    cmp_values.append(val)
                elif val > 0xFF:
                    # Might be multiple characters packed
                    chars = []
                    temp = val
                    while temp > 0:
                        byte = temp & 0xFF
                        if 0x20 <= byte <= 0x7e:
                            chars.append(chr(byte))
                        temp >>= 8
                    if chars:
                        packed_str = ''.join(chars)
                        if 'pico' in packed_str.lower():
                            print(f"        [!] Packed string found: {packed_str}")

        if cmp_values:
            # Try to form a string from comparison values
            potential_str = ''.join(chr(v) for v in cmp_values if 0x20 <= v <= 0x7e)
            print(f"    [*] Characters from cmp instructions: {potential_str[:80]}")
            match = re.search(r'picoCTF\{[^}]+\}', potential_str)
            if match:
                return match.group(0)

        # Also check .rodata section for data arrays
        result2 = subprocess.run(
            ['objdump', '-s', '-j', '.rodata', binary_path],
            capture_output=True, text=True, timeout=10
        )
        if result2.stdout:
            # Parse the hex dump
            hex_data = ''
            for line in result2.stdout.split('\n'):
                match = re.match(r'\s*[0-9a-fA-F]+\s+((?:[0-9a-fA-F]+\s*)+)', line)
                if match:
                    hex_data += match.group(1).replace(' ', '')

            if hex_data:
                try:
                    raw_bytes = bytes.fromhex(hex_data)
                    decoded = raw_bytes.decode('utf-8', errors='ignore')
                    flag_match = re.search(r'picoCTF\{[^}]+\}', decoded)
                    if flag_match:
                        print(f"    [+] Flag in .rodata: {flag_match.group(0)}")
                        return flag_match.group(0)
                except Exception:
                    pass

    except Exception as e:
        print(f"    [!] objdump analysis failed: {e}")

    return None


def try_common_xor_decode(binary_path):
    """
    Approach 5: Read the binary and try common XOR decoding patterns.
    """
    print("\n[*] Approach 5: Trying XOR decode on binary data...")

    with open(binary_path, 'rb') as f:
        data = f.read()

    # Try single-byte XOR keys
    for key in range(1, 256):
        decoded = bytes(b ^ key for b in data)
        if b'picoCTF{' in decoded:
            match = re.search(rb'picoCTF\{[^}]+\}', decoded)
            if match:
                flag = match.group(0).decode()
                print(f"    [+] XOR key 0x{key:02x}: {flag}")
                return flag

    # Search for the flag pattern in raw binary
    match = re.search(rb'picoCTF\{[^}]+\}', data)
    if match:
        flag = match.group(0).decode()
        print(f"    [+] Flag found in raw binary: {flag}")
        return flag

    # Look for reversed flag
    reversed_data = data[::-1]
    match = re.search(rb'picoCTF\{[^}]+\}', reversed_data)
    if match:
        flag = match.group(0).decode()
        print(f"    [+] Flag found reversed in binary: {flag}")
        return flag

    print("    [*] No simple XOR decode found")
    return None


def try_gdb_debug(binary_path):
    """
    Approach 6: Use GDB to set a breakpoint at comparison and read values.
    """
    print("\n[*] Approach 6: Automated GDB debugging...")

    gdb_commands = """
set pagination off
set confirm off
catch syscall write
run <<< "picoCTF{test_input}"
bt
info registers
x/s $rdi
x/s $rsi
continue
quit
"""

    try:
        result = subprocess.run(
            ['gdb', '-batch', '-ex', 'set pagination off',
             '-ex', 'break strcmp', '-ex', 'break memcmp',
             '-ex', 'break strncmp',
             '-ex', 'run <<< "AAAA"',
             '-ex', 'x/s $rdi', '-ex', 'x/s $rsi',
             '-ex', 'continue',
             '-ex', 'quit',
             binary_path],
            capture_output=True, text=True, timeout=10
        )

        output = result.stdout + result.stderr
        if 'picoCTF' in output:
            match = re.search(r'picoCTF\{[^}]+\}', output)
            if match:
                print(f"    [+] Flag from GDB: {match.group(0)}")
                return match.group(0)

        # Print any interesting output
        for line in output.split('\n'):
            if any(x in line for x in ['0x', 'strcmp', 'memcmp', 'pico', 'flag']):
                print(f"        {line.strip()}")

    except FileNotFoundError:
        print("    [!] GDB not found, skipping...")
    except Exception as e:
        print(f"    [!] GDB error: {e}")

    return None


def main():
    if len(sys.argv) < 2:
        # If no binary specified, look for common names in current directory
        common_names = [
            'password_db', 'secure_password', 'password_database',
            'secure_db', 'auth', 'challenge', 'binary', 'a.out',
            'secure_password_database'
        ]

        binary_path = None
        for name in common_names:
            if os.path.isfile(name):
                binary_path = name
                break

        if binary_path is None:
            print("Secure Password Database - picoCTF 2026 (Reverse Engineering, 200 pts)")
            print()
            print("Usage: python3 solve.py <binary>")
            print()
            print("This script attempts to extract the flag from the password")
            print("authentication binary using multiple approaches:")
            print("  1. String extraction (strings)")
            print("  2. Library call tracing (ltrace)")
            print("  3. Input/output transformation analysis")
            print("  4. Disassembly analysis (objdump)")
            print("  5. XOR brute-force decoding")
            print("  6. Automated GDB debugging")
            sys.exit(1)
    else:
        binary_path = sys.argv[1]

    if not os.path.isfile(binary_path):
        print(f"[!] File not found: {binary_path}")
        sys.exit(1)

    # Make sure binary is executable
    os.chmod(binary_path, 0o755)

    # Get binary info
    try:
        file_result = subprocess.run(
            ['file', binary_path], capture_output=True, text=True
        )
        print(f"[*] Binary: {file_result.stdout.strip()}")
    except Exception:
        pass

    print(f"[*] File size: {os.path.getsize(binary_path)} bytes")
    print()

    flag = None

    # Approach 1: String search
    flag = find_flag_in_strings(binary_path)

    # Approach 2: ltrace
    if flag is None:
        flag = try_ltrace(binary_path)

    # Approach 3: I/O transformation analysis
    if flag is None:
        flag = analyze_transformation(binary_path)

    # Approach 4: Disassembly analysis
    if flag is None:
        flag = extract_hardcoded_values(binary_path)

    # Approach 5: XOR brute-force
    if flag is None:
        flag = try_common_xor_decode(binary_path)

    # Approach 6: GDB debugging
    if flag is None:
        flag = try_gdb_debug(binary_path)

    # Final output
    print()
    if flag:
        print(f"{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("[!] Flag not found automatically.")
        print()
        print("[*] Manual investigation steps:")
        print("    1. Open the binary in Ghidra: ghidra -> New Project -> Import binary")
        print("    2. Find main() and trace the password check logic")
        print("    3. Look for hardcoded arrays/strings used in comparisons")
        print("    4. The 'database display' feature may reveal the encoding:")
        print("       - Run the binary with various inputs")
        print("       - Note how the 'database' representation changes")
        print("       - Reverse the transformation on the expected value")
        print("    5. Try: ltrace -s 256 ./binary")
        print("    6. Try: gdb ./binary, then 'break strcmp' and examine args")


if __name__ == '__main__':
    main()
