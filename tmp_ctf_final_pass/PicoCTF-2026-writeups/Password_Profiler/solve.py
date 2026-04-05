#!/usr/bin/env python3
"""
Password Profiler - picoCTF 2026
Category: General Skills | Points: 100

Automated solution to crack a SHA-1 hash using publicly available breach
data sources (Have I Been Pwned Pwned Passwords API, CrackStation-style
lookups, and local wordlist cracking).

Usage:
    python3 solve.py [hash_file]
    python3 solve.py                      # reads 'hash.txt' by default
    python3 solve.py challenge_hash.txt   # reads from specified file
    SHA1_HASH=<hash> python3 solve.py     # direct hash via env var

Dependencies:
    pip install requests
"""

import os
import sys
import re
import hashlib
import requests

FLAG_PATTERN = re.compile(r"picoCTF\{[^}]+\}")
SHA1_PATTERN = re.compile(r"^[0-9a-fA-F]{40}$")


def read_hash_from_file(filepath):
    """Read a SHA-1 hash from a file."""
    try:
        with open(filepath, "r") as f:
            content = f.read().strip()
        # The file might contain just the hash, or have extra text
        for line in content.splitlines():
            line = line.strip()
            if SHA1_PATTERN.match(line):
                return line.lower()
        # If no pure hash line found, try to find a hash anywhere
        match = re.search(r"[0-9a-fA-F]{40}", content)
        if match:
            return match.group(0).lower()
        print(f"[!] No SHA-1 hash found in {filepath}")
        print(f"    File contents: {content[:200]}")
        return None
    except FileNotFoundError:
        print(f"[!] File not found: {filepath}")
        return None


def lookup_hibp(sha1_hash):
    """
    Look up a SHA-1 hash using the Have I Been Pwned Pwned Passwords API.
    Uses k-anonymity: only sends first 5 characters of the hash.
    Returns the plaintext password if found, None otherwise.

    Note: The HIBP API expects the SHA-1 of the PASSWORD, and returns
    hash suffixes. If the challenge provides a SHA-1 hash of a breached
    password, we check if that hash appears in the HIBP database.
    """
    sha1_upper = sha1_hash.upper()
    prefix = sha1_upper[:5]
    suffix = sha1_upper[5:]

    print(f"[*] Querying HIBP Pwned Passwords API (prefix: {prefix}...)")
    try:
        resp = requests.get(
            f"https://api.pwnedpasswords.com/range/{prefix}",
            headers={"User-Agent": "picoCTF-solver"},
            timeout=10,
        )
        if resp.status_code == 200:
            for line in resp.text.splitlines():
                parts = line.strip().split(":")
                if len(parts) == 2:
                    hash_suffix, count = parts
                    if hash_suffix.strip() == suffix:
                        print(f"[+] Hash found in HIBP! Seen {count.strip()} times in breaches.")
                        print("[*] HIBP confirms this is a known breached password hash.")
                        print("[*] The hash exists in breach data -- now we need the plaintext.")
                        return True  # Hash exists but HIBP doesn't give us plaintext
            print("[-] Hash not found in HIBP database.")
        else:
            print(f"[!] HIBP API returned status {resp.status_code}")
    except Exception as e:
        print(f"[!] HIBP lookup failed: {e}")

    return False


def lookup_crackstation_style(sha1_hash):
    """
    Attempt to look up the hash using free online hash lookup APIs.
    Tries multiple services that provide reverse hash lookups.
    """
    services = [
        {
            "name": "nitrxgen",
            "url": f"https://www.nitrxgen.net/md5db/{sha1_hash}",
            "parser": lambda r: r.text.strip() if r.text.strip() else None,
        },
        {
            "name": "hashtoolkit",
            "url": f"https://hashtoolkit.com/reverse-sha1-hash/?hash={sha1_hash}",
            "parser": lambda r: extract_from_html(r.text),
        },
    ]

    for service in services:
        print(f"[*] Trying {service['name']}...")
        try:
            resp = requests.get(
                service["url"],
                headers={"User-Agent": "Mozilla/5.0"},
                timeout=10,
            )
            if resp.status_code == 200:
                result = service["parser"](resp)
                if result:
                    print(f"[+] Found via {service['name']}: {result}")
                    return result
        except Exception as e:
            print(f"[!] {service['name']} failed: {e}")

    return None


def extract_from_html(html):
    """Extract plaintext from HTML response of hash lookup services."""
    # Look for common patterns in hash lookup result pages
    patterns = [
        re.compile(r'<span class="res">(.*?)</span>'),
        re.compile(r'class="text-success"[^>]*>(.*?)<'),
        re.compile(r"Decrypted.*?:\s*(.*?)(?:<|$)"),
        re.compile(r"Result.*?:\s*(.*?)(?:<|$)"),
    ]
    for pattern in patterns:
        match = pattern.search(html)
        if match:
            result = match.group(1).strip()
            if result and len(result) < 100:
                return result
    return None


def crack_with_wordlist(sha1_hash, wordlist_paths=None):
    """
    Attempt to crack the hash using local wordlists.
    Computes SHA-1 of each word and compares.
    """
    if wordlist_paths is None:
        wordlist_paths = [
            "/usr/share/wordlists/rockyou.txt",
            "/usr/share/wordlists/rockyou.txt.gz",
            "/usr/share/seclists/Passwords/Common-Credentials/10-million-password-list-top-1000000.txt",
            "/usr/share/seclists/Passwords/Leaked-Databases/rockyou.txt",
            "/usr/share/john/password.lst",
            "rockyou.txt",
            "wordlist.txt",
            "passwords.txt",
        ]

    for wordlist in wordlist_paths:
        if not os.path.exists(wordlist):
            continue

        print(f"[*] Cracking with wordlist: {wordlist}")

        open_func = open
        if wordlist.endswith(".gz"):
            import gzip
            open_func = gzip.open

        try:
            with open_func(wordlist, "rb") as f:
                count = 0
                for line in f:
                    try:
                        word = line.strip().decode("utf-8", errors="ignore")
                    except Exception:
                        continue

                    computed_hash = hashlib.sha1(word.encode("utf-8")).hexdigest()
                    if computed_hash == sha1_hash:
                        print(f"[+] CRACKED! Plaintext: {word}")
                        return word

                    count += 1
                    if count % 1000000 == 0:
                        print(f"    ...tried {count:,} passwords")

            print(f"[-] Not found in {wordlist} ({count:,} entries checked)")
        except Exception as e:
            print(f"[!] Error reading {wordlist}: {e}")

    return None


def try_common_passwords(sha1_hash):
    """Try a small built-in list of extremely common passwords."""
    print("[*] Trying common passwords...")
    common = [
        "password", "123456", "12345678", "qwerty", "abc123", "monkey",
        "1234567", "letmein", "trustno1", "dragon", "baseball", "iloveyou",
        "master", "sunshine", "ashley", "michael", "shadow", "123123",
        "654321", "superman", "qazwsx", "michael", "football", "password1",
        "password123", "batman", "login", "admin", "welcome", "hello",
        "charlie", "donald", "starwars", "1234567890", "123456789",
        "00000000", "0000", "1111", "1234", "passwd", "pass", "test",
        "guest", "access", "p@ssw0rd", "P@ssw0rd", "P@ssword1",
        "picoctf", "picoCTF", "flag", "secret", "hidden",
    ]

    for word in common:
        computed = hashlib.sha1(word.encode("utf-8")).hexdigest()
        if computed == sha1_hash:
            print(f"[+] CRACKED! Plaintext: {word}")
            return word

    print(f"[-] Not in common password list ({len(common)} entries)")
    return None


def main():
    print("=" * 60)
    print("  Password Profiler - picoCTF 2026 Solver")
    print("  Category: General Skills | Points: 100")
    print("=" * 60)
    print()

    # Determine the SHA-1 hash to crack
    sha1_hash = os.environ.get("SHA1_HASH", "").strip().lower()

    if not sha1_hash:
        # Try to read from file
        hash_file = sys.argv[1] if len(sys.argv) > 1 else "hash.txt"
        sha1_hash = read_hash_from_file(hash_file)

    if not sha1_hash:
        print("[!] No SHA-1 hash provided.")
        print(f"    Usage: python3 {sys.argv[0]} [hash_file]")
        print(f"    Or:    SHA1_HASH=<hash> python3 {sys.argv[0]}")
        sys.exit(1)

    if not SHA1_PATTERN.match(sha1_hash):
        print(f"[!] Invalid SHA-1 hash format: {sha1_hash}")
        print("    Expected: 40 hexadecimal characters")
        sys.exit(1)

    print(f"  Target SHA-1: {sha1_hash}")
    print()

    # Phase 1: Try common passwords first (instant)
    plaintext = try_common_passwords(sha1_hash)

    # Phase 2: Check HIBP to confirm it's a breached password
    if not plaintext:
        lookup_hibp(sha1_hash)

    # Phase 3: Try online hash lookup services
    if not plaintext:
        plaintext = lookup_crackstation_style(sha1_hash)

    # Phase 4: Try local wordlist cracking
    if not plaintext:
        plaintext = crack_with_wordlist(sha1_hash)

    # Results
    if plaintext:
        flag = f"picoCTF{{{plaintext}}}"

        # Check if the flag was already in picoCTF format
        if FLAG_PATTERN.match(plaintext):
            flag = plaintext

        print()
        print("=" * 60)
        print(f"  Cracked password: {plaintext}")
        print(f"  Flag: {flag}")
        print("=" * 60)
        print()
        print("[*] Note: The flag format may vary. Try both:")
        print(f"    - {flag}")
        print(f"    - {plaintext}")
    else:
        print()
        print("[-] Could not crack the hash automatically.")
        print("[*] Manual steps to try:")
        print(f"    1. Go to https://crackstation.net/ and paste: {sha1_hash}")
        print(f"    2. Go to https://hashes.com/en/decrypt/hash and paste: {sha1_hash}")
        print(f"    3. Use hashcat: hashcat -m 100 '{sha1_hash}' /path/to/rockyou.txt")
        print(f"    4. Use john: echo '{sha1_hash}' > hash.john && john --format=raw-sha1 hash.john --wordlist=/path/to/rockyou.txt")


if __name__ == "__main__":
    main()
