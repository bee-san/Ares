# Hashgate - picoCTF 2026

**Category:** Web Exploitation
**Points:** 100

## Challenge Description

You have gotten access to an organisation's portal. Submit your email and password, and it redirects you to your profile. The application uses hashing for authentication -- can you bypass it?

## Approach

This is a web exploitation challenge where an authentication system uses hashing in a vulnerable way. The description tells us the app takes an email and password, hashes them for authentication, and redirects to a profile page. At 100 points with 1371 solves, this is an introductory web challenge with a well-known vulnerability class.

### Likely Vulnerability: Hash-Based Authentication Bypass

Several common vulnerabilities exist when applications use hashing for authentication:

#### 1. PHP Type Juggling / Magic Hashes
If the application is written in PHP and uses loose comparison (`==`) to compare hash values, it is vulnerable to "magic hash" attacks:
- In PHP, strings matching `0+e[0-9]+` are interpreted as zero in scientific notation
- If `md5("some_input")` produces a hash like `0e462097431906509019562988736854`, PHP's loose comparison treats it as `0`
- Two different inputs whose hashes both start with `0e` followed by only digits will compare as equal

Known magic hash inputs for MD5:
- `240610708` -> `0e462097431906509019562988736854`
- `QNKCDZO` -> `0e830400451993494058024219903391`
- `aabg7XSs` -> `0e087386482136013740957780965295`

#### 2. Hash Length Extension Attack
If the server constructs authentication tokens like `hash(secret + user_data)`, hash length extension attacks allow forging valid tokens without knowing the secret.

#### 3. Predictable or Weak Hashing
The application might use a simple or reversible hash that can be cracked or predicted.

#### 4. Hash Comparison Bypass via Array Injection
In PHP, passing an array instead of a string to hash functions causes them to return `NULL`:
- `md5(array()) === md5(array())` -> `NULL === NULL` -> `true`
- Send `email[]=foo&password[]=bar` to trigger array processing

#### 5. Always-True Comparison
The application might compare the hash of user input against a stored hash incorrectly:
```php
// Vulnerable: hash of empty string might match
if (md5($input) == $stored_hash) { ... }
```

### Analysis Strategy

1. Visit the web application and examine the login form
2. View page source and look for client-side hashing, comments, or hints
3. Intercept requests with Burp Suite or browser dev tools
4. Identify the hashing algorithm and comparison method
5. Try type juggling, array injection, and other bypass techniques

## Solution

### Step 1: Reconnaissance
```
Visit the challenge URL and examine:
- The login form HTML source
- JavaScript files (client-side hashing?)
- Network requests when submitting the form
- HTTP response headers
- Cookies set after login
```

### Step 2: Examine the source code / client-side logic
```
View page source (Ctrl+U) and look for:
- JavaScript that hashes the password before sending
- Comments revealing the hash algorithm or comparison method
- Hidden form fields or API endpoints
- References to PHP, Flask, or other backend technologies
```

### Step 3: Test for type juggling (PHP)
If the backend is PHP, try magic hash values:
```
Email: 240610708
Password: QNKCDZO

# Or any pair of values whose MD5 hashes are both "magic" (0e... with only digits)
```

### Step 4: Test for array injection
```
# Modify the POST request to send arrays instead of strings:
POST /login
Content-Type: application/x-www-form-urlencoded

email[]=admin&password[]=anything

# This causes md5(array) to return NULL, bypassing the comparison
```

### Step 5: Test for hash comparison bypass
```
# If the app compares: hash(user_input) == stored_hash
# Try inputs whose hashes are known magic hashes
# Or try: password=0 (if compared loosely against a 0e... hash)
```

### Step 6: Inspect the profile page
Once authenticated (via any bypass), the profile page should contain the flag. Check:
- The page content
- Cookies
- Response headers
- Redirected URL parameters

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
