#!/usr/bin/env python3
"""
Piece by Piece - picoCTF 2026
Category: General Skills (50 points)

Description: After logging in, you will find multiple file parts in your
home directory. These parts need to be combined and extracted to reveal the flag.

This script automates the process of:
1. Connecting to the challenge via SSH
2. Listing and identifying split file parts
3. Combining them with cat
4. Extracting the resulting archive
5. Reading the flag

Usage:
    python3 solve.py --host <HOST> --port <PORT> --user <USER> --password <PASS>
    python3 solve.py --local   (if files are already downloaded locally)

Example:
    python3 solve.py --host rescued-float.picoctf.net --port 55123 --user ctf-player --password abc123
"""

import argparse
import subprocess
import os
import glob
import zipfile
import re
import sys
import shutil

try:
    import paramiko
    HAS_PARAMIKO = True
except ImportError:
    HAS_PARAMIKO = False


def run_ssh_command(client, command):
    """Execute a command over SSH and return stdout."""
    stdin, stdout, stderr = client.exec_command(command)
    output = stdout.read().decode(errors='replace')
    error = stderr.read().decode(errors='replace')
    return output, error


def solve_remote(host, port, username, password):
    """
    Solve the challenge remotely via SSH.
    Connects, finds file parts, combines them, extracts, and reads the flag.
    """
    if not HAS_PARAMIKO:
        print("[!] paramiko not installed. Install with: pip install paramiko")
        print("[*] Falling back to system SSH command...")
        return solve_remote_system_ssh(host, port, username, password)

    print(f"[*] Connecting to {username}@{host}:{port}...")
    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    try:
        client.connect(host, port=port, username=username, password=password, timeout=30)
    except Exception as e:
        print(f"[!] SSH connection failed: {e}")
        return None

    print("[+] Connected!")

    # Step 1: List files in home directory
    output, _ = run_ssh_command(client, "ls -la ~")
    print(f"[*] Home directory contents:\n{output}")

    # Step 2: Identify file parts
    output, _ = run_ssh_command(client, "ls ~ | sort")
    files = [f.strip() for f in output.strip().split('\n') if f.strip()]
    print(f"[*] Files found: {files}")

    # Identify patterns of split files
    # Common patterns: file.zip.001, file.aa, file_part1, etc.

    # Try to find the base name and pattern
    zip_parts = [f for f in files if re.match(r'.*\.\d{3}$', f)]       # .001, .002, ...
    split_parts = [f for f in files if re.match(r'.*\.a[a-z]$', f)]     # .aa, .ab, ...
    part_files = [f for f in files if re.match(r'.*part\d+.*', f, re.I)] # part1, part2, ...
    generic_parts = [f for f in files if re.match(r'.*\.(part\d+|p\d+|chunk\d+)', f, re.I)]

    if zip_parts:
        parts = sorted(zip_parts)
        base_name = re.sub(r'\.\d{3}$', '', parts[0])
        combine_cmd = f"cat ~/{base_name}.* > /tmp/combined_file"
    elif split_parts:
        parts = sorted(split_parts)
        base_name = re.sub(r'\.a[a-z]$', '', parts[0])
        combine_cmd = f"cat ~/{base_name}.* > /tmp/combined_file"
    elif part_files:
        parts = sorted(part_files)
        # Try glob-based combination
        common_prefix = os.path.commonprefix(parts)
        combine_cmd = f"cat ~/{common_prefix}* > /tmp/combined_file"
    elif generic_parts:
        parts = sorted(generic_parts)
        common_prefix = os.path.commonprefix(parts)
        combine_cmd = f"cat ~/{common_prefix}* > /tmp/combined_file"
    else:
        # Fallback: try combining all non-hidden files
        non_hidden = [f for f in files if not f.startswith('.')]
        print(f"[*] No obvious pattern found. Trying all files: {non_hidden}")
        parts = sorted(non_hidden)
        if len(parts) > 1:
            file_list = ' '.join(f'~/{f}' for f in parts)
            combine_cmd = f"cat {file_list} > /tmp/combined_file"
        else:
            combine_cmd = f"cp ~/{parts[0]} /tmp/combined_file" if parts else None

    if not parts:
        print("[!] No file parts found!")
        client.close()
        return None

    print(f"[*] Identified {len(parts)} file parts: {parts}")

    # Step 3: Combine the parts
    print(f"[*] Combining: {combine_cmd}")
    output, error = run_ssh_command(client, combine_cmd)
    if error:
        print(f"[!] Combine error: {error}")

    # Step 4: Identify the combined file type
    output, _ = run_ssh_command(client, "file /tmp/combined_file")
    print(f"[*] File type: {output.strip()}")

    # Step 5: Extract the archive
    flag = None

    if 'zip' in output.lower():
        # Try without password first
        output, error = run_ssh_command(client, "cd /tmp && unzip -o combined_file 2>&1")
        print(f"[*] Unzip output: {output}")

        if 'password' in output.lower() or 'password' in error.lower():
            # Try common passwords
            common_passwords = ['picoCTF', 'password', 'flag', 'ctf', 'pico', '1234', 'admin']
            for pwd in common_passwords:
                output, error = run_ssh_command(client,
                    f"cd /tmp && unzip -o -P '{pwd}' combined_file 2>&1")
                if 'extracting' in output.lower() or 'inflating' in output.lower():
                    print(f"[+] ZIP password: {pwd}")
                    break

    elif 'gzip' in output.lower():
        run_ssh_command(client, "cd /tmp && mv combined_file combined_file.gz && gunzip combined_file.gz")
    elif 'tar' in output.lower():
        run_ssh_command(client, "cd /tmp && tar xf combined_file")
    elif 'xz' in output.lower():
        run_ssh_command(client, "cd /tmp && mv combined_file combined_file.xz && unxz combined_file.xz")
    elif 'bzip2' in output.lower():
        run_ssh_command(client, "cd /tmp && mv combined_file combined_file.bz2 && bunzip2 combined_file.bz2")

    # Step 6: Find and read the flag
    # Search for flag in extracted files
    output, _ = run_ssh_command(client, "ls -la /tmp/")
    print(f"[*] /tmp contents after extraction:\n{output}")

    # Look for flag files
    output, _ = run_ssh_command(client, "find /tmp -name 'flag*' -o -name '*.txt' 2>/dev/null | head -20")
    flag_files = [f.strip() for f in output.strip().split('\n') if f.strip()]

    for ff in flag_files:
        output, _ = run_ssh_command(client, f"cat '{ff}'")
        flag_match = re.search(r'picoCTF\{[^}]+\}', output)
        if flag_match:
            flag = flag_match.group(0)
            print(f"\n[+] FLAG found in {ff}: {flag}")
            break

    if not flag:
        # Broader search: grep for the flag pattern in all extracted files
        output, _ = run_ssh_command(client,
            "grep -r 'picoCTF{' /tmp/ 2>/dev/null | head -5")
        flag_match = re.search(r'picoCTF\{[^}]+\}', output)
        if flag_match:
            flag = flag_match.group(0)
            print(f"\n[+] FLAG: {flag}")

    if not flag:
        # Try reading the combined file directly
        output, _ = run_ssh_command(client, "strings /tmp/combined_file | grep picoCTF")
        flag_match = re.search(r'picoCTF\{[^}]+\}', output)
        if flag_match:
            flag = flag_match.group(0)
            print(f"\n[+] FLAG (from strings): {flag}")

    client.close()

    if not flag:
        print("[!] Flag not found automatically. Try connecting manually via SSH.")
    return flag


def solve_remote_system_ssh(host, port, username, password):
    """Fallback: use system ssh/sshpass command."""
    print("[*] Attempting solution with system SSH...")
    print(f"[*] Run these commands manually:")
    print(f"    ssh {username}@{host} -p {port}")
    print(f"    Password: {password}")
    print(f"    ls -la")
    print(f"    cat *.zip.* > combined.zip  (or appropriate pattern)")
    print(f"    unzip combined.zip")
    print(f"    cat flag.txt")
    return None


def solve_local(directory="."):
    """
    Solve locally if files have been downloaded.
    Combines split files and extracts the archive.
    """
    print(f"[*] Looking for file parts in: {os.path.abspath(directory)}")

    # Find file parts
    all_files = sorted(os.listdir(directory))
    print(f"[*] Files: {all_files}")

    # Identify split file patterns
    zip_parts = sorted([f for f in all_files if re.match(r'.*\.\d{3}$', f)])
    split_parts = sorted([f for f in all_files if re.match(r'.*\.a[a-z]$', f)])
    part_files = sorted([f for f in all_files if re.match(r'.*part\d+.*', f, re.I)])

    if zip_parts:
        parts = zip_parts
    elif split_parts:
        parts = split_parts
    elif part_files:
        parts = part_files
    else:
        print("[!] No obvious split file pattern found.")
        return None

    print(f"[+] Found {len(parts)} file parts: {parts}")

    # Combine parts
    combined_path = os.path.join(directory, "combined_file")
    with open(combined_path, 'wb') as outf:
        for part in parts:
            part_path = os.path.join(directory, part)
            with open(part_path, 'rb') as inf:
                outf.write(inf.read())

    print(f"[+] Combined file written to: {combined_path}")
    print(f"[+] Combined file size: {os.path.getsize(combined_path)} bytes")

    # Identify file type
    try:
        result = subprocess.run(['file', combined_path], capture_output=True, text=True)
        file_type = result.stdout.strip()
        print(f"[*] File type: {file_type}")
    except FileNotFoundError:
        file_type = ""

    # Extract
    extract_dir = os.path.join(directory, "extracted")
    os.makedirs(extract_dir, exist_ok=True)

    if 'zip' in file_type.lower() or combined_path.endswith('.zip'):
        try:
            with zipfile.ZipFile(combined_path, 'r') as zf:
                # Try without password
                try:
                    zf.extractall(extract_dir)
                    print(f"[+] Extracted to: {extract_dir}")
                except RuntimeError:
                    # Password protected
                    common_passwords = [b'picoCTF', b'password', b'flag', b'ctf', b'pico', b'1234']
                    for pwd in common_passwords:
                        try:
                            zf.extractall(extract_dir, pwd=pwd)
                            print(f"[+] ZIP password: {pwd.decode()}")
                            break
                        except RuntimeError:
                            continue
        except zipfile.BadZipFile:
            print("[!] Not a valid ZIP. Trying other formats...")
            subprocess.run(['tar', 'xf', combined_path, '-C', extract_dir],
                           capture_output=True)
    elif 'tar' in file_type.lower() or 'gzip' in file_type.lower():
        subprocess.run(['tar', 'xf', combined_path, '-C', extract_dir],
                       capture_output=True)

    # Find flag
    for root, dirs, files_list in os.walk(extract_dir):
        for fname in files_list:
            fpath = os.path.join(root, fname)
            try:
                with open(fpath, 'r', errors='replace') as f:
                    content = f.read()
                flag_match = re.search(r'picoCTF\{[^}]+\}', content)
                if flag_match:
                    flag = flag_match.group(0)
                    print(f"\n[+] FLAG found in {fpath}: {flag}")
                    return flag
            except Exception:
                continue

    # Try strings on combined file
    try:
        result = subprocess.run(['strings', combined_path], capture_output=True, text=True)
        flag_match = re.search(r'picoCTF\{[^}]+\}', result.stdout)
        if flag_match:
            print(f"\n[+] FLAG (from strings): {flag_match.group(0)}")
            return flag_match.group(0)
    except FileNotFoundError:
        pass

    print("[!] Flag not found automatically.")
    return None


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Piece by Piece Solver - picoCTF 2026")
    parser.add_argument("--host", type=str, help="SSH hostname")
    parser.add_argument("--port", type=int, default=22, help="SSH port")
    parser.add_argument("--user", type=str, default="ctf-player", help="SSH username")
    parser.add_argument("--password", type=str, help="SSH password")
    parser.add_argument("--local", action="store_true",
                        help="Solve locally (files already downloaded)")
    parser.add_argument("--dir", type=str, default=".",
                        help="Directory containing local file parts")
    args = parser.parse_args()

    if args.local:
        flag = solve_local(args.dir)
    elif args.host and args.password:
        flag = solve_remote(args.host, args.port, args.user, args.password)
    else:
        print("Usage:")
        print("  Remote: python3 solve.py --host <HOST> --port <PORT> --password <PASS>")
        print("  Local:  python3 solve.py --local --dir <directory>")
        print()
        print("Example:")
        print("  python3 solve.py --host rescued-float.picoctf.net --port 55123 --password abc123")
        sys.exit(1)

    if flag:
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
