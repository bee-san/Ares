# Old Sessions - picoCTF 2026

**Category:** Web Exploitation
**Points:** 100

## Challenge Description

Proper session timeout controls are critical for securing user accounts. If a user logs in on a public or shared computer and leaves without logging out, an attacker can reuse the session.

## Approach

### Understanding the Vulnerability

This challenge demonstrates the **Session Fixation / Session Reuse** vulnerability. The web application fails to properly invalidate old session tokens, allowing an attacker to:

1. Find or obtain an old/expired session token (e.g., from browser history, logs, or a cookie jar).
2. Reuse that token to gain access to the user's account without authentication.

The core issue is **improper session timeout controls** -- the server never expires old sessions, or the expiration check is broken, allowing old session cookies to remain valid indefinitely.

### Common Patterns in picoCTF Session Challenges

Based on similar picoCTF challenges, this likely involves one or more of the following:

1. **Flask Session Cookies** -- The app uses Flask's signed cookies. Old sessions are still valid because the server does not track session expiry server-side.
2. **JWT Tokens** -- The app uses JSON Web Tokens with a weak or guessable secret, or the expiration (`exp`) claim is not validated.
3. **Session Cookie Tampering** -- The session cookie contains user information (like `username` or `admin` status) that can be modified.
4. **Provided Old Session Data** -- The challenge gives you access to an old session token (perhaps via a backup file, log file, or Git history) that still works.

### Reconnaissance

Key things to look for:
- A login page that sets a session cookie
- Source code (often provided) that shows how sessions are managed
- Hidden files like `.git/`, backup files, or log files containing old tokens
- The cookie format (Flask signed cookie, JWT, simple base64, etc.)

## Solution

### Step 1: Explore the Application

Visit the challenge URL and inspect the application:
- Check the login page
- Register or note any provided credentials
- Inspect cookies set by the application

### Step 2: Identify Old Session Data

Look for old session tokens. Common locations:
- A `/sessions/` or `/backup/` directory
- Source code comments or configuration files
- A downloadable file provided with the challenge
- Response headers or hidden form fields

### Step 3: Analyze the Session Format

Decode the session cookie:
- **Flask**: Base64-decode the cookie (format: `payload.timestamp.signature`). Use `flask-unsign` to decode.
- **JWT**: Decode at jwt.io or with `pyjwt`. Check for weak secrets or `alg: none` vulnerability.
- **Simple cookie**: May be plain base64 or a simple encoding.

### Step 4: Reuse or Forge the Session

Depending on the vulnerability:

**If old sessions are provided:**
- Simply set the old session cookie in your browser and access the protected page.

**If you need to forge a session:**
- Brute-force the Flask secret key using `flask-unsign --unsign --cookie <cookie> --wordlist <wordlist>`
- Once the secret is known, forge a new cookie: `flask-unsign --sign --cookie "{'user': 'admin'}" --secret <key>`

**If JWT with weak validation:**
- Modify the payload (e.g., change `user` to `admin`)
- Re-sign with the discovered secret or exploit `alg: none`

### Step 5: Access the Flag

With the valid (old or forged) session cookie set, navigate to the protected endpoint (e.g., `/flag`, `/dashboard`, `/admin`) to retrieve the flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
