# Forensics Git 1 - picoCTF 2026

**Category:** Forensics
**Points:** 300

## Challenge Description
Can you find the flag in this disk image? Download the disk image. (Second in the Forensics Git series - flag hidden in git history)

## Approach
This is the second challenge in the Forensics Git series. We are given a disk image file that contains a git repository somewhere on it. Unlike the first challenge in the series (which likely involved a straightforward flag in the working tree), this challenge hides the flag within the git history -- meaning we need to examine past commits, branches, diffs, or other git artifacts to find it.

### Key Concepts

1. **Disk Image Analysis**: The challenge provides a raw disk image (`.img` or `.img.gz`). Before we can access the git repo, we need to mount or extract the filesystem from the image. Standard tools include:
   - `mmls` / `fdisk` -- to identify partition layout and offsets
   - `mount -o loop,offset=...` -- to mount the filesystem
   - The Sleuth Kit (`fls`, `icat`, `tsk_recover`) -- for filesystem-level analysis
   - `strings` / `grep` -- for quick raw searches

2. **Git History Forensics**: Once we have access to the repository, the flag is not in the current working tree -- it has been removed or never appeared in the latest commit. We need to look at:
   - **Commit history** (`git log --all -p`) -- the flag may have been added then deleted in a subsequent commit
   - **Branches** (`git branch -a`) -- the flag could be on a different branch
   - **Diffs between commits** (`git diff`, `git show`) -- reveals what changed
   - **Stash** (`git stash list`, `git stash show -p`) -- a common hiding spot
   - **Tags and notes** (`git tag -l`, `git notes list`) -- metadata that can store data

3. **Common Hiding Pattern**: In picoCTF git challenges, the flag is typically:
   - Committed in an earlier commit and then deleted
   - Placed on a separate feature/secret branch
   - Stored in a git stash entry
   - Embedded in a commit message or tag annotation

## Solution

### Step 1: Decompress the disk image (if compressed)
```bash
# If the file is gzip-compressed
gunzip disk.img.gz
# Or if it's already a raw image, skip this step
file disk.img
```

### Step 2: Analyze the disk image partitions
```bash
# Identify partitions
mmls disk.img
# Or
fdisk -l disk.img
```
Note the start sector of the Linux partition. Calculate the byte offset as `start_sector * 512`.

### Step 3: Mount the filesystem
```bash
sudo mkdir -p /mnt/evidence
sudo mount -o loop,ro,offset=<byte_offset> disk.img /mnt/evidence
```
If the image is a single filesystem (no partition table), use offset 0 or mount without offset:
```bash
sudo mount -o loop,ro disk.img /mnt/evidence
```

### Step 4: Locate the git repository
```bash
find /mnt/evidence -name ".git" -type d 2>/dev/null
```
This should reveal the path to a `.git` directory within the mounted image.

### Step 5: Explore git history for the flag
```bash
cd /mnt/evidence/<path_to_repo>

# View all commits across all branches
git log --all --oneline --graph

# Search all diffs for the flag pattern
git log --all -p | grep "picoCTF{"

# Check all branches
git branch -a

# If there are multiple branches, diff them
git diff main..feature-branch

# Check stash
git stash list
git stash show -p

# Check tags
git tag -l
git show <tag_name>

# Check individual commits
git show <commit_hash>
```

### Step 6: Alternative -- raw search
If mounting fails or you want a quick answer:
```bash
strings disk.img | grep "picoCTF{"
```

## Solution Script
```
python3 solve.py <disk_image_path>
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
