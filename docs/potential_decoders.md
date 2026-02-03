# Potential Decoders to Add

This document lists encodings, ciphers, and esoteric languages that Ciphey/Ares does not currently support but could be implemented.

## Currently Implemented

For reference, these decoders already exist:
- A1Z26, Atbash, Base32, Base58 (Bitcoin/Flickr/Monero/Ripple), Base64, Base65536, Base91
- Binary, Braille, Brainfuck, Caesar, Citrix CTX1, Hexadecimal, Morse Code
- Railfence, Reverse, ROT47, Substitution (generic), URL, Vigenere, Z85

---

## Encodings

### Base Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Base16** | Hex with padding/strict RFC compliance | Low | Hexadecimal may cover this |
| **Base36** | Alphanumeric encoding (0-9, A-Z) | Medium | Used in URL shorteners |
| **Base45** | QR-code optimized encoding | Medium | Used in EU COVID certificates |
| **Base62** | URL-safe base encoding | Medium | Common in short URLs |
| **Base85 (Ascii85)** | Adobe variant, different from Z85 | High | Very common in PDFs |
| **Base92** | High-density text encoding | Low | Rare usage |
| **Base100** | Emoji-based encoding | Low | Fun/CTF oriented |

### Legacy/System Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **UUEncode** | Unix-to-Unix encoding | High | Common in email attachments |
| **XXEncode** | Similar to UUEncode | Medium | Less common variant |
| **yEnc** | Binary-to-text for Usenet | Low | Niche usage |
| **BinHex** | Classic Mac encoding | Low | Legacy format |
| **Quoted-Printable** | Email encoding (MIME) | High | Very common in emails |
| **MIME Base64** | Base64 with line breaks | Medium | Email standard |

### Web/Programming Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **HTML Entities** | `&amp;`, `&#65;`, `&#x41;` | High | Extremely common |
| **Unicode Escapes** | `\u0041`, `\x41` | High | Common in code |
| **Punycode** | Internationalized domain names | Medium | IDN encoding |
| **Percent Encoding (extended)** | Full RFC 3986 compliance | Low | URL decoder may cover |
| **JSON String Escapes** | `\n`, `\t`, `\"` etc. | Medium | Common in APIs |
| **XML Entities** | Similar to HTML entities | Medium | Common in configs |

### Numeric Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Octal** | Base-8 encoding | Medium | Common in Unix permissions |
| **Decimal ASCII** | Space-separated decimal values | Medium | Simple encoding |
| **BCD** | Binary Coded Decimal | Low | Hardware-oriented |

---

## Classical Ciphers

### Substitution Ciphers

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **ROT13** | Caesar with shift 13 | High | Extremely common, quick win |
| **ROT5** | Digit rotation (0-9) | Medium | Often combined with ROT13 |
| **ROT18** | ROT13 + ROT5 combined | Medium | Letters and digits |
| **Affine Cipher** | `E(x) = (ax + b) mod 26` | High | Classic cipher |
| **Beaufort Cipher** | Variant of Vigenere | Medium | Similar to Vigenere |
| **Autokey Cipher** | Self-keying Vigenere variant | Medium | More secure Vigenere |
| **Porta Cipher** | Digraphic substitution | Low | Historical |
| **Gronsfeld Cipher** | Vigenere with numeric key | Low | Vigenere variant |

### Transposition Ciphers

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Columnar Transposition** | Column-based rearrangement | High | Very common in CTFs |
| **Double Columnar** | Two passes of columnar | Medium | More secure variant |
| **Route Cipher** | Grid-based path reading | Medium | Various route patterns |
| **Scytale** | Ancient Greek cipher | Low | Historical interest |

### Polygraphic Ciphers

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Playfair Cipher** | Digraph substitution | High | Classic CTF cipher |
| **Four-Square Cipher** | Extended Playfair | Medium | More secure |
| **Two-Square Cipher** | Simplified Four-Square | Low | Rare |
| **Bifid Cipher** | Polybius + transposition | Medium | Interesting hybrid |
| **Trifid Cipher** | 3D version of Bifid | Low | Complex |

### Military/Historical Ciphers

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **ADFGX/ADFGVX** | WWI German cipher | Medium | Historical significance |
| **Nihilist Cipher** | Russian nihilist cipher | Low | Historical |
| **VIC Cipher** | Cold War Soviet cipher | Low | Very complex |

---

## Symbol/Visual Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Pigpen Cipher** | Masonic/Freemason cipher | High | Very common in puzzles |
| **Templar Cipher** | Knights Templar variant | Low | Similar to Pigpen |
| **Dancing Men** | Sherlock Holmes cipher | Low | Fun/literary |
| **Semaphore** | Flag signaling system | Medium | Visual encoding |
| **Flag Codes** | Maritime signal flags | Low | Niche |
| **Tap Code** | Prison knock code | Medium | 5x5 Polybius grid |
| **Gold Bug Cipher** | Poe's cipher | Low | Literary reference |

---

## Phone/Keyboard Encodings

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **T9/Phone Keypad** | Multi-tap phone encoding | High | Common in CTFs |
| **DTMF Tones** | Touch-tone phone signals | Low | Audio-based |
| **Keyboard Shift** | QWERTY shifted typing | Medium | e.g., "jryyb" = "hello" |
| **Dvorak to QWERTY** | Keyboard layout conversion | Low | Rare |

---

## Esoteric Programming Languages

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Ook!** | Brainfuck variant | High | Direct BF translation |
| **Whitespace** | Spaces, tabs, newlines only | High | Hidden in plain text |
| **JSFuck** | JavaScript with 6 chars | Medium | Web CTF staple |
| **COW** | Brainfuck with "moo" | Low | Fun variant |
| **Piet** | Image-based language | Low | Requires image processing |
| **Befunge** | 2D stack-based language | Low | Complex to implement |
| **Malbolge** | Intentionally difficult | Low | Very hard to decode |
| **LOLCODE** | Internet meme language | Low | Novelty |

---

## Compression/Archive Formats (Detection)

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Gzip Header** | Detect gzip compressed data | Medium | Common wrapper |
| **Zlib Header** | Detect zlib compressed data | Medium | Common in protocols |
| **DEFLATE** | Raw deflate detection | Low | Usually wrapped |

---

## Hash-Related (Detection Only)

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **Hash Identification** | Identify hash type | High | Useful for CTFs |
| **bcrypt Detection** | Identify bcrypt hashes | Medium | Format: `$2a$...` |
| **JWT Decoder** | Decode JSON Web Tokens | High | Very common |

---

## Miscellaneous

| Name | Description | Priority | Notes |
|------|-------------|----------|-------|
| **NATO Phonetic** | Alpha, Bravo, Charlie... | Medium | Easy to implement |
| **Bacon Cipher** | Binary hidden in font styles | Medium | Steganographic |
| **Book Cipher** | References to book pages | Low | Requires book |
| **XOR (common keys)** | XOR with single byte keys | High | Brute-forceable |
| **Letter Number** | Multiple variants (A=0, A=1) | Low | A1Z26 may cover |
| **Polybius Square** | 5x5 grid coordinates | High | Base for many ciphers |

---

## Recommended Implementation Order

Based on frequency in CTFs and real-world usage:

### High Priority (Common in CTFs)
1. ROT13 (trivial, very common)
2. HTML Entities
3. Unicode Escapes
4. Playfair Cipher
5. Columnar Transposition
6. Ascii85/Base85
7. Polybius Square
8. XOR (single-byte brute force)
9. Quoted-Printable
10. JWT Decoder

### Medium Priority
1. Ook! (simple Brainfuck mapping)
2. Whitespace
3. T9/Phone Keypad
4. Affine Cipher
5. Pigpen Cipher
6. UUEncode
7. Tap Code
8. Base45
9. Keyboard Shift

### Lower Priority (Niche/Complex)
1. ADFGVX
2. Bifid/Trifid
3. Esoteric languages (Piet, Befunge, etc.)
4. Legacy encodings (BinHex, yEnc)
