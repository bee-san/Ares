# StegoRSA - picoCTF 2026

**Category:** Cryptography
**Points:** 100

## Challenge Description
A message has been encrypted using RSA. The public key is gone... but someone might have been careless with the private key. (RSA with key hidden via steganography)

## Approach

This is a combination challenge involving two techniques:
1. **Steganography** -- The RSA private key is hidden inside an image file
2. **RSA decryption** -- Once the private key is recovered, use it to decrypt the ciphertext

### Step 1: Extract the Hidden Private Key

The challenge provides an image file (likely PNG or JPEG) and an encrypted ciphertext file. The RSA private key has been embedded in the image using steganography.

Common steganography techniques to check:
- **LSB (Least Significant Bit) encoding** in PNG images -- use `zsteg` to detect
- **Steghide** for JPEG images -- use `steghide extract -sf image.jpg`
- **Metadata/EXIF** -- check with `exiftool`
- **Appended data** -- use `binwalk` to find data appended after the image
- **String search** -- use `strings` to look for PEM-formatted key data

The private key will likely be in PEM format:
```
-----BEGIN RSA PRIVATE KEY-----
...base64-encoded key data...
-----END RSA PRIVATE KEY-----
```

### Step 2: Decrypt the Ciphertext

Once the private key is extracted, use it to decrypt the provided ciphertext. This can be done with:
- Python's `pycryptodome` library
- OpenSSL command line: `openssl rsautl -decrypt -inkey private.pem -in ciphertext.bin`
- Python's `cryptography` library

## Solution

1. Examine the provided image with steganography tools:
   - `zsteg image.png` (for PNG)
   - `steghide extract -sf image.jpg` (for JPEG)
   - `binwalk image.png` (check for embedded files)
   - `strings image.png | grep -i "BEGIN"` (look for PEM headers)
   - `exiftool image.png` (check metadata)
2. Extract the RSA private key from the image.
3. Save the key to a file (e.g., `private.pem`).
4. Decrypt the ciphertext using the private key.
5. The decrypted plaintext contains the flag.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
