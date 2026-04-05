# Fool the Lockout - picoCTF 2026

**Category:** Web Exploitation
**Points:** 200

## Challenge Description

Your friend is building a simple website with a login page. To stop brute forcing and credential stuffing, they've added rate limiting.

## Approach

This challenge implements **IP-based rate limiting** on a login form to prevent brute-force attacks. After a certain number of failed login attempts (typically 3-5), the server locks out the IP address for a period of time.

### The Vulnerability

The rate limiter determines the client's IP address from **user-controlled HTTP headers** rather than the actual TCP connection source. Specifically, it trusts the `X-Forwarded-For` header, which is commonly used by reverse proxies to pass along the original client IP. Since this header can be freely set by the client, an attacker can spoof a different IP address on each request to bypass the per-IP rate limit.

This is a well-known misconfiguration in web applications that rely on proxy headers for rate limiting without proper validation. The server should only trust `X-Forwarded-For` from known proxy IPs, but in this challenge it blindly trusts any value.

### Attack Strategy

1. **Identify the rate limit**: Send a few login requests and observe when the lockout kicks in
2. **Bypass via header spoofing**: Add an `X-Forwarded-For` header with a random/unique IP on each request
3. **Brute-force credentials**: With the rate limit bypassed, enumerate passwords from a common wordlist
4. **Obtain the flag**: Successfully log in with the correct credentials to reveal the flag

### Common Credentials

picoCTF web challenges often use simple credentials. The username is typically hinted at in the challenge (e.g., `admin`), and the password is in a common wordlist. The challenge may provide a password list, or you may use a standard list like `rockyou.txt` or a smaller curated set.

## Solution

### Step 1: Explore the Login Page

Visit the challenge URL and inspect the login form. Note:
- The form fields (usually `username` and `password`)
- The endpoint it POSTs to (e.g., `/login` or `/api/login`)
- Any error messages that indicate rate limiting ("Too many attempts", "Locked out", etc.)

### Step 2: Identify Rate Limiting Behavior

```bash
# Send a few requests to trigger the rate limit
for i in $(seq 1 5); do
    curl -s -X POST http://CHALLENGE_URL/login \
        -d "username=admin&password=wrong$i" | grep -i "lock\|rate\|too many"
done
```

You should see a lockout message after ~3-5 attempts.

### Step 3: Bypass with X-Forwarded-For

```bash
# This request bypasses the rate limit by spoofing a different IP
curl -s -X POST http://CHALLENGE_URL/login \
    -H "X-Forwarded-For: 10.0.0.99" \
    -d "username=admin&password=test"
```

Each unique `X-Forwarded-For` value resets the rate limit counter.

### Step 4: Brute-Force the Password

Use the solve script to iterate through passwords with a rotating `X-Forwarded-For` header. For each login attempt, generate a random IP to avoid hitting the rate limit.

### Step 5: Log in and Get the Flag

Once the correct password is found, the server response will contain the flag or redirect to a page with the flag.

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
