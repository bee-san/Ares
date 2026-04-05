# Printer Shares 3 - picoCTF 2026

**Category:** General Skills
**Points:** 300

## Challenge Description
I accidentally left the debug script in place... Well, I think that's fine - No one could possibly access my super secure secret!

## Approach
This is the third challenge in the Printer Shares series, which explores misconfigured SMB (Server Message Block) network shares. The earlier challenges in the series introduced basic SMB enumeration and anonymous access. In this installment, the challenge author "accidentally" left a debug script on the share that leaks sensitive information -- specifically, a secret (the flag).

The attack surface is an SMB service exposed on a non-standard port. The key progression from earlier Printer Shares challenges:

- **Printer Shares 1**: Basic SMB enumeration and anonymous access to retrieve `flag.txt` from the `shares` share.
- **Printer Shares 2**: Required deeper enumeration of additional shares or files beyond the obvious ones.
- **Printer Shares 3**: A debug script was left behind on the share. This script, when examined, contains or reveals the "super secure secret" (the flag). The vulnerability is an information disclosure via a leftover debug/diagnostic script that was never cleaned up before deployment.

The methodology involves:
1. Enumerating available SMB shares using `smbclient -L`
2. Connecting to shares with anonymous/null authentication
3. Exploring the share contents recursively to find the debug script
4. Reading the debug script to extract the flag or running it to reveal the secret

## Solution

### Step 1: Enumerate Available Shares
First, list all available SMB shares on the target server:
```bash
smbclient -L //<TARGET_HOST> -p <TARGET_PORT> -N
```

The `-N` flag enables null (anonymous) authentication. The `-L` flag lists available shares. This reveals shares such as `shares`, `IPC$`, and potentially others.

### Step 2: Connect to the Share
Connect to the relevant share:
```bash
smbclient //<TARGET_HOST>/shares -p <TARGET_PORT> -N
```

### Step 3: Enumerate Files Recursively
Once connected, explore the share contents:
```
smb: \> ls
smb: \> recurse ON
smb: \> ls
```

Look for any scripts (`.sh`, `.py`, `.bat`) or unusual files beyond `flag.txt` and `dummy.txt`. The debug script may be in a subdirectory or at the root of the share.

### Step 4: Retrieve and Examine the Debug Script
Download the debug script:
```
smb: \> get debug.sh
```

Or view it directly. The debug script typically contains hardcoded credentials, paths, or directly echoes/prints the secret (the flag). For example, the script might contain something like:
```bash
#!/bin/bash
# Debug script - TODO: remove before production
SECRET="picoCTF{...}"
echo "Debug: secret is $SECRET"
```

### Step 5: Extract the Flag
Read the downloaded debug script locally to extract the flag:
```bash
cat debug.sh
```

The flag is embedded in the debug script as a hardcoded value.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
