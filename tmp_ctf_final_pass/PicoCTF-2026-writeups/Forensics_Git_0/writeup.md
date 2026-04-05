# Forensics Git 0 - picoCTF 2026

**Category:** Forensics
**Points:** 200

## Challenge Description

Can you find the flag in this disk image? Download the disk image.

## Approach

This is the introductory challenge in the "Forensics Git" series (200 points, 1469 solves). We are given a disk image that contains a git repository, and we need to extract the flag from it. As the "0" (first) challenge in the series, the flag is likely accessible through straightforward git history inspection without needing advanced recovery techniques.

### Key Concepts

1. **Disk image analysis**: The disk image file (likely `.img`, `.dd`, or `.raw` format) contains a filesystem with a git repository inside it.
2. **Git internals**: Git stores the complete history of a project. Flags in git-based CTF challenges are commonly hidden in:
   - Previous commits (removed in later commits)
   - Deleted branches
   - Commit messages
   - Git tags or annotated tags
   - File contents at specific points in history
   - Stashed changes
3. **Mounting disk images**: To access the filesystem, we need to mount the disk image or use forensic tools to extract its contents.

### Common Hiding Places for Flags in Git Repos

1. **Commit history**: The flag was in a file that was later deleted or modified. Use `git log` and `git show` to inspect historical commits.
2. **Different branches**: The flag exists on a branch other than `main`/`master`. Use `git branch -a` to list all branches.
3. **Commit messages**: The flag is embedded in a commit message. Use `git log` to read all messages.
4. **Git tags**: The flag is in a tag annotation. Use `git tag -l` and `git show <tag>`.
5. **Git stash**: The flag was stashed. Use `git stash list` and `git stash show -p`.
6. **Deleted content in diffs**: Use `git log -p` to see all diffs and search for the flag pattern.

### Forensic Workflow

1. Identify the disk image format and partition layout
2. Mount the filesystem or extract files
3. Locate the `.git` directory
4. Inspect git history, branches, tags, and stashes
5. Search for the flag string

## Solution

### Step 1: Examine the disk image
```bash
file disk.img
fdisk -l disk.img
mmls disk.img  # if using The Sleuth Kit
```

### Step 2: Mount the disk image
```bash
# Simple mount (if single partition or no partition table)
sudo mkdir -p /mnt/evidence
sudo mount -o loop,ro disk.img /mnt/evidence

# If partitioned, calculate the offset
# offset = start_sector * sector_size (usually 512)
sudo mount -o loop,ro,offset=<calculated_offset> disk.img /mnt/evidence
```

Alternative: use `7z` or The Sleuth Kit to extract without mounting:
```bash
# Using 7z
7z x disk.img -o/tmp/extracted/

# Using sleuthkit
fls -r -o <offset> disk.img
icat -o <offset> disk.img <inode_number> > recovered_file
```

### Step 3: Locate the git repository
```bash
find /mnt/evidence -name ".git" -type d 2>/dev/null
# or
ls -la /mnt/evidence/
```

### Step 4: Inspect the git repository
```bash
cd /mnt/evidence/<repo_directory>

# View full commit history
git log --all --oneline --graph

# Search all commits for the flag
git log --all -p | grep -i "picoCTF{"

# Check all branches
git branch -a

# Check tags
git tag -l
git tag -l | xargs -I{} git show {}

# Check stash
git stash list
git stash show -p

# Check commit messages for the flag
git log --all --format='%H %s' | grep -i "flag\|pico\|secret"

# Look at all file changes
git log --all --name-only --oneline
```

### Step 5: Examine specific commits
```bash
# Show the contents of a specific commit
git show <commit_hash>

# Show a file at a specific commit
git show <commit_hash>:<filename>

# Diff between commits
git diff <commit1> <commit2>
```

### Step 6: Quick method -- grep raw strings from disk image
If the above is too complex, a simple `strings` + `grep` often works for introductory forensics:
```bash
strings disk.img | grep "picoCTF{"
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
