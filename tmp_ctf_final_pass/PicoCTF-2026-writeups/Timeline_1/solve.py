#!/usr/bin/env python3
"""
Timeline 1 - picoCTF 2026 (Forensics, 300 pts)

Automated solver for the Timeline 1 disk image forensics challenge.
This script performs filesystem timeline analysis on the provided disk
image to locate and extract the flag.

The approach:
  1. Identify partitions using mmls
  2. List all files (including deleted) using fls
  3. Generate a mactime timeline
  4. Search for flag indicators in filenames, file contents, and timestamps
  5. Extract and reconstruct the flag

Prerequisites:
  - The Sleuth Kit (apt install sleuthkit)
  - Python 3.6+

Usage:
  python3 solve.py disk.img
  python3 solve.py --image disk.img --output flag.txt
  python3 solve.py disk.img --strings-only   # fallback: strings search
"""

import argparse
import subprocess
import sys
import re
import os
import tempfile


def run_cmd(cmd, check=False):
    """Run a shell command and return stdout."""
    result = subprocess.run(
        cmd, shell=True, capture_output=True, text=True
    )
    if check and result.returncode != 0:
        print(f"[!] Command failed: {cmd}", file=sys.stderr)
        print(f"    stderr: {result.stderr.strip()}", file=sys.stderr)
    return result.stdout, result.stderr, result.returncode


def check_dependencies():
    """Verify that required tools are installed."""
    tools = ['mmls', 'fls', 'icat', 'mactime', 'fsstat', 'strings']
    missing = []
    for tool in tools:
        _, _, rc = run_cmd(f"which {tool}")
        if rc != 0:
            missing.append(tool)
    if missing:
        print(f"[!] Missing tools: {', '.join(missing)}")
        print("[*] Install The Sleuth Kit: sudo apt install sleuthkit")
        if set(missing) - {'strings'}:
            return False
    return True


def find_partitions(image_path):
    """Use mmls to find partitions and their offsets."""
    print("[*] Analyzing partition table with mmls...")
    stdout, stderr, rc = run_cmd(f"mmls '{image_path}'")

    if rc != 0:
        # mmls failed -- image might be a raw filesystem without a partition table
        print("[*] mmls failed -- trying direct filesystem access (offset=0)")
        return [{'start': 0, 'desc': 'raw filesystem'}]

    partitions = []
    for line in stdout.splitlines():
        # Parse mmls output lines like: 002:  000    0002048    0001024    Linux (0x83)
        match = re.search(
            r'(\d+):\s+\d+\s+(\d+)\s+(\d+)\s+(.+)',
            line
        )
        if match:
            slot = match.group(1)
            start = int(match.group(2))
            length = int(match.group(3))
            desc = match.group(4).strip()

            # Skip meta entries and unallocated space
            if 'Unalloc' in desc or 'Meta' in desc or length == 0:
                continue

            partitions.append({
                'slot': slot,
                'start': start,
                'length': length,
                'desc': desc,
            })

    if not partitions:
        # Fallback: try offset 0
        print("[*] No usable partitions found -- trying offset 0")
        partitions = [{'start': 0, 'desc': 'raw'}]

    for p in partitions:
        print(f"    Partition: offset={p['start']}, desc={p.get('desc', 'N/A')}")

    return partitions


def get_fs_type(image_path, offset):
    """Use fsstat to determine the filesystem type."""
    stdout, _, rc = run_cmd(f"fsstat -o {offset} '{image_path}' 2>/dev/null | head -5")
    if rc == 0 and stdout:
        print(f"[*] Filesystem info (offset {offset}):")
        for line in stdout.strip().splitlines()[:3]:
            print(f"    {line}")
    return stdout


def list_files(image_path, offset):
    """Use fls to recursively list all files including deleted."""
    print(f"[*] Listing files with fls (offset={offset})...")
    stdout, _, rc = run_cmd(f"fls -r -p -o {offset} '{image_path}'")
    if rc != 0:
        return []

    files = []
    for line in stdout.splitlines():
        # Parse fls output: type inode filename
        # e.g., r/r 45: home/user/flag.txt
        # Deleted files: r/r * 46(realloc): home/user/deleted.txt
        match = re.match(r'([rd]/[rd])\s+(\*?)\s*(\d+)(?:\(realloc\))?:\s+(.+)', line)
        if match:
            ftype = match.group(1)
            deleted = match.group(2).strip() == '*'
            inode = int(match.group(3))
            path = match.group(4).strip()
            files.append({
                'type': ftype,
                'deleted': deleted,
                'inode': inode,
                'path': path,
            })

    print(f"    Found {len(files)} files ({sum(1 for f in files if f['deleted'])} deleted)")
    return files


def generate_timeline(image_path, offset, tmpdir):
    """Generate a mactime timeline from the disk image."""
    body_file = os.path.join(tmpdir, 'body.txt')
    timeline_file = os.path.join(tmpdir, 'timeline.txt')

    print(f"[*] Generating body file...")
    run_cmd(f"fls -m / -r -o {offset} '{image_path}' > '{body_file}'")

    print(f"[*] Generating timeline with mactime...")
    run_cmd(f"mactime -b '{body_file}' > '{timeline_file}'")

    try:
        with open(timeline_file, 'r') as f:
            timeline = f.read()
        print(f"    Timeline: {len(timeline.splitlines())} entries")
        return timeline
    except Exception:
        return ""


def extract_file(image_path, offset, inode):
    """Extract a file from the disk image by inode number."""
    stdout, _, rc = run_cmd(f"icat -o {offset} '{image_path}' {inode}")
    if rc == 0:
        return stdout
    return ""


def search_strings(image_path):
    """Fallback: search for flag-like strings in the raw image."""
    print("[*] Searching raw strings in disk image...")
    stdout, _, _ = run_cmd(f"strings -a '{image_path}'")

    flag_pattern = re.compile(r'picoCTF\{[^}]+\}')
    flags = flag_pattern.findall(stdout)
    if flags:
        return flags

    # Search for partial flag indicators
    partial_pattern = re.compile(r'(?:flag|pico|ctf|secret|hidden).*', re.IGNORECASE)
    partials = partial_pattern.findall(stdout)
    return partials[:20] if partials else []


def search_for_flag(image_path, offset, files, timeline):
    """Search files, timeline, and raw content for the flag."""
    flag_pattern = re.compile(r'picoCTF\{[^}]+\}')

    # Strategy 1: Search filenames for flag indicators
    print("\n[*] Strategy 1: Searching filenames...")
    suspicious_files = []
    for f in files:
        name_lower = f['path'].lower()
        if any(kw in name_lower for kw in ['flag', 'secret', 'hidden', 'key', 'pico', 'ctf']):
            suspicious_files.append(f)
            marker = " [DELETED]" if f['deleted'] else ""
            print(f"    Found: {f['path']} (inode {f['inode']}){marker}")

    # Strategy 2: Extract and check suspicious files
    print("\n[*] Strategy 2: Extracting suspicious files...")
    for f in suspicious_files:
        content = extract_file(image_path, offset, f['inode'])
        if content:
            match = flag_pattern.search(content)
            if match:
                return match.group()
            # Check if content itself looks like a flag value (without wrapper)
            stripped = content.strip()
            if stripped and len(stripped) < 200:
                print(f"    {f['path']}: {stripped[:100]}")

    # Strategy 3: Search deleted files
    print("\n[*] Strategy 3: Checking deleted files...")
    deleted_files = [f for f in files if f['deleted']]
    for f in deleted_files:
        content = extract_file(image_path, offset, f['inode'])
        if content:
            match = flag_pattern.search(content)
            if match:
                return match.group()
            if 'pico' in content.lower() or 'flag' in content.lower():
                print(f"    Interesting deleted file: {f['path']}: {content.strip()[:100]}")

    # Strategy 4: Check ALL files for flag content
    print("\n[*] Strategy 4: Scanning all file contents...")
    for f in files:
        if f['type'].startswith('r'):  # regular files only
            content = extract_file(image_path, offset, f['inode'])
            if content:
                match = flag_pattern.search(content)
                if match:
                    return match.group()

    # Strategy 5: Search timeline for encoded flag in filenames
    print("\n[*] Strategy 5: Analyzing timeline for patterns...")
    if timeline:
        match = flag_pattern.search(timeline)
        if match:
            return match.group()

        # Look for files created in a suspicious pattern
        # (e.g., single-character filenames that spell out the flag)
        timeline_lines = timeline.strip().splitlines()
        short_names = []
        for line in timeline_lines:
            # mactime format: date,size,type,mode,uid,gid,inode,name
            parts = line.split('\t') if '\t' in line else line.split()
            if parts:
                name = parts[-1] if len(parts) > 1 else ''
                basename = os.path.basename(name)
                if len(basename) == 1 and basename.isalnum():
                    short_names.append((line, basename))

        if short_names:
            candidate = ''.join(c for _, c in short_names)
            print(f"    Single-char files in timeline order: {candidate}")
            if len(candidate) >= 5:
                return f"picoCTF{{{candidate}}}"

    # Strategy 6: Search file paths that may encode the flag
    print("\n[*] Strategy 6: Checking file paths for encoded flag...")
    all_names = sorted(files, key=lambda f: f['inode'])
    for f in all_names:
        basename = os.path.basename(f['path'])
        if re.match(r'^[A-Za-z0-9_{}]+$', basename) and 'pico' in basename.lower():
            print(f"    Suspicious filename: {basename}")

    # Strategy 7: Concatenate file contents in timeline order
    print("\n[*] Strategy 7: Checking files in chronological order...")
    # Files with numeric or sequential names may contain flag fragments
    fragment_files = [f for f in files if re.match(r'.*\d+.*\.txt$', f['path'])]
    if fragment_files:
        fragment_files.sort(key=lambda x: x['path'])
        combined = ""
        for f in fragment_files:
            content = extract_file(image_path, offset, f['inode'])
            combined += content.strip()
        if combined:
            match = flag_pattern.search(combined)
            if match:
                return match.group()
            if len(combined) < 200:
                print(f"    Combined fragments: {combined}")

    # Strategy 8: Raw strings search
    print("\n[*] Strategy 8: Raw strings search (fallback)...")
    results = search_strings(image_path)
    for r in results:
        if flag_pattern.match(r):
            return r
        print(f"    {r[:100]}")

    return None


def main():
    parser = argparse.ArgumentParser(
        description='Timeline 1 solver - picoCTF 2026 Forensics (300 pts)'
    )
    parser.add_argument(
        'image', nargs='?', default=None,
        help='Path to the disk image file'
    )
    parser.add_argument(
        '--image', dest='image_flag',
        help='Path to the disk image file (alternative)'
    )
    parser.add_argument(
        '--output', '-o',
        help='Output file for the flag'
    )
    parser.add_argument(
        '--strings-only', action='store_true',
        help='Only search raw strings (skip TSK analysis)'
    )
    parser.add_argument(
        '--offset', type=int, default=None,
        help='Manually specify partition offset in sectors'
    )

    args = parser.parse_args()
    image_path = args.image or args.image_flag

    print("=" * 60)
    print("  Timeline 1 - picoCTF 2026 Solver")
    print("  Forensics | 300 pts")
    print("=" * 60)
    print()

    if not image_path:
        # Try to find a disk image in the current directory
        for candidate in ['disk.img', 'disk.dd', 'timeline.img', 'image.img',
                          'disk.raw', 'challenge.img']:
            if os.path.exists(candidate):
                image_path = candidate
                break

        if not image_path:
            import glob
            imgs = glob.glob('*.img') + glob.glob('*.dd') + glob.glob('*.raw')
            if imgs:
                image_path = imgs[0]

    if not image_path or not os.path.exists(image_path):
        print("[!] No disk image found.")
        print("[*] Usage: python3 solve.py <disk_image>")
        print("[*] Download the disk image from the challenge page first.")
        sys.exit(1)

    print(f"[*] Disk image: {image_path}")
    print(f"[*] Image size: {os.path.getsize(image_path)} bytes")
    print()

    # Check for required tools
    if not check_dependencies():
        print("[!] Cannot proceed without The Sleuth Kit tools.")
        print("[*] Falling back to strings search...")
        results = search_strings(image_path)
        for r in results:
            print(f"    {r}")
        sys.exit(1)

    # Strings-only mode
    if args.strings_only:
        results = search_strings(image_path)
        for r in results:
            print(f"    {r}")
        sys.exit(0)

    # Step 1: Find partitions
    if args.offset is not None:
        partitions = [{'start': args.offset, 'desc': 'user-specified'}]
    else:
        partitions = find_partitions(image_path)

    flag = None

    with tempfile.TemporaryDirectory() as tmpdir:
        for part in partitions:
            offset = part['start']
            print(f"\n{'=' * 40}")
            print(f"[*] Analyzing partition at offset {offset}")
            print(f"{'=' * 40}")

            # Get filesystem info
            get_fs_type(image_path, offset)

            # List all files
            files = list_files(image_path, offset)
            if not files:
                print("[*] No files found at this offset, skipping...")
                continue

            # Generate timeline
            timeline = generate_timeline(image_path, offset, tmpdir)

            # Search for the flag
            flag = search_for_flag(image_path, offset, files, timeline)

            if flag:
                break

    print("\n" + "=" * 60)
    if flag:
        # Ensure flag is wrapped in picoCTF{} format
        if not flag.startswith('picoCTF{'):
            flag = f"picoCTF{{{flag}}}"
        print(f"[+] FLAG: {flag}")
        if args.output:
            with open(args.output, 'w') as f:
                f.write(flag + '\n')
            print(f"[+] Flag written to {args.output}")
    else:
        print("[-] Flag not found automatically.")
        print("[*] Manual investigation steps:")
        print("    1. Generate timeline: fls -m / -r -o <offset> disk.img > body.txt")
        print("    2. Create timeline:   mactime -b body.txt > timeline.txt")
        print("    3. Search timeline:   grep -i 'flag\\|pico\\|secret' timeline.txt")
        print("    4. List deleted files: fls -r -d -o <offset> disk.img")
        print("    5. Extract by inode:  icat -o <offset> disk.img <inode_number>")
        print("    6. Check slack space: blkls -o <offset> disk.img | strings")
        print("    7. Strings fallback:  strings -a disk.img | grep -i pico")
    print("=" * 60)


if __name__ == '__main__':
    main()
