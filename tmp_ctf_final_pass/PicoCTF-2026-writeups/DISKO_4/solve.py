#!/usr/bin/env python3
"""
DISKO 4 - picoCTF 2026 (Forensics, 200 pts)

Recover a deleted flag file from a disk image.
The file was deleted but the data remains on disk.

Usage:
    python3 solve.py [disk_image.dd.gz]

Requires: The Sleuth Kit (apt install sleuthkit), or just Python for basic recovery.
"""

import gzip
import os
import re
import struct
import subprocess
import sys


def find_disk_image():
    """Find the disk image file in the current directory."""
    candidates = []
    for f in os.listdir("."):
        if f.endswith((".dd", ".dd.gz", ".img", ".img.gz", ".raw")):
            candidates.append(f)
    # Also check common names
    for name in ["disko-4.dd.gz", "disko-4.dd", "disko4.dd.gz", "disko4.dd"]:
        if os.path.exists(name):
            return name
    if candidates:
        return candidates[0]
    return None


def decompress_if_needed(filepath):
    """Decompress gzip file if needed, return path to raw disk image."""
    if filepath.endswith(".gz"):
        raw_path = filepath[:-3]  # Remove .gz extension
        if not os.path.exists(raw_path):
            print(f"[*] Decompressing {filepath}...")
            with gzip.open(filepath, "rb") as f_in:
                with open(raw_path, "wb") as f_out:
                    while True:
                        chunk = f_in.read(1024 * 1024)
                        if not chunk:
                            break
                        f_out.write(chunk)
            print(f"[+] Decompressed to {raw_path}")
        return raw_path
    return filepath


def search_strings(disk_path):
    """Search for the flag using strings-style extraction."""
    print(f"[*] Searching for flag strings in {disk_path}...")
    flag_pattern = re.compile(rb"picoCTF\{[^\}]+\}")

    flags_found = set()
    chunk_size = 1024 * 1024  # 1MB chunks

    with open(disk_path, "rb") as f:
        offset = 0
        overlap = b""
        while True:
            chunk = f.read(chunk_size)
            if not chunk:
                break
            # Search in overlap + current chunk to catch flags spanning boundaries
            search_data = overlap + chunk
            matches = flag_pattern.findall(search_data)
            for m in matches:
                try:
                    flag = m.decode("utf-8", errors="ignore")
                    flags_found.add(flag)
                    print(f"[+] Found flag: {flag}")
                except:
                    pass
            # Keep last 100 bytes as overlap for boundary-spanning flags
            overlap = chunk[-100:]
            offset += len(chunk)

    return flags_found


def try_sleuthkit_recovery(disk_path):
    """Attempt recovery using The Sleuth Kit tools."""
    flags_found = set()

    # Check if sleuthkit is available
    try:
        subprocess.run(["fls", "-V"], capture_output=True, check=True)
    except (FileNotFoundError, subprocess.CalledProcessError):
        print("[!] SleuthKit not found. Install with: sudo apt install sleuthkit")
        return flags_found

    # Try different partition offsets (common values)
    offsets = [0, 2048, 4096, 1, 63, 128]

    for offset in offsets:
        try:
            # List files including deleted ones
            result = subprocess.run(
                ["fls", "-r", "-o", str(offset), disk_path],
                capture_output=True, text=True, timeout=30
            )

            if result.returncode != 0:
                continue

            print(f"[*] fls succeeded with offset {offset}")
            lines = result.stdout.strip().split("\n")

            for line in lines:
                # Look for deleted files (marked with *) or flag-related files
                if "*" in line or "flag" in line.lower():
                    print(f"[*] Interesting entry: {line}")

                    # Extract inode number
                    inode_match = re.search(r"(\d+)(?:-\d+)?(?:-\d+)?:", line)
                    if inode_match:
                        inode = inode_match.group(1)
                        # Try to recover the file using icat
                        try:
                            icat_result = subprocess.run(
                                ["icat", "-o", str(offset), disk_path, inode],
                                capture_output=True, timeout=10
                            )
                            if icat_result.returncode == 0:
                                content = icat_result.stdout
                                flag_match = re.search(rb"picoCTF\{[^\}]+\}", content)
                                if flag_match:
                                    flag = flag_match.group(0).decode()
                                    flags_found.add(flag)
                                    print(f"[+] Recovered flag from inode {inode}: {flag}")
                        except subprocess.TimeoutExpired:
                            pass

        except subprocess.TimeoutExpired:
            continue
        except Exception as e:
            continue

    return flags_found


def try_tsk_recover(disk_path):
    """Attempt bulk recovery using tsk_recover."""
    flags_found = set()

    try:
        subprocess.run(["tsk_recover", "-h"], capture_output=True)
    except FileNotFoundError:
        print("[!] tsk_recover not found.")
        return flags_found

    output_dir = "recovered_files"
    os.makedirs(output_dir, exist_ok=True)

    offsets = [0, 2048]
    for offset in offsets:
        try:
            subprocess.run(
                ["tsk_recover", "-o", str(offset), disk_path, output_dir],
                capture_output=True, timeout=60
            )
            # Search recovered files for flags
            for root, dirs, files in os.walk(output_dir):
                for fname in files:
                    fpath = os.path.join(root, fname)
                    try:
                        with open(fpath, "rb") as f:
                            content = f.read()
                            flag_match = re.search(rb"picoCTF\{[^\}]+\}", content)
                            if flag_match:
                                flag = flag_match.group(0).decode()
                                flags_found.add(flag)
                                print(f"[+] Found flag in {fpath}: {flag}")
                    except:
                        pass
        except (subprocess.TimeoutExpired, Exception):
            continue

    return flags_found


def try_extundelete(disk_path):
    """Attempt recovery using extundelete (for ext filesystems)."""
    flags_found = set()

    try:
        subprocess.run(["extundelete", "--help"], capture_output=True)
    except FileNotFoundError:
        print("[!] extundelete not found. Install with: sudo apt install extundelete")
        return flags_found

    try:
        result = subprocess.run(
            ["extundelete", disk_path, "--restore-all"],
            capture_output=True, text=True, timeout=60
        )
        # Check RECOVERED_FILES directory
        recovered_dir = "RECOVERED_FILES"
        if os.path.exists(recovered_dir):
            for root, dirs, files in os.walk(recovered_dir):
                for fname in files:
                    fpath = os.path.join(root, fname)
                    try:
                        with open(fpath, "rb") as f:
                            content = f.read()
                            flag_match = re.search(rb"picoCTF\{[^\}]+\}", content)
                            if flag_match:
                                flag = flag_match.group(0).decode()
                                flags_found.add(flag)
                                print(f"[+] Found flag in {fpath}: {flag}")
                    except:
                        pass
    except (subprocess.TimeoutExpired, Exception) as e:
        print(f"[!] extundelete error: {e}")

    return flags_found


def main():
    # Determine disk image path
    if len(sys.argv) > 1:
        disk_image = sys.argv[1]
    else:
        disk_image = find_disk_image()
        if not disk_image:
            print("[!] No disk image found. Usage: python3 solve.py <disk_image>")
            print("[!] Expected file: disko-4.dd.gz or disko-4.dd")
            sys.exit(1)

    print(f"[*] Using disk image: {disk_image}")

    # Decompress if needed
    raw_image = decompress_if_needed(disk_image)

    # Print file info
    try:
        result = subprocess.run(["file", raw_image], capture_output=True, text=True)
        print(f"[*] File type: {result.stdout.strip()}")
    except:
        pass

    all_flags = set()

    # Method 1: Direct string search (fastest, often works for deleted files)
    print("\n[=== Method 1: String Search ===]")
    flags = search_strings(raw_image)
    all_flags.update(flags)

    # Method 2: SleuthKit recovery
    print("\n[=== Method 2: SleuthKit (fls + icat) ===]")
    flags = try_sleuthkit_recovery(raw_image)
    all_flags.update(flags)

    # Method 3: tsk_recover bulk recovery
    print("\n[=== Method 3: tsk_recover ===]")
    flags = try_tsk_recover(raw_image)
    all_flags.update(flags)

    # Method 4: extundelete
    print("\n[=== Method 4: extundelete ===]")
    flags = try_extundelete(raw_image)
    all_flags.update(flags)

    # Summary
    print("\n" + "=" * 60)
    if all_flags:
        print(f"[+] Found {len(all_flags)} flag(s):")
        for flag in sorted(all_flags):
            print(f"    {flag}")
    else:
        print("[!] No flags found automatically.")
        print("[*] Manual steps to try:")
        print(f"    1. strings {raw_image} | grep picoCTF")
        print(f"    2. fls -r -o 2048 {raw_image}")
        print(f"    3. tsk_recover -o 2048 {raw_image} output/")
        print(f"    4. Mount and inspect: sudo mount -o loop,ro {raw_image} /mnt/disk")


if __name__ == "__main__":
    main()
