# Encrypted Pastebin - Web Security Challenge Writeup

**Difficulty:** Easy  
**Category:** Web Security / Cryptography  
**Flags:** 3/4 (Partial completion)  
**Platform:** Hacker101 CTF

---

## 🎯 Challenge Overview

The Encrypted Pastebin challenge combines web security with cryptographic vulnerabilities. This application encrypts user-submitted content using AES-128 encryption, but implements it in a way that's vulnerable to padding oracle attacks - a sophisticated cryptographic exploit that allows attackers to decrypt data without knowing the encryption key.

---

## 🔍 Initial Reconnaissance

### Application Analysis
Upon first inspection, the pastebin application appeared to function like a standard note-sharing service:

1. **User Input**: Users can submit a title and message
2. **URL Generation**: After submission, the application generates a new URL
3. **Encrypted URLs**: The URL contains what appears to be encrypted/encoded data
4. **Content Retrieval**: The encrypted URL parameter is used to retrieve and decrypt the stored content

### Key Observations
- URLs contain encrypted strings after form submission
- Challenge hints mention **AES-128 bit encryption**
- The application likely encrypts the content and uses it as a URL parameter
- Potential vulnerability in how the encryption/decryption is handled

---

## 🔐 Understanding AES-128 and Block Ciphers

### AES-128 Basics
**Advanced Encryption Standard (AES)** with 128-bit keys:
- **Block Size**: 16 bytes (128 bits)
- **Key Size**: 16 bytes (128 bits)  
- **Mode**: Commonly CBC (Cipher Block Chaining)
- **Padding**: PKCS#7 padding to fill incomplete blocks

### How Block Cipher Padding Works
```
Original Data: "Hello World"     (11 bytes)
Block Size:    16 bytes required
Padding Added: "Hello World\x05\x05\x05\x05\x05"  (16 bytes total)
```

When decrypting, the system checks if padding is valid. **This validation check is what creates the vulnerability.**

---

## 🚩 Flag 1: Basic Encryption Manipulation

### Discovery Process
I started by testing how the application handles different inputs and observing the resulting encrypted URLs.

### Exploitation with Burp Suite
Using Burp Suite to intercept and modify requests:

1. **Intercepted Request**: Captured the form submission
2. **Modified Input**: Changed the encryped form input to a single character "d"
3. **Observed Response**: Analyzed how the encryption changed with minimal input
4. **Flag Discovery**: The manipulation revealed the first flag

### 🏁 First Flag Captured
```
^FLAG^1fd8fe727722f2c9baa7dfa876398ba8b76b1b7fe144c7473c6f159e60f4dabe$FLAG$
```

**Technique Used**: Basic input manipulation and observation of encryption behavior.

---

## 🔓 Understanding Padding Oracle Attacks

### What is a Padding Oracle Attack?

A **Padding Oracle Attack** is a cryptographic vulnerability that occurs when:

1. **Application uses block cipher** (like AES) with padding
2. **Error messages differ** between padding errors and other decryption errors  
3. **Attacker can repeatedly query** the system with modified ciphertext
4. **Oracle response** reveals whether padding is valid or invalid

### How It Works (Simplified)

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Attacker      │    │   Web App        │    │   Response      │
│                 │────│   (Oracle)       │────│                 │
│ Sends modified  │    │ Tries to decrypt │    │ "Padding Error" │
│ ciphertext      │    │ Reports padding  │    │ OR "Other Error"│
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### The Attack Process

1. **Byte-by-byte decryption**: Attack one byte at a time
2. **Padding manipulation**: Modify ciphertext to test padding validity  
3. **Oracle queries**: Send hundreds/thousands of requests
4. **Pattern analysis**: Valid padding responses reveal plaintext bytes
5. **Full decryption**: Reconstruct entire original message

### Why It's Dangerous
- **No key required**: Decrypt data without knowing the encryption key
- **Stealth attack**: Uses normal application functionality
- **Complete compromise**: Can decrypt any intercepted ciphertext

---

## 🛠️ Automated Exploitation Tool

### Research and Discovery
After understanding the padding oracle concept, I researched existing tools and found:

**Tool Used**: [eggburg's Hacker101 CTF Encrypted Pastebin Script](https://github.com/eggburg/hacker101_CTF_Encrypted_Pastebin)

### Script Analysis
The automated tool performs:
- **Systematic padding oracle attack**
- **Byte-by-byte decryption** of encrypted URLs
- **Flag extraction** from decrypted content
- **Multiple attack vectors** for different flag locations


**Note**: The script successfully exploited the padding oracle vulnerability but the final flag remained elusive, possibly due to:
- Different encryption context
- Additional security measures  
- Script limitations for specific edge cases

---

## 🔧 Technical Deep Dive

### Padding Oracle Attack Mechanics

```python
# Simplified attack logic
def padding_oracle_attack(ciphertext_blocks):
    plaintext = ""
    
    for block in ciphertext_blocks:
        for byte_position in range(16):  # AES block size
            for guess in range(256):     # All possible byte values
                modified_ciphertext = modify_byte(block, byte_position, guess)
                
                response = send_to_oracle(modified_ciphertext)
                
                if is_valid_padding(response):
                    # Found correct byte!
                    plaintext += chr(guess ^ padding_value ^ original_byte)
                    break
    
    return plaintext
```

### Vulnerability Root Cause
```python
# Vulnerable decryption code (conceptual)
try:
    decrypted = aes_decrypt(ciphertext)
    if not valid_padding(decrypted):
        return "Padding Error"        # ❌ Information leak!
    return process_content(decrypted)
except DecryptionError:
    return "Decryption Failed"        # ❌ Different error message!
```

---

## 🛡️ Security Recommendations

### Immediate Fixes

1. **Uniform Error Handling**
```python
# Secure approach - same error for all failures
try:
    decrypted = aes_decrypt(ciphertext)
    validate_padding(decrypted)
    return process_content(decrypted)
except (DecryptionError, PaddingError):
    return "Invalid request"  # ✅ Same error for all cases
```

2. **Authentication Before Decryption**
```python
# Use authenticated encryption (AES-GCM)
ciphertext, auth_tag = aes_gcm_encrypt(plaintext, key)
plaintext = aes_gcm_decrypt(ciphertext, auth_tag, key)  # ✅ Prevents tampering
```

3. **Rate Limiting**
```python
# Prevent automated oracle attacks
@rate_limit(max_requests=10, per_minute=1)
def decrypt_endpoint(request):
    # ... decryption logic
```

### Long-term Security Measures

- **Use authenticated encryption** modes (GCM, ChaCha20-Poly1305)
- **Implement proper error handling** that doesn't leak information
- **Add request rate limiting** to prevent automated attacks
- **Use secure random IDs** instead of encrypted content in URLs
- **Regular security audits** of cryptographic implementations

---

## 📚 Learning Resources

### Cryptographic Security
- 📖 [Cryptopals Challenges](https://cryptopals.com/) - Hands-on crypto vulnerabilities
- 📺 [LiveOverflow Crypto Videos](https://www.youtube.com/watch?v=4gE7YF9CaVE) - Padding oracle explanations
- 📚 [Applied Cryptography by Bruce Schneier](https://www.schneier.com/books/applied-cryptography/) - Comprehensive crypto reference

### Padding Oracle Attacks
- 🔬 [PadBuster Tool](https://github.com/AonCyberLabs/PadBuster) - Manual padding oracle exploitation
- 📖 [OWASP Padding Oracle](https://owasp.org/www-community/attacks/Padding_Oracle_Attack) - Attack methodology
- 🎓 [PortSwigger Academy](https://portswigger.net/web-security/logic-flaws) - Logic flaw exploitation

---

## 🎓 Key Takeaways

### Technical Skills Developed
1. **Cryptographic vulnerability analysis** - Understanding block cipher weaknesses
2. **Oracle attack methodology** - Systematic information extraction techniques  
3. **Automated exploitation** - Using existing tools for complex attacks
4. **Error message analysis** - Identifying information leakage vectors

### Security Lessons Learned
1. **Error handling matters** - Different error messages can reveal sensitive information
2. **Encryption ≠ Security** - Implementation flaws can completely undermine strong encryption
3. **Side-channel attacks** - Applications can leak information through behavior, not just data
4. **Defense in depth** - Multiple security layers prevent single-point failures

---

## 🔗 References and Credits

- **Primary Tool**: [eggburg's Hacker101 CTF Script](https://github.com/eggburg/hacker101_CTF_Encrypted_Pastebin) - Automated padding oracle exploitation
- **Burp Suite**: Manual request interception and modification
- **Hacker101 CTF**: Educational platform for hands-on security learning

---

## 🏆 Challenge Status

- **Completion**: 1/4 flags (25% complete)
- **Techniques Mastered**: 
  - Basic encryption manipulation ✅
  - Padding oracle attack theory ⏳ (partial) 
  - Automated exploitation tools ⏳ (partial)
  - Advanced cryptographic analysis ⏳ (partial)
- **Skills Demonstrated**: Cryptographic vulnerability assessment, tool utilization, systematic attack methodology

This challenge provided valuable hands-on experience with cryptographic vulnerabilities that exist at the intersection of web security and applied cryptography - skills highly valued in both penetration testing and security research roles.
