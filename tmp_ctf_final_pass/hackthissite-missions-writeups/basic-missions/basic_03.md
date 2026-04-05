# Basic Mission 3 – JavaScript Validation

**Objective:** Bypass JavaScript password verification.

---

## Approach

1. View the source and JavaScript code.
2. The password check is done entirely in JS.
3. Locate the password or force bypass.

---

## Solution Steps

- View page source or linked JS file.
- Find the `if (password == "correct")` condition.
- Submit using that value.

---

## Concept

Client-side JavaScript can be read and altered. Sensitive logic should never rely on it alone.

---

## Takeaways

- JavaScript code is fully visible to the user.
- Don’t store secrets or logic in client-side scripts.
