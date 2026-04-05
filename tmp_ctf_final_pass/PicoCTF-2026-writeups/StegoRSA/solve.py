#!/usr/bin/env python3
"""
StegoRSA - picoCTF 2026 (Cryptography, 100 pts)

The RSA private key is hidden inside an image via steganography.
We need to:
  1. Extract the private key from the image
  2. Use it to decrypt the ciphertext
  3. Recover the flag

Usage:
  python3 solve.py [image_file] [ciphertext_file]
  e.g.: python3 solve.py stego.png ciphertext.txt

If no files are given, the script searches the current directory.
"""

import sys
import os
import re
import subprocess
import base64
import argparse
import glob


def find_challenge_files():
    """Search current directory for image and ciphertext files."""
    images = []
    ciphertexts = []
    for f in os.listdir("."):
        ext = f.lower().split(".")[-1] if "." in f else ""
        if ext in ("png", "jpg", "jpeg", "bmp", "gif", "tiff"):
            images.append(f)
        elif ext in ("txt", "enc", "bin", "ct", "encrypted"):
            ciphertexts.append(f)
        elif f in ("ciphertext", "flag.enc", "message.enc", "encrypted"):
            ciphertexts.append(f)
    return images, ciphertexts


def extract_key_zsteg(image_file):
    """Try extracting hidden data using zsteg (PNG/BMP)."""
    print(f"[*] Trying zsteg on {image_file}...")
    try:
        result = subprocess.run(
            ["zsteg", image_file, "--all"],
            capture_output=True, text=True, timeout=30
        )
        output = result.stdout + result.stderr

        # Look for PEM key markers
        if "BEGIN" in output and "PRIVATE" in output:
            print("[+] zsteg found potential key data!")
            # Try to extract with specific channel
            for channel in ["b1,rgb,lsb,xy", "b1,r,lsb,xy", "b1,bgr,lsb,xy",
                            "b2,rgb,lsb,xy", "b1,rgba,lsb,xy"]:
                r2 = subprocess.run(
                    ["zsteg", image_file, "-E", channel],
                    capture_output=True, timeout=30
                )
                data = r2.stdout.decode(errors="ignore")
                if "BEGIN RSA PRIVATE KEY" in data or "BEGIN PRIVATE KEY" in data:
                    return extract_pem_key(data)

        # Also try default extraction
        result2 = subprocess.run(
            ["zsteg", image_file],
            capture_output=True, text=True, timeout=30
        )
        for line in result2.stdout.split("\n"):
            if "BEGIN" in line or "PRIVATE" in line:
                # Extract the channel info and use it
                parts = line.split(":")
                if len(parts) >= 2:
                    channel = parts[0].strip()
                    r3 = subprocess.run(
                        ["zsteg", image_file, "-E", channel],
                        capture_output=True, timeout=30
                    )
                    data = r3.stdout.decode(errors="ignore")
                    key = extract_pem_key(data)
                    if key:
                        return key

        return None
    except FileNotFoundError:
        print("[!] zsteg not installed (gem install zsteg)")
        return None
    except Exception as e:
        print(f"[!] zsteg error: {e}")
        return None


def extract_key_steghide(image_file, passphrase=""):
    """Try extracting hidden data using steghide (JPEG)."""
    print(f"[*] Trying steghide on {image_file}...")
    try:
        result = subprocess.run(
            ["steghide", "extract", "-sf", image_file, "-p", passphrase, "-f", "-xf", "-"],
            capture_output=True, timeout=30
        )
        data = result.stdout.decode(errors="ignore")
        if "BEGIN" in data and "PRIVATE" in data:
            print("[+] steghide found key data!")
            return extract_pem_key(data)
        return None
    except FileNotFoundError:
        print("[!] steghide not installed")
        return None
    except Exception as e:
        print(f"[!] steghide error: {e}")
        return None


def extract_key_strings(image_file):
    """Try finding PEM key data using strings."""
    print(f"[*] Searching strings in {image_file}...")
    try:
        result = subprocess.run(
            ["strings", "-n", "10", image_file],
            capture_output=True, text=True, timeout=30
        )
        return extract_pem_key(result.stdout)
    except Exception as e:
        print(f"[!] strings error: {e}")
        return None


def extract_key_binwalk(image_file):
    """Try finding embedded files using binwalk."""
    print(f"[*] Trying binwalk on {image_file}...")
    try:
        result = subprocess.run(
            ["binwalk", "--extract", "--directory=/tmp/stego_extract", image_file],
            capture_output=True, text=True, timeout=30
        )
        # Check extracted files for keys
        extract_dir = f"/tmp/stego_extract"
        if os.path.exists(extract_dir):
            for root, dirs, files in os.walk(extract_dir):
                for f in files:
                    filepath = os.path.join(root, f)
                    try:
                        with open(filepath, "r") as fh:
                            content = fh.read()
                            key = extract_pem_key(content)
                            if key:
                                print(f"[+] Found key in binwalk extract: {filepath}")
                                return key
                    except Exception:
                        pass
        return None
    except FileNotFoundError:
        print("[!] binwalk not installed")
        return None
    except Exception as e:
        print(f"[!] binwalk error: {e}")
        return None


def extract_key_exiftool(image_file):
    """Check image metadata for hidden key data."""
    print(f"[*] Checking EXIF metadata of {image_file}...")
    try:
        result = subprocess.run(
            ["exiftool", image_file],
            capture_output=True, text=True, timeout=30
        )
        return extract_pem_key(result.stdout)
    except FileNotFoundError:
        print("[!] exiftool not installed")
        return None
    except Exception as e:
        print(f"[!] exiftool error: {e}")
        return None


def extract_key_raw(image_file):
    """Read the raw file and search for PEM key data."""
    print(f"[*] Searching raw bytes of {image_file}...")
    with open(image_file, "rb") as f:
        data = f.read()

    text = data.decode(errors="ignore")
    return extract_pem_key(text)


def extract_pem_key(text):
    """Extract a PEM-formatted private key from text."""
    # Match RSA PRIVATE KEY or PRIVATE KEY
    patterns = [
        r"(-----BEGIN RSA PRIVATE KEY-----[\s\S]*?-----END RSA PRIVATE KEY-----)",
        r"(-----BEGIN PRIVATE KEY-----[\s\S]*?-----END PRIVATE KEY-----)",
        r"(-----BEGIN EC PRIVATE KEY-----[\s\S]*?-----END EC PRIVATE KEY-----)",
    ]
    for pattern in patterns:
        match = re.search(pattern, text)
        if match:
            key_pem = match.group(1)
            # Clean up: ensure proper line breaks
            key_pem = key_pem.replace("\\n", "\n")
            return key_pem
    return None


def decrypt_rsa_pycryptodome(key_pem, ciphertext_data):
    """Decrypt ciphertext using pycryptodome."""
    try:
        from Crypto.PublicKey import RSA
        from Crypto.Cipher import PKCS1_OAEP, PKCS1_v1_5

        key = RSA.import_key(key_pem)
        print(f"[*] RSA key: {key.size_in_bits()}-bit, n={str(key.n)[:40]}...")

        # Try PKCS1_OAEP first
        try:
            cipher = PKCS1_OAEP.new(key)
            plaintext = cipher.decrypt(ciphertext_data)
            return plaintext
        except (ValueError, TypeError):
            pass

        # Try PKCS1_v1_5
        try:
            cipher = PKCS1_v1_5.new(key)
            plaintext = cipher.decrypt(ciphertext_data, sentinel=b"DECRYPTION_FAILED")
            if plaintext != b"DECRYPTION_FAILED":
                return plaintext
        except (ValueError, TypeError):
            pass

        # Try raw/textbook RSA (no padding)
        try:
            c = int.from_bytes(ciphertext_data, "big")
            m = pow(c, key.d, key.n)
            plaintext = m.to_bytes((m.bit_length() + 7) // 8, "big")
            return plaintext
        except Exception:
            pass

        return None
    except ImportError:
        return None


def decrypt_rsa_openssl(key_pem, ciphertext_file):
    """Decrypt using OpenSSL command line."""
    key_file = "/tmp/stego_private.pem"
    with open(key_file, "w") as f:
        f.write(key_pem)

    # Try PKCS1 OAEP
    for padding in ["-oaep", "", "-raw"]:
        try:
            cmd = ["openssl", "rsautl", "-decrypt", "-inkey", key_file, "-in", ciphertext_file]
            if padding:
                cmd.append(padding)
            result = subprocess.run(cmd, capture_output=True, timeout=10)
            if result.returncode == 0 and result.stdout:
                return result.stdout
        except Exception:
            pass

    # Try with pkeyutl (newer OpenSSL)
    for padding in ["-pkeyopt", ""]:
        try:
            cmd = ["openssl", "pkeyutl", "-decrypt", "-inkey", key_file, "-in", ciphertext_file]
            result = subprocess.run(cmd, capture_output=True, timeout=10)
            if result.returncode == 0 and result.stdout:
                return result.stdout
        except Exception:
            pass

    os.unlink(key_file)
    return None


def decrypt_rsa_cryptography(key_pem, ciphertext_data):
    """Decrypt using the cryptography library."""
    try:
        from cryptography.hazmat.primitives import serialization
        from cryptography.hazmat.primitives.asymmetric import padding
        from cryptography.hazmat.primitives import hashes

        private_key = serialization.load_pem_private_key(
            key_pem.encode(), password=None
        )

        # Try OAEP
        try:
            plaintext = private_key.decrypt(
                ciphertext_data,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None
                )
            )
            return plaintext
        except Exception:
            pass

        # Try OAEP with SHA1
        try:
            plaintext = private_key.decrypt(
                ciphertext_data,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA1()),
                    algorithm=hashes.SHA1(),
                    label=None
                )
            )
            return plaintext
        except Exception:
            pass

        # Try PKCS1v15
        try:
            plaintext = private_key.decrypt(
                ciphertext_data,
                padding.PKCS1v15()
            )
            return plaintext
        except Exception:
            pass

        return None
    except ImportError:
        return None


def main():
    parser = argparse.ArgumentParser(description="StegoRSA solver - picoCTF 2026")
    parser.add_argument("image_file", nargs="?", help="Image file with hidden key")
    parser.add_argument("ciphertext_file", nargs="?", help="Encrypted ciphertext file")
    args = parser.parse_args()

    print("=" * 60)
    print("  StegoRSA - picoCTF 2026 Solver")
    print("=" * 60)
    print()

    # Find files if not specified
    image_file = args.image_file
    ciphertext_file = args.ciphertext_file

    if not image_file or not ciphertext_file:
        images, ciphertexts = find_challenge_files()
        if not image_file and images:
            image_file = images[0]
            print(f"[*] Auto-detected image: {image_file}")
        if not ciphertext_file and ciphertexts:
            ciphertext_file = ciphertexts[0]
            print(f"[*] Auto-detected ciphertext: {ciphertext_file}")

    if not image_file:
        print("[!] No image file found. Usage: python3 solve.py <image> <ciphertext>")
        sys.exit(1)

    if not ciphertext_file:
        print("[!] No ciphertext file found. Usage: python3 solve.py <image> <ciphertext>")
        sys.exit(1)

    # ---- Step 1: Extract private key from image ----
    print(f"\n[*] === Step 1: Extract Private Key from {image_file} ===\n")

    key_pem = None
    extractors = [
        extract_key_raw,          # Check raw bytes first (fast)
        extract_key_strings,      # strings command
        extract_key_exiftool,     # EXIF metadata
    ]

    # Add format-specific extractors
    ext = image_file.lower().split(".")[-1] if "." in image_file else ""
    if ext in ("png", "bmp"):
        extractors.insert(1, extract_key_zsteg)
    if ext in ("jpg", "jpeg"):
        extractors.insert(1, extract_key_steghide)

    extractors.append(extract_key_binwalk)  # binwalk last (creates files)

    for extractor in extractors:
        key_pem = extractor(image_file)
        if key_pem:
            print(f"\n[+] Private key extracted successfully!")
            print(f"[+] Key preview: {key_pem[:80]}...")
            break

    if not key_pem:
        print("[!] Could not extract private key from image")
        print("[!] Try manually with: zsteg, steghide, binwalk, or strings")
        sys.exit(1)

    # Save the extracted key
    with open("extracted_private.pem", "w") as f:
        f.write(key_pem)
    print("[*] Key saved to: extracted_private.pem")

    # ---- Step 2: Read and decrypt ciphertext ----
    print(f"\n[*] === Step 2: Decrypt {ciphertext_file} ===\n")

    with open(ciphertext_file, "rb") as f:
        ct_raw = f.read()

    # Check if ciphertext is base64-encoded
    try:
        ct_text = ct_raw.decode().strip()
        ct_data = base64.b64decode(ct_text)
        print(f"[*] Ciphertext appears to be base64-encoded ({len(ct_data)} bytes decoded)")
    except Exception:
        ct_data = ct_raw
        print(f"[*] Ciphertext is raw binary ({len(ct_data)} bytes)")

    # Also try if it might be hex-encoded
    try:
        ct_text = ct_raw.decode().strip()
        ct_hex = bytes.fromhex(ct_text)
        if len(ct_hex) > 0:
            print(f"[*] Also trying hex-decoded interpretation ({len(ct_hex)} bytes)")
    except Exception:
        ct_hex = None

    # Try decryption methods
    plaintext = None
    for ct in [ct_data, ct_hex] if ct_hex else [ct_data]:
        if ct is None:
            continue

        # Try pycryptodome
        plaintext = decrypt_rsa_pycryptodome(key_pem, ct)
        if plaintext:
            print("[+] Decrypted with pycryptodome!")
            break

        # Try cryptography library
        plaintext = decrypt_rsa_cryptography(key_pem, ct)
        if plaintext:
            print("[+] Decrypted with cryptography library!")
            break

    # Try OpenSSL as fallback
    if not plaintext:
        plaintext = decrypt_rsa_openssl(key_pem, ciphertext_file)
        if plaintext:
            print("[+] Decrypted with OpenSSL!")

    if plaintext:
        decoded = plaintext.decode(errors="replace")
        print(f"\n[+] Decrypted message: {decoded}")

        flag_match = re.search(r"picoCTF\{[^}]+\}", decoded)
        if flag_match:
            print(f"\n{'=' * 60}")
            print(f"  FLAG: {flag_match.group()}")
            print(f"{'=' * 60}")
        else:
            print("[*] Flag format not found in decrypted text")
            print(f"[*] Full decrypted output: {decoded}")
    else:
        print("[!] Could not decrypt the ciphertext")
        print("[!] The private key might need different handling")
        print("[!] Try: openssl rsautl -decrypt -inkey extracted_private.pem -in " + ciphertext_file)

    print("\n[*] Done!")


if __name__ == "__main__":
    main()
