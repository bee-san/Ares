# DISKO 4 - picoCTF 2026

**Category:** Forensics
**Points:** 200

## Challenge Description
Can you find the flag in this disk image? This time I deleted the file! Let see you get it now!

## Approach

This is the fourth challenge in the picoCTF DISKO forensics series. The progression of the series is:

- **DISKO 1** (Easy): Flag is a plain string in the disk image -- solvable with `strings | grep picoCTF`
- **DISKO 2** (Medium): Multiple partitions -- must identify and extract the correct (Linux) partition, then search for the flag
- **DISKO 3** (Medium): Flag is hidden in a compressed file (`flag.gz`) inside a mounted FAT32 filesystem
- **DISKO 4** (Medium): The flag file has been **deleted** from the filesystem -- requires file recovery techniques

### Vulnerability / Technique

When a file is "deleted" from a filesystem, the data is not immediately erased. Instead, the filesystem metadata (directory entries, inode references) is updated to mark the space as available. The actual file content remains on disk until overwritten by new data.

### Recovery Tools

Several tools can recover deleted files from disk images:

1. **SleuthKit (`fls` + `icat`)**: `fls` lists all files including deleted ones (marked with `*`), and `icat` extracts file content by inode number
2. **`tsk_recover`**: Automatically recovers all unallocated/deleted files from a disk image
3. **`extundelete`**: Specifically for ext3/ext4 filesystems
4. **`photorec`/`scalpel`**: Carve files based on file signatures regardless of filesystem state
5. **Autopsy/FTK Imager**: GUI-based forensic tools

### Filesystem Analysis

Based on the DISKO series pattern, the disk image is likely:
- A gzip-compressed raw disk image (`.dd.gz`)
- Contains a DOS/MBR boot sector
- Uses either FAT32 or ext2/ext4 filesystem
- The flag was stored in a file that has since been deleted

## Solution

### Step 1: Decompress the disk image
```bash
gunzip disko-4.dd.gz
```

### Step 2: Identify the filesystem
```bash
file disko-4.dd
fdisk -l disko-4.dd
```

### Step 3: List all files including deleted ones
```bash
# Using SleuthKit fls (shows deleted files with * prefix)
fls -r -o 2048 disko-4.dd
```

### Step 4: Recover deleted files
```bash
# Method 1: Using tsk_recover to recover all deleted files
tsk_recover -o 2048 disko-4.dd output_dir/

# Method 2: If you identify the specific inode with fls
icat -o 2048 disko-4.dd <inode_number> > recovered_flag.txt

# Method 3: Using extundelete (for ext filesystems)
extundelete disko-4.dd --restore-all

# Method 4: Simple string search (may still work if data not overwritten)
strings disko-4.dd | grep -i picoCTF
```

### Step 5: Read the recovered flag
```bash
cat output_dir/flag.txt
# or
cat recovered_flag.txt
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
