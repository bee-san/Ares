# Forensics Git 2 - picoCTF 2026

**Category:** Forensics
**Points:** 400

## Challenge Description
The agents interrupted the perpetrator's disk deletion routine. Can you recover this git repo? Download the disk image.

## Approach
This challenge provides a disk image that was captured mid-deletion. The perpetrator was trying to destroy a git repository stored on the disk, but the process was interrupted. Our goal is to recover the `.git` directory (or its remnants) from the disk image and reconstruct the repository to find the flag.

Key forensic concepts at play:
1. **Disk image analysis** -- When files are "deleted," the data often remains on disk until overwritten. The filesystem metadata (inodes, directory entries) may be removed, but the raw bytes persist.
2. **Git internals** -- Git stores everything as objects (blobs, trees, commits, tags) in `.git/objects/`. Even if some refs are deleted, we can recover objects and use `git fsck` to find dangling commits.
3. **File carving** -- Tools like `photorec`, `scalpel`, or manual carving with `dd` can extract files from raw disk images.

The approach involves:
- Mounting or analyzing the disk image to identify partitions
- Extracting/recovering the `.git` directory structure
- Using git internals commands to reconstruct the repo history
- Searching commits, branches, and blobs for the flag

## Solution

### Step 1: Analyze the disk image
```bash
file disk.img
fdisk -l disk.img
mmls disk.img
```
Identify the filesystem type and partition layout.

### Step 2: Mount or extract the filesystem
```bash
# Calculate offset: start_sector * 512
sudo mount -o loop,offset=<offset> disk.img /mnt/evidence

# Or use sleuthkit tools
fls -r -o <offset> disk.img
```

### Step 3: Recover deleted files
If the `.git` directory was partially deleted, use file recovery:
```bash
# Using photorec
photorec disk.img

# Or using tsk_recover (The Sleuth Kit)
tsk_recover -o <offset> disk.img /tmp/recovered/

# Or using extundelete for ext filesystems
extundelete --restore-all disk.img
```

### Step 4: Reconstruct the git repository
```bash
cd /tmp/recovered
# If .git directory is intact enough:
git fsck --lost-found
git log --all --oneline
git reflog

# Check dangling commits
git fsck --unreachable
git show <dangling_commit_hash>

# Check all branches including remote refs
git branch -a
git log --all --graph --oneline
```

### Step 5: Search for the flag
```bash
# Search all git objects for the flag pattern
git log --all -p | grep -i "picoCTF{"
git fsck --lost-found && cat .git/lost-found/other/*

# Search blobs directly
for obj in $(find .git/objects -type f | sed 's|.git/objects/||;s|/||'); do
    git cat-file -p "$obj" 2>/dev/null | grep -l "picoCTF{" && echo "Found in: $obj"
done

# Or grep raw object files after decompressing
find .git/objects -type f -exec sh -c 'python3 -c "import zlib,sys; print(zlib.decompress(open(sys.argv[1],\"rb\").read()))" {} 2>/dev/null | grep picoCTF' \;
```

### Step 6: Check stash, tags, and other refs
```bash
git stash list
git tag -l
git notes list
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
