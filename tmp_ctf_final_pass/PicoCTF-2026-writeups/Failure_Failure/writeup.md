# Failure Failure - picoCTF 2026

**Category:** General Skills
**Points:** 200

## Challenge Description

Welcome to Failure Failure -- a high-available system. This challenge simulates a real-world failover scenario.

## Approach

This is a General Skills challenge that simulates a high-availability / failover environment. The name "Failure Failure" is a play on the concept of a "failure of a failure" -- when the primary system fails AND the failover mechanism also fails (or is intentionally broken), revealing something interesting.

In real-world infrastructure, high-availability (HA) systems use techniques like:
- **Active/Passive failover**: A standby service takes over when the primary fails
- **Service monitoring**: Tools like `systemctl`, `supervisord`, or watchdog scripts detect failures
- **Redundant storage**: Data is replicated across multiple locations

The challenge likely provides SSH access to a server running multiple services. The goal is to explore the failover configuration, understand how the system handles failures, and find the flag hidden somewhere in the failover mechanism.

### Key Areas to Investigate

1. **Running services**: Check `systemctl list-units` or `service --status-all` for active/inactive services
2. **Service configurations**: Examine `/etc/systemd/system/` for custom service files
3. **Failover scripts**: Look for cron jobs, watchdog scripts, or systemd timers that handle failover
4. **Log files**: Check `/var/log/`, `journalctl`, and application-specific logs for clues
5. **Backup/redundant files**: Look for backup configurations, replicated data stores, or secondary services
6. **Environment variables**: Failover configs sometimes store secrets in env vars
7. **Process inspection**: Use `ps aux` to see what is actually running vs what should be running

### Common Patterns in This Challenge Type

- The flag may be split across the primary and failover service configurations
- Stopping or crashing the primary service triggers the failover, which reveals the flag
- The failover configuration file itself contains the flag as a "secret" or "token"
- Logs from a previous failover event contain the flag
- A backup/replica database or file store contains the flag

## Solution

### Step 1: Connect and Enumerate

```bash
ssh user@challenge-server -p PORT
```

Once connected, enumerate the system:

```bash
# List all services
systemctl list-units --type=service --all

# Check for custom services
ls -la /etc/systemd/system/

# Look for failover-related scripts
find / -name "*failover*" -o -name "*backup*" -o -name "*replica*" 2>/dev/null

# Check cron jobs
crontab -l
cat /etc/crontab
ls -la /etc/cron.d/
```

### Step 2: Examine Failover Configuration

```bash
# Read service unit files
cat /etc/systemd/system/primary.service
cat /etc/systemd/system/failover.service

# Check environment files referenced by services
cat /etc/default/primary
cat /etc/sysconfig/failover

# Look at ExecStartPre, ExecStart, ExecStop, ExecStartPost directives
# The flag may be in one of the scripts these reference
```

### Step 3: Trigger the Failover

```bash
# Stop the primary service to trigger failover
sudo systemctl stop primary.service

# Or simulate a failure
kill -9 $(pgrep primary-app)

# Watch what happens
journalctl -f
systemctl status failover.service
```

### Step 4: Inspect the Result

After triggering failover, check:

```bash
# New service output
curl localhost:PORT
cat /tmp/failover-output.txt

# Logs
journalctl -u failover.service
journalctl -u primary.service

# Any new files created
find /tmp -newer /etc/hostname
```

### Step 5: Look in Common Hidden Locations

```bash
# Check all config files for flag patterns
grep -r "picoCTF" /etc/ 2>/dev/null
grep -r "picoCTF" /var/ 2>/dev/null
grep -r "picoCTF" /opt/ 2>/dev/null
grep -r "picoCTF" /home/ 2>/dev/null

# Check environment of running processes
cat /proc/*/environ 2>/dev/null | tr '\0' '\n' | grep -i flag

# Check systemd unit overrides
systemctl cat primary.service
systemctl cat failover.service
```

The flag will be revealed through one of these enumeration steps -- either directly in a configuration file, in the output after triggering failover, or in the logs of the failover event.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
