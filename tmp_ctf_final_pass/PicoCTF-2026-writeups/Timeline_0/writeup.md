# Timeline 0 - picoCTF 2026

**Category:** Forensics
**Points:** 100

## Challenge Description

Can you find the flag in this disk image? Wrap what you find in the picoCTF flag format. Download the disk image.

We are given a disk image file and need to extract a hidden flag. The challenge name "Timeline" strongly hints at using filesystem timeline analysis with Sleuth Kit tools (`fls`, `mactime`) to locate the flag within the image.

## Approach

### Understanding Filesystem Timelines

A filesystem timeline is a chronological record of all file operations (creation, modification, access, change) in a disk image. The Sleuth Kit provides two key tools for this:

1. **`fls`** -- Lists files and directories in a filesystem image, including deleted files. With the `-m` flag, it outputs in "body file" format suitable for timeline creation.
2. **`mactime`** -- Reads the body file and produces a human-readable sorted timeline of all MAC (Modified, Accessed, Changed) timestamps.

### Analysis Workflow

1. **Examine the disk image** with `mmls` to identify partitions and their offsets.
2. **Generate a body file** using `fls -m "/" -r` to recursively list all files.
3. **Create a timeline** using `mactime` to sort all entries chronologically.
4. **Search for the flag** by grepping through filenames, or by extracting suspicious files with `icat`.

### Alternative Quick Methods

For a 100-point challenge, the flag might also be findable via:
- `strings disk.img | grep -i "picoCTF\|flag"` -- brute-force string search
- `fls -r -o <offset> disk.img | grep -i flag` -- search file listing directly
- Mounting the image and browsing the filesystem

## Solution

### Step 1: Examine the disk image

```bash
# Check what type of image we have
file disk.img

# List partitions
mmls disk.img
```

Example output from `mmls`:
```
DOS Partition Table
Offset Sector: 0
Units are in 512-byte sectors

      Slot      Start        End          Length       Description
000:  Meta      0000000000   0000000000   0000000001   Primary Table (#0)
001:  -------   0000000000   0000002047   0000002048   Unallocated
002:  000:000   0000002048   0000206847   0000204800   Linux (0x83)
```

### Step 2: List files in the partition

```bash
# Use the offset from mmls (e.g., 2048 sectors)
fls -o 2048 -r disk.img
```

### Step 3: Generate timeline body file

```bash
# Create body file with recursive file listing
fls -m "/" -o 2048 -r disk.img > body.txt

# Generate the timeline
mactime -b body.txt > timeline.txt
```

### Step 4: Search for the flag

```bash
# Search in the timeline for flag-related filenames
grep -i "flag" timeline.txt

# Or search for any interesting files
grep -i "flag\|secret\|hidden\|picoctf" timeline.txt

# Also try raw strings search
strings disk.img | grep -i "picoCTF"
```

### Step 5: Extract the flag file

```bash
# Once you identify the file and its inode from fls output:
# fls output might show something like:
#   r/r 42: flag.txt
# Extract it:
icat -o 2048 disk.img 42
```

### Step 6: Wrap the flag

The challenge says "Wrap what you find in the picoCTF flag format," meaning the raw content might not already be in `picoCTF{...}` format. Take whatever string you find and wrap it:

```
picoCTF{found_string_here}
```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
