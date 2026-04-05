# Secret Box - picoCTF 2026

**Category:** Web Exploitation
**Points:** 200

## Challenge Description
This secret box is designed to conceal your secrets. It's perfectly secure -- only you can see what's inside. Or can you? Try to uncover secrets that don't belong to you.

## Approach
This challenge presents a web application that allows users to store and retrieve "secrets" in their personal box. The description strongly hints at an **Insecure Direct Object Reference (IDOR)** vulnerability -- the claim that "only you can see what's inside" combined with the prompt to "uncover secrets that don't belong to you" is a classic IDOR setup.

### Vulnerability Analysis

**IDOR (Insecure Direct Object Reference)** occurs when an application exposes internal object references (such as database IDs, sequential numbers, or predictable tokens) in a way that allows an attacker to manipulate them and access other users' data without proper authorization checks.

In this challenge, the web application likely:
1. Assigns each user a numeric or sequential identifier (e.g., user ID, box ID, or secret ID)
2. Uses this identifier in API requests or URL parameters to retrieve secrets
3. Fails to verify that the requesting user is authorized to access the requested resource

### Attack Surface

Common IDOR patterns in web CTF challenges:
- **URL path parameters**: `/api/secrets/1`, `/api/secrets/2`, etc.
- **Query parameters**: `/view?id=1`, `/view?secret_id=1`
- **Cookie values**: A cookie like `user_id=5` or `box_id=abc123`
- **POST body parameters**: `{"secret_id": 1}` in JSON requests
- **HTTP headers**: Custom headers carrying user/session identifiers

The flag is typically stored in another user's secret box (often user ID 0, 1, or an admin account) that can be accessed by modifying the reference.

## Solution

### Step 1: Create an Account and Store a Secret
Navigate to the challenge URL and interact with the application normally:
- Register or log in (if registration is available)
- Store a test secret in your box
- Observe the resulting requests in your browser's Developer Tools (Network tab)

### Step 2: Identify the Object Reference
Examine the HTTP requests when viewing your secret. Look for:
- The URL structure (e.g., `/api/box/7` or `/secret?id=42`)
- Cookie values that contain identifiers
- Request/response bodies with ID fields

For example, you might see a request like:
```
GET /api/secret?id=42 HTTP/1.1
Cookie: session=<your_token>
```

Or the secret might be fetched via:
```
GET /api/box/7
```

### Step 3: Enumerate Other Users' Secrets
Modify the identifier to access other users' secrets. Iterate through possible IDs:
```
GET /api/secret?id=1
GET /api/secret?id=2
...
GET /api/secret?id=0
GET /api/secret?id=admin
```

### Step 4: Find the Flag
One of the secrets belonging to another user will contain the flag in the format `picoCTF{...}`. This is typically at a low ID number (ID 0 or 1 for the first/admin user) or may require iterating through several IDs.

### Alternative: Cookie/Session Manipulation
If the IDOR is cookie-based, you may need to:
1. Decode your session cookie (often base64 or JWT)
2. Modify the user ID field within it
3. Re-encode and send the modified cookie

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
