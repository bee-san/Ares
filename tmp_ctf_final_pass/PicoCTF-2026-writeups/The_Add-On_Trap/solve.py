#!/usr/bin/env python3
"""
The Add/On Trap - picoCTF 2026
Category: Reverse Engineering | Points: 200

Automated solver that extracts and analyzes a browser extension to find the flag.

Usage:
    python3 solve.py <extension_file>

Example:
    python3 solve.py addon.crx
    python3 solve.py addon.xpi
    python3 solve.py addon.zip

The script will:
1. Extract the extension archive
2. Parse manifest.json for clues
3. Search all files for flag patterns, encoded strings, and suspicious data
4. Attempt to decode obfuscated strings (base64, hex, XOR, char codes)
5. Report findings
"""

import argparse
import base64
import json
import os
import re
import struct
import sys
import tempfile
import zipfile


def strip_crx_header(data):
    """Strip CRX2 or CRX3 header to get the underlying ZIP data."""
    # CRX3 magic: "Cr24" followed by version 3
    if data[:4] == b"Cr24":
        version = struct.unpack("<I", data[4:8])[0]
        if version == 3:
            header_size = struct.unpack("<I", data[8:12])[0]
            return data[12 + header_size:]
        elif version == 2:
            pubkey_len = struct.unpack("<I", data[8:12])[0]
            sig_len = struct.unpack("<I", data[12:16])[0]
            offset = 16 + pubkey_len + sig_len
            return data[offset:]
    # If it starts with PK (ZIP magic), it's already a plain ZIP
    if data[:2] == b"PK":
        return data
    # Try to find ZIP magic in the first 1024 bytes
    idx = data.find(b"PK\x03\x04")
    if idx >= 0:
        return data[idx:]
    return data


def extract_extension(filepath, output_dir):
    """Extract the browser extension to output_dir."""
    with open(filepath, "rb") as f:
        data = f.read()

    zip_data = strip_crx_header(data)

    # Write cleaned ZIP to temp file and extract
    tmp_zip = os.path.join(output_dir, "_temp.zip")
    with open(tmp_zip, "wb") as f:
        f.write(zip_data)

    try:
        with zipfile.ZipFile(tmp_zip, "r") as zf:
            zf.extractall(output_dir)
        os.remove(tmp_zip)
        return True
    except zipfile.BadZipFile:
        print(f"[!] Could not extract as ZIP. Trying raw extraction...")
        os.remove(tmp_zip)
        return False


def find_flags(text):
    """Find picoCTF flag patterns in text."""
    return re.findall(r'picoCTF\{[^}]+\}', text)


def decode_base64_strings(text):
    """Find and decode base64-encoded strings."""
    decoded = []
    # Match base64 strings (at least 16 chars, padded or not)
    candidates = re.findall(r'["\']([A-Za-z0-9+/]{16,}={0,2})["\']', text)
    for b64 in candidates:
        try:
            raw = base64.b64decode(b64)
            decoded_str = raw.decode("utf-8", errors="ignore")
            if decoded_str.isprintable() and len(decoded_str) > 4:
                decoded.append((b64[:40] + "...", decoded_str))
        except Exception:
            continue
    return decoded


def decode_hex_strings(text):
    """Find and decode hex-encoded strings."""
    decoded = []
    # \x hex escapes
    hex_escaped = re.findall(r'["\']((\\x[0-9a-fA-F]{2}){4,})["\']', text)
    for match in hex_escaped:
        hex_str = match[0]
        try:
            raw = bytes.fromhex(hex_str.replace("\\x", ""))
            decoded_str = raw.decode("utf-8", errors="ignore")
            if decoded_str.isprintable():
                decoded.append(("hex_escape", decoded_str))
        except Exception:
            continue

    # Hex string patterns like "7069636f435446"
    hex_strings = re.findall(r'["\']([0-9a-fA-F]{16,})["\']', text)
    for hs in hex_strings:
        try:
            raw = bytes.fromhex(hs)
            decoded_str = raw.decode("utf-8", errors="ignore")
            if decoded_str.isprintable() and len(decoded_str) > 4:
                decoded.append(("hex_string", decoded_str))
        except Exception:
            continue
    return decoded


def decode_char_codes(text):
    """Find String.fromCharCode or charCodeAt patterns."""
    decoded = []
    # String.fromCharCode(num, num, num, ...)
    patterns = re.findall(
        r'String\.fromCharCode\s*\(([^)]+)\)', text
    )
    for match in patterns:
        try:
            codes = [int(x.strip()) for x in match.split(",") if x.strip().isdigit()]
            result = "".join(chr(c) for c in codes)
            if result and len(result) > 4:
                decoded.append(("fromCharCode", result))
        except Exception:
            continue

    # Array of character codes: [112, 105, 99, 111, ...]
    arrays = re.findall(r'\[(\s*\d+\s*(?:,\s*\d+\s*){4,})\]', text)
    for arr in arrays:
        try:
            codes = [int(x.strip()) for x in arr.split(",")]
            if all(32 <= c < 127 for c in codes):
                result = "".join(chr(c) for c in codes)
                decoded.append(("char_array", result))
        except Exception:
            continue

    return decoded


def decode_xor_strings(text):
    """Look for XOR-encoded strings with a key."""
    decoded = []
    # Pattern: array XORed with a key
    xor_patterns = re.findall(
        r'\[(\s*\d+\s*(?:,\s*\d+\s*){8,})\].*?(?:key|xor|cipher)\s*[=:]\s*["\']([^"\']+)["\']',
        text, re.IGNORECASE | re.DOTALL
    )
    for arr_str, key in xor_patterns:
        try:
            data = [int(x.strip()) for x in arr_str.split(",")]
            key_bytes = key.encode()
            result = "".join(chr(d ^ key_bytes[i % len(key_bytes)]) for i, d in enumerate(data))
            if "pico" in result.lower() or result.isprintable():
                decoded.append(("xor", result))
        except Exception:
            continue
    return decoded


def find_urls(text):
    """Find URLs that might be exfiltration endpoints."""
    urls = re.findall(r'https?://[^\s"\'<>]+', text)
    return urls


def find_suspicious_patterns(text):
    """Find patterns that suggest data exfiltration or hidden behavior."""
    patterns = {
        "fetch_calls": re.findall(r'fetch\s*\([^)]+\)', text),
        "xhr_calls": re.findall(r'XMLHttpRequest|\.open\s*\(|\.send\s*\(', text),
        "eval_usage": re.findall(r'\beval\s*\(', text),
        "atob_btoa": re.findall(r'\b(atob|btoa)\s*\([^)]+\)', text),
        "cookie_access": re.findall(r'document\.cookie|chrome\.cookies', text),
        "storage_access": re.findall(r'chrome\.storage|localStorage|sessionStorage', text),
        "sendBeacon": re.findall(r'navigator\.sendBeacon', text),
        "image_tracking": re.findall(r'new\s+Image\s*\(\s*\)\.src\s*=', text),
        "websocket": re.findall(r'new\s+WebSocket', text),
    }
    return {k: v for k, v in patterns.items() if v}


def analyze_manifest(manifest_path):
    """Analyze the manifest.json for interesting information."""
    try:
        with open(manifest_path, "r", errors="ignore") as f:
            manifest = json.load(f)
    except (json.JSONDecodeError, FileNotFoundError):
        print(f"[!] Could not parse manifest.json")
        return

    print(f"\n[*] Manifest Analysis:")
    print(f"    Name: {manifest.get('name', 'N/A')}")
    print(f"    Version: {manifest.get('version', 'N/A')}")
    print(f"    Description: {manifest.get('description', 'N/A')}")
    print(f"    Manifest Version: {manifest.get('manifest_version', 'N/A')}")

    # Check permissions
    perms = manifest.get("permissions", [])
    host_perms = manifest.get("host_permissions", [])
    optional_perms = manifest.get("optional_permissions", [])
    all_perms = perms + host_perms + optional_perms
    if all_perms:
        print(f"    Permissions: {', '.join(str(p) for p in all_perms)}")

    # Check content scripts
    content_scripts = manifest.get("content_scripts", [])
    for cs in content_scripts:
        matches = cs.get("matches", [])
        js_files = cs.get("js", [])
        print(f"    Content Script: {js_files} -> {matches}")

    # Check background
    bg = manifest.get("background", {})
    if bg:
        print(f"    Background: {bg}")

    # Check web_accessible_resources
    war = manifest.get("web_accessible_resources", [])
    if war:
        print(f"    Web Accessible Resources: {war}")

    # Check all string values in manifest for flag or encoded data
    manifest_str = json.dumps(manifest)
    flags = find_flags(manifest_str)
    if flags:
        print(f"\n    [+] FLAG FOUND IN MANIFEST: {flags[0]}")

    b64_decoded = decode_base64_strings(manifest_str)
    for src, decoded in b64_decoded:
        print(f"    [+] Base64 in manifest: {src} -> {decoded}")
        sub_flags = find_flags(decoded)
        if sub_flags:
            print(f"    [+] FLAG IN DECODED MANIFEST DATA: {sub_flags[0]}")

    return manifest


def scan_file(filepath):
    """Scan a single file for flags and encoded data."""
    findings = {"flags": [], "base64": [], "hex": [], "charcode": [],
                "xor": [], "urls": [], "suspicious": {}}

    try:
        with open(filepath, "r", errors="ignore") as f:
            content = f.read()
    except Exception:
        return findings

    # Direct flag search
    findings["flags"] = find_flags(content)

    # Encoded strings
    findings["base64"] = decode_base64_strings(content)
    findings["hex"] = decode_hex_strings(content)
    findings["charcode"] = decode_char_codes(content)
    findings["xor"] = decode_xor_strings(content)

    # Check decoded strings for flags
    for category in ["base64", "hex", "charcode", "xor"]:
        for src, decoded in findings[category]:
            sub_flags = find_flags(decoded)
            findings["flags"].extend(sub_flags)

    # URLs and suspicious patterns
    findings["urls"] = find_urls(content)
    findings["suspicious"] = find_suspicious_patterns(content)

    return findings


def main():
    parser = argparse.ArgumentParser(
        description="The Add/On Trap solver - picoCTF 2026"
    )
    parser.add_argument(
        "extension",
        help="Path to the browser extension file (.crx, .xpi, or .zip)"
    )
    parser.add_argument(
        "--output-dir", "-o",
        help="Directory to extract to (default: temp directory)",
        default=None
    )
    args = parser.parse_args()

    if not os.path.isfile(args.extension):
        print(f"[!] File not found: {args.extension}")
        sys.exit(1)

    print(f"[*] The Add/On Trap Solver - picoCTF 2026")
    print(f"[*] Analyzing: {args.extension}\n")

    # Extract
    output_dir = args.output_dir or tempfile.mkdtemp(prefix="addon_trap_")
    print(f"[*] Extracting to: {output_dir}")

    if not extract_extension(args.extension, output_dir):
        print("[!] Extraction failed. Try manually unzipping the file.")
        sys.exit(1)

    print(f"[+] Extraction successful\n")

    # List extracted files
    all_files = []
    for root, dirs, files in os.walk(output_dir):
        for fname in files:
            fpath = os.path.join(root, fname)
            rel_path = os.path.relpath(fpath, output_dir)
            all_files.append((fpath, rel_path))

    print(f"[*] Extracted {len(all_files)} files:")
    for _, rel in all_files:
        print(f"    {rel}")

    # Analyze manifest
    manifest_path = os.path.join(output_dir, "manifest.json")
    if os.path.isfile(manifest_path):
        analyze_manifest(manifest_path)
    else:
        print("\n[!] No manifest.json found at root level")
        # Search for it
        for fpath, rel in all_files:
            if os.path.basename(fpath) == "manifest.json":
                print(f"    Found manifest at: {rel}")
                analyze_manifest(fpath)
                break

    # Scan all files
    print(f"\n[*] Scanning all files for flags and encoded data...\n")
    all_flags = set()
    all_decoded = []
    all_suspicious = {}

    for fpath, rel in all_files:
        findings = scan_file(fpath)

        if findings["flags"]:
            for flag in findings["flags"]:
                all_flags.add(flag)
                print(f"    [+] FLAG in {rel}: {flag}")

        for category in ["base64", "hex", "charcode", "xor"]:
            for src, decoded in findings[category]:
                all_decoded.append((rel, category, src, decoded))
                print(f"    [~] {category} in {rel}: {decoded[:80]}")

        if findings["suspicious"]:
            for pattern, matches in findings["suspicious"].items():
                if pattern not in all_suspicious:
                    all_suspicious[pattern] = []
                all_suspicious[pattern].append((rel, matches))

        if findings["urls"]:
            for url in findings["urls"]:
                if "picoctf" in url.lower() or "flag" in url.lower():
                    print(f"    [~] Interesting URL in {rel}: {url}")

    # Report suspicious patterns
    if all_suspicious:
        print(f"\n[*] Suspicious patterns found:")
        for pattern, file_matches in all_suspicious.items():
            for rel, matches in file_matches:
                print(f"    [{pattern}] in {rel}: {len(matches)} occurrence(s)")

    # Final report
    print(f"\n{'='*60}")
    if all_flags:
        for flag in all_flags:
            print(f"[+] FLAG FOUND: {flag}")
    else:
        print("[-] No flag found automatically.")
        print("[*] Manual investigation may be needed:")
        print(f"    1. Examine files in: {output_dir}")
        print(f"    2. Look for obfuscated JS (use a JS beautifier)")
        print(f"    3. Check for data in image files (steganography)")
        print(f"    4. Try loading the extension in a browser and monitoring network traffic")
        print(f"    5. Check browser console output when the extension runs")

        if all_decoded:
            print(f"\n[*] Decoded strings that might be relevant:")
            for rel, cat, src, decoded in all_decoded:
                print(f"    [{cat}] {rel}: {decoded[:100]}")

    print(f"{'='*60}")


if __name__ == "__main__":
    main()
