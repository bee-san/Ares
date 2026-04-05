# No FA - picoCTF 2026

**Category:** Web Exploitation
**Points:** 200

## Challenge Description
Seems like some data has been leaked! Can you get the flag? (Likely involves bypassing or exploiting a broken 2FA/authentication mechanism)

## Approach
The challenge name "No FA" is a play on "No 2FA" (no two-factor authentication), hinting that the application's multi-factor authentication mechanism is either missing, broken, or bypassable. This is a classic web exploitation pattern seen in many CTF challenges.

### Vulnerability Analysis

The core vulnerability in challenges like this typically falls into one of these categories:

1. **Missing server-side OTP validation**: The application presents a 2FA/OTP form to the user but never actually validates the submitted value on the server side. Removing or blanking the OTP parameter causes the server to skip validation entirely.

2. **Client-side only enforcement**: The 2FA page exists purely as a client-side gate. The actual flag/data endpoint does not check whether 2FA was completed -- you can navigate directly to the protected resource.

3. **Parameter tampering**: The OTP field can be removed from the POST request altogether, and the server accepts the request without it because it only checks `if otp_param: validate(otp)` rather than requiring the field's presence.

4. **Leaked credentials/data in source**: Given the description mentions "data has been leaked," the flag or authentication tokens may be visible in:
   - HTML source comments
   - JavaScript files
   - API responses
   - HTTP headers
   - Cookies

5. **Predictable/static OTP**: The OTP might be hardcoded, always "000000", or derivable from public information.

### Attack Strategy

The most effective approach is to:
1. Register/login to the application normally
2. When presented with the 2FA/OTP page, intercept the request
3. Remove or modify the OTP parameter
4. Observe the server response -- it may return the flag directly
5. Also check page source, cookies, and response headers for leaked data

## Solution

### Step 1: Access the application
Navigate to the challenge URL in your browser and observe the application flow.

### Step 2: Register or login
If the application has a registration form, create an account with any credentials. The registration typically accepts anything.

```
Username: test
Password: test123
```

### Step 3: Reach the 2FA page
After login/registration, you should be redirected to a 2FA/OTP verification page.

### Step 4: Intercept the request with Burp Suite (or similar proxy)
1. Configure your browser to use Burp Suite as a proxy
2. Enter any value in the OTP field and click submit
3. Intercept the POST request in Burp Suite

The request might look like:
```http
POST /verify-2fa HTTP/1.1
Host: challenge-url.picoctf.org
Content-Type: application/x-www-form-urlencoded

otp=123456
```

### Step 5: Bypass the OTP
**Method A -- Remove the OTP parameter entirely:**
```http
POST /verify-2fa HTTP/1.1
Host: challenge-url.picoctf.org
Content-Type: application/x-www-form-urlencoded

```
(Empty body, or remove just the `otp=` value)

**Method B -- Send an empty OTP value:**
```http
POST /verify-2fa HTTP/1.1
Host: challenge-url.picoctf.org
Content-Type: application/x-www-form-urlencoded

otp=
```

**Method C -- Change Content-Type to JSON:**
```http
POST /verify-2fa HTTP/1.1
Host: challenge-url.picoctf.org
Content-Type: application/json
Accept: application/json

{}
```

**Method D -- Navigate directly to the flag endpoint:**
Try accessing common endpoints directly without going through 2FA:
```
/flag
/dashboard
/home
/secret
/api/flag
```

### Step 6: Check for leaked data
Inspect the page source, JavaScript files, and network responses for any leaked information:
```bash
curl -s <challenge_url> | grep -i "picoCTF\|flag\|secret\|password\|token"
```

### Step 7: Retrieve the flag
The server response after a successful bypass should contain the flag:
```
picoCTF{...}
```

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
