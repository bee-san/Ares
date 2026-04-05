#!/usr/bin/env python3
"""
Undo - picoCTF 2026
Category: General Skills | Points: 100

Automated solver that reverses Linux text transformations to recover the flag.

Usage:
    python3 solve.py <transformed_file>
    python3 solve.py <transformed_file> --transformations "base64 rev tr_upper rot13"
    echo "TRANSFORMED_STRING" | python3 solve.py -

If the transformations are known, pass them with --transformations in the ORDER
THEY WERE APPLIED (the script will reverse them automatically).

If not specified, the script will try to auto-detect and brute-force common
transformation sequences.

Supported transformations:
    base64      - base64 encoding
    rev         - character reversal per line
    tac         - line order reversal
    tr_upper    - lowercase -> uppercase
    tr_lower    - uppercase -> lowercase
    rot13       - ROT13 cipher
    xxd         - hex dump (xxd format)
    xxd_plain   - plain hex dump
    fold_N      - fold/wrap at N characters (e.g., fold_80)
"""

import argparse
import base64
import codecs
import itertools
import re
import sys


FLAG_PATTERN = re.compile(r'picoCTF\{[^}]+\}')


def undo_base64(data):
    """Reverse base64 encoding."""
    # Strip whitespace that might have been introduced
    cleaned = data.strip().replace("\n", "").replace("\r", "").replace(" ", "")
    # Add padding if needed
    padding = 4 - len(cleaned) % 4
    if padding != 4:
        cleaned += "=" * padding
    try:
        decoded = base64.b64decode(cleaned)
        return decoded.decode("utf-8", errors="replace")
    except Exception:
        return None


def undo_rev(data):
    """Reverse character order per line (inverse of `rev`)."""
    lines = data.split("\n")
    return "\n".join(line[::-1] for line in lines)


def undo_tac(data):
    """Reverse line order (inverse of `tac`)."""
    lines = data.split("\n")
    # Remove trailing empty line if present
    if lines and lines[-1] == "":
        lines = lines[:-1]
    return "\n".join(reversed(lines))


def undo_tr_upper(data):
    """Reverse tr 'a-z' 'A-Z' (convert uppercase back to lowercase)."""
    return data.lower()


def undo_tr_lower(data):
    """Reverse tr 'A-Z' 'a-z' (convert lowercase back to uppercase)."""
    return data.upper()


def undo_rot13(data):
    """Reverse ROT13 (self-inverse)."""
    return codecs.decode(data, "rot_13")


def undo_xxd(data):
    """Reverse xxd hex dump format."""
    result = []
    for line in data.strip().split("\n"):
        # xxd format: "00000000: 7069 636f 4354 467b  picoCTF{"
        parts = line.split(":")
        if len(parts) >= 2:
            hex_part = parts[1].split("  ")[0].strip()
            hex_clean = hex_part.replace(" ", "")
            try:
                result.append(bytes.fromhex(hex_clean).decode("utf-8", errors="replace"))
            except ValueError:
                continue
    return "".join(result) if result else None


def undo_xxd_plain(data):
    """Reverse xxd -p (plain hex dump)."""
    cleaned = data.strip().replace("\n", "").replace("\r", "").replace(" ", "")
    try:
        return bytes.fromhex(cleaned).decode("utf-8", errors="replace")
    except ValueError:
        return None


def undo_fold(data):
    """Reverse fold (rejoin wrapped lines)."""
    return data.replace("\n", "")


# Map of transformation names to their undo functions
UNDO_MAP = {
    "base64": undo_base64,
    "rev": undo_rev,
    "tac": undo_tac,
    "tr_upper": undo_tr_upper,
    "tr_lower": undo_tr_lower,
    "rot13": undo_rot13,
    "xxd": undo_xxd,
    "xxd_plain": undo_xxd_plain,
    "fold": undo_fold,
}


def check_flag(data):
    """Check if data contains a picoCTF flag."""
    if data is None:
        return None
    match = FLAG_PATTERN.search(data)
    return match.group(0) if match else None


def detect_encoding(data):
    """Try to detect what encoding/transformation was last applied."""
    detections = []

    stripped = data.strip()

    # Check for xxd format (lines starting with hex offset)
    if re.match(r'^[0-9a-f]{8}:', stripped, re.MULTILINE):
        detections.append("xxd")

    # Check for plain hex (only hex characters and newlines)
    hex_clean = stripped.replace("\n", "").replace("\r", "").replace(" ", "")
    if re.match(r'^[0-9a-fA-F]+$', hex_clean) and len(hex_clean) % 2 == 0:
        detections.append("xxd_plain")

    # Check for base64 (valid base64 chars, possibly with = padding)
    b64_clean = stripped.replace("\n", "").replace("\r", "")
    if re.match(r'^[A-Za-z0-9+/]+=*$', b64_clean) and len(b64_clean) >= 4:
        detections.append("base64")

    # Check if all uppercase (might be tr_upper)
    alpha_only = re.sub(r'[^a-zA-Z]', '', stripped)
    if alpha_only and alpha_only == alpha_only.upper():
        detections.append("tr_upper")

    # Check if all lowercase
    if alpha_only and alpha_only == alpha_only.lower():
        detections.append("tr_lower")

    return detections


def apply_undo_sequence(data, sequence):
    """Apply a sequence of undo operations (in reverse order of application)."""
    current = data
    for transform in reversed(sequence):
        # Handle fold_N pattern
        if transform.startswith("fold_"):
            undo_func = undo_fold
        elif transform in UNDO_MAP:
            undo_func = UNDO_MAP[transform]
        else:
            return None

        result = undo_func(current)
        if result is None:
            return None
        current = result

    return current


def brute_force_transformations(data, max_depth=4):
    """Try all combinations of transformations up to max_depth."""
    transforms = ["base64", "rev", "tac", "tr_upper", "rot13", "xxd_plain", "fold"]

    # First check the raw data
    flag = check_flag(data)
    if flag:
        return flag, []

    # Start with most likely single transforms, then increase depth
    for depth in range(1, max_depth + 1):
        print(f"[*] Trying depth {depth} ({len(transforms)**depth} combinations)...")

        for combo in itertools.product(transforms, repeat=depth):
            result = apply_undo_sequence(data, list(combo))
            if result is not None:
                flag = check_flag(result)
                if flag:
                    return flag, list(combo)

    return None, []


def smart_peel(data, max_depth=6):
    """Intelligently peel transformations one at a time using detection."""
    current = data
    applied = []

    for depth in range(max_depth):
        # Check if we already have the flag
        flag = check_flag(current)
        if flag:
            return flag, applied

        # Detect likely encodings
        detections = detect_encoding(current)

        # Also always try rev and rot13 since they're hard to detect
        to_try = list(dict.fromkeys(detections + ["rev", "rot13", "tac", "fold"]))

        found_progress = False
        for transform in to_try:
            undo_func = UNDO_MAP.get(transform)
            if not undo_func:
                continue

            result = undo_func(current)
            if result is None:
                continue

            # Check if this transformation produced a flag
            flag = check_flag(result)
            if flag:
                applied.append(transform)
                return flag, applied

            # Check if the result looks "more decoded" (contains more printable chars
            # or is closer to the flag pattern)
            if "pico" in result.lower() or "ctf{" in result.lower():
                current = result
                applied.append(transform)
                found_progress = True
                print(f"    [~] Applied undo_{transform}: partial flag visible")
                break

        if not found_progress:
            # Try each and see if we get closer (heuristic)
            for transform in to_try:
                undo_func = UNDO_MAP.get(transform)
                if not undo_func:
                    continue
                result = undo_func(current)
                if result and result != current:
                    # Recurse with this option
                    sub_flag = check_flag(result)
                    if sub_flag:
                        applied.append(transform)
                        return sub_flag, applied

            break  # No progress possible

    return None, applied


def main():
    parser = argparse.ArgumentParser(
        description="Undo solver - picoCTF 2026"
    )
    parser.add_argument(
        "input",
        help="Path to the transformed file, or '-' for stdin"
    )
    parser.add_argument(
        "--transformations", "-t",
        help="Space-separated list of transformations in order applied "
             "(e.g., 'base64 rev tr_upper rot13')",
        default=None
    )
    parser.add_argument(
        "--brute-force", "-b",
        action="store_true",
        help="Brute-force all transformation combinations (slow but thorough)"
    )
    parser.add_argument(
        "--max-depth", "-d",
        type=int, default=4,
        help="Maximum transformation depth for brute-force (default: 4)"
    )
    args = parser.parse_args()

    # Read input
    if args.input == "-":
        data = sys.stdin.read()
    else:
        try:
            with open(args.input, "r", errors="ignore") as f:
                data = f.read()
        except FileNotFoundError:
            print(f"[!] File not found: {args.input}")
            sys.exit(1)

    print(f"[*] Undo Solver - picoCTF 2026")
    print(f"[*] Input length: {len(data)} characters")
    print(f"[*] First 100 chars: {data[:100]!r}\n")

    # Check if flag is already visible in raw data
    flag = check_flag(data)
    if flag:
        print(f"[+] Flag found directly in input: {flag}")
        return

    # Method 1: If transformations are specified, apply them in reverse
    if args.transformations:
        transforms = args.transformations.strip().split()
        print(f"[*] Applying inverse of: {' | '.join(transforms)}")
        result = apply_undo_sequence(data, transforms)
        if result:
            flag = check_flag(result)
            if flag:
                print(f"\n{'='*60}")
                print(f"[+] FLAG FOUND: {flag}")
                print(f"{'='*60}")
                return
            else:
                print(f"[-] Result: {result[:200]!r}")
                print(f"[-] No flag pattern found in result")
        else:
            print(f"[-] Transformation sequence failed")
        return

    # Method 2: Smart detection-based peeling
    print(f"[*] Attempting smart detection-based decoding...")
    detections = detect_encoding(data)
    print(f"    Detected encodings: {detections if detections else 'none obvious'}\n")

    flag, applied = smart_peel(data)
    if flag:
        print(f"\n{'='*60}")
        print(f"[+] FLAG FOUND: {flag}")
        print(f"[+] Transformations undone: {' -> '.join(applied)}")
        print(f"{'='*60}")
        return

    # Method 3: Brute-force if requested or as fallback
    if args.brute_force or not applied:
        print(f"\n[*] Trying brute-force approach (max depth: {args.max_depth})...")
        flag, combo = brute_force_transformations(data, args.max_depth)
        if flag:
            print(f"\n{'='*60}")
            print(f"[+] FLAG FOUND: {flag}")
            print(f"[+] Original transformation sequence: {' | '.join(combo)}")
            print(f"[+] Undo sequence: {' | '.join(reversed(combo))}")
            print(f"{'='*60}")
            return

    # If nothing worked
    print(f"\n[-] Could not automatically recover the flag.")
    print(f"[*] Suggestions:")
    print(f"    1. Specify the transformations with --transformations")
    print(f"       Example: python3 solve.py data.txt -t 'base64 rev tr_upper'")
    print(f"    2. Try with --brute-force --max-depth 5 for deeper search")
    print(f"    3. Manually examine the data and identify transformations")
    print(f"    4. Common pipelines to try manually:")
    print(f"       cat data.txt | rev | base64 -d")
    print(f"       cat data.txt | tr 'A-Z' 'a-z' | rev | base64 -d")
    print(f"       cat data.txt | tr 'A-Za-z' 'N-ZA-Mn-za-m' | base64 -d")
    print(f"       cat data.txt | xxd -r -p")
    print(f"       cat data.txt | base64 -d | rev")
    print(f"       cat data.txt | base64 -d | base64 -d")


if __name__ == "__main__":
    main()
