# Password Profiler - picoCTF 2026

**Category:** General Skills
**Points:** 100

## Challenge Description
We intercepted a suspicious file from a system, but instead of the password itself, it only contains its SHA-1 hash. Using publicly available breach data, can you identify the original password?

## Approach
This challenge tests the ability to reverse a SHA-1 hash back to its plaintext password using publicly available breach databases. The concept is based on the real-world scenario where leaked password databases (from data breaches) can be used to look up hashes and recover the original passwords.

### Key Concepts

**SHA-1 Hashing**: SHA-1 (Secure Hash Algorithm 1) produces a 40-character hexadecimal digest. While SHA-1 is a one-way function (you cannot mathematically reverse it), weak or commonly-used passwords can be recovered through:
- **Rainbow tables**: Precomputed tables mapping hashes to plaintext
- **Breach databases**: Collections of passwords from real-world data breaches
- **Online lookup services**: Services like CrackStation, Have I Been Pwned (Pwned Passwords), and others

**Breach Data Lookup**: The description explicitly mentions "publicly available breach data," pointing to services like:
- **Have I Been Pwned - Pwned Passwords API**: Contains over 800 million compromised passwords, searchable by SHA-1 hash. The API uses a k-anonymity model where you send only the first 5 characters of the SHA-1 hash.
- **CrackStation**: An online hash lookup service with a massive precomputed database supporting MD5, SHA-1, SHA-256, and other hash types.
- **Hashcat / John the Ripper**: Offline cracking tools that can be used with wordlists from breach data.

### The Attack

1. Read the provided file to obtain the SHA-1 hash
2. Look up the hash in a breach database or hash lookup service
3. The plaintext password IS the flag (wrapped in `picoCTF{}` format)

## Solution

### Step 1: Obtain the SHA-1 Hash
Download and read the challenge file:
```bash
cat hash.txt
```
This file contains a SHA-1 hash (40 hex characters), for example:
```
5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8
```

### Step 2: Look Up the Hash Using Online Services

**Method A: CrackStation (easiest)**
1. Go to https://crackstation.net/
2. Paste the SHA-1 hash
3. Submit -- the plaintext password is returned

**Method B: Have I Been Pwned - Pwned Passwords API**
The HIBP API uses k-anonymity -- you send only the first 5 hex chars of the SHA-1 hash:
```bash
# If the hash is 5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8
# Send first 5 chars: 5BAA6
curl https://api.pwnedpasswords.com/range/5BAA6
```
This returns a list of hash suffixes. Search for your hash's suffix in the response.

**Method C: hashcat with breach wordlist**
```bash
hashcat -m 100 hash.txt /usr/share/wordlists/rockyou.txt
```

### Step 3: Construct the Flag
The recovered plaintext password is the flag content. Wrap it in the picoCTF flag format:
```
picoCTF{<recovered_password>}
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
