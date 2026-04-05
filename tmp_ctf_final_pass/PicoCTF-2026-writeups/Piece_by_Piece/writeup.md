# Piece by Piece - picoCTF 2026

**Category:** General Skills
**Points:** 50

## Challenge Description

After logging in, you will find multiple file parts in your home directory. These parts need to be combined and extracted to reveal the flag.

## Approach

This is a straightforward Linux/General Skills challenge involving **split files** and **archive extraction**. The challenge tests your ability to:

1. Connect to a remote system via SSH
2. Identify and work with split file parts
3. Reassemble split files using standard Linux tools
4. Extract archives (potentially password-protected)

### Understanding Split Files

Files can be split using the `split` command in Linux, which divides a file into smaller pieces. These pieces typically have names like:
- `file.zip.001`, `file.zip.002`, `file.zip.003`, ...
- `file.aa`, `file.ab`, `file.ac`, ...
- `flag_part1`, `flag_part2`, `flag_part3`, ...

To reassemble, you simply concatenate them in order using `cat`:
```
cat file.zip.* > file.zip
```

### Archive Extraction

After reassembly, the resulting file is likely a ZIP archive (possibly password-protected). Tools to use:
- `unzip file.zip` -- standard extraction
- `unzip -P <password> file.zip` -- extraction with password
- `7z x file.zip` -- alternative extractor
- `file combined_file` -- identify the file type first

### Password Handling

If the ZIP is password-protected, the password may be:
- Provided in the challenge description or a hint file
- A common/simple password
- Hidden somewhere in the file names or directory structure

## Solution

### Step-by-step:

1. **Start the challenge instance** on the picoCTF platform to get SSH credentials.

2. **Connect via SSH**:
   ```bash
   ssh ctf-player@<hostname> -p <port>
   ```
   Use the password provided by the challenge.

3. **List files** in the home directory:
   ```bash
   ls -la
   ```
   You should see multiple file parts (e.g., `flag.zip.001`, `flag.zip.002`, etc.).

4. **Identify the file parts** and their naming pattern:
   ```bash
   ls -la flag*
   file flag*
   ```

5. **Combine the parts** using `cat`:
   ```bash
   cat flag.zip.* > flag.zip
   ```
   Or if named differently:
   ```bash
   cat flag_part* > combined.zip
   ```

6. **Check the combined file type**:
   ```bash
   file flag.zip
   ```

7. **Extract the archive**:
   ```bash
   unzip flag.zip
   ```
   If password-protected, look for a password hint or try:
   ```bash
   unzip -P <password> flag.zip
   ```

8. **Read the flag**:
   ```bash
   cat flag.txt
   ```

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
