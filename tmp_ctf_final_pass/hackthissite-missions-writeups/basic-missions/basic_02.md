# Basic Mission 2 – HTML Form Field Tampering

**Objective:** Bypass a client-side form validation check.

---

## Approach

1. View the HTML source.
2. Notice a hidden form field or disabled input.
3. Modify it using DevTools and resubmit.

---

## Solution Steps

- Open DevTools → Elements.
- Change `name="password"` value directly in HTML.
- Submit with the correct value.

---

## Concept

This mission highlights the danger of trusting client-side validation in HTML.

---

## Takeaways

- HTML forms can be easily edited in-browser.
- Input validation must happen server-side.
