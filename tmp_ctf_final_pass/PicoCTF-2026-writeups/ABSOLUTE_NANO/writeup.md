# ABSOLUTE NANO - picoCTF 2026

**Category:** General Skills
**Points:** 200

## Challenge Description
You have complete power with nano. Think you can get the flag?

## Approach

This challenge drops you into a `nano` text editor session on a remote server. The goal is to escape the editor environment and read the flag file. This is a classic **GTFOBins-style** challenge -- `nano` has built-in capabilities to execute shell commands and read files, which can be leveraged to break out of the restricted environment.

Key capabilities of `nano` that are useful here:

1. **Read File (Ctrl+R)**: Insert the contents of another file into the current buffer. This can be used to directly read `/flag.txt` or similar flag file paths.

2. **Execute Command (Ctrl+R, then Ctrl+X in older nano / Ctrl+T in some versions)**: After pressing Ctrl+R (Read File), pressing Ctrl+X or Ctrl+T switches to "Execute Command" mode. This allows you to run arbitrary shell commands and pipe their output into the editor buffer.

3. **Spawn a shell**: Using the execute command feature, you can run `sh`, `bash`, or `reset; sh 1>&0 2>&0` to get a full interactive shell.

According to [GTFOBins](https://gtfobins.github.io/gtfobins/nano/), nano can be used to:
- Read files directly
- Execute arbitrary commands
- Spawn interactive shells

## Solution

### Method 1: Read the flag file directly with Ctrl+R

1. Connect to the challenge server (SSH or netcat).
2. You are dropped into `nano`.
3. Press **Ctrl+R** (Read File).
4. Type the path to the flag file, e.g., `/flag.txt` or `/home/ctf/flag.txt`, and press **Enter**.
5. The flag contents are inserted into the buffer and displayed on screen.

Common flag file locations to try:
- `/flag.txt`
- `/flag`
- `/home/ctf/flag.txt`
- `/root/flag.txt`
- `~/flag.txt`

### Method 2: Execute a command to find and read the flag

1. In `nano`, press **Ctrl+R** (Read File).
2. Press **Ctrl+X** (or **Ctrl+T** depending on version) to switch to "Execute Command" mode.
3. Type: `cat /flag.txt` and press **Enter**.
4. The flag is inserted into the editor buffer.

If you don't know where the flag is:
1. Execute: `find / -name "flag*" 2>/dev/null`
2. This shows all files with "flag" in the name.
3. Then read the discovered file.

### Method 3: Spawn a full shell

1. Press **Ctrl+R**, then **Ctrl+X** (or **Ctrl+T**).
2. Type: `reset; bash 1>&0 2>&0` and press **Enter**.
3. You now have a full shell. Use `ls`, `find`, and `cat` to locate and read the flag.

### Method 4: Use Ctrl+T directly (some nano versions)

1. Press **Ctrl+T** (Execute Command / Spell Check depending on version).
2. Type: `cat /flag.txt`
3. The output appears in the buffer.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
