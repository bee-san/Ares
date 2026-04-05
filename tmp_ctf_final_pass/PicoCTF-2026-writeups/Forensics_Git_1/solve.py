#!/usr/bin/env python3
"""
Forensics Git 1 - picoCTF 2026
Category: Forensics | Points: 300

Extracts a flag hidden in the git history of a repository stored on a disk image.
This script automates: disk analysis -> mount/extract -> git history search -> flag extraction.

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


def quick_strings_search(image_path):
    """Phase 0: Quickly search the raw image using strings + grep."""
    print("[*] Phase 0: Quick raw strings search on disk image...")
    output = run(f"strings {image_path} | grep -oP 'picoCTF\\{{[^}}]+\\}}'")
    if output:
        flags = list(set(FLAG_PATTERN.findall(output)))
        if flags:
            print(f"[+] Found flag(s) in raw image: {flags}")
            return flags
    print("    No flag found via raw strings (expected -- flag may need git context)")
    return []


def get_partition_offsets(image_path):
    """Identify partition offsets in the disk image."""
    print("[*] Analyzing disk image partitions...")

    # Try mmls first (The Sleuth Kit)
    mmls_out = run(f"mmls {image_path} 2>/dev/null")
    offsets = []

    if mmls_out:
        print(f"    mmls output:\n{mmls_out}")
        for line in mmls_out.split("\n"):
            # Match lines with Linux/ext partitions
            match = re.search(
                r"(\d{6,})\s+\d{6,}\s+\d{6,}\s+.*(?:Linux|ext|0x83)", line, re.IGNORECASE
            )
            if match:
                sector = int(match.group(1))
                offsets.append(sector * 512)

    if not offsets:
        # Try fdisk
        fdisk_out = run(f"fdisk -l {image_path} 2>/dev/null")
        if fdisk_out:
            print(f"    fdisk output:\n{fdisk_out}")
            for line in fdisk_out.split("\n"):
                parts = line.split()
                # fdisk lines like: disk.img1  2048  206847  204800  100M  83  Linux
                if len(parts) >= 2 and parts[0].startswith(os.path.basename(image_path)):
                    try:
                        start = parts[1].replace("*", "")
                        offsets.append(int(start) * 512)
                    except ValueError:
                        pass

    if not offsets:
        print("    No partitions found -- treating image as a raw filesystem")
        offsets = [0]

    print(f"    Byte offsets to try: {offsets}")
    return offsets


def mount_image(image_path, offset, mountpoint):
    """Mount the disk image at the given offset."""
    os.makedirs(mountpoint, exist_ok=True)
    cmd = f"sudo mount -o loop,ro,offset={offset} {image_path} {mountpoint} 2>/dev/null"
    ret = os.system(cmd)
    return ret == 0


def unmount(mountpoint):
    """Unmount a mountpoint."""
    os.system(f"sudo umount {mountpoint} 2>/dev/null")


def find_git_repos(search_root):
    """Find all .git directories under search_root."""
    output = run(f"find {search_root} -name '.git' -type d 2>/dev/null")
    if output:
        return [d.strip() for d in output.split("\n") if d.strip()]
    return []


def search_git_history(repo_path):
    """
    Thoroughly search a git repository's history for the flag.
    This is the core of the challenge -- the flag is hidden in git history.
    """
    flags_found = []

    # Make sure we're looking at the repo root (parent of .git)
    if repo_path.endswith(".git"):
        repo_root = os.path.dirname(repo_path)
    else:
        repo_root = repo_path

    git_dir = os.path.join(repo_root, ".git")
    if not os.path.isdir(git_dir):
        print(f"    [-] No .git directory at {git_dir}")
        return flags_found

    print(f"[*] Searching git history at: {repo_root}")
    os.chdir(repo_root)

    # ── 1. git log --all -p: search all commit diffs ──
    print("    Checking commit diffs (git log --all -p)...")
    log_output = run("git log --all -p 2>/dev/null")
    matches = FLAG_PATTERN.findall(log_output)
    if matches:
        print(f"    [+] Found in commit diffs: {matches}")
        flags_found.extend(matches)

    # ── 2. Check all branches ──
    print("    Checking branches...")
    branches_output = run("git branch -a 2>/dev/null")
    if branches_output:
        branches = [b.strip().lstrip("* ").strip() for b in branches_output.split("\n")]
        print(f"    Branches: {branches}")
        for branch in branches:
            if "->" in branch:
                continue
            # Check the tip of each branch
            show_output = run(f"git show {branch} 2>/dev/null")
            m = FLAG_PATTERN.findall(show_output)
            if m:
                print(f"    [+] Found on branch {branch}: {m}")
                flags_found.extend(m)

            # Diff each branch against the default
            diff_output = run(f"git log {branch} -p 2>/dev/null")
            m = FLAG_PATTERN.findall(diff_output)
            if m:
                flags_found.extend(m)

    # ── 3. Check commit messages themselves ──
    print("    Checking commit messages...")
    log_msgs = run("git log --all --format='%H %s%n%b' 2>/dev/null")
    m = FLAG_PATTERN.findall(log_msgs)
    if m:
        print(f"    [+] Found in commit messages: {m}")
        flags_found.extend(m)

    # ── 4. Check stash ──
    print("    Checking stash...")
    stash_list = run("git stash list 2>/dev/null")
    if stash_list:
        print(f"    Stash entries: {stash_list}")
        stash_diff = run("git stash show -p 2>/dev/null")
        m = FLAG_PATTERN.findall(stash_diff)
        if m:
            print(f"    [+] Found in stash: {m}")
            flags_found.extend(m)
        # Check all stash entries
        for i in range(10):
            sd = run(f"git stash show -p stash@{{{i}}} 2>/dev/null")
            if not sd:
                break
            m = FLAG_PATTERN.findall(sd)
            flags_found.extend(m)

    # ── 5. Check tags ──
    print("    Checking tags...")
    tags = run("git tag -l 2>/dev/null")
    if tags:
        for tag in tags.split("\n"):
            tag = tag.strip()
            if tag:
                tag_content = run(f"git show {tag} 2>/dev/null")
                m = FLAG_PATTERN.findall(tag_content)
                if m:
                    print(f"    [+] Found in tag {tag}: {m}")
                    flags_found.extend(m)

    # ── 6. Check git notes ──
    print("    Checking notes...")
    notes = run("git notes list 2>/dev/null")
    if notes:
        for line in notes.split("\n"):
            parts = line.strip().split()
            if len(parts) >= 2:
                note_content = run(f"git notes show {parts[1]} 2>/dev/null")
                m = FLAG_PATTERN.findall(note_content)
                if m:
                    print(f"    [+] Found in notes: {m}")
                    flags_found.extend(m)

    # ── 7. Check dangling/unreachable objects ──
    print("    Checking dangling objects (git fsck)...")
    fsck_output = run("git fsck --unreachable --no-reflogs 2>&1")
    if fsck_output:
        for line in fsck_output.split("\n"):
            hash_match = re.search(r"([0-9a-f]{40})", line)
            if hash_match:
                obj_hash = hash_match.group(1)
                obj_content = run(f"git cat-file -p {obj_hash} 2>/dev/null")
                m = FLAG_PATTERN.findall(obj_content)
                if m:
                    print(f"    [+] Found in dangling object {obj_hash}: {m}")
                    flags_found.extend(m)

    # ── 8. Check reflog ──
    print("    Checking reflog...")
    reflog = run("git reflog --all 2>/dev/null")
    if reflog:
        for line in reflog.split("\n"):
            hash_match = re.search(r"^([0-9a-f]+)", line)
            if hash_match:
                show = run(f"git show {hash_match.group(1)} 2>/dev/null")
                m = FLAG_PATTERN.findall(show)
                if m:
                    print(f"    [+] Found in reflog entry: {m}")
                    flags_found.extend(m)

    return list(set(flags_found))


def recover_with_tsk(image_path, offset, workdir):
    """Use The Sleuth Kit to recover files when mount fails."""
    recovered_dir = os.path.join(workdir, "recovered")
    os.makedirs(recovered_dir, exist_ok=True)
    sector_offset = offset // 512

    print(f"[*] TSK recovery at sector offset {sector_offset}...")

    # Recover all files
    run(f"tsk_recover -o {sector_offset} {image_path} {recovered_dir} 2>/dev/null")

    # List files via fls
    fls_output = run(f"fls -r -o {sector_offset} {image_path} 2>/dev/null")
    if fls_output:
        git_entries = [l for l in fls_output.split("\n") if ".git" in l or "flag" in l.lower()]
        if git_entries:
            print(f"    Found relevant entries:\n" + "\n".join(git_entries[:15]))

    return recovered_dir


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 solve.py <disk_image_path>")
        print("Example: python3 solve.py disk.img")
        sys.exit(1)

    image_path = os.path.abspath(sys.argv[1])
    if not os.path.exists(image_path):
        print(f"[-] File not found: {image_path}")
        sys.exit(1)

    # Handle gzip-compressed images
    if image_path.endswith(".gz"):
        print("[*] Decompressing gzipped image...")
        run(f"gunzip -k {image_path}")
        image_path = image_path[:-3]

    workdir = tempfile.mkdtemp(prefix="forensics_git1_")
    mountpoint = os.path.join(workdir, "mnt")
    print(f"[*] Working directory: {workdir}")
    all_flags = []

    try:
        # ── Phase 0: Quick raw strings search ──
        raw_flags = quick_strings_search(image_path)
        all_flags.extend(raw_flags)

        # ── Phase 1: Get partition offsets ──
        offsets = get_partition_offsets(image_path)

        # ── Phase 2: Mount and search git history ──
        for offset in offsets:
            print(f"\n[*] Trying offset {offset} ({offset // 512} sectors)...")

            if mount_image(image_path, offset, mountpoint):
                print(f"[+] Mounted successfully at {mountpoint}")

                # Find .git directories
                git_dirs = find_git_repos(mountpoint)
                if git_dirs:
                    for gd in git_dirs:
                        flags = search_git_history(gd)
                        all_flags.extend(flags)
                else:
                    print("    No .git directories found on this partition")
                    # Search files directly anyway
                    output = run(
                        f"find {mountpoint} -type f -exec grep -l 'picoCTF{{' {{}} + 2>/dev/null"
                    )
                    if output:
                        for fpath in output.split("\n"):
                            content = run(f"cat '{fpath}' 2>/dev/null")
                            m = FLAG_PATTERN.findall(content)
                            all_flags.extend(m)

                unmount(mountpoint)
            else:
                print(f"    Mount failed at offset {offset}, trying TSK recovery...")
                recovered = recover_with_tsk(image_path, offset, workdir)
                git_dirs = find_git_repos(recovered)
                if git_dirs:
                    for gd in git_dirs:
                        flags = search_git_history(gd)
                        all_flags.extend(flags)
                # Also search recovered files directly
                output = run(
                    f"grep -r -oP 'picoCTF\\{{[^}}]+\\}}' {recovered} 2>/dev/null"
                )
                if output:
                    m = FLAG_PATTERN.findall(output)
                    all_flags.extend(m)

        # ── Results ──
        print("\n" + "=" * 60)
        print("RESULTS")
        print("=" * 60)
        unique_flags = list(set(all_flags))
        if unique_flags:
            for flag in unique_flags:
                print(f"[+] FLAG: {flag}")
        else:
            print("[-] No flag found automatically.")
            print()
            print("Manual investigation steps:")
            print("  1. Mount:    sudo mount -o loop,ro disk.img /mnt")
            print("  2. Find git: find /mnt -name '.git' -type d")
            print("  3. cd into the repo directory")
            print("  4. git log --all --oneline --graph")
            print("  5. git log --all -p | grep picoCTF")
            print("  6. git branch -a   (check all branches)")
            print("  7. git stash list  (check stash)")
            print("  8. git tag -l      (check tags)")
            print("  9. strings disk.img | grep picoCTF")

    finally:
        unmount(mountpoint)
        print(f"\n[*] Work directory: {workdir}")
        print(f"[*] Cleanup: rm -rf {workdir}")


if __name__ == "__main__":
    main()
