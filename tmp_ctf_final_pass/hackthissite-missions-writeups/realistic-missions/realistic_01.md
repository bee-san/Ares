# Realistic Challenge 1 – Uncle Arnold’s Local Band Review Page

**Objective:** understand and manipulate the voting system to submit a valid vote.

---

## Approach
1. Analyze URL parameters controlling the vote (`id`, `vote`, `PHPSESSID`).
2. Attempt to change the `vote` parameter to large values (e.g., 5000).
3. Observe server behavior on submitting different vote values.
4. Identify the maximum accepted vote value by incrementally testing.
5. Confirm votes above 999 are rejected silently (page reloads with no effect).

---

## Solution Steps
1. Access voting URL with modified vote parameter, e.g.:  
   `...?id=3&vote=999&PHPSESSID=...`
2. Submit votes with values ≤ 999 — accepted by the server.
3. Values > 999 cause no change (page reloads silently).
4. Use developer tools to monitor requests and confirm successful votes.

---

## Concept
Demonstrates **input parameter tampering** and the importance of **server-side validation** and constraints.

---

## Takeaways
- Always test boundaries for input parameters.
- Silent page reloads often indicate server-side input rejection without error messages.
- Session IDs may link votes to users but don’t guarantee vote acceptance.
- Never trust client-controlled inputs without proper server validation.
