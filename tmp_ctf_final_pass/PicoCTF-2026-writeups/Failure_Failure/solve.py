#!/usr/bin/env python3
"""
Failure Failure - picoCTF 2026
Category: General Skills | Points: 200

A high-availability failover scenario challenge.
This script automates the enumeration and flag extraction process
by connecting via SSH and exploring the failover configuration.

Usage:
    python3 solve.py REMOTE_HOST PORT USERNAME PASSWORD
    python3 solve.py REMOTE_HOST PORT USERNAME --keyfile=~/.ssh/id_rsa

Example:
    python3 solve.py titan.picoctf.net 12345 ctf-player password123

Requirements:
    pip install paramiko pwntools
"""

import sys
import re
import time

try:
    import paramiko
except ImportError:
    print("Install paramiko: pip install paramiko")
    sys.exit(1)


# ============================================================
# CONFIGURATION
# ============================================================

# Commands to run for enumeration (in order)
ENUM_COMMANDS = [
    # --- Phase 1: Basic enumeration ---
    "echo '=== WHOAMI ===' && whoami",
    "echo '=== HOSTNAME ===' && hostname",
    "echo '=== HOME DIR ===' && ls -la ~",

    # --- Phase 2: Service enumeration ---
    "echo '=== SERVICES ===' && systemctl list-units --type=service --all 2>/dev/null || service --status-all 2>/dev/null",
    "echo '=== CUSTOM SYSTEMD ===' && ls -la /etc/systemd/system/ 2>/dev/null",
    "echo '=== TIMERS ===' && systemctl list-timers --all 2>/dev/null",

    # --- Phase 3: Search for failover configs ---
    "echo '=== FAILOVER FILES ===' && find / -maxdepth 4 \\( -name '*failover*' -o -name '*backup*' -o -name '*replica*' -o -name '*standby*' -o -name '*secondary*' -o -name '*ha-*' -o -name '*primary*' \\) 2>/dev/null",

    # --- Phase 4: Search for flag directly ---
    "echo '=== FLAG SEARCH ===' && grep -rl 'picoCTF' /etc/ /var/ /opt/ /home/ /tmp/ /root/ 2>/dev/null",
    "echo '=== FLAG IN ENV ===' && env | grep -i flag 2>/dev/null; cat /proc/*/environ 2>/dev/null | tr '\\0' '\\n' | grep -i flag 2>/dev/null",

    # --- Phase 5: Check cron and scheduled tasks ---
    "echo '=== CRON ===' && crontab -l 2>/dev/null; cat /etc/crontab 2>/dev/null; ls -la /etc/cron.d/ 2>/dev/null",

    # --- Phase 6: Log inspection ---
    "echo '=== JOURNAL RECENT ===' && journalctl --no-pager -n 50 2>/dev/null",
    "echo '=== SYSLOG ===' && tail -50 /var/log/syslog 2>/dev/null; tail -50 /var/log/messages 2>/dev/null",

    # --- Phase 7: Process inspection ---
    "echo '=== PROCESSES ===' && ps aux 2>/dev/null",

    # --- Phase 8: Network services ---
    "echo '=== LISTENING PORTS ===' && ss -tlnp 2>/dev/null || netstat -tlnp 2>/dev/null",

    # --- Phase 9: Read service configs ---
    "echo '=== SERVICE CONFIGS ===' && for f in /etc/systemd/system/*.service; do echo \"--- $f ---\"; cat \"$f\" 2>/dev/null; done",

    # --- Phase 10: Check /opt and application dirs ---
    "echo '=== OPT DIR ===' && ls -laR /opt/ 2>/dev/null",
    "echo '=== TMP DIR ===' && ls -la /tmp/ 2>/dev/null",

    # --- Phase 11: Check sudo privileges ---
    "echo '=== SUDO PRIVS ===' && sudo -l 2>/dev/null",
]

# Commands to trigger failover (run after enumeration if flag not found)
FAILOVER_COMMANDS = [
    # Try stopping services that look like primary/main services
    "echo '=== TRIGGERING FAILOVER ===' && for svc in $(systemctl list-units --type=service --state=running --plain --no-legend 2>/dev/null | awk '{print $1}'); do echo \"Service: $svc\"; done",
    # Attempt to stop primary-looking services (safe approach: just restart)
    "sudo systemctl stop primary.service 2>/dev/null; sudo systemctl stop main.service 2>/dev/null; sudo systemctl stop app.service 2>/dev/null",
    # Check what happened
    "echo '=== POST-FAILOVER STATUS ===' && systemctl list-units --type=service --all 2>/dev/null",
    "echo '=== POST-FAILOVER LOGS ===' && journalctl --no-pager -n 30 2>/dev/null",
    "echo '=== POST-FAILOVER FLAG ===' && grep -rl 'picoCTF' /etc/ /var/ /opt/ /home/ /tmp/ 2>/dev/null",
]


# ============================================================
# SSH CONNECTION AND EXECUTION
# ============================================================

def ssh_connect(host, port, username, password=None, keyfile=None):
    """Establish SSH connection."""
    client = paramiko.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())

    connect_kwargs = {
        "hostname": host,
        "port": port,
        "username": username,
    }

    if keyfile:
        connect_kwargs["key_filename"] = keyfile
    elif password:
        connect_kwargs["password"] = password

    print(f"[*] Connecting to {host}:{port} as {username}...")
    client.connect(**connect_kwargs)
    print("[+] Connected!")
    return client


def run_command(client, cmd, timeout=10):
    """Execute a command over SSH and return output."""
    stdin, stdout, stderr = client.exec_command(cmd, timeout=timeout)
    out = stdout.read().decode(errors="ignore")
    err = stderr.read().decode(errors="ignore")
    return out + err


def search_for_flag(text):
    """Search for picoCTF flag pattern in text."""
    matches = re.findall(r'picoCTF\{[^}]+\}', text)
    return matches


# ============================================================
# MAIN SOLVE LOGIC
# ============================================================

def solve(host, port, username, password=None, keyfile=None):
    client = ssh_connect(host, port, username, password, keyfile)
    all_output = ""
    flags_found = set()

    # Phase 1: Enumeration
    print("\n[*] Phase 1: Enumeration")
    print("=" * 60)

    for cmd in ENUM_COMMANDS:
        print(f"\n[>] {cmd[:80]}...")
        output = run_command(client, cmd)
        all_output += output
        if output.strip():
            print(output[:500])  # Print first 500 chars

        # Check for flags in output
        flags = search_for_flag(output)
        if flags:
            for f in flags:
                flags_found.add(f)
                print(f"\n{'!' * 60}")
                print(f"[!!!] FLAG FOUND: {f}")
                print(f"{'!' * 60}")

    if flags_found:
        print(f"\n[+] Found {len(flags_found)} flag(s):")
        for f in flags_found:
            print(f"    {f}")
        client.close()
        return flags_found

    # Phase 2: Trigger failover
    print("\n\n[*] Phase 2: Triggering failover")
    print("=" * 60)

    for cmd in FAILOVER_COMMANDS:
        print(f"\n[>] {cmd[:80]}...")
        output = run_command(client, cmd)
        all_output += output
        if output.strip():
            print(output[:500])

        flags = search_for_flag(output)
        if flags:
            for f in flags:
                flags_found.add(f)
                print(f"\n{'!' * 60}")
                print(f"[!!!] FLAG FOUND: {f}")
                print(f"{'!' * 60}")

    # Phase 3: Post-failover deep scan
    print("\n\n[*] Phase 3: Post-failover deep scan")
    print("=" * 60)

    time.sleep(2)  # Wait for failover to complete

    deep_scan = [
        "grep -rl 'picoCTF' / 2>/dev/null | head -20",
        "find / -maxdepth 3 -name 'flag*' -o -name '*.flag' 2>/dev/null",
        "journalctl --no-pager 2>/dev/null | grep -i 'picoCTF\\|flag'",
    ]

    for cmd in deep_scan:
        print(f"\n[>] {cmd[:80]}...")
        output = run_command(client, cmd, timeout=15)
        all_output += output
        if output.strip():
            print(output[:500])

        # If we find files containing the flag, read them
        for line in output.strip().split('\n'):
            line = line.strip()
            if line and not line.startswith('[') and '/' in line:
                file_output = run_command(client, f"cat '{line}' 2>/dev/null")
                flags = search_for_flag(file_output)
                if flags:
                    for f in flags:
                        flags_found.add(f)
                        print(f"\n[!!!] FLAG FOUND in {line}: {f}")

    # Final report
    print("\n" + "=" * 60)
    if flags_found:
        print(f"[+] Total flags found: {len(flags_found)}")
        for f in flags_found:
            print(f"    FLAG: {f}")
    else:
        print("[-] No flags found automatically.")
        print("    Manual investigation may be needed.")
        print("    Review the enumeration output above for clues.")
        print("\n    Common things to try manually:")
        print("    - Read service unit files and referenced scripts")
        print("    - Stop/restart specific services and check logs")
        print("    - Look for hidden files: find / -name '.*' 2>/dev/null")
        print("    - Check database files: find / -name '*.db' -o -name '*.sqlite' 2>/dev/null")

    client.close()
    return flags_found


# ============================================================
# ENTRY POINT
# ============================================================

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print(f"Usage: {sys.argv[0]} HOST PORT USERNAME [PASSWORD]")
        print(f"       {sys.argv[0]} HOST PORT USERNAME --keyfile=PATH")
        print()
        print(f"Example: {sys.argv[0]} titan.picoctf.net 12345 ctf-player password")
        sys.exit(1)

    host = sys.argv[1]
    port = int(sys.argv[2])
    username = sys.argv[3]

    password = None
    keyfile = None

    if len(sys.argv) >= 5:
        arg = sys.argv[4]
        if arg.startswith("--keyfile="):
            keyfile = arg.split("=", 1)[1]
        else:
            password = arg

    solve(host, port, username, password=password, keyfile=keyfile)
