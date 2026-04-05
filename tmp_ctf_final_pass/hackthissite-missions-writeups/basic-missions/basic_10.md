# Basic Mission 10 – Cookie Tampering

**Objective:** manipulate the cookie to gain access.

---

## Approach
1. Inspect cookies after page load.
2. Note `level_authorized = No`.
3. Use browser dev tools or JS injection to set `level_authorized = Yes`.
4. Reload to reveal the password.

---

## Solution Steps
1. Open DevTools → Storage → Cookies, set `level_authorized` to `Yes`.
2. Refresh → password page loads with: `PASSWORD_HERE`.

---

## Concept
Demonstrates **JavaScript/cookie tampering** to bypass client-side auth checks.

---

## Takeaways
- Never trust cookie or client-side values.
- Always verify authorization on the server.
