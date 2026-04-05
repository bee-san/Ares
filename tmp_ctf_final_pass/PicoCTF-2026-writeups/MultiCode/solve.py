#!/usr/bin/env python3
"""
MultiCode - picoCTF 2026
Category: General Skills | Points: 200

Multiple layers of encoding/obfuscation hide the flag.  No encryption --
just encodings like base64, hex, ROT13, binary, octal, morse, base32,
decimal ASCII, URL encoding, and Atbash.

This script peels layers automatically until the picoCTF{...} flag appears.

Usage:
    python3 solve.py                     # interactive -- paste the encoded text
    python3 solve.py encoded_message.txt # read from a file
    echo "<encoded>" | python3 solve.py  # pipe in the data
"""

import base64
import binascii
import re
import string
import sys
import urllib.parse


# ---------------------------------------------------------------------------
# Decoder functions -- each returns decoded text or None on failure
# ---------------------------------------------------------------------------

def try_base64(data: str):
    """Decode Base64.  Accepts standard and URL-safe alphabets."""
    # Quick sanity: base64 strings are a multiple of 4 (or close with padding)
    stripped = data.strip()
    if not stripped:
        return None
    # Allow A-Za-z0-9+/= and whitespace
    if not re.fullmatch(r'[A-Za-z0-9+/=\s]+', stripped):
        # Try URL-safe variant
        if not re.fullmatch(r'[A-Za-z0-9\-_=\s]+', stripped):
            return None
    try:
        cleaned = re.sub(r'\s+', '', stripped)
        # Pad if needed
        missing_padding = len(cleaned) % 4
        if missing_padding:
            cleaned += '=' * (4 - missing_padding)
        decoded = base64.b64decode(cleaned, validate=True)
        text = decoded.decode('utf-8', errors='strict')
        # Reject if result is mostly non-printable
        if sum(c in string.printable for c in text) / max(len(text), 1) < 0.8:
            return None
        return text
    except Exception:
        return None


def try_base32(data: str):
    """Decode Base32."""
    stripped = data.strip().upper()
    if not stripped:
        return None
    if not re.fullmatch(r'[A-Z2-7=\s]+', stripped):
        return None
    try:
        cleaned = re.sub(r'\s+', '', stripped)
        missing_padding = len(cleaned) % 8
        if missing_padding:
            cleaned += '=' * (8 - missing_padding)
        decoded = base64.b32decode(cleaned)
        text = decoded.decode('utf-8', errors='strict')
        if sum(c in string.printable for c in text) / max(len(text), 1) < 0.8:
            return None
        return text
    except Exception:
        return None


def try_hex(data: str):
    """Decode hexadecimal (with or without 0x prefix, spaces, colons)."""
    stripped = data.strip()
    # Remove common hex prefixes/separators
    cleaned = re.sub(r'(0x|\\x|:|\s+)', '', stripped)
    if not cleaned:
        return None
    if not re.fullmatch(r'[0-9a-fA-F]+', cleaned):
        return None
    if len(cleaned) % 2 != 0:
        return None
    try:
        decoded = bytes.fromhex(cleaned).decode('utf-8', errors='strict')
        if sum(c in string.printable for c in decoded) / max(len(decoded), 1) < 0.8:
            return None
        return decoded
    except Exception:
        return None


def try_binary(data: str):
    """Decode binary (groups of 8 bits)."""
    stripped = data.strip()
    cleaned = re.sub(r'[\s,]+', '', stripped)
    if not cleaned:
        return None
    if not re.fullmatch(r'[01]+', cleaned):
        return None
    if len(cleaned) % 8 != 0:
        return None
    try:
        chars = [chr(int(cleaned[i:i+8], 2)) for i in range(0, len(cleaned), 8)]
        text = ''.join(chars)
        if sum(c in string.printable for c in text) / max(len(text), 1) < 0.8:
            return None
        return text
    except Exception:
        return None


def try_octal(data: str):
    """Decode octal (space/comma separated groups of 3-digit octal numbers)."""
    stripped = data.strip()
    parts = re.split(r'[\s,]+', stripped)
    if len(parts) < 4:
        return None
    if not all(re.fullmatch(r'[0-7]{1,3}', p) for p in parts):
        return None
    try:
        text = ''.join(chr(int(p, 8)) for p in parts)
        if sum(c in string.printable for c in text) / max(len(text), 1) < 0.8:
            return None
        return text
    except Exception:
        return None


def try_decimal(data: str):
    """Decode decimal ASCII (space/comma separated decimal numbers)."""
    stripped = data.strip()
    parts = re.split(r'[\s,]+', stripped)
    if len(parts) < 4:
        return None
    try:
        nums = [int(p) for p in parts]
    except ValueError:
        return None
    if not all(0 <= n <= 127 for n in nums):
        return None
    # Ensure these look like ASCII values, not just small numbers
    if not all(32 <= n <= 126 or n in (9, 10, 13) for n in nums):
        return None
    try:
        text = ''.join(chr(n) for n in nums)
        return text
    except Exception:
        return None


def try_rot13(data: str):
    """Apply ROT13.  Always succeeds, so we only use it when other decoders fail
       and the result looks more 'flag-like'."""
    stripped = data.strip()
    if not stripped:
        return None
    # ROT13 only makes sense on text that has letters
    if not any(c.isalpha() for c in stripped):
        return None
    table = str.maketrans(
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz',
        'NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm'
    )
    return stripped.translate(table)


def try_atbash(data: str):
    """Apply Atbash cipher (A<->Z, B<->Y, ...)."""
    stripped = data.strip()
    if not stripped:
        return None
    if not any(c.isalpha() for c in stripped):
        return None
    result = []
    for c in stripped:
        if c.isupper():
            result.append(chr(ord('Z') - (ord(c) - ord('A'))))
        elif c.islower():
            result.append(chr(ord('z') - (ord(c) - ord('a'))))
        else:
            result.append(c)
    return ''.join(result)


def try_url_decode(data: str):
    """Decode URL/percent encoding."""
    stripped = data.strip()
    if '%' not in stripped:
        return None
    try:
        decoded = urllib.parse.unquote(stripped)
        if decoded == stripped:
            return None  # nothing changed
        return decoded
    except Exception:
        return None


MORSE_CODE_DICT = {
    '.-': 'A', '-...': 'B', '-.-.': 'C', '-..': 'D', '.': 'E',
    '..-.': 'F', '--.': 'G', '....': 'H', '..': 'I', '.---': 'J',
    '-.-': 'K', '.-..': 'L', '--': 'M', '-.': 'N', '---': 'O',
    '.--.': 'P', '--.-': 'Q', '.-.': 'R', '...': 'S', '-': 'T',
    '..-': 'U', '...-': 'V', '.--': 'W', '-..-': 'X', '-.--': 'Y',
    '--..': 'Z', '-----': '0', '.----': '1', '..---': '2',
    '...--': '3', '....-': '4', '.....': '5', '-....': '6',
    '--...': '7', '---..': '8', '----.': '9',
    '.-.-.-': '.', '--..--': ',', '..--..': '?', '.----.': "'",
    '-.-.--': '!', '-..-.': '/', '-.--.': '(', '-.--.-': ')',
    '.-...': '&', '---...': ':', '-.-.-.': ';', '-...-': '=',
    '.-.-.': '+', '-....-': '-', '..--.-': '_', '.-..-.': '"',
    '...-..-': '$', '.--.-.': '@', '-.--.-': '}', '-.--.': '{',
}


def try_morse(data: str):
    """Decode Morse code (dots and dashes separated by spaces/slashes)."""
    stripped = data.strip()
    # Must contain dots and dashes
    if '.' not in stripped and '-' not in stripped:
        return None
    # Should be mostly dots, dashes, spaces, slashes
    morse_chars = set('.-/ \t\n')
    if sum(c in morse_chars for c in stripped) / max(len(stripped), 1) < 0.9:
        return None
    try:
        # Split words by ' / ' or '/' and letters by space
        words = re.split(r'\s*/\s*', stripped)
        decoded_words = []
        for word in words:
            letters = word.strip().split()
            decoded_word = ''
            for letter in letters:
                if letter in MORSE_CODE_DICT:
                    decoded_word += MORSE_CODE_DICT[letter]
                else:
                    return None  # unrecognized morse sequence
            decoded_words.append(decoded_word)
        return ' '.join(decoded_words)
    except Exception:
        return None


# ---------------------------------------------------------------------------
# Main decoding loop
# ---------------------------------------------------------------------------

# Ordered by specificity -- more specific decoders first, ROT13/Atbash last
DECODERS = [
    ("Morse",    try_morse),
    ("Binary",   try_binary),
    ("Octal",    try_octal),
    ("Decimal",  try_decimal),
    ("Hex",      try_hex),
    ("URL",      try_url_decode),
    ("Base32",   try_base32),
    ("Base64",   try_base64),
    ("ROT13",    try_rot13),
    ("Atbash",   try_atbash),
]

FLAG_PATTERN = re.compile(r'picoCTF\{[^}]+\}')


def contains_flag(text):
    return FLAG_PATTERN.search(text)


def peel_layers(data: str, max_layers: int = 20):
    """Iteratively decode layers until the flag is found or no progress."""
    current = data.strip()
    layers = []

    for iteration in range(max_layers):
        print(f"\n{'='*60}")
        print(f"Layer {iteration + 1} -- current data ({len(current)} chars):")
        preview = current[:120] + ("..." if len(current) > 120 else "")
        print(f"  {preview}")

        # Check for flag
        match = contains_flag(current)
        if match:
            flag = match.group(0)
            print(f"\n{'='*60}")
            print(f"FLAG FOUND after {len(layers)} decoding layer(s)!")
            print(f"  Layers applied: {' -> '.join(name for name, _ in layers) if layers else '(none)'}")
            print(f"  Flag: {flag}")
            print(f"{'='*60}")
            return flag

        # Try each decoder
        decoded = False
        for name, decoder in DECODERS:
            # For ROT13 and Atbash, only try if other decoders failed
            # and the result looks more promising
            result = decoder(current)
            if result is not None and result != current:
                if name in ("ROT13", "Atbash"):
                    # Only accept if the result contains 'pico' or looks more flag-like
                    if 'pico' in result.lower() or contains_flag(result):
                        print(f"  -> Decoded with: {name}")
                        layers.append((name, decoder))
                        current = result
                        decoded = True
                        break
                else:
                    print(f"  -> Decoded with: {name}")
                    layers.append((name, decoder))
                    current = result
                    decoded = True
                    break

        if not decoded:
            # Last resort: try ROT13 and Atbash unconditionally
            for name, decoder in [("ROT13", try_rot13), ("Atbash", try_atbash)]:
                result = decoder(current)
                if result is not None and result != current:
                    print(f"  -> Trying: {name} (speculative)")
                    layers.append((name, decoder))
                    current = result
                    decoded = True
                    break

        if not decoded:
            print(f"\n[!] No decoder matched at layer {iteration + 1}.")
            print(f"[*] Layers decoded so far: {' -> '.join(name for name, _ in layers)}")
            print(f"[*] Final text: {current}")
            return None

    print(f"\n[!] Reached maximum layers ({max_layers}) without finding flag.")
    print(f"[*] Final text: {current}")
    return None


def main():
    print("=" * 60)
    print("MultiCode - picoCTF 2026")
    print("Peel multiple encoding layers to reveal the flag")
    print("=" * 60)

    # Get input from file, stdin, or interactive prompt
    if len(sys.argv) > 1:
        filepath = sys.argv[1]
        try:
            with open(filepath, 'r') as f:
                data = f.read()
            print(f"[*] Read {len(data)} characters from {filepath}")
        except FileNotFoundError:
            print(f"[!] File not found: {filepath}")
            sys.exit(1)
    elif not sys.stdin.isatty():
        data = sys.stdin.read()
        print(f"[*] Read {len(data)} characters from stdin")
    else:
        print("\n[*] Paste the encoded message below, then press Enter twice (empty line to finish):")
        lines = []
        try:
            while True:
                line = input()
                if line == '' and lines:
                    break
                lines.append(line)
        except EOFError:
            pass
        data = '\n'.join(lines)
        print(f"[*] Read {len(data)} characters")

    if not data.strip():
        print("[!] No input data provided.")
        sys.exit(1)

    flag = peel_layers(data)

    if flag:
        print(f"\n[+] Flag: {flag}")
    else:
        print("\n[*] Automatic decoding did not find the flag.")
        print("[*] Try CyberChef (https://gchq.github.io/CyberChef/) for interactive decoding.")
        print("[*] Tips:")
        print("    - Look for patterns: all hex chars? base64 padding (=)?")
        print("    - dots and dashes? Morse code.")
        print("    - 3-digit groups of 0-7? Octal.")
        print("    - Groups of 8 binary digits? Binary-to-ASCII.")
        print("    - Try ROT13 / ROT47 if nothing else works.")
        sys.exit(1)


if __name__ == "__main__":
    main()
