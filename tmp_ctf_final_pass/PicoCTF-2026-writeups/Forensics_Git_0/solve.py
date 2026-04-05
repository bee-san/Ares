#!/usr/bin/env python3
"""
Forensics Git 0 - picoCTF 2026 (Forensics, 200 pts)

Find the flag hidden in a git repository within a disk image.

This script:
  1. Analyzes the disk image to find partitions
  2. Extracts or mounts the filesystem
  3. Locates the git repository
  4. Searches git history (commits, branches, tags, stashes) for the flag
  5. Falls back to raw string search if git inspection fails

Usage:
    python3 solve.py disk.img
    python3 solve.py <path_to_disk_image>

Dependencies:
    - Python 3.6+
    - Optional: The Sleuth Kit (mmls, fls, icat)
    - Optional: git (for detailed history inspection)
"""

import argparse
import subprocess
import sys
import os
import re
import tempfile
import shutil
import struct
import zlib
import glob


def run_cmd(cmd, timeout=30, check=False):
    """Run a shell command and return output."""
    try:
        result = subprocess.run(
            cmd, shell=True, capture_output=True, text=True, timeout=timeout
        )
        if check and result.returncode != 0:
            return None
        return result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        return None
    except Exception as e:
        return str(e)


def search_flag_in_text(text):
    """Search for picoCTF flag pattern in text."""
    if not text:
        return []
    matches = re.findall(r'picoCTF\{[^}]+\}', text)
    return list(set(matches))


def method_strings_grep(image_path):
    """
    Method 1: The simplest approach -- run strings on the disk image
    and grep for the flag. Works surprisingly often for introductory challenges.
    """
    print("\n[*] === Method 1: strings + grep ===")
    print(f"[*] Searching raw strings in {image_path}...")

    output = run_cmd(f'strings "{image_path}" | grep -i "picoCTF{{"', timeout=120)
    flags = search_flag_in_text(output)

    if flags:
        print(f"[+] Found {len(flags)} flag(s) via strings:")
        for f in flags:
            print(f"    {f}")
        return flags

    # Try with different string lengths
    output = run_cmd(f'strings -n 5 "{image_path}" | grep -i "pico"', timeout=120)
    flags = search_flag_in_text(output)
    if flags:
        print(f"[+] Found {len(flags)} flag(s) via strings -n 5:")
        for f in flags:
            print(f"    {f}")
        return flags

    print("[-] No flags found via raw strings search.")
    return []


def method_python_search(image_path):
    """
    Method 2: Read the disk image in Python and search for the flag pattern.
    Handles cases where strings(1) might miss things.
    """
    print("\n[*] === Method 2: Python binary search ===")
    print(f"[*] Scanning {image_path} for flag pattern...")

    flags = []
    chunk_size = 1024 * 1024  # 1 MB chunks
    overlap = 256  # overlap to catch flags spanning chunk boundaries

    try:
        file_size = os.path.getsize(image_path)
        print(f"[*] Image size: {file_size:,} bytes ({file_size / (1024*1024):.1f} MB)")

        with open(image_path, 'rb') as f:
            offset = 0
            prev_tail = b''

            while offset < file_size:
                chunk = f.read(chunk_size)
                if not chunk:
                    break

                # Search in the combined previous tail + current chunk
                search_data = prev_tail + chunk
                matches = re.findall(rb'picoCTF\{[^}]+\}', search_data)

                for m in matches:
                    flag = m.decode('ascii', errors='replace')
                    if flag not in flags:
                        flags.append(flag)
                        print(f"[+] Found flag at ~offset {offset}: {flag}")

                prev_tail = chunk[-overlap:] if len(chunk) > overlap else chunk
                offset += len(chunk)

    except PermissionError:
        print("[!] Permission denied. Try running with sudo.")
    except Exception as e:
        print(f"[!] Error: {e}")

    if flags:
        print(f"[+] Total flags found: {len(flags)}")
    else:
        print("[-] No flags found via binary search.")

    return flags


def method_mount_and_git(image_path):
    """
    Method 3: Mount the disk image and inspect the git repository.
    Requires sudo for mounting.
    """
    print("\n[*] === Method 3: Mount + git inspection ===")

    # Create a temporary mount point
    mount_dir = tempfile.mkdtemp(prefix='forensics_git_')
    flags = []

    try:
        # Try direct mount first (works for raw filesystem images)
        print(f"[*] Attempting to mount {image_path} at {mount_dir}...")
        result = run_cmd(f'sudo mount -o loop,ro "{image_path}" "{mount_dir}" 2>&1')

        if result and 'mount:' in result.lower() and 'failed' in result.lower():
            # Try with offset -- parse partition table first
            print("[*] Direct mount failed, checking partition table...")
            fdisk_out = run_cmd(f'fdisk -l "{image_path}"')
            if fdisk_out:
                print(f"[*] Partition info:\n{fdisk_out}")

                # Parse start sector from fdisk output
                sector_match = re.findall(r'(\d+)\s+\d+\s+\d+\s+[\d.]+[KMGT]', fdisk_out)
                for start_sector in sector_match:
                    offset = int(start_sector) * 512
                    print(f"[*] Trying mount with offset {offset}...")
                    result = run_cmd(
                        f'sudo mount -o loop,ro,offset={offset} "{image_path}" "{mount_dir}" 2>&1'
                    )
                    if not (result and 'failed' in result.lower()):
                        break

        # Check if mount succeeded
        contents = os.listdir(mount_dir)
        if not contents:
            print("[-] Mount appears empty or failed.")
            return flags

        print(f"[+] Mounted successfully. Contents: {contents}")

        # Find .git directories
        git_dirs = []
        for root, dirs, files in os.walk(mount_dir):
            if '.git' in dirs:
                git_dirs.append(os.path.join(root, '.git'))
                # Don't recurse into .git
                dirs.remove('.git')

        if not git_dirs:
            print("[-] No .git directories found in mounted image.")
            # Still search for flags in files
            print("[*] Searching files for flag pattern...")
            for root, dirs, files in os.walk(mount_dir):
                for fname in files:
                    fpath = os.path.join(root, fname)
                    try:
                        with open(fpath, 'r', errors='replace') as f:
                            content = f.read()
                            found = search_flag_in_text(content)
                            if found:
                                print(f"[+] Found in {fpath}: {found}")
                                flags.extend(found)
                    except Exception:
                        pass
            return flags

        # Inspect each git repository found
        for git_dir in git_dirs:
            repo_dir = os.path.dirname(git_dir)
            print(f"\n[*] Found git repo: {repo_dir}")
            flags.extend(inspect_git_repo(repo_dir))

    except Exception as e:
        print(f"[!] Error during mount/inspection: {e}")

    finally:
        # Unmount and clean up
        run_cmd(f'sudo umount "{mount_dir}" 2>/dev/null')
        try:
            os.rmdir(mount_dir)
        except Exception:
            pass

    return flags


def inspect_git_repo(repo_dir):
    """Thoroughly inspect a git repository for flags."""
    flags = []
    env = {'GIT_DIR': os.path.join(repo_dir, '.git'), 'GIT_WORK_TREE': repo_dir}
    env_str = f'GIT_DIR="{os.path.join(repo_dir, ".git")}" GIT_WORK_TREE="{repo_dir}"'

    print(f"[*] Inspecting git repo at {repo_dir}")

    # 1. Check git log (all branches)
    print("[*] Checking commit history...")
    log_output = run_cmd(f'{env_str} git log --all --oneline 2>/dev/null')
    if log_output:
        print(f"[*] Commits:\n{log_output.strip()}")
        found = search_flag_in_text(log_output)
        if found:
            print(f"[+] Flag in commit messages: {found}")
            flags.extend(found)

    # 2. Check all diffs
    print("[*] Checking all diffs...")
    diff_output = run_cmd(f'{env_str} git log --all -p 2>/dev/null', timeout=60)
    if diff_output:
        found = search_flag_in_text(diff_output)
        if found:
            print(f"[+] Flag in diffs: {found}")
            flags.extend(found)

    # 3. Check branches
    print("[*] Checking branches...")
    branch_output = run_cmd(f'{env_str} git branch -a 2>/dev/null')
    if branch_output:
        print(f"[*] Branches: {branch_output.strip()}")

    # 4. Check tags
    print("[*] Checking tags...")
    tag_output = run_cmd(f'{env_str} git tag -l 2>/dev/null')
    if tag_output and tag_output.strip():
        print(f"[*] Tags: {tag_output.strip()}")
        for tag in tag_output.strip().split('\n'):
            tag = tag.strip()
            if tag:
                show_output = run_cmd(f'{env_str} git show {tag} 2>/dev/null')
                found = search_flag_in_text(show_output)
                if found:
                    print(f"[+] Flag in tag {tag}: {found}")
                    flags.extend(found)

    # 5. Check stash
    print("[*] Checking stash...")
    stash_output = run_cmd(f'{env_str} git stash list 2>/dev/null')
    if stash_output and stash_output.strip():
        print(f"[*] Stashes: {stash_output.strip()}")
        stash_show = run_cmd(f'{env_str} git stash show -p 2>/dev/null')
        found = search_flag_in_text(stash_show)
        if found:
            print(f"[+] Flag in stash: {found}")
            flags.extend(found)

    # 6. Check git notes
    print("[*] Checking git notes...")
    notes_output = run_cmd(f'{env_str} git notes list 2>/dev/null')
    if notes_output and notes_output.strip():
        print(f"[*] Notes found: {notes_output.strip()}")
        found = search_flag_in_text(notes_output)
        if found:
            flags.extend(found)

    # 7. Check current working tree files
    print("[*] Checking working tree files...")
    for root, dirs, files in os.walk(repo_dir):
        if '.git' in root:
            continue
        for fname in files:
            fpath = os.path.join(root, fname)
            try:
                with open(fpath, 'r', errors='replace') as f:
                    content = f.read(10000)  # limit per file
                    found = search_flag_in_text(content)
                    if found:
                        print(f"[+] Flag in {fpath}: {found}")
                        flags.extend(found)
            except Exception:
                pass

    # 8. Grep through all git objects directly
    print("[*] Searching git objects directly...")
    objects_dir = os.path.join(repo_dir, '.git', 'objects')
    if os.path.isdir(objects_dir):
        for root, dirs, files in os.walk(objects_dir):
            # Skip pack and info dirs for now
            if 'pack' in root or 'info' in root:
                continue
            for fname in files:
                obj_path = os.path.join(root, fname)
                try:
                    with open(obj_path, 'rb') as f:
                        compressed = f.read()
                    decompressed = zlib.decompress(compressed)
                    text = decompressed.decode('ascii', errors='replace')
                    found = search_flag_in_text(text)
                    if found:
                        # Reconstruct object hash
                        parts = obj_path.replace(objects_dir + '/', '').split('/')
                        obj_hash = ''.join(parts)
                        print(f"[+] Flag in git object {obj_hash}: {found}")
                        flags.extend(found)
                except Exception:
                    pass

        # Also check pack files
        pack_dir = os.path.join(objects_dir, 'pack')
        if os.path.isdir(pack_dir):
            for pack_file in glob.glob(os.path.join(pack_dir, '*.pack')):
                try:
                    with open(pack_file, 'rb') as f:
                        data = f.read()
                    text = data.decode('ascii', errors='replace')
                    found = search_flag_in_text(text)
                    if found:
                        print(f"[+] Flag in pack file {pack_file}: {found}")
                        flags.extend(found)
                except Exception:
                    pass

    return list(set(flags))


def method_extract_7z(image_path):
    """
    Method 4: Use 7z to extract files from the disk image without mounting.
    """
    print("\n[*] === Method 4: 7z extraction ===")

    extract_dir = tempfile.mkdtemp(prefix='forensics_git_7z_')
    flags = []

    try:
        print(f"[*] Extracting {image_path} with 7z...")
        result = run_cmd(f'7z x "{image_path}" -o"{extract_dir}" -y 2>&1', timeout=120)
        if result:
            print(f"[*] 7z output: {result[:500]}")

        # Search extracted files
        for root, dirs, files in os.walk(extract_dir):
            for fname in files:
                fpath = os.path.join(root, fname)
                try:
                    with open(fpath, 'r', errors='replace') as f:
                        content = f.read(50000)
                    found = search_flag_in_text(content)
                    if found:
                        print(f"[+] Flag in {fpath}: {found}")
                        flags.extend(found)
                except Exception:
                    pass

        # Check for git repos in extracted content
        for root, dirs, files in os.walk(extract_dir):
            if '.git' in dirs:
                repo_dir = root
                print(f"[*] Found git repo at {repo_dir}")
                flags.extend(inspect_git_repo(repo_dir))
                dirs.remove('.git')

    except Exception as e:
        print(f"[!] Error during 7z extraction: {e}")

    finally:
        shutil.rmtree(extract_dir, ignore_errors=True)

    return list(set(flags))


def main():
    parser = argparse.ArgumentParser(
        description='Forensics Git 0 solver - picoCTF 2026'
    )
    parser.add_argument('image', nargs='?', help='Path to disk image file')
    args = parser.parse_args()

    print("=" * 60)
    print("  Forensics Git 0 - picoCTF 2026 Solver")
    print("  Forensics | 200 pts")
    print("=" * 60)
    print()

    # Find the disk image
    image_path = args.image
    if not image_path:
        # Look for common disk image files in current directory
        for ext in ['*.img', '*.dd', '*.raw', '*.iso', '*.disk', '*.gz']:
            matches = glob.glob(ext)
            if matches:
                image_path = matches[0]
                break

    if not image_path or not os.path.exists(image_path):
        print("[!] No disk image file specified or found.")
        print()
        print("Usage: python3 solve.py <disk_image>")
        print()
        print("Download the disk image from the challenge and run:")
        print("    python3 solve.py disk.img")
        print()
        print("Manual approach:")
        print("    # Quick try:")
        print("    strings disk.img | grep 'picoCTF{'")
        print()
        print("    # Mount and inspect git:")
        print("    sudo mount -o loop,ro disk.img /mnt/evidence")
        print("    cd /mnt/evidence/<repo>")
        print("    git log --all -p | grep 'picoCTF{'")
        sys.exit(1)

    print(f"[*] Disk image: {image_path}")
    print(f"[*] Size: {os.path.getsize(image_path):,} bytes")
    print()

    all_flags = []

    # Method 1: Quick strings search
    flags = method_strings_grep(image_path)
    all_flags.extend(flags)

    # Method 2: Python binary search
    if not all_flags:
        flags = method_python_search(image_path)
        all_flags.extend(flags)

    # Method 3: Mount and git inspect (needs sudo)
    if not all_flags:
        flags = method_mount_and_git(image_path)
        all_flags.extend(flags)

    # Method 4: 7z extraction
    if not all_flags:
        flags = method_extract_7z(image_path)
        all_flags.extend(flags)

    # Final report
    all_flags = list(set(all_flags))
    print()
    print("=" * 60)
    if all_flags:
        print(f"[+] Found {len(all_flags)} unique flag(s):")
        for f in all_flags:
            print(f"    {f}")
        print()
        print(f"FLAG: {all_flags[0]}")
    else:
        print("[-] No flags found automatically.")
        print()
        print("[*] Manual steps to try:")
        print("    1. strings disk.img | grep picoCTF")
        print("    2. sudo mount -o loop,ro disk.img /mnt/evidence")
        print("    3. Find and cd into the git repo")
        print("    4. git log --all -p | grep picoCTF")
        print("    5. git branch -a && git tag -l && git stash list")
        print("    6. Check each branch/tag/stash for the flag")
    print("=" * 60)


if __name__ == '__main__':
    main()
