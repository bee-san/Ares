# Credential Stuffing - picoCTF 2026

**Category:** Web Exploitation
**Points:** 100

## Challenge Description

Credential stuffing is the automated injection of stolen username and password pairs into website login forms.

## Approach

This challenge is about **credential stuffing** -- a real-world attack technique where an attacker uses lists of previously breached username/password pairs to attempt automated logins on a target website. The challenge likely provides:

1. A web application with a login form
2. A wordlist or credential dump file (or reference to a well-known one)
3. The goal: find the correct credentials that grant access to the flag

### Key Observations

- **100 points** with **1344 solves** means this is a straightforward challenge
- The description explicitly tells us the technique: inject stolen username/password pairs
- We need to automate login attempts with a credential list

### Attack Strategy

1. **Enumerate the login form**: Identify the login endpoint, HTTP method (POST), and required parameters (e.g., `username`, `password`)
2. **Obtain the credential list**: The challenge likely provides a file with username:password pairs, or hints at using a common wordlist
3. **Automate the attack**: Use Python with the `requests` library to iterate through credential pairs, submitting each to the login form
4. **Detect success**: Check the HTTP response for indicators of successful login (e.g., redirect, different status code, presence of "flag" or "picoCTF" in the response body, absence of "Invalid" or "Incorrect" error messages)

### Common picoCTF Web Challenge Patterns

- Login forms that accept POST requests with `username` and `password` fields
- Success indicated by a 302 redirect or a response containing the flag
- Credential lists provided as downloadable files on the challenge page
- Sometimes cookies need to be maintained across requests (session handling)

## Solution

### Step-by-step:

1. **Visit the challenge URL** and inspect the login form (view source, check network tab in browser dev tools).
2. **Identify the login endpoint**: Usually `/login`, `/api/login`, or the form's `action` attribute.
3. **Download the credential list**: Check the challenge description or page for downloadable files.
4. **Run the solve script**: Automate credential testing with Python requests.
5. **Extract the flag** from the successful login response.

### Manual Verification

Before scripting, try a few manual logins to understand:
- What does a failed login look like? (e.g., "Invalid credentials", HTTP 401)
- What parameters does the form submit?
- Are there any anti-CSRF tokens or cookies required?

## Solution Script

```
python3 solve.py
```

## Flag

```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
