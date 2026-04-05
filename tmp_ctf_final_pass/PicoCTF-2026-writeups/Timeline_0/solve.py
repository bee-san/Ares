#!/usr/bin/env python3
"""
Timeline 0 - picoCTF 2026
Category: Forensics | Points: 100

This script automates the analysis of a disk image to find a hidden flag
using Sleuth Kit tools (fls, mactime, icat) and string searching.

Prerequisites:
    - sleuthkit (apt install sleuthkit)
    - Python 3.6+

Usage:
    python3 solve.py <disk_image>
    python3 solve.py disk.img
"""

import subprocess
import sys
import os
import re
import tempfile


def run_cmd(cmd, description="", capture=True):
    """Run a shell command and return output."""
    if description:
        print(f"[*] {description}")
    try:
        result = subprocess.run(
            cmd, shell=True, capture_output=capture,
            text=True, timeout=120
        )
        if capture:
            return result.stdout, result.stderr, result.returncode
        return "", "", result.returncode
    except FileNotFoundError:
        return "", "Command not found", 1
    except subprocess.TimeoutExpired:
        return "", "Command timed out", 1


def check_dependencies():
    """Verify required tools are installed."""
    tools = ["mmls", "fls", "mactime", "icat", "strings"]
    missing = []
    for tool in tools:
        _, _, rc = run_cmd(f"which {tool}")
        if rc != 0:
            missing.append(tool)
    if missing:
        print(f"[!] Missing tools: {', '.join(missing)}")
        print("[!] Install with: sudo apt install sleuthkit")
        if "strings" in missing:
            print("[!] strings is usually in binutils: sudo apt install binutils")
        return False
    return True


def get_partitions(image):
    """Use mmls to find partition offsets."""
    stdout, stderr, rc = run_cmd(f"mmls '{image}'", "Examining partition table")

    if rc != 0:
        print(f"[*] mmls failed (rc={rc}), image may be a raw filesystem")
        print(f"[*] Trying offset=0...")
        return [0]

    print(stdout)

    # Parse partition offsets - look for Linux/data partitions
    offsets = []
    for line in stdout.split("\n"):
        # Match lines with partition entries (not meta or unallocated)
        if any(x in line.lower() for x in ["linux", "ntfs", "fat", "hfs", "ext"]):
            parts = line.split()
            for part in parts:
                try:
                    offset = int(part)
                    if offset > 0:
                        offsets.append(offset)
                        break
                except ValueError:
                    continue

    if not offsets:
        # Try all numeric values that could be offsets
        for line in stdout.split("\n"):
            parts = line.split()
            if len(parts) >= 3 and "Unallocated" not in line and "Meta" not in line:
                try:
                    offset = int(parts[2])  # Start column
                    if offset > 0:
                        offsets.append(offset)
                except (ValueError, IndexError):
                    continue

    if not offsets:
        print("[*] No partitions found, trying offset=0")
        offsets = [0]

    return offsets


def search_strings(image):
    """Brute-force string search for the flag."""
    print("\n[*] === Method 1: Raw String Search ===")
    stdout, _, _ = run_cmd(
        f"strings '{image}' | grep -i 'picoCTF'",
        "Searching for picoCTF strings in image"
    )

    flags = []
    if stdout.strip():
        for line in stdout.strip().split("\n"):
            print(f"    [+] Found: {line.strip()}")
            flags.append(line.strip())

    # Also search for common flag file content patterns
    stdout2, _, _ = run_cmd(
        f"strings '{image}' | grep -iE '(flag|secret|hidden|key)' | head -20",
        "Searching for flag-related strings"
    )
    if stdout2.strip():
        for line in stdout2.strip().split("\n"):
            print(f"    [?] Possible: {line.strip()}")

    return flags


def analyze_filesystem(image, offset):
    """Use fls to list files and search for flag-related entries."""
    print(f"\n[*] === Method 2: Filesystem Analysis (offset={offset}) ===")

    offset_arg = f"-o {offset}" if offset > 0 else ""

    # List all files recursively
    stdout, stderr, rc = run_cmd(
        f"fls {offset_arg} -r -p '{image}'",
        f"Listing all files (offset={offset})"
    )

    if rc != 0:
        print(f"[!] fls failed: {stderr.strip()}")
        return []

    findings = []
    all_files = stdout.strip().split("\n") if stdout.strip() else []
    print(f"[*] Found {len(all_files)} file entries")

    # Search for flag-related files
    for line in all_files:
        lower = line.lower()
        if any(keyword in lower for keyword in ["flag", "secret", "hidden", "picoctf", "key.txt", "password"]):
            print(f"    [+] Interesting: {line.strip()}")
            findings.append(line.strip())

    # Try to extract interesting files
    for finding in findings:
        # Parse inode from fls output (format: "r/r INODE: filename")
        match = re.search(r'[rd]/[rd*-]\s+(\d+)(?:\(\w+\))?:', finding)
        if match:
            inode = match.group(1)
            content, _, rc = run_cmd(
                f"icat {offset_arg} '{image}' {inode}",
                f"Extracting inode {inode}"
            )
            if content.strip():
                print(f"    [+] Content: {content.strip()}")
                findings.append(content.strip())

    return findings


def create_timeline(image, offset):
    """Generate a filesystem timeline and search for the flag."""
    print(f"\n[*] === Method 3: Timeline Analysis (offset={offset}) ===")

    offset_arg = f"-o {offset}" if offset > 0 else ""

    with tempfile.NamedTemporaryFile(mode='w', suffix='.body', delete=False) as body_file:
        body_path = body_file.name

    with tempfile.NamedTemporaryFile(mode='w', suffix='.timeline', delete=False) as timeline_file:
        timeline_path = timeline_file.name

    try:
        # Generate body file
        stdout, stderr, rc = run_cmd(
            f"fls -m '/' {offset_arg} -r '{image}' > '{body_path}'",
            "Generating timeline body file"
        )

        # Check body file has content
        if os.path.getsize(body_path) == 0:
            print("[!] Body file is empty")
            return []

        # Generate timeline
        stdout, stderr, rc = run_cmd(
            f"mactime -b '{body_path}' > '{timeline_path}'",
            "Creating timeline with mactime"
        )

        # Read and search timeline
        with open(timeline_path, 'r', errors='replace') as f:
            timeline_content = f.read()

        # Search for flag references
        findings = []
        for line in timeline_content.split("\n"):
            lower = line.lower()
            if any(kw in lower for kw in ["flag", "secret", "hidden", "picoctf", "key"]):
                print(f"    [+] Timeline hit: {line.strip()}")
                findings.append(line.strip())

        # Also search the body file for flag-like filenames
        with open(body_path, 'r', errors='replace') as f:
            body_content = f.read()

        for line in body_content.split("\n"):
            if "flag" in line.lower() or "picoctf" in line.lower():
                print(f"    [+] Body file hit: {line.strip()}")

        # Print timeline summary
        total_lines = len(timeline_content.split("\n"))
        print(f"[*] Timeline contains {total_lines} entries")

        # Print first and last few entries for context
        lines = timeline_content.strip().split("\n")
        if len(lines) > 10:
            print("[*] First 5 timeline entries:")
            for line in lines[:5]:
                print(f"    {line}")
            print("[*] Last 5 timeline entries:")
            for line in lines[-5:]:
                print(f"    {line}")
        else:
            print("[*] Full timeline:")
            for line in lines:
                print(f"    {line}")

        return findings

    finally:
        # Cleanup temp files
        for path in [body_path, timeline_path]:
            try:
                os.unlink(path)
            except OSError:
                pass


def main():
    print("=" * 60)
    print("  Timeline 0 - picoCTF 2026 Solver")
    print("  Forensics Disk Image Analysis")
    print("=" * 60)
    print()

    # Get disk image path
    if len(sys.argv) < 2:
        # Look for common image filenames in current directory
        common_names = [
            "disk.img", "disk.raw", "image.img", "image.raw",
            "timeline.img", "timeline.raw", "disk.dd", "image.dd",
            "challenge.img", "disk.flag.img"
        ]
        image = None
        for name in common_names:
            if os.path.exists(name):
                image = name
                break

        if not image:
            # Check for any .img or .raw files
            for f in os.listdir("."):
                if f.endswith((".img", ".raw", ".dd", ".iso", ".E01")):
                    image = f
                    break

        if not image:
            print("Usage: python3 solve.py <disk_image>")
            print("\nNo disk image found in current directory.")
            sys.exit(1)
    else:
        image = sys.argv[1]

    if not os.path.exists(image):
        print(f"[!] File not found: {image}")
        sys.exit(1)

    print(f"[*] Analyzing: {image}")
    print(f"[*] Size: {os.path.getsize(image)} bytes")
    print()

    # Check dependencies
    if not check_dependencies():
        print("\n[!] Some tools missing, results may be incomplete")

    # Method 1: Raw string search (quick win)
    flags = search_strings(image)
    if flags:
        for flag in flags:
            if "picoCTF{" in flag:
                print(f"\n{'='*60}")
                print(f"[FLAG] {flag}")
                print(f"{'='*60}")
                return

    # Method 2 & 3: Filesystem and timeline analysis
    offsets = get_partitions(image)

    all_findings = []
    for offset in offsets:
        findings = analyze_filesystem(image, offset)
        all_findings.extend(findings)

        timeline_findings = create_timeline(image, offset)
        all_findings.extend(timeline_findings)

    # Summary
    print(f"\n{'='*60}")
    print("[*] Analysis complete!")

    if all_findings:
        print("[*] Findings:")
        for f in set(all_findings):
            print(f"    - {f}")

        # Check if any finding contains the flag pattern
        for f in all_findings:
            match = re.search(r'picoCTF\{[^}]+\}', f)
            if match:
                print(f"\n[FLAG] {match.group(0)}")
                return

        # If flag content found but not in picoCTF format
        print("\n[*] Remember: the challenge says to wrap the answer")
        print("[*] in picoCTF{} format: picoCTF{your_finding_here}")
    else:
        print("[-] No flag-related content found automatically.")
        print("[*] Try manual analysis:")
        print("    1. Mount the image: sudo mount -o loop,ro disk.img /mnt")
        print("    2. Browse files: ls -laR /mnt")
        print("    3. Check all text files: find /mnt -type f -exec grep -l picoCTF {} \\;")

    print(f"{'='*60}")


if __name__ == "__main__":
    main()
