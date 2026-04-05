#!/usr/bin/env python3
"""
Forensics Git 2 - picoCTF 2026
Category: Forensics | Points: 400

Recovers a git repository from a disk image where deletion was interrupted.
This script automates the disk image analysis, file recovery, git reconstruction,
and flag extraction process.

Usage:
    python3 solve.py <disk_image_path>
    python3 solve.py disk.img
"""

import subprocess
import sys
import os
import re
import tempfile
import shutil
import zlib
import hashlib
import struct

# ──────────────────────────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────────────────────────
FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")


def run(cmd, capture=True, check=False):
    """Run a shell command and return stdout."""
    result = subprocess.run(
        cmd, shell=True, capture_output=capture, text=True, check=check
    )
    return result.stdout.strip() if capture else ""


def analyze_disk_image(image_path):
    """Analyze the disk image to find partitions and filesystem info."""
    print("[*] Analyzing disk image...")

    # Get file info
    file_info = run(f"file {image_path}")
    print(f"    File type: {file_info}")

    # Try mmls (The Sleuth Kit) for partition layout
    mmls_output = run(f"mmls {image_path} 2>/dev/null")
    if mmls_output:
        print(f"    Partition layout:\n{mmls_output}")

    # Try fdisk as fallback
    fdisk_output = run(f"fdisk -l {image_path} 2>/dev/null")
    if fdisk_output:
        print(f"    fdisk output:\n{fdisk_output}")

    return mmls_output, fdisk_output


def extract_partition_offsets(mmls_output, fdisk_output):
    """Parse partition info to get offsets for Linux partitions."""
    offsets = []

    if mmls_output:
        # Parse mmls output: look for Linux partitions
        for line in mmls_output.split("\n"):
            # mmls format: 002:  000:   000002048   000206847   000204800   Linux (0x83)
            match = re.search(r"(\d{9,})\s+\d{9,}\s+\d{9,}\s+.*(?:Linux|ext)", line, re.IGNORECASE)
            if match:
                sector = int(match.group(1))
                offsets.append(sector * 512)

    if not offsets and fdisk_output:
        # Parse fdisk output
        for line in fdisk_output.split("\n"):
            match = re.search(r"\s+(\d+)\s+\d+\s+\d+\s+.*(?:Linux|ext)", line, re.IGNORECASE)
            if match:
                sector = int(match.group(1))
                offsets.append(sector * 512)

    # Default: try offset 0 (whole image might be a filesystem)
    if not offsets:
        offsets = [0]

    return offsets


def try_mount_and_recover(image_path, offset, workdir):
    """Try to mount the partition and recover .git directory."""
    mountpoint = os.path.join(workdir, "mnt")
    os.makedirs(mountpoint, exist_ok=True)

    print(f"[*] Trying to mount at offset {offset}...")

    # Try mounting (may need root)
    mount_cmd = f"sudo mount -o loop,ro,offset={offset} {image_path} {mountpoint} 2>/dev/null"
    ret = os.system(mount_cmd)

    if ret == 0:
        print("[+] Mount successful!")
        # Search for .git directories
        git_dirs = run(f"find {mountpoint} -name '.git' -type d 2>/dev/null")
        if git_dirs:
            print(f"[+] Found .git directories:\n{git_dirs}")
            return mountpoint, git_dirs.split("\n")
        # Also check if the entire mount is a bare repo
        if os.path.exists(os.path.join(mountpoint, "objects")):
            return mountpoint, [mountpoint]
        os.system(f"sudo umount {mountpoint} 2>/dev/null")

    return None, []


def recover_with_tsk(image_path, offset, workdir):
    """Use The Sleuth Kit tools to recover files."""
    recovered_dir = os.path.join(workdir, "recovered")
    os.makedirs(recovered_dir, exist_ok=True)

    print(f"[*] Attempting TSK recovery at offset {offset}...")
    sector_offset = offset // 512

    # tsk_recover to get all deleted files
    run(f"tsk_recover -o {sector_offset} {image_path} {recovered_dir} 2>/dev/null")

    # Also try fls to list files and icat to recover specific ones
    fls_output = run(f"fls -r -o {sector_offset} {image_path} 2>/dev/null")
    if fls_output:
        print("[+] File listing recovered via fls")
        # Look for .git related files
        git_files = [l for l in fls_output.split("\n") if ".git" in l.lower() or "flag" in l.lower()]
        if git_files:
            print(f"[+] Git-related files found:\n" + "\n".join(git_files[:20]))

    return recovered_dir


def recover_with_photorec(image_path, workdir):
    """Use photorec for file carving."""
    print("[*] Attempting photorec file carving...")
    carve_dir = os.path.join(workdir, "carved")
    os.makedirs(carve_dir, exist_ok=True)

    # photorec in non-interactive mode
    run(f"photorec /d {carve_dir} /cmd {image_path} options,keep_corrupted_file,enable,search 2>/dev/null")
    return carve_dir


def recover_with_strings(image_path):
    """Last resort: search raw disk image with strings."""
    print("[*] Searching disk image with strings for flag pattern...")
    output = run(f"strings {image_path} | grep -oP 'picoCTF\\{{[^}}]+\\}}'")
    if output:
        flags = FLAG_PATTERN.findall(output)
        return flags
    return []


def search_git_repo(git_dir):
    """Search a recovered git repo for the flag."""
    flags_found = []
    repo_dir = os.path.dirname(git_dir) if git_dir.endswith(".git") else git_dir

    print(f"[*] Searching git repo at {repo_dir}...")

    # Ensure we're in the repo directory
    os.chdir(repo_dir)

    # 1. Check git log across all refs
    log_output = run("git log --all -p 2>/dev/null")
    matches = FLAG_PATTERN.findall(log_output)
    flags_found.extend(matches)

    # 2. Check git fsck for dangling/unreachable objects
    fsck_output = run("git fsck --unreachable --no-reflogs 2>/dev/null")
    if fsck_output:
        print(f"[+] git fsck found unreachable objects")
        for line in fsck_output.split("\n"):
            # Extract object hashes
            hash_match = re.search(r"([0-9a-f]{40})", line)
            if hash_match:
                obj_hash = hash_match.group(1)
                obj_content = run(f"git cat-file -p {obj_hash} 2>/dev/null")
                m = FLAG_PATTERN.findall(obj_content)
                flags_found.extend(m)

    # 3. Check git fsck --lost-found
    run("git fsck --lost-found 2>/dev/null")
    lost_found = os.path.join(git_dir, "lost-found")
    if os.path.exists(lost_found):
        for root, dirs, files in os.walk(lost_found):
            for f in files:
                fpath = os.path.join(root, f)
                try:
                    content = open(fpath).read()
                    m = FLAG_PATTERN.findall(content)
                    flags_found.extend(m)
                except:
                    pass

    # 4. Check stash
    stash_output = run("git stash list 2>/dev/null")
    if stash_output:
        stash_diff = run("git stash show -p 2>/dev/null")
        m = FLAG_PATTERN.findall(stash_diff)
        flags_found.extend(m)

    # 5. Check all branches
    branches = run("git branch -a 2>/dev/null")
    if branches:
        for branch in branches.split("\n"):
            branch = branch.strip().lstrip("* ")
            if branch.startswith("remotes/"):
                branch = branch.split("/", 2)[-1] if "/" in branch[8:] else branch
            content = run(f"git log {branch} -p 2>/dev/null")
            m = FLAG_PATTERN.findall(content)
            flags_found.extend(m)

    # 6. Check tags
    tags = run("git tag -l 2>/dev/null")
    if tags:
        for tag in tags.split("\n"):
            tag = tag.strip()
            content = run(f"git show {tag} 2>/dev/null")
            m = FLAG_PATTERN.findall(content)
            flags_found.extend(m)

    # 7. Check notes
    notes = run("git notes list 2>/dev/null")
    if notes:
        for line in notes.split("\n"):
            parts = line.strip().split()
            if parts:
                content = run(f"git notes show {parts[-1]} 2>/dev/null")
                m = FLAG_PATTERN.findall(content)
                flags_found.extend(m)

    # 8. Brute force: decompress all objects
    objects_dir = os.path.join(git_dir, "objects")
    if os.path.exists(objects_dir):
        for root, dirs, files in os.walk(objects_dir):
            # Skip pack and info directories for now
            if "pack" in root or "info" in root:
                continue
            for f in files:
                fpath = os.path.join(root, f)
                try:
                    with open(fpath, "rb") as fp:
                        data = zlib.decompress(fp.read())
                        text = data.decode("utf-8", errors="replace")
                        m = FLAG_PATTERN.findall(text)
                        flags_found.extend(m)
                except:
                    pass

    # 9. Check pack files
    pack_dir = os.path.join(git_dir, "objects", "pack")
    if os.path.exists(pack_dir):
        # Unpack pack files first
        for f in os.listdir(pack_dir):
            if f.endswith(".pack"):
                pack_path = os.path.join(pack_dir, f)
                # Verify and unpack
                run(f"git unpack-objects < {pack_path} 2>/dev/null")
                # Also search raw pack content
                pack_content = run(f"git verify-pack -v {pack_path} 2>/dev/null")
                if pack_content:
                    for line in pack_content.split("\n"):
                        hash_match = re.search(r"^([0-9a-f]{40})", line)
                        if hash_match:
                            obj_content = run(f"git cat-file -p {hash_match.group(1)} 2>/dev/null")
                            m = FLAG_PATTERN.findall(obj_content)
                            flags_found.extend(m)

    return list(set(flags_found))


def search_directory_for_flag(directory):
    """Recursively search a directory for the flag pattern."""
    flags = []
    if not os.path.exists(directory):
        return flags

    for root, dirs, files in os.walk(directory):
        for f in files:
            fpath = os.path.join(root, f)
            try:
                with open(fpath, "r", errors="replace") as fp:
                    content = fp.read()
                    m = FLAG_PATTERN.findall(content)
                    flags.extend(m)
            except:
                pass
    return flags


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 solve.py <disk_image_path>")
        print("Example: python3 solve.py disk.img")
        sys.exit(1)

    image_path = os.path.abspath(sys.argv[1])
    if not os.path.exists(image_path):
        print(f"[-] File not found: {image_path}")
        sys.exit(1)

    workdir = tempfile.mkdtemp(prefix="forensics_git2_")
    print(f"[*] Working directory: {workdir}")
    all_flags = []

    try:
        # ── Phase 1: Quick string search on raw image ──
        print("\n" + "=" * 60)
        print("Phase 1: Raw string search on disk image")
        print("=" * 60)
        raw_flags = recover_with_strings(image_path)
        if raw_flags:
            print(f"[+] Found flags in raw image: {raw_flags}")
            all_flags.extend(raw_flags)

        # ── Phase 2: Analyze disk image ──
        print("\n" + "=" * 60)
        print("Phase 2: Disk image analysis")
        print("=" * 60)
        mmls_out, fdisk_out = analyze_disk_image(image_path)
        offsets = extract_partition_offsets(mmls_out, fdisk_out)
        print(f"[*] Partition offsets to check: {offsets}")

        # ── Phase 3: Try mounting and direct recovery ──
        print("\n" + "=" * 60)
        print("Phase 3: Mount and recover")
        print("=" * 60)
        for offset in offsets:
            mountpoint, git_dirs = try_mount_and_recover(image_path, offset, workdir)
            if git_dirs:
                for gd in git_dirs:
                    flags = search_git_repo(gd)
                    all_flags.extend(flags)
                os.system(f"sudo umount {mountpoint} 2>/dev/null")

            # Try TSK recovery
            recovered = recover_with_tsk(image_path, offset, workdir)
            if os.path.exists(recovered):
                # Look for .git in recovered files
                for root, dirs, files in os.walk(recovered):
                    if ".git" in dirs:
                        git_path = os.path.join(root, ".git")
                        flags = search_git_repo(git_path)
                        all_flags.extend(flags)
                # Also search all recovered files directly
                flags = search_directory_for_flag(recovered)
                all_flags.extend(flags)

        # ── Phase 4: File carving with photorec ──
        if not all_flags:
            print("\n" + "=" * 60)
            print("Phase 4: File carving")
            print("=" * 60)
            carved_dir = recover_with_photorec(image_path, workdir)
            flags = search_directory_for_flag(carved_dir)
            all_flags.extend(flags)

        # ── Results ──
        print("\n" + "=" * 60)
        print("RESULTS")
        print("=" * 60)
        unique_flags = list(set(all_flags))
        if unique_flags:
            for flag in unique_flags:
                print(f"[+] FLAG FOUND: {flag}")
        else:
            print("[-] No flag found automatically.")
            print("    Manual steps to try:")
            print("    1. Mount the image: sudo mount -o loop,ro disk.img /mnt")
            print("    2. Look for .git: find /mnt -name '.git' -type d")
            print("    3. Run: git fsck --lost-found in the repo")
            print("    4. Check: git log --all -p | grep picoCTF")
            print("    5. Try: strings disk.img | grep picoCTF")
            print("    6. Use Autopsy for GUI-based analysis")

    finally:
        # Cleanup
        os.system(f"sudo umount {workdir}/mnt 2>/dev/null")
        print(f"\n[*] Work directory preserved at: {workdir}")
        print("[*] Run 'rm -rf {workdir}' to clean up when done.")


if __name__ == "__main__":
    main()
