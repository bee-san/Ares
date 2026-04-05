# Timeline 1 - picoCTF 2026

**Category:** Forensics
**Points:** 300

## Challenge Description

Can you find the flag in this disk image? Wrap what you find in the picoCTF flag format. Download the disk image. (Harder version of Timeline 0 -- may require deeper filesystem timeline analysis)

## Approach

This is the second challenge in the "Timeline" forensics series. While Timeline 0 likely involved straightforward file extraction from a disk image, Timeline 1 requires **filesystem timeline analysis** -- constructing and examining a chronological timeline of file system events (creation, modification, access, and deletion) to locate the flag.

### Key Concepts

1. **MAC timestamps**: Every file in a filesystem records three key timestamps -- **M**odification, **A**ccess, and **C**hange (or creation on some systems). These are collectively known as MAC times.
2. **The Sleuth Kit (TSK)**: A suite of command-line forensic tools for analyzing disk images and file systems. Key tools include:
   - `mmls` -- list partitions in a disk image
   - `fsstat` -- display file system information
   - `fls` -- list files (including deleted) in a disk image
   - `icat` -- extract a file by its inode number
   - `mactime` -- generate a chronological timeline from a body file
3. **Body file format**: The intermediate format used by `fls -m` and consumed by `mactime` to produce human-readable timelines.
4. **Deleted file recovery**: When a file is deleted, the directory entry is removed but the inode and data blocks may remain. `fls` shows deleted entries marked with `*`.

### Analysis Strategy

Timeline 1, being worth 300 points (vs. a simpler Timeline 0), likely hides the flag in one of these forensic artifacts:

1. **Deleted files**: The flag may have been written to a file that was subsequently deleted. The file contents are still recoverable via inode.
2. **Temporal anomalies**: The flag could be embedded in file timestamps themselves (e.g., encoded in modification times).
3. **Hidden in metadata**: The flag might appear in file names, extended attributes, or alternate data streams visible only through timeline analysis.
4. **Fragmented across files**: Parts of the flag may be scattered across multiple files, and the correct reassembly order is determined by sorting on timestamps.
5. **Slack space or unallocated areas**: The flag could reside in disk slack space between file boundaries or in unallocated blocks.

### Workflow

1. Identify the partition layout with `mmls`
2. Determine the filesystem type with `fsstat`
3. Generate a body file with `fls -m / -r -o <offset>`
4. Create the timeline with `mactime -b <bodyfile>`
5. Analyze the timeline for anomalies, suspicious filenames, or hidden patterns
6. Extract suspicious files with `icat`
7. Reconstruct the flag

## Solution

### Step 1: Examine the disk image

```bash
# Identify the disk image type
file disk.img

# List partitions and find the offset
mmls disk.img
```

Example output might show a Linux partition starting at sector 2048. The offset in sectors is important for all subsequent commands.

### Step 2: Examine the filesystem

```bash
# Get filesystem details (use -o for the partition offset in sectors)
fsstat -o 2048 disk.img
```

This confirms the filesystem type (e.g., ext4) and provides metadata such as block size, inode count, and volume label.

### Step 3: Generate the timeline body file

```bash
# Create a body file with all file metadata including deleted files
# -m / sets the mount point prefix
# -r enables recursive listing
# -o specifies the partition offset in sectors
fls -m / -r -o 2048 disk.img > body.txt
```

### Step 4: Generate the human-readable timeline

```bash
# Convert the body file to a sorted chronological timeline
mactime -b body.txt > timeline.txt

# Search the timeline for flag-related content
grep -i "flag\|pico\|ctf\|secret\|hidden" timeline.txt
```

### Step 5: Look for deleted files and anomalies

```bash
# List all deleted files (marked with *)
fls -r -d -o 2048 disk.img

# Search for interesting filenames
fls -r -p -o 2048 disk.img | grep -i "flag\|secret\|hidden\|txt\|key"
```

### Step 6: Extract suspicious files

```bash
# Extract a file by inode number (e.g., inode 42)
icat -o 2048 disk.img 42

# Search all strings in the disk image as a fallback
strings -a disk.img | grep -i "pico\|flag\|ctf"
```

### Step 7: Analyze timeline patterns

For Timeline 1 specifically, the flag may require examining the **order** or **content** of files based on their timestamps:

```bash
# Sort timeline by date and look for files created in rapid succession
# (may indicate programmatic flag planting)
cat timeline.txt | sort | less

# Check for unusual timestamp patterns (e.g., files with identical timestamps)
cat timeline.txt | awk '{print $1, $2}' | sort | uniq -c | sort -rn | head
```

### Step 8: Reconstruct the flag

If the flag is split across multiple files sorted by timestamp:
```bash
# Extract each file's content in chronological order and concatenate
for inode in $(cat timeline.txt | grep "flag_part" | awk '{print $NF}' | cut -d'-' -f1); do
    icat -o 2048 disk.img $inode
done
```

Wrap the recovered string in the flag format: `picoCTF{recovered_string}`

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
